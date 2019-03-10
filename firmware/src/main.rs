#![no_std]
#![no_main]

// pick a panicking behavior
// you can put a breakpoint on `rust_begin_unwind` to catch panics
extern crate panic_halt;
mod uart;
mod motors;

use core::fmt::Write;
use cortex_m_rt::entry;
use stm32f4::stm32f405;

use crate::uart::Uart;
use crate::motors::{
    Direction,
    Motor,
};

use crate::motors::left::{
    LeftMotor,
    LeftEncoder,
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

    writeln!(uart, "Initialized!");

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
            left_motor.change_direction(dir);
        }

        let speed = if i < 25000u64 {
            (i as u32) / 5
        } else {
            ((50000u64 - i) as u32) / 5
        };

        left_motor.change_speed(speed);

        if i % 100 == 0 {
            let count = left_encoder.count();
            writeln!(uart, "encoder: {}", count);
        }

        i += 1;
    }
}
