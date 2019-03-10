use core::cell::RefCell;
use core::fmt;
use core::fmt::Write;

use cortex_m::interrupt::free as interrupt_free;
use cortex_m::interrupt::CriticalSection;
use cortex_m::interrupt::Mutex;

use cortex_m_rt::interrupt as isr;

use stm32f4::stm32f405;
use stm32f4::stm32f405::interrupt;

const BUFFER_LEN: usize = 64;

static UART: Mutex<RefCell<Option<stm32f405::USART1>>> = Mutex::new(RefCell::new(None));
static BUFFER: Mutex<RefCell<([u8; BUFFER_LEN], usize)>> =
    Mutex::new(RefCell::new(([0; BUFFER_LEN], 0)));

pub struct Uart {}

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
                .tcie()
                .set_bit()
        });

        interrupt_free(|cs| UART.borrow(cs).replace(Some(uart)));

        nvic.enable(interrupt::USART1);

        Uart {}
    }

    fn add_byte(&self, c: u8, cs: &CriticalSection) {
        let mut buffer = BUFFER.borrow(cs).borrow_mut();

        if buffer.1 < BUFFER_LEN {
            let len = buffer.1;
            buffer.0[len] = c as u8;
            buffer.1 += 1;
        }
    }

    pub fn add_str(&self, s: &str) {
        interrupt_free(|cs| {
            for &c in s.as_bytes() {
                self.add_byte(c, cs);
            }
        });
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.add_str(s);

        // TODO: Should probably return Error if buffer is full
        Ok(())
    }
}

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
