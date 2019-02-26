#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                         // extern crate panic_abort; // requires nightly
                         // extern crate panic_itm; // logs messages over ITM; requires ITM support
                         //extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

//use cortex_m::asm;
use cortex_m_rt::entry;

use stm32f4::stm32f405;

mod uart;

#[derive(Copy, Clone)]
enum Direction {
    Forward,
    Backward,
}

impl core::ops::Not for Direction {
    type Output = Direction;

    fn not(self) -> Self::Output {
        match self {
            Direction::Forward => Direction::Backward,
            Direction::Backward => Direction::Forward,
        }
    }
}

fn left_motor_setup(
    rcc: &stm32f405::RCC,
    pwm_timer: &stm32f405::TIM4,
    pwm_gpio: &stm32f405::GPIOB,
) {
    // Enable clock for gpio b
    rcc.ahb1enr.modify(|_, w| w.gpioben().set_bit());

    // Enable clock for timer 4
    rcc.apb1enr.modify(|_, w| w.tim4en().set_bit());

    // Set pins to alternate function
    pwm_gpio
        .moder
        .modify(|_, w| w.moder6().alternate().moder7().alternate());

    // Set the alternate function to timer 4 channel 1 and 2
    pwm_gpio.afrl.modify(|_, w| w.afrl6().af2().afrl7().af2());

    // setup the timer
    pwm_timer.psc.write(|w| unsafe { w.psc().bits(10u16) });
    pwm_timer.cr1.write(|w| w.arpe().set_bit());
    pwm_timer.arr.write(|w| w.arr().bits(10000u32));
    pwm_timer.ccr1.write(|w| w.ccr1().bits(5000u32));
    pwm_timer.ccr2.write(|w| w.ccr2().bits(5000u32));
    pwm_timer.ccmr1_output.write(|w| unsafe {
        w.oc1m()
            .bits(0b110)
            .oc1pe()
            .set_bit()
            .oc2m()
            .bits(0b110)
            .oc2pe()
            .set_bit()
    });
    pwm_timer.egr.write(|w| w.ug().set_bit());
    pwm_timer
        .ccer
        .write(|w| w.cc1e().set_bit().cc2e().clear_bit());
    pwm_timer.cr1.modify(|_, w| w.cen().set_bit());
}

fn left_motor_change_speed(pwm_timer: &stm32f405::TIM4, speed: u32) {
    pwm_timer.ccr1.write(|w| w.ccr1().bits(speed));
    pwm_timer.ccr2.write(|w| w.ccr2().bits(speed));
}

fn left_motor_change_direction(pwm_timer: &stm32f405::TIM4, direction: Direction) {
    match direction {
        Direction::Forward => pwm_timer
            .ccer
            .write(|w| w.cc1e().set_bit().cc2e().clear_bit()),
        Direction::Backward => pwm_timer
            .ccer
            .write(|w| w.cc1e().clear_bit().cc2e().set_bit()),
    }
}

/*
fn left_encoder_setup(
    rcc: &stm32f405::RCC,
    timer2: &stm32f405::TIM2,
    gpioa: &stm32f405::GPIOA,
    gpiob: &stm32f405::GPIOB,
) {
    // enable clock for gpio a and gpio b
    rcc.ahb1enr.modify(|_, w| w.gpioben().set_bit().gpioaen().set_bit());

    // enable clock for timer 2
    rcc.apb1enr.modify(|_, w| w.tim2en().set_bit());

    // set pins to alternate function
    gpioa.moder.modify(|_, w| w.moder5().alternate());
    gpiob.moder.modify(|_, w| w.moder3().alternate());

    // set the alternate function to timer 2 channels 1 and 2
    gpioa.afrl.modify(|_, w| w.afrl5().af1());
    gpiob.afrl.modify(|_, w| w.afrl3().af1());

    // setup timer for encoder mode
    timer2.smcr.write(|w| unsafe { w.sms().bits(0b011) });
}
*/

fn mco2_setup(rcc: &stm32f405::RCC, gpioc: &stm32f405::GPIOC) {
    rcc.ahb1enr.write(|w| w.gpiocen().set_bit());
    rcc.cfgr.modify(|_, w| w.mco2().sysclk());
    gpioc.moder.write(|w| w.moder9().alternate());
    gpioc.afrh.write(|w| w.afrh9().af0());
}

#[entry]
fn main() -> ! {
    let peripherals = stm32f405::Peripherals::take().unwrap();
    let mut core_peripherals = stm32f405::CorePeripherals::take().unwrap();

    peripherals
        .RCC
        .ahb1enr
        .write(|w| w.gpioben().set_bit().gpioaen().set_bit());

    peripherals
        .GPIOA
        .moder
        .write(|w| w.moder6().output().moder7().output().moder8().output());
    peripherals
        .GPIOA
        .odr
        .write(|w| w.odr6().clear_bit().odr7().set_bit());

    peripherals.GPIOB.moder.write(|w| {
        w.moder12()
            .output()
            .moder13()
            .output()
            .moder14()
            .output()
            .moder15()
            .output()
    });

    peripherals.GPIOB.odr.write(|w| {
        w.odr12()
            .set_bit()
            .odr13()
            .set_bit()
            .odr14()
            .set_bit()
            .odr15()
            .set_bit()
    });

    mco2_setup(&peripherals.RCC, &peripherals.GPIOC);
    left_motor_setup(&peripherals.RCC, &peripherals.TIM4, &peripherals.GPIOB);
    uart::setup(
        &peripherals.RCC,
        &mut core_peripherals.NVIC,
        peripherals.USART1,
        &peripherals.GPIOA,
    );

    let mut i = 0u64;
    let mut dir = Direction::Forward;

    loop {
        if i < 10000u64 {
            peripherals.GPIOB.odr.modify(|_, w| {
                w.odr12()
                    .set_bit()
                    .odr13()
                    .clear_bit()
                    .odr14()
                    .clear_bit()
                    .odr15()
                    .clear_bit()
            });
        } else if i < 20000u64 {
            peripherals.GPIOB.odr.modify(|_, w| {
                w.odr12()
                    .clear_bit()
                    .odr13()
                    .set_bit()
                    .odr14()
                    .clear_bit()
                    .odr15()
                    .clear_bit()
            });
        } else if i < 30000u64 {
            peripherals.GPIOB.odr.modify(|_, w| {
                w.odr12()
                    .clear_bit()
                    .odr13()
                    .clear_bit()
                    .odr14()
                    .set_bit()
                    .odr15()
                    .clear_bit()
            });
        } else if i < 40000u64 {
            peripherals.GPIOB.odr.modify(|_, w| {
                w.odr12()
                    .clear_bit()
                    .odr13()
                    .clear_bit()
                    .odr14()
                    .clear_bit()
                    .odr15()
                    .set_bit()
            });
        } else if i < 50000u64 {
            peripherals.GPIOB.odr.modify(|_, w| {
                w.odr12()
                    .clear_bit()
                    .odr13()
                    .clear_bit()
                    .odr14()
                    .clear_bit()
                    .odr15()
                    .clear_bit()
            });
        } else {
            i = 0;
        }

        if i == 0 {
            dir = !dir;
            left_motor_change_direction(&peripherals.TIM4, dir);
        }

        let speed = if i < 25000u64 {
            (i as u32) / 5
        } else {
            ((50000u64 - i) as u32) / 5
        };

        left_motor_change_speed(&peripherals.TIM4, speed);

        i += 1;
    }
}
