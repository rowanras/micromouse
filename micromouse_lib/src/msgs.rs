use core::convert::From;
use core::i16;
use core::u32;

pub trait ReadExact {
    type Error;
    fn peek(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;
    fn take(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;
}

pub trait WriteExact {
    type Error;
    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
}

pub enum Msg {
    Time(f64),
    EnableLogging,
    DisableLogging,
    LeftEncoder(f64),
    RightEncoder(f64),
    LeftMotorPower(f64),
    RightMotorPower(f64),
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

impl Msg {
    pub fn parse_bytes<R: ReadExact<Error = E>, E>(
        buf: &mut R,
    ) -> Result<Self, ParseError<E>> {
        let mut id = [0; 1];
        buf.peek(&mut id)?;

        match id {
            [0x00] => {
                let mut msg = [0; 5];
                buf.take(&mut msg)?;
                let [_, b1, b2, b3, b4] = msg;
                Ok(Msg::Time(
                    u32::from_le_bytes([b1, b2, b3, b4]) as f64 / 1000.0,
                ))
            }
            [0x01] => {
                let mut msg = [0; 1];
                buf.take(&mut msg)?;
                Ok(Msg::EnableLogging)
            }
            [0x02] => {
                let mut msg = [0; 1];
                buf.take(&mut msg)?;
                Ok(Msg::DisableLogging)
            }
            [0x10] => {
                let mut msg = [0; 3];
                buf.take(&mut msg)?;
                let [_, b1, b2] = msg;
                Ok(Msg::LeftEncoder(i16::from_le_bytes([b1, b2]) as f64))
            }
            [0x11] => {
                let mut msg = [0; 3];
                buf.take(&mut msg)?;
                let [_, b1, b2] = msg;
                Ok(Msg::RightEncoder(i16::from_le_bytes([b1, b2]) as f64))
            }
            [0x12] => {
                let mut msg = [0; 3];
                buf.take(&mut msg)?;
                let [_, b1, b2] = msg;
                Ok(Msg::LeftMotorPower(i16::from_le_bytes([b1, b2]) as f64 / 1000.0))
            }
            [0x13] => {
                let mut msg = [0; 3];
                buf.take(&mut msg)?;
                let [_, b1, b2] = msg;
                Ok(Msg::RightMotorPower(i16::from_le_bytes([b1, b2]) as f64 / 1000.0))
            }
            [id] => Err(ParseError::UnknownMsg(id)),
        }
    }

    pub fn generate_bytes<W: WriteExact<Error = E>, E>(
        &self,
        buf: &mut W,
    ) -> Result<(), E> {
        match self {
            &Msg::Time(m) => {
                let [b1, b2, b3, b4] = (m as u32).to_le_bytes();
                buf.write(&[0x00, b1, b2, b3, b4])
            }
            &Msg::EnableLogging => buf.write(&[0x01]),
            &Msg::DisableLogging => buf.write(&[0x02]),
            &Msg::LeftEncoder(m) => {
                let [b1, b2] = (m as i16).to_le_bytes();
                buf.write(&[0x10, b1, b2])
            }
            &Msg::RightEncoder(m) => {
                let [b1, b2] = (m as i16).to_le_bytes();
                buf.write(&[0x11, b1, b2])
            }
            &Msg::LeftMotorPower(m) => {
                let [b1, b2] = (m as i16).to_le_bytes();
                buf.write(&[0x12, b1, b2])
            }
            &Msg::RightMotorPower(m) => {
                let [b1, b2] = (m as i16).to_le_bytes();
                buf.write(&[0x13, b1, b2])
            }
        }
    }
}
