use core::convert::From;
use core::f32;
use core::u32;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use arrayvec::ArrayVec;

use crate::control::Target;
use crate::control::TARGET_BUFFER_SIZE;
use crate::mouse::MAX_MSGS;

pub trait ReadExact {
    type Error;
    fn peek(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;
    fn take(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;
}

pub trait WriteExact {
    type Error;
    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
}

#[derive(FromPrimitive, Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum MsgId {
    // Core
    Time = 0x00,
    Logged = 0x01,
    Provided = 0x02,

    // Raw in/out
    LeftPos = 0x10,
    RightPos = 0x11,
    LeftPower = 0x12,
    RightPower = 0x13,
    Battery = 0x14,
    LeftDistance = 0x15,
    RightDistance = 0x16,
    FrontDistance = 0x17,

    // Calculated
    LinearPos = 0x20,
    AngularPos = 0x21,
    LinearPower = 0x22,
    AngularPower = 0x23,
    LinearSet = 0x24,
    AngularSet = 0x25,
    AddLinear = 0x26,
    AddAngular = 0x27,
    LinearTarget = 0x28,
    AngularTarget = 0x29,
    LinearBuffer = 0x2a,
    AngularBuffer = 0x2c,

    // Config
    LinearP = 0xa0,
    LinearI = 0xa1,
    LinearD = 0xa2,
    LinearAcc = 0xa3,
    AngularP = 0xa4,
    AngularI = 0xa5,
    AngularD = 0xa6,
    AngularAcc = 0xa7,

    // Bluetooth AT commands
    At = 0x2b,
}

#[derive(Debug)]
pub enum Msg {
    // Core
    Time(f32),
    Logged(ArrayVec<[MsgId; MAX_MSGS]>),
    Provided(ArrayVec<[MsgId; MAX_MSGS]>),

    // Raw in/out
    LeftPos(f32),
    RightPos(f32),
    LeftPower(f32),
    RightPower(f32),
    Battery(f32),
    LeftDistance(u8),
    RightDistance(u8),
    FrontDistance(u8),

    // Calculated
    LinearPos(f32),
    AngularPos(f32),
    LinearPower(f32),
    AngularPower(f32),
    LinearSet(f32),
    AngularSet(f32),
    AddLinear(Target),
    AddAngular(Target),
    LinearTarget(Target),
    AngularTarget(Target),
    LinearBuffer(ArrayVec<[Target; TARGET_BUFFER_SIZE]>),
    AngularBuffer(ArrayVec<[Target; TARGET_BUFFER_SIZE]>),

    // Config
    LinearP(f32),
    LinearI(f32),
    LinearD(f32),
    LinearAcc(f32),
    AngularP(f32),
    AngularI(f32),
    AngularD(f32),
    AngularAcc(f32),

