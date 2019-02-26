use core::cell::RefCell;

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

pub fn setup(
    rcc: &stm32f405::RCC,
    nvic: &mut stm32f405::NVIC,
    uart: stm32f405::USART1,
    gpioa: &stm32f405::GPIOA,
) {
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

    add_byte('H' as u8);
    add_byte('e' as u8);
    add_byte('l' as u8);
    add_byte('l' as u8);
    add_byte('o' as u8);
    add_byte('\n' as u8);

    add_str("World!\n");

    add_str("⛜\n");

    interrupt_free(|cs| UART.borrow(cs).replace(Some(uart)));

    nvic.enable(interrupt::USART1);
}

fn add_byte(c: u8) {
    interrupt_free(|cs| {
        let mut buffer = BUFFER.borrow(cs).borrow_mut();

        if buffer.1 < BUFFER_LEN {
            let len = buffer.1;
            buffer.0[len] = c as u8;
            buffer.1 += 1;
        }
    });
}

fn add_str(s: &str) {
    for &c in s.as_bytes() {
        add_byte(c);
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

                    buffer.0.rotate_left(1);
                    buffer.1 -= 1;
                }
                uart.sr.write(|w| w.tc().clear_bit());
            }
        }
    });
}
