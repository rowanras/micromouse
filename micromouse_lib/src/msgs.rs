use core::convert::From;
use core::f32;
use core::u32;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use arrayvec::ArrayVec;

pub trait ReadExact {
    type Error;
    fn peek(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;
    fn take(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;
}

pub trait WriteExact {
    type Error;
    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
}

#[derive(FromPrimitive, Copy, Clone)]
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

    // Calculated
    LinearPos = 0x20,
    AngularPos = 0x21,
    LinearSet = 0x22,
    AngularSet = 0x23,
    AddLinear = 0x24,
    AddAngular = 0x25,

    // Config
    LinearP = 0xa0,
    LinearI = 0xa1,
    LinearD = 0xa2,
    LinearAcc = 0xa3,
    AngularP = 0xa4,
    AngularI = 0xa5,
    AngularD = 0xa6,
    AngularAcc = 0xa7,
}

pub enum Msg {
    // Core
    Time(f32),
    Logged(ArrayVec<[MsgId; 8]>),
    Provided(ArrayVec<[MsgId; 8]>),

    // Raw in/out
    LeftPos(f32),
    RightPos(f32),
    LeftPower(f32),
    RightPower(f32),

    // Calculated
    LinearPos(f32),
    AngularPos(f32),
    LinearSet(f32),
    AngularSet(f32),
    AddLinear(f32, f32),
    AddAngular(f32, f32),

    // Config
    LinearP(f32),
    LinearI(f32),
    LinearD(f32),
    LinearAcc(f32),
    AngularP(f32),
    AngularI(f32),
    AngularD(f32),
    AngularAcc(f32),
}

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
fn parse_id<R: ReadExact<Error = E>, E>(
    buf: &mut R,
    msg: Msg,
) -> Result<Msg, ParseError<E>> {
    let mut msgbuf = [0; 1];
    buf.take(&mut msgbuf)?;
    Ok(msg)
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
    msg: fn(ArrayVec<[MsgId; 8]>) -> Msg,
) -> Result<Msg, ParseError<E>> {
    // attept to get the length
    let mut lenbuf = [0; 2];
    buf.peek(&mut lenbuf)?;
    let [_id, len] = lenbuf;

    let msgbuf = &mut [0; 10][0..(len+2) as usize];
    buf.take(msgbuf)?;

    let msgids: ArrayVec<[MsgId; 8]> = msgbuf.into_iter().skip(2).filter_map(|&mut m| MsgId::from_u8(m)).collect();

    Ok(msg(msgids))
}

#[allow(dead_code)]
fn write_id<W: WriteExact<Error = E>, E>(
    buf: &mut W,
    msgid: MsgId,
) -> Result<(), E> {
    buf.write(&[msgid as u8])
}

fn write_f32<W: WriteExact<Error = E>, E>(
    buf: &mut W,
    msgid: MsgId,
    msg: f32,
) -> Result<(), E> {
    let [a1, a2, a3, a4] = u32::to_le_bytes(f32::to_bits(msg));
    buf.write(&[msgid as u8, a1, a2, a3, a4])
}

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
    let bytes: ArrayVec<[u8; 9]> = [msgid as u8]
        .into_iter()
        .map(|&m| m)
        .chain([msgids.len() as u8].into_iter().map(|&l| l))
        .chain(msgids.into_iter().map(|&m| m as u8))
        .collect();
    buf.write(&bytes)
}

impl Msg {
    pub fn parse_bytes<R: ReadExact<Error = E>, E>(
        buf: &mut R,
    ) -> Result<Self, ParseError<E>> {
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

            Some(MsgId::LinearPos) => parse_f32(buf, Msg::LinearPos),
            Some(MsgId::AngularPos) => parse_f32(buf, Msg::AngularPos),
            Some(MsgId::LinearSet) => parse_f32(buf, Msg::LinearSet),
            Some(MsgId::AngularSet) => parse_f32(buf, Msg::AngularSet),
            Some(MsgId::AddLinear) => parse_2f32(buf, Msg::AddLinear),
            Some(MsgId::AddAngular) => parse_2f32(buf, Msg::AddAngular),

            Some(MsgId::LinearP) => parse_f32(buf, Msg::LinearP),
            Some(MsgId::LinearI) => parse_f32(buf, Msg::LinearI),
            Some(MsgId::LinearD) => parse_f32(buf, Msg::LinearD),
            Some(MsgId::LinearAcc) => parse_f32(buf, Msg::LinearAcc),

            Some(MsgId::AngularP) => parse_f32(buf, Msg::AngularP),
            Some(MsgId::AngularI) => parse_f32(buf, Msg::AngularI),
            Some(MsgId::AngularD) => parse_f32(buf, Msg::AngularD),
            Some(MsgId::AngularAcc) => parse_f32(buf, Msg::AngularAcc),

            None => Err(ParseError::UnknownMsg(id[0])),
        }
    }

    pub fn generate_bytes<W: WriteExact<Error = E>, E>(
        &self,
        buf: &mut W,
    ) -> Result<(), E> {
        match self {
            &Msg::Time(m) => write_f32(buf, MsgId::Time, m),
            &Msg::Logged(ref m) => write_msgids(buf, MsgId::Logged, m),
            &Msg::Provided(ref m) => write_msgids(buf, MsgId::Provided, m),

            &Msg::LeftPos(m) => write_f32(buf, MsgId::LeftPos, m),
            &Msg::RightPos(m) => write_f32(buf, MsgId::RightPos, m),
            &Msg::LeftPower(m) => write_f32(buf, MsgId::LeftPower, m),
            &Msg::RightPower(m) => write_f32(buf, MsgId::RightPower, m),

            &Msg::LinearPos(m) => write_f32(buf, MsgId::LinearPos, m),
            &Msg::AngularPos(m) => write_f32(buf, MsgId::AngularPos, m),
            &Msg::LinearSet(m) => write_f32(buf, MsgId::LinearSet, m),
            &Msg::AngularSet(m) => write_f32(buf, MsgId::AngularSet, m),
            &Msg::AddLinear(m1, m2) => {
                write_2f32(buf, MsgId::AddLinear, m1, m2)
            }
            &Msg::AddAngular(m1, m2) => {
                write_2f32(buf, MsgId::AddAngular, m1, m2)
            }

            &Msg::LinearP(m) => write_f32(buf, MsgId::LinearP, m),
            &Msg::LinearI(m) => write_f32(buf, MsgId::LinearI, m),
            &Msg::LinearD(m) => write_f32(buf, MsgId::LinearD, m),
            &Msg::LinearAcc(m) => write_f32(buf, MsgId::LinearAcc, m),

            &Msg::AngularP(m) => write_f32(buf, MsgId::AngularP, m),
            &Msg::AngularI(m) => write_f32(buf, MsgId::AngularI, m),
            &Msg::AngularD(m) => write_f32(buf, MsgId::AngularD, m),
            &Msg::AngularAcc(m) => write_f32(buf, MsgId::AngularAcc, m),
        }
    }
}
