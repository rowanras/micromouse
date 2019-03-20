#![no_std]
#![no_main]

// pick a panicking behavior
// you can put a breakpoint on `rust_begin_unwind` to catch panics
extern crate panic_halt;

mod battery;
mod motors;
mod plan;
mod time;
mod uart;

use core::fmt::Write;
use core::str;
use cortex_m_rt::entry;
use stm32f4::stm32f405;

use ignore_result::Ignore;

use crate::battery::Battery;
use crate::time::Time;

use crate::uart::Command;
use crate::uart::Uart;

use crate::motors::control::MotorControl;
use crate::motors::left::{LeftEncoder, LeftMotor};
use crate::motors::right::{RightEncoder, RightMotor};

use crate::plan::Plan;

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
        &mut core_peripherals.NVIC,
        peripherals.USART1,
        &peripherals.GPIOA,
    );

    let left_motor = LeftMotor::setup(
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

    let right_motor = RightMotor::setup(
        &peripherals.RCC,
        peripherals.TIM4,
        &peripherals.GPIOB,
    );

    let right_encoder = RightEncoder::setup(
        &peripherals.RCC,
        &peripherals.GPIOA,
        peripherals.TIM5,
    );

    let left_control = MotorControl::new(
        1.0,
        0.00000,
        0.0,
        30,
        left_motor,
        left_encoder,
        "left",
    );

    let right_control = MotorControl::new(
        1.0,
        0.00000,
        00000.0,
        30,
        right_motor,
        right_encoder,
        "right",
    );

    let mut plan = Plan::new(right_control, left_control, 100, "plan");

    writeln!(uart, "").ignore();
    writeln!(uart, "start").ignore();
    uart.flush_tx(&mut time, 1000);

    let mut last_time: u32 = 0;
    let mut on = false;

    let mut report = true;

    loop {
        let now: u32 = time.now();

        if let Some(line) = uart.read_line() {
            if let Ok(string) = str::from_utf8(&line) {
                let string = string.trim_matches(|c| c as u8 == 0).trim();
                writeln!(uart, ">> {}", string).ignore();
                if string.starts_with('!') {
                    writeln!(uart, "Stopping report").ignore();
                    report = false;
                } else if string.starts_with('@') {
                    writeln!(uart, "Starting report").ignore();
                    report = true;
                } else {
                    let mut args = string.split_whitespace();

                    let command = args.next();

                    if command == Some(plan.keyword_command()) {
                        plan.handle_command(&mut uart, args);
                    } else {
                        writeln!(uart, "Invalid Command!").ignore();
                    }
                }
            }
        }

        plan.update(now);

        if now - last_time >= 100u32 {
            if report {
                writeln!(
                    uart,
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    now,
                    plan.left_control().position(),
                    plan.left_control().error(),
                    plan.left_control().target(),
                    plan.left_control().motor_velocity(),
                    plan.linear_acceleration(),
                    plan.linear_velocity(),
                    plan.linear_position(),
                    plan.delta_time(),
                    battery.raw(),
                    battery.is_dead(),
                )
                .ignore();
            }

            if on {
                peripherals.GPIOB.odr.modify(|_, w| w.odr13().clear_bit());
                on = false;
            } else {
                peripherals.GPIOB.odr.modify(|_, w| w.odr13().set_bit());
                on = true;
            }

            last_time = now;
        }

        battery.update(now);
        uart.flush_tx(&mut time, 50);
    }
}
