#![no_std]
#![no_main]

// pick a panicking behavior
// you can put a breakpoint on `rust_begin_unwind` to catch panics
extern crate panic_halt;

mod battery;
mod motors;
mod time;
mod uart;

use core::fmt::Write;
use cortex_m_rt::entry;
use stm32f4::stm32f405;

use crate::battery::Battery;
use crate::time::Time;
use crate::uart::Uart;

use crate::motors::control::MotorControl;
use crate::motors::left::{LeftEncoder, LeftMotor};
use crate::motors::right::{RightEncoder, RightMotor};
use crate::motors::Encoder;
use crate::motors::Motor;

fn mco2_setup(rcc: &stm32f405::RCC, gpioc: &stm32f405::GPIOC) {
    rcc.ahb1enr.write(|w| w.gpiocen().set_bit());
    rcc.cfgr.modify(|_, w| w.mco2().sysclk());
    gpioc.moder.write(|w| w.moder9().alternate());
    gpioc.afrh.write(|w| w.afrh9().af0());
}

#[entry]
fn main() -> ! {
    let peripherals = stm32f405::Peripherals::take().unwrap();

    peripherals.RCC.ahb1enr.write(|w| w.gpioben().set_bit());

    //peripherals
    //.GPIOA
    //.moder
    //.write(|w| w.moder6().output().moder7().output().moder8().output());
    //peripherals
    //.GPIOA
    //.odr
    //.write(|w| w.odr6().clear_bit().odr7().set_bit());

    peripherals.GPIOB.moder.modify(|_, w| {
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
            .clear_bit()
            .odr13()
            .clear_bit()
            .odr14()
            .clear_bit()
            .odr15()
            .clear_bit()
    });

    //mco2_setup(&peripherals.RCC, &peripherals.GPIOC);

    let mut time = Time::setup(&peripherals.RCC, peripherals.TIM1);

    let mut battery =
        Battery::setup(&peripherals.RCC, &peripherals.GPIOB, peripherals.ADC1);

    let mut uart = Uart::setup(
        &peripherals.RCC,
        peripherals.USART1,
        &peripherals.GPIOA,
    );

    let mut left_motor = LeftMotor::setup(
        &peripherals.RCC,
        peripherals.TIM3,
        &peripherals.GPIOA,
    );

    let left_encoder = LeftEncoder::setup(
        &peripherals.RCC,
        &peripherals.GPIOA,
        &peripherals.GPIOB,
        peripherals.TIM2,
    );

    let mut right_motor = RightMotor::setup(
        &peripherals.RCC,
        peripherals.TIM4,
        &peripherals.GPIOB,
    );

    let right_encoder = RightEncoder::setup(
        &peripherals.RCC,
        &peripherals.GPIOA,
        peripherals.TIM5,
    );

    let mut left_control = MotorControl::new(
        75.0,
        0.0,
        50000.0,
        |t| 10000,
        left_motor,
        left_encoder,
    );

    //right_motor.change_velocity(500);
    //left_motor.change_velocity(-1000);

    writeln!(uart, "Initialized!");
    uart.flush();

    let mut last_time: u32 = 0;
    let mut on = false;

    loop {
        let now: u32 = time.now();

        if now - last_time >= 1000u32 {
            writeln!(
                uart,
                "{}\t{}\t{}\t{}\t{}\t{}",
                now,
                left_control.position(),
                left_control.error(),
                left_control.target(),
                battery.raw(),
                battery.is_dead(),
            );

            if on {
                peripherals
                    .GPIOB
                    .odr
                    .modify(|_, w| w.odr13().clear_bit());
                on = false;
            } else {
                peripherals
                    .GPIOB
                    .odr
                    .modify(|_, w| w.odr13().set_bit());
                on = true;
            }

            last_time = now;
        }

        left_control.update(now);
        battery.update(now);
        uart.flush();
    }
}
