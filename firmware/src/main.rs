#![no_std]
#![no_main]

// pick a panicking behavior
// you can put a breakpoint on `rust_begin_unwind` to catch panics
extern crate panic_halt;
mod time;
mod uart;
mod motors;

use core::fmt::Write;
use cortex_m_rt::entry;
use stm32f4::stm32f405;
use pid_control::PIDController;

use crate::time::Time;
use crate::uart::Uart;
use crate::motors::{
    Direction,
    Motor,
    Encoder,
};

use crate::motors::left::{
    LeftMotor,
    LeftEncoder,
};

use crate::motors::right::{
    RightMotor,
    RightEncoder,
};

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
        .write(|w| w.gpioben().set_bit());

    //peripherals
        //.GPIOA
        //.moder
        //.write(|w| w.moder6().output().moder7().output().moder8().output());
    //peripherals
        //.GPIOA
        //.odr
        //.write(|w| w.odr6().clear_bit().odr7().set_bit());

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

    //mco2_setup(&peripherals.RCC, &peripherals.GPIOC);

    let mut time = Time::setup(
        &peripherals.RCC,
        peripherals.TIM1
    );

    let mut uart = Uart::setup(
        &peripherals.RCC,
        &mut core_peripherals.NVIC,
        peripherals.USART1,
        &peripherals.GPIOA,
    );

    let mut left_motor = LeftMotor::setup(
        &peripherals.RCC,
        peripherals.TIM4,
        &peripherals.GPIOB,
    );

    let left_encoder = LeftEncoder::setup(
        &peripherals.RCC,
        &peripherals.GPIOA,
        &peripherals.GPIOB,
        peripherals.TIM2,
    );

    let mut right_motor = RightMotor::setup(
        &peripherals.RCC,
        peripherals.TIM3,
        &peripherals.GPIOA,
    );

    let mut right_encoder = RightEncoder::setup(
        &peripherals.RCC,
        &peripherals.GPIOA,
        peripherals.TIM5,
    );

    writeln!(uart, "Initialized!");
    uart.flush();

    //let mut i = 0u64;
    let mut left_dir = Direction::Forward;
    let mut right_dir = Direction::Forward;

    //right_motor.change_speed(5000);
    right_motor.change_direction(Direction::Backward);

    let mut last_time: u32 = 0;
    let mut on = false;

    loop {

        let now: u32 = time.now();

        if now - last_time >= 1000u32 {
            writeln!(uart, "{}:{}", last_time, now);
            if on {
                peripherals.GPIOB.odr.modify(|_, w| w.odr13().clear_bit());
                on = false;
            } else {
                peripherals.GPIOB.odr.modify(|_, w| w.odr13().set_bit());
                on = true;
            }

            last_time = now;
        }

        uart.flush();
/*
        if i < 1000 {
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
        } else if i < 2000 {
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
        } else if i < 3000 {
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
        } else if i < 4000 {
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
        } else if i < 5000 {
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
            //i = 0;
        }

        if i % 5000 == 0 {
            left_dir = !left_dir;
            left_motor.change_direction(left_dir);
        }

        if (i + 250) % 5000 == 0 {
            right_dir = !right_dir;
            right_motor.change_direction(right_dir);
        }

        if i % 1000 == 0 {
            let left_count = left_encoder.count();
            let right_count = right_encoder.count();
            //writeln!(uart, "{}:{}", left_count, right_count);
        }
    */
    }
}
