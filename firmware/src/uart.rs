use core::fmt;
use core::fmt::Write;

use stm32f4::stm32f405;

const BUFFER_LEN: usize = 64;

//static UART: Mutex<RefCell<Option<stm32f405::USART1>>> = Mutex::new(RefCell::new(None));
//static BUFFER: Mutex<RefCell<([u8; BUFFER_LEN], usize)>> =
//Mutex::new(RefCell::new(([0; BUFFER_LEN], 0)));

pub struct Uart {
    uart: stm32f405::USART1,
    buffer: [u8; BUFFER_LEN],
    length: usize,
}

impl Uart {
    pub fn setup(
        rcc: &stm32f405::RCC,
        uart: stm32f405::USART1,
        gpioa: &stm32f405::GPIOA,
    ) -> Uart {
        // enable clock for usart
        rcc.apb2enr.modify(|_, w| w.usart1en().set_bit());

        // enable clock for gpioa
        rcc.ahb1enr.modify(|_, w| w.gpioaen().set_bit());

        // set pins to alternate function
        gpioa.moder.modify(|_, w| {
            w.moder9().alternate().moder10().alternate()
        });

        // set the alternate function to usart1 rx and tx
        gpioa.afrh.modify(|_, w| w.afrh9().af7().afrh10().af7());

        // set buadrate
        uart.brr.write(|w| unsafe { w.bits(0x683) });

        // enable rx and tx
        uart.cr1.write(|w| {
            w.ue().set_bit().re().set_bit().te().set_bit()
            //.tcie()
            //.set_bit()
        });

        //interrupt_free(|cs| UART.borrow(cs).replace(Some(uart)));

        //nvic.enable(interrupt::USART1);

        Uart {
            uart,
            buffer: [0; BUFFER_LEN],
            length: 0,
        }
    }

    fn add_byte(&mut self, c: u8) {
        if self.length < BUFFER_LEN {
            self.buffer[self.length] = c;
            self.length += 1;
        }
    }

    pub fn add_str(&mut self, s: &str) {
        for &c in s.as_bytes() {
            self.add_byte(c);
        }
    }

    pub fn flush(&mut self) {
        while self.length > 0 {
            if self.uart.sr.read().txe().bit_is_set() {
                self.uart
                    .dr
                    .write(|w| w.dr().bits(self.buffer[0] as u16));

                for i in 1..self.length {
                    self.buffer[i - 1] = self.buffer[i];
                }

                self.length -= 1;
                self.buffer[self.length] = 0;
            }
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.add_str(s);

        // TODO: Should probably return Error if buffer is full
        Ok(())
    }
}

/*
#[isr]
fn USART1() {
    interrupt_free(|cs| {
        if let Some(uart) = UART.borrow(cs).borrow().as_ref() {
            if uart.sr.read().tc().bit() {
                let mut buffer = BUFFER.borrow(cs).borrow_mut();

                if buffer.1 > 0 {
                    uart.dr.write(|w| w.dr().bits(buffer.0[0] as u16));

                    for i in 1..buffer.1 {
                        buffer.0[i-1] = buffer.0[i];
                    }

                    let len = buffer.1;
                    buffer.0[len] = 0;

                    //buffer.0.rotate_left(1);
                    buffer.1 -= 1;
                }
                uart.sr.write(|w| w.tc().clear_bit());
            }
        }
    });
}
*/