    // Bluetooth AT commands
    At(ArrayVec<[u8; 64]>),
}

#[derive(Debug)]
pub enum ParseError<E> {
    UnknownMsg(u8),
    ReadExact(E),
}

impl<E> From<E> for ParseError<E> {
    fn from(err: E) -> ParseError<E> {
        ParseError::ReadExact(err)
    }
}

#[allow(dead_code)]
fn parse_id<R: ReadExact<Error = E>, E>(buf: &mut R, msg: Msg) -> Result<Msg, ParseError<E>> {
    let mut msgbuf = [0; 1];
    buf.take(&mut msgbuf)?;
    Ok(msg)
}

fn parse_u8<R: ReadExact<Error = E>, E>(
    buf: &mut R,
    msg: fn(u8) -> Msg,
) -> Result<Msg, ParseError<E>> {
    let mut msgbuf = [0; 2];
    buf.take(&mut msgbuf)?;
    let [_, a1] = msgbuf;
    Ok(msg(a1))
}

#[allow(dead_code)]
fn parse_u32<R: ReadExact<Error = E>, E>(
    buf: &mut R,
    msg: fn(u32) -> Msg,
) -> Result<Msg, ParseError<E>> {
    let mut msgbuf = [0; 5];
    buf.take(&mut msgbuf)?;
    let [_, a1, a2, a3, a4] = msgbuf;
    Ok(msg(u32::from_le_bytes([a1, a2, a3, a4])))
}

fn parse_f32<R: ReadExact<Error = E>, E>(
    buf: &mut R,
    msg: fn(f32) -> Msg,
) -> Result<Msg, ParseError<E>> {
    let mut msgbuf = [0; 5];
    buf.take(&mut msgbuf)?;
    let [_, a1, a2, a3, a4] = msgbuf;
    Ok(msg(f32::from_bits(u32::from_le_bytes([a1, a2, a3, a4]))))
}

#[allow(dead_code)]
fn parse_2f32<R: ReadExact<Error = E>, E>(
    buf: &mut R,
    msg: fn(f32, f32) -> Msg,
) -> Result<Msg, ParseError<E>> {
    let mut msgbuf = [0; 9];
    buf.take(&mut msgbuf)?;
    let [_, a1, a2, a3, a4, b1, b2, b3, b4] = msgbuf;
    Ok(msg(
        f32::from_bits(u32::from_le_bytes([a1, a2, a3, a4])),
        f32::from_bits(u32::from_le_bytes([b1, b2, b3, b4])),
    ))
}

fn parse_msgids<R: ReadExact<Error = E>, E>(
    buf: &mut R,
    msg: fn(ArrayVec<[MsgId; MAX_MSGS]>) -> Msg,
) -> Result<Msg, ParseError<E>> {
    // attept to get the length
    let mut lenbuf = [0; 2];
    buf.peek(&mut lenbuf)?;
    let [_id, len] = lenbuf;

    let msgbuf = &mut [0; MAX_MSGS + 2][0..(len + 2) as usize];
    buf.take(msgbuf)?;

    let msgids: ArrayVec<[MsgId; MAX_MSGS]> = msgbuf
        .into_iter()
        .skip(2)
        .filter_map(|&mut m| MsgId::from_u8(m))
        .collect();

    Ok(msg(msgids))
}

fn parse_target<R: ReadExact<Error = E>, E>(
    buf: &mut R,
    msg: fn(Target) -> Msg,
) -> Result<Msg, ParseError<E>> {
    let mut msgbuf = [0; 9];
    buf.take(&mut msgbuf)?;
    let [_, a1, a2, a3, a4, b1, b2, b3, b4] = msgbuf;
    Ok(msg(Target {
        velocity: f32::from_bits(u32::from_le_bytes([a1, a2, a3, a4])),
        distance: f32::from_bits(u32::from_le_bytes([b1, b2, b3, b4])),
    }))
}

fn parse_targets<R: ReadExact<Error = E>, E>(
    buf: &mut R,
    msg: fn(ArrayVec<[Target; TARGET_BUFFER_SIZE]>) -> Msg,
) -> Result<Msg, ParseError<E>> {
    // attept to get the length
    let mut lenbuf = [0; 2];
    buf.peek(&mut lenbuf)?;
    let [_id, len] = lenbuf;

    let targetbuf = &mut [0; TARGET_BUFFER_SIZE * 8 + 2][0..(len * 8 + 2) as usize];
    buf.take(targetbuf)?;
    let targetbuf: ArrayVec<[u8; TARGET_BUFFER_SIZE * 8]> =
        targetbuf.into_iter().skip(2).map(|&mut b| b).collect();

    let targets = targetbuf
        .chunks(8)
        .map(|buf| Target {
            velocity: f32::from_bits(u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]])),
            distance: f32::from_bits(u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]])),
        })
        .collect();

    Ok(msg(targets))
}

fn parse_line<R: ReadExact<Error = E>, E>(
    buf: &mut R,
    msg: fn(ArrayVec<[u8; 64]>) -> Msg,
) -> Result<Msg, ParseError<E>> {
    let mut atbuf = ArrayVec::new();
    let mut inbuf = [0; 1];
    while inbuf[0] != 0x0a {
        buf.take(&mut inbuf)?;
        atbuf.try_push(inbuf[0]);
    }

    Ok(msg(atbuf))
}

#[allow(dead_code)]
fn write_id<W: WriteExact<Error = E>, E>(buf: &mut W, msgid: MsgId) -> Result<(), E> {
    buf.write(&[msgid as u8])
}

fn write_u8<W: WriteExact<Error = E>, E>(buf: &mut W, msgid: MsgId, msg: u8) -> Result<(), E> {
    buf.write(&[msgid as u8, msg])
}

#[allow(dead_code)]
fn write_u32<W: WriteExact<Error = E>, E>(buf: &mut W, msgid: MsgId, msg: u32) -> Result<(), E> {
    let [a1, a2, a3, a4] = u32::to_le_bytes(msg);
    buf.write(&[msgid as u8, a1, a2, a3, a4])
}

