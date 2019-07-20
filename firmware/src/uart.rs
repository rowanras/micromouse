use core::fmt;
use core::fmt::Write;

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m_rt_macros::interrupt as isr;

use stm32f4xx_hal::stm32 as stm32f405;
use stm32f4xx_hal::stm32::Interrupt as interrupt;

use micromouse_lib::msgs::ReadExact;
use micromouse_lib::msgs::WriteExact;

pub trait Command {
    fn keyword_command(&self) -> &str;
    fn handle_command<'a, I: Iterator<Item = &'a str>>(
        &mut self,
        uart: &mut Uart,
        args: I,
    );
}

const RX_BUFFER_LEN: usize = 1024;
const TX_BUFFER_LEN: usize = 1024;

struct Buffer<T> {
    bytes: T,
    len: usize,
}

static UART: Mutex<RefCell<Option<stm32f405::USART1>>> =
    Mutex::new(RefCell::new(None));

static RX_BUF: Mutex<RefCell<Buffer<[u8; RX_BUFFER_LEN]>>> =
    Mutex::new(RefCell::new(Buffer { bytes: [0; RX_BUFFER_LEN], len: 0 }));

static TX_BUF: Mutex<RefCell<Buffer<[u8; TX_BUFFER_LEN]>>> =
    Mutex::new(RefCell::new(Buffer { bytes: [0; TX_BUFFER_LEN], len: 0 }));

#[derive(PartialEq)]
pub enum TxError {
    BufferFull,
    Busy,
    NotInitialized,
}

impl<T> From<arrayvec::CapacityError<T>> for TxError {
    fn from(_err: arrayvec::CapacityError<T>) -> TxError {
        TxError::BufferFull
    }
}

#[derive(PartialEq)]
pub enum RxError {
    BufferEmpty,
    Busy,
    NotInitialized,
}

pub struct Uart { }

impl Uart {
    pub fn setup(
        rcc: &stm32f405::RCC,
        nvic: &mut stm32f405::NVIC,
        uart: stm32f405::USART1,
        gpioa: &stm32f405::GPIOA,
    ) -> Uart {
        // enable clock for usart
        rcc.apb2enr.modify(|_, w| w.usart1en().set_bit());

        // enable clock for gpioa
        rcc.ahb1enr.modify(|_, w| w.gpioaen().set_bit());

        // set pins to alternate function
        gpioa
            .moder
            .modify(|_, w| w.moder9().alternate().moder10().alternate());

        // set the alternate function to usart1 rx and tx
        gpioa.afrh.modify(|_, w| w.afrh9().af7().afrh10().af7());

        // set buadrate
        uart.brr.write(|w| unsafe { w.bits(0x683) });

        // enable rx and tx
        uart.cr1.write(|w| {
            w.ue()
                .set_bit()
                .re()
                .set_bit()
                .te()
                .set_bit()
                .rxneie()
                .set_bit()
                .tcie()
                .set_bit()
        });

        cortex_m::interrupt::free(|cs| UART.borrow(cs).replace(Some(uart)));

        nvic.enable(interrupt::USART1);

        Uart { }
    }

    fn add_byte(&mut self, c: u8) -> Result<(), TxError> {
        cortex_m::interrupt::free(|cs| {
            if let Ok(mut buf) = TX_BUF.borrow(cs).try_borrow_mut() {
                if buf.len < buf.bytes.len() {
                    if buf.len == 0 {
                        if let Some(uart) = UART.borrow(cs).borrow().as_ref() {
                            uart.dr.write(|w| w.dr().bits(c as u16));
                            buf.len = 1;
                            Ok(())
                        } else {
                            Err(TxError::Busy)
                        }
                    } else {
                        for i in (1..buf.len).rev() {
                            buf.bytes[i] = buf.bytes[i-1];
                        }
                        buf.bytes[0] = c;
                        buf.len += 1;

                        Ok(())
                    }
                } else {
                    Err(TxError::BufferFull)
                }
            } else {
                Err(TxError::Busy)
            }
        })
    }

    pub fn add_bytes(&mut self, b: &[u8]) -> Result<(), TxError> {
        for &c in b {
            self.add_byte(c)?;
        }

        Ok(())
    }

    pub fn add_str(&mut self, s: &str) -> Result<(), TxError> {
        for &c in s.as_bytes() {
            self.add_byte(c)?;
        }

        Ok(())
    }

    pub fn clear_tx(&mut self) -> Result<(), TxError> {
        cortex_m::interrupt::free(|cs| {
            if let Ok(mut buf) = TX_BUF.borrow(cs).try_borrow_mut() {
                buf.len = 0;
                buf.bytes = [0; TX_BUFFER_LEN];
                Ok(())
            } else {
                Err(TxError::Busy)
            }
        })
    }

    pub fn clear_rx(&mut self) -> Result<(), TxError> {
        cortex_m::interrupt::free(|cs| {
            if let Ok(mut buf) = RX_BUF.borrow(cs).try_borrow_mut() {
                buf.len = 0;
                buf.bytes = [0; RX_BUFFER_LEN];
                Ok(())
            } else {
                Err(TxError::Busy)
            }
        })
    }

