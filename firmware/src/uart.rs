use core::fmt;
use core::fmt::Write;

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m_rt_macros::interrupt as isr;

use stm32f4xx_hal::stm32 as stm32f405;
use stm32f4xx_hal::stm32::Interrupt as interrupt;

use crate::time::Time;

pub trait Command {
    fn keyword_command(&self) -> &str;
    fn handle_command<'a, I: Iterator<Item = &'a str>>(
        &mut self,
        uart: &mut Uart,
        args: I,
    );
}

const BUFFER_LEN: usize = 1024;

static UART: Mutex<RefCell<Option<stm32f405::USART1>>> =
    Mutex::new(RefCell::new(None));
static RX_BUF: Mutex<RefCell<([u8; BUFFER_LEN], usize)>> =
    Mutex::new(RefCell::new(([0; BUFFER_LEN], 0)));

pub enum UartError {
    BufferFull,
}

pub struct Uart {
    tx_buffer: [u8; BUFFER_LEN],
    tx_length: usize,
}

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
            //.tcie()
            //.set_bit()
        });

        cortex_m::interrupt::free(|cs| UART.borrow(cs).replace(Some(uart)));

        nvic.enable(interrupt::USART1);

        Uart {
            tx_buffer: [0; BUFFER_LEN],
            tx_length: 0,
        }
    }

    fn add_byte(&mut self, c: u8) -> Result<usize, UartError> {
        if self.tx_length < BUFFER_LEN {
            self.tx_buffer[self.tx_length] = c;
            self.tx_length += 1;

            Ok(BUFFER_LEN - self.tx_length)
        } else {
            Err(UartError::BufferFull)
        }
    }

    pub fn add_str(&mut self, s: &str) -> Result<usize, UartError> {
        for &c in s.as_bytes() {
            self.add_byte(c)?;
        }
        Ok(self.tx_length)
    }

    pub fn flush_tx(&mut self, time: &mut Time, timeout: u32) {
        let start_time = time.now();
        while self.tx_length > 0 && time.now() - start_time <= timeout {
            cortex_m::interrupt::free(|cs| {
                if let Some(uart) = UART.borrow(cs).borrow().as_ref() {
                    if uart.sr.read().txe().bit_is_set() {
                        uart.dr
                            .write(|w| w.dr().bits(self.tx_buffer[0] as u16));

                        for i in 1..self.tx_length {
                            self.tx_buffer[i - 1] = self.tx_buffer[i];
                        }

                        self.tx_length -= 1;
                        self.tx_buffer[self.tx_length] = 0;
                    }
                }
            });
        }
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        cortex_m::interrupt::free(|cs| {
            if let Ok(mut buf) = RX_BUF.borrow(cs).try_borrow_mut() {
                if buf.1 > 0 {
                    let c = buf.0[0];

                    for i in 1..buf.1 {
                        buf.0[i - 1] = buf.0[i];
                    }

                    buf.1 -= 1;

                    let len = buf.1;
                    buf.0[len] = 0;

                    Some(c)
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    pub fn read_line(&mut self) -> Option<[u8; BUFFER_LEN]> {
        cortex_m::interrupt::free(|cs| {
            if let Ok(mut buf) = RX_BUF.borrow(cs).try_borrow_mut() {
                if buf.1 > 0 && buf.0[buf.1 - 1] == 10 {
                    let new_buf = buf.0.clone();

                    buf.0 = [0; BUFFER_LEN];
                    buf.1 = 0;

                    Some(new_buf)
                } else {
                    None
                }
            } else {
                None
            }
        })
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
            let rx = uart.dr.read().dr().bits() as u8;
            //uart.dr.write(|w| w.dr().bits(rx as u16));
            if let Ok(mut buf) = RX_BUF.borrow(cs).try_borrow_mut() {
                if buf.1 < BUFFER_LEN {
                    let len = buf.1;
                    buf.0[len] = rx;

                    buf.1 += 1;
                }
            }
        }
    });
}