fn write_f32<W: WriteExact<Error = E>, E>(buf: &mut W, msgid: MsgId, msg: f32) -> Result<(), E> {
    let [a1, a2, a3, a4] = u32::to_le_bytes(f32::to_bits(msg));
    buf.write(&[msgid as u8, a1, a2, a3, a4])
}

#[allow(dead_code)]
fn write_2f32<W: WriteExact<Error = E>, E>(
    buf: &mut W,
    msgid: MsgId,
    msg1: f32,
    msg2: f32,
) -> Result<(), E> {
    let [a1, a2, a3, a4] = u32::to_le_bytes(f32::to_bits(msg1));
    let [b1, b2, b3, b4] = u32::to_le_bytes(f32::to_bits(msg2));
    buf.write(&[msgid as u8, a1, a2, a3, a4, b1, b2, b3, b4])
}

fn write_msgids<W: WriteExact<Error = E>, E>(
    buf: &mut W,
    msgid: MsgId,
    msgids: &[MsgId],
) -> Result<(), E> {
    let bytes: ArrayVec<[u8; MAX_MSGS + 2]> = [msgid as u8]
        .into_iter()
        .map(|&m| m)
        .chain([MAX_MSGS.min(msgids.len()) as u8].into_iter().map(|&l| l))
        .chain(msgids.into_iter().map(|&m| m as u8))
        .collect();
    buf.write(&bytes)
}

fn write_target<W: WriteExact<Error = E>, E>(
    buf: &mut W,
    msgid: MsgId,
    msg: Target,
) -> Result<(), E> {
    let [a1, a2, a3, a4] = u32::to_le_bytes(f32::to_bits(msg.velocity));
    let [b1, b2, b3, b4] = u32::to_le_bytes(f32::to_bits(msg.distance));
    buf.write(&[msgid as u8, a1, a2, a3, a4, b1, b2, b3, b4])
}

fn write_targets<W: WriteExact<Error = E>, E>(
    buf: &mut W,
    msgid: MsgId,
    targets: &[Target],
) -> Result<(), E> {
    let bytes: ArrayVec<[u8; 768]> = [msgid as u8]
        .into_iter()
        .map(|&m| m)
        .chain([MAX_MSGS.min(targets.len()) as u8].into_iter().map(|&l| l))
        .chain(targets.into_iter().flat_map(|t| {
            ArrayVec::from(u32::to_le_bytes(f32::to_bits(t.velocity)))
                .into_iter()
                .chain(ArrayVec::from(u32::to_le_bytes(f32::to_bits(t.distance))).into_iter())
        }))
        .collect();
    buf.write(&bytes)
}

impl Msg {
    pub fn parse_bytes<R: ReadExact<Error = E>, E>(buf: &mut R) -> Result<Self, ParseError<E>> {
        let mut id = [0; 1];
        buf.peek(&mut id)?;

        match MsgId::from_u8(id[0]) {
            Some(MsgId::Time) => parse_f32(buf, Msg::Time),
            Some(MsgId::Logged) => parse_msgids(buf, Msg::Logged),
            Some(MsgId::Provided) => parse_msgids(buf, Msg::Provided),

            Some(MsgId::LeftPos) => parse_f32(buf, Msg::LeftPos),
            Some(MsgId::RightPos) => parse_f32(buf, Msg::RightPos),
            Some(MsgId::LeftPower) => parse_f32(buf, Msg::LeftPower),
            Some(MsgId::RightPower) => parse_f32(buf, Msg::RightPower),
            Some(MsgId::Battery) => parse_f32(buf, Msg::Battery),
            Some(MsgId::LeftDistance) => parse_u8(buf, Msg::LeftDistance),
            Some(MsgId::FrontDistance) => parse_u8(buf, Msg::FrontDistance),
            Some(MsgId::RightDistance) => parse_u8(buf, Msg::RightDistance),

            Some(MsgId::LinearPos) => parse_f32(buf, Msg::LinearPos),
            Some(MsgId::AngularPos) => parse_f32(buf, Msg::AngularPos),
            Some(MsgId::LinearPower) => parse_f32(buf, Msg::LinearPower),
            Some(MsgId::AngularPower) => parse_f32(buf, Msg::AngularPower),
            Some(MsgId::LinearSet) => parse_f32(buf, Msg::LinearSet),
            Some(MsgId::AngularSet) => parse_f32(buf, Msg::AngularSet),
            Some(MsgId::AddLinear) => parse_target(buf, Msg::AddLinear),
            Some(MsgId::AddAngular) => parse_target(buf, Msg::AddAngular),
            Some(MsgId::LinearTarget) => parse_target(buf, Msg::LinearTarget),
            Some(MsgId::AngularTarget) => parse_target(buf, Msg::AngularTarget),
            Some(MsgId::LinearBuffer) => parse_targets(buf, Msg::LinearBuffer),
            Some(MsgId::AngularBuffer) => parse_targets(buf, Msg::AngularBuffer),

            Some(MsgId::LinearP) => parse_f32(buf, Msg::LinearP),
            Some(MsgId::LinearI) => parse_f32(buf, Msg::LinearI),
            Some(MsgId::LinearD) => parse_f32(buf, Msg::LinearD),
            Some(MsgId::LinearAcc) => parse_f32(buf, Msg::LinearAcc),

            Some(MsgId::AngularP) => parse_f32(buf, Msg::AngularP),
            Some(MsgId::AngularI) => parse_f32(buf, Msg::AngularI),
            Some(MsgId::AngularD) => parse_f32(buf, Msg::AngularD),
            Some(MsgId::AngularAcc) => parse_f32(buf, Msg::AngularAcc),

            Some(MsgId::At) => parse_line(buf, Msg::At),

            None => Err(ParseError::UnknownMsg(id[0])),
        }
    }