    pub fn tx_len(&self) -> Result<usize, TxError> {
        cortex_m::interrupt::free(|cs| {
            if let Ok(buf) = TX_BUF.borrow(cs).try_borrow().as_ref() {
                Ok(buf.len)
            } else {
                Err(TxError::Busy)
            }
        })
    }

    pub fn rx_len(&self) -> Result<usize, RxError> {
        cortex_m::interrupt::free(|cs| {
            if let Ok(buf) = RX_BUF.borrow(cs).try_borrow().as_ref() {
                Ok(buf.len)
            } else {
                Err(RxError::Busy)
            }
        })
    }

    pub fn read_byte(&mut self) -> Result<u8, RxError> {
        cortex_m::interrupt::free(|cs| {
            if let Ok(mut buf) = RX_BUF.borrow(cs).try_borrow_mut() {
                if buf.len > 0 {
                    let c = buf.bytes[0];
                    for i in 1..buf.len {
                        buf.bytes[i-1] = buf.bytes[i];
                    }
                    buf.len -= 1;
                    let len = buf.len;
                    buf.bytes[len] = 0;
                    Ok(c)
                } else {
                    Err(RxError::BufferEmpty)
                }
            } else {
                Err(RxError::Busy)
            }
        })
    }

    pub fn read_line(&mut self) -> Result<[u8; RX_BUFFER_LEN], RxError> {
        cortex_m::interrupt::free(|cs| {
            if let Ok(mut buf) = RX_BUF.borrow(cs).try_borrow_mut() {
                if buf.len > 0 && buf.bytes[buf.len-1] == 0x0A {
                    let new_buf = buf.bytes.clone();
                    buf.bytes = [0; RX_BUFFER_LEN];
                    buf.len = 0;
                    Ok(new_buf)
                } else {
                    Err(RxError::BufferEmpty)
                }
            } else {
                Err(RxError::Busy)
            }
        })
    }

    pub fn read_exact(&mut self, fill_buf: &mut [u8]) -> Result<(), RxError> {
        cortex_m::interrupt::free(|cs| {
            if let Ok(mut buf) = RX_BUF.borrow(cs).try_borrow_mut() {
                if buf.len >= fill_buf.len() {
                    fill_buf.clone_from_slice(&buf.bytes[0..fill_buf.len()]);
                    buf.len -= fill_buf.len();
                    Ok(())
                } else {
                    Err(RxError::BufferEmpty)
                }
            } else {
                Err(RxError::Busy)
            }
        })
    }
}

impl ReadExact for Uart {
    type Error = RxError;
    fn peek(&mut self, fill_buf: &mut [u8]) -> Result<(), RxError> {
        cortex_m::interrupt::free(|cs| {
            if let Ok(buf) = RX_BUF.borrow(cs).try_borrow() {
                if buf.len >= fill_buf.len() {
                    fill_buf.clone_from_slice(&buf.bytes[0..fill_buf.len()]);
                    Ok(())
                } else {
                    Err(RxError::BufferEmpty)
                }
            } else {
                Err(RxError::Busy)
            }
        })
    }

    fn take(&mut self, fill_buf: &mut [u8]) -> Result<(), RxError> {
        cortex_m::interrupt::free(|cs| {
            if let Ok(mut buf) = RX_BUF.borrow(cs).try_borrow_mut() {
                if buf.len >= fill_buf.len() {
                    fill_buf.clone_from_slice(&buf.bytes[0..fill_buf.len()]);
                    buf.len -= fill_buf.len();
                    Ok(())
                } else {
                    Err(RxError::BufferEmpty)
                }
            } else {
                Err(RxError::Busy)
            }
        })
    }
}

impl WriteExact for Uart {
    type Error = TxError;
    fn write(&mut self, fill_buf: &[u8]) -> Result<(), TxError> {
        self.add_bytes(fill_buf)
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.add_str(s).map(|_| ()).map_err(|_| fmt::Error)
    }
}


#[isr]
fn USART1() {
    cortex_m::interrupt::free(|cs| {
        if let Some(uart) = UART.borrow(cs).borrow().as_ref() {
            if uart.sr.read().rxne().bit() {
                let rx = uart.dr.read().dr().bits() as u8;
                //uart.dr.write(|w| w.dr().bits(rx as u16));
                if let Ok(mut buf) = RX_BUF.borrow(cs).try_borrow_mut() {
                    if buf.len < RX_BUFFER_LEN {
                        let len = buf.len;
                        buf.bytes[len] = rx;
                        buf.len += 1;
                    }
                }
            }

            if uart.sr.read().tc().bit() {
                if let Ok(mut buf) = TX_BUF.borrow(cs).try_borrow_mut() {
                    // decrement the byte we just send
                    if buf.len > 0 {
                        buf.len -= 1;
                    }

                    // send the next byte
                    if buf.len > 0 {
                        let index = buf.len-1;
                        uart.dr.write(|w| w.dr().bits(buf.bytes[index] as u16));
                        buf.bytes[index] = 0;
                    }
                }

                uart.sr.modify(|_, w| w.tc().clear_bit());
            }
        }
    });
}
