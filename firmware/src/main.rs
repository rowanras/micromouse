#![no_std]
#![no_main]

/**
 *
 * Important mechanical parameters:
 *
 *  - wheel diameter: 32mm
 *  - wheel circumference: 100mm
 *  - gear raitio: 75:1
 *  - counts per motor rev: 12
 *  - counts per wheel rev: 900
 *  - counts per mm: 9
 *  - wheelbase diameter: 73mm
 *  - wheelbase circumference: 229.336mm
 *  - ticks per spin: 2064.03
 *
 */
// pick a panicking behavior
// you can put a breakpoint on `rust_begin_unwind` to catch panics
extern crate panic_halt;

mod battery;
mod bot;
mod control;
mod distance;
mod motors;
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

use crate::motors::left::{LeftEncoder, LeftMotor};
use crate::motors::right::{RightEncoder, RightMotor};

use crate::motors::Encoder;

use crate::distance::left::LeftDistance;

use crate::bot::Bot;
use crate::bot::BotConfig;

use crate::control::Control;

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

    /*
    let mut left_distance = LeftDistance::setup(
        &peripherals.RCC,
        &peripherals.GPIOB,
        peripherals.I2C2,
    );
    */

    let config = BotConfig {
        left_p: 2000.0,
        left_i: 2.0,
        left_d: 15000.0,

        right_p: 2000.0,
        right_i: 2.0,
        right_d: 15000.0,

        spin_p: 0.01,
        spin_i: 0.0,
        spin_d: 0.0,
        spin_err: 1.0,
        spin_settle: 1000,

        linear_p: 0.01,
        linear_i: 0.0,
        linear_d: 0.0,
        linear_spin_p: 0.01,
        linear_spin_i: 0.0,
        linear_spin_d: 0.0,
        linear_err: 0.5,
        linear_settle: 1000,

        ticks_per_spin: 2064.03,
        ticks_per_cell: 1620.0,
    };

    let bot =
        Bot::new(left_motor, left_encoder, right_motor, right_encoder, config);

    let mut control = Control::new(bot);

    writeln!(uart, "\n\nstart").ignore();
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

                    if command == Some(control.keyword_command()) {
                        control.handle_command(&mut uart, args);
                    } else {
                        writeln!(uart, "Invalid Command!").ignore();
                    }
                }
            }
        }

        if now - last_time >= 20u32 {
            if report {
                writeln!(
                    uart,
                    "{}\t{:.2}\t{:.2}\t{}\t{}",
                    now,
                    //control.bot().left_pos(),
                    //control.bot().right_pos(),
                    //control.bot().spin_velocity(),
                    //control.bot().right_target(),
                    //control.bot().spin_pos(),
                    control.bot().left_velocity(),
                    control.bot().right_velocity(),
                    //control.bot().left_power(),
                    //control.bot().right_power(),
                    //control.bot().linear_velocity(),
                    //control.bot().spin_velocity(),
                    //control.bot().linear_pos(),
                    //control.bot().spin_pos(),
                    //control.bot().linear_pos(),
                    //control.current_move_name(),
                    //left_distance.read_range_single(),
                    battery.raw(),
                    battery.is_dead(),
                )
                .ignore();
            }

            if battery.is_dead() {
                peripherals.GPIOB.odr.modify(|_, w| w.odr12().clear_bit());
            } else {
                peripherals.GPIOB.odr.modify(|_, w| w.odr12().set_bit());
            }

            if control.is_idle() {
                peripherals.GPIOB.odr.modify(|_, w| w.odr14().clear_bit());
            } else {
                peripherals.GPIOB.odr.modify(|_, w| w.odr14().set_bit());
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

        control.update(now);
        battery.update(now);
        uart.flush_tx(&mut time, 50);
    }
}