    pub fn generate_bytes<W: WriteExact<Error = E>, E>(&self, buf: &mut W) -> Result<(), E> {
        match self {
            &Msg::Time(m) => write_f32(buf, MsgId::Time, m),
            &Msg::Logged(ref m) => write_msgids(buf, MsgId::Logged, m),
            &Msg::Provided(ref m) => write_msgids(buf, MsgId::Provided, m),

            &Msg::LeftPos(m) => write_f32(buf, MsgId::LeftPos, m),
            &Msg::RightPos(m) => write_f32(buf, MsgId::RightPos, m),
            &Msg::LeftPower(m) => write_f32(buf, MsgId::LeftPower, m),
            &Msg::RightPower(m) => write_f32(buf, MsgId::RightPower, m),
            &Msg::Battery(m) => write_f32(buf, MsgId::Battery, m),
            &Msg::LeftDistance(d) => write_u8(buf, MsgId::LeftDistance, d),
            &Msg::FrontDistance(d) => write_u8(buf, MsgId::FrontDistance, d),
            &Msg::RightDistance(d) => write_u8(buf, MsgId::RightDistance, d),

            &Msg::LinearPos(m) => write_f32(buf, MsgId::LinearPos, m),
            &Msg::AngularPos(m) => write_f32(buf, MsgId::AngularPos, m),
            &Msg::LinearPower(m) => write_f32(buf, MsgId::LinearPower, m),
            &Msg::AngularPower(m) => write_f32(buf, MsgId::AngularPower, m),
            &Msg::LinearSet(m) => write_f32(buf, MsgId::LinearSet, m),
            &Msg::AngularSet(m) => write_f32(buf, MsgId::AngularSet, m),
            &Msg::AddLinear(t) => write_target(buf, MsgId::AddLinear, t),
            &Msg::AddAngular(t) => write_target(buf, MsgId::AddAngular, t),
            &Msg::LinearTarget(t) => write_target(buf, MsgId::LinearTarget, t),
            &Msg::AngularTarget(t) => write_target(buf, MsgId::AngularTarget, t),
            &Msg::LinearBuffer(ref t) => write_targets(buf, MsgId::LinearBuffer, t),
            &Msg::AngularBuffer(ref t) => write_targets(buf, MsgId::AngularBuffer, t),

            &Msg::LinearP(m) => write_f32(buf, MsgId::LinearP, m),
            &Msg::LinearI(m) => write_f32(buf, MsgId::LinearI, m),
            &Msg::LinearD(m) => write_f32(buf, MsgId::LinearD, m),
            &Msg::LinearAcc(m) => write_f32(buf, MsgId::LinearAcc, m),

            &Msg::AngularP(m) => write_f32(buf, MsgId::AngularP, m),
            &Msg::AngularI(m) => write_f32(buf, MsgId::AngularI, m),
            &Msg::AngularD(m) => write_f32(buf, MsgId::AngularD, m),
            &Msg::AngularAcc(m) => write_f32(buf, MsgId::AngularAcc, m),

            &Msg::At(_) => Ok(()),
        }
    }
}
