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
 *  Positive spin is clockwise (right)
 *  Positive linear is forward
 *
 */
// pick a panicking behavior
// you can put a breakpoint on `rust_begin_unwind` to catch panics
extern crate panic_halt;

pub mod battery;
pub mod bot;
pub mod config;
pub mod control;
pub mod motors;
pub mod navigate;
pub mod plan;
pub mod time;
pub mod uart;
pub mod vl6180x;

use core::fmt::Write;
use core::str;
use cortex_m_rt::entry;
use stm32f4xx_hal as stm32f4;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::stm32 as stm32f405;

use ignore_result::Ignore;

use crate::battery::Battery;
use crate::time::Time;

use crate::uart::Command;
use crate::uart::Uart;

use crate::motors::left::{LeftEncoder, LeftMotor};
use crate::motors::right::{RightEncoder, RightMotor};

use crate::bot::Bot;
use crate::config::BotConfig;

use crate::control::Control;

use crate::plan::Plan;

use crate::navigate::LessRandomNavigate;
use crate::navigate::RandomNavigate;

// Setup the master clock out
pub fn mco2_setup(rcc: &stm32f405::RCC, gpioc: &stm32f405::GPIOC) {
    rcc.ahb1enr.write(|w| w.gpiocen().set_bit());
    rcc.cfgr.modify(|_, w| w.mco2().sysclk());
    gpioc.moder.write(|w| w.moder9().alternate());
    gpioc.afrh.write(|w| w.afrh9().af0());
}

#[entry]
fn main() -> ! {
    let p = stm32f4::stm32::Peripherals::take().unwrap();
    let mut cp = stm32f405::CorePeripherals::take().unwrap();

    // Init non-hal things
    let mut time = Time::setup(&p.RCC, p.TIM1);

    while time.now() < 10000 {}

    let mut battery = Battery::setup(&p.RCC, &p.GPIOB, p.ADC1);

    let mut uart = Uart::setup(&p.RCC, &mut cp.NVIC, p.USART1, &p.GPIOA);

    let left_motor = LeftMotor::setup(&p.RCC, p.TIM3, &p.GPIOA);

    let left_encoder = LeftEncoder::setup(&p.RCC, &p.GPIOA, &p.GPIOB, p.TIM2);

    let right_motor = RightMotor::setup(&p.RCC, p.TIM4, &p.GPIOB);
    let right_encoder = RightEncoder::setup(&p.RCC, &p.GPIOA, p.TIM5);

    // Init the hal things
    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpioa = p.GPIOA.split();
    let gpiob = p.GPIOB.split();
    let gpioc = p.GPIOC.split();

    let mut red_led = gpiob.pb12.into_push_pull_output();
    let mut green_led = gpiob.pb13.into_push_pull_output();
    let mut blue_led = gpiob.pb14.into_push_pull_output();
    let mut orange_led = gpiob.pb15.into_push_pull_output();

    let left_button = gpioc.pc10.into_pull_up_input();
    let middle_button = gpioc.pc11.into_pull_up_input();
    let right_button = gpioc.pc12.into_pull_up_input();

    orange_led.set_high();
    blue_led.set_low();

    writeln!(uart, "Initializing").ignore();

    let mut front_distance = {
        let scl = gpiob.pb8.into_open_drain_output().into_alternate_af4();
        let sda = gpiob.pb9.into_open_drain_output().into_alternate_af4();

        let mut gpio0 = gpioc.pc0.into_open_drain_output();
        gpio0.set_high();

        let mut gpio1 = gpioc.pc1.into_open_drain_output();
        gpio1.set_high();

        let i2c =
            stm32f4::i2c::I2c::i2c1(p.I2C1, (scl, sda), 100.khz(), clocks);

        time.delay(10000);

        let mut distance = vl6180x::VL6180x::new(i2c, 0x29);
        distance.init_private_registers();
        distance.init_default();
        distance
    };

    orange_led.set_low();
    blue_led.set_high();

    let mut left_distance = {
        let scl = gpiob.pb10.into_open_drain_output().into_alternate_af4();
        let sda = gpiob.pb11.into_open_drain_output().into_alternate_af4();

        let mut gpio0 = gpioc.pc2.into_open_drain_output();
        gpio0.set_high();

        let mut gpio1 = gpioc.pc3.into_open_drain_output();
        gpio1.set_high();

        let i2c =
            stm32f4::i2c::I2c::i2c2(p.I2C2, (scl, sda), 100.khz(), clocks);

        time.delay(1000);

        let mut distance = vl6180x::VL6180x::new(i2c, 0x29);
        distance.init_private_registers();
        distance.init_default();
        distance
    };

    orange_led.set_high();
    blue_led.set_high();

    let mut right_distance = {
        let scl = gpioa.pa8.into_open_drain_output().into_alternate_af4();
        let sda = gpioc.pc9.into_open_drain_output().into_alternate_af4();

        let mut gpio0 = gpioc.pc4.into_open_drain_output();
        gpio0.set_high();

        let mut gpio1 = gpioc.pc5.into_open_drain_output();
        gpio1.set_high();

        let i2c =
            stm32f4::i2c::I2c::i2c3(p.I2C3, (scl, sda), 100.khz(), clocks);

        time.delay(1000);

        let mut distance = vl6180x::VL6180x::new(i2c, 0x29);
        distance.init_private_registers();
        distance.init_default();
        distance
    };

    blue_led.set_low();
    orange_led.set_low();

    writeln!(uart, "Reading id registers").ignore();

    for _ in 0..2 {
        let buf = front_distance.get_id_bytes();

        writeln!(uart, "{:x?}", buf).ignore();

        orange_led.toggle();
    }

    for _ in 0..2 {
        let buf = left_distance.get_id_bytes();

        writeln!(uart, "{:x?}", buf).ignore();

        orange_led.toggle();
    }

    for _ in 0..2 {
        let buf = right_distance.get_id_bytes();

        writeln!(uart, "{:x?}", buf).ignore();

        orange_led.toggle();
    }

    let config = BotConfig {
        left_p: 2000.0,
        left_i: 4.0,
        left_d: 15000.0,
        right_p: 2000.0,
        right_i: 4.0,
        right_d: 15000.0,
        spin_p: 0.01,
        spin_i: 0.0,
        spin_d: 0.0,
        spin_err: 15.0,
        spin_settle: 50,
        linear_p: 0.0185,
        linear_i: 0.0,
        linear_d: 0.1,
        linear_spin_p: 0.015,
        linear_spin_i: 0.000000002,
        linear_spin_d: 0.0,
        linear_spin_pos_p: 2.0,
        linear_err: 10.0,
        linear_front_err: 5.0,
        linear_settle: 50,
        ticks_per_spin: 2064.03,
        ticks_per_cell: 1620.0,
        cell_width: 180.0,
        cell_offset: 53.0,
        wall_threshold: 120.0,
        front_wall_distance: 35.0,
    };

    let bot = Bot::new(
        left_motor,
        left_encoder,
        right_motor,
        right_encoder,
        front_distance,
        left_distance,
        right_distance,
        config,
    );

    let control = Control::new(bot);

    /*
    let navigate = RandomNavigate::new([
        15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
    ]);
    */

    let navigate = LessRandomNavigate::new([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    ]);

    let mut plan = Plan::new(control, navigate);

    writeln!(uart, "\n\nstart").ignore();

    let mut last_time: u32 = 0;

    let mut report = false;

    loop {
        let now: u32 = time.now();

        if let Ok(line) = uart.read_line() {
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

        if now - last_time >= 20u32 {
            if report {
                writeln!(
                    uart,
                    "{}",
                    now,
                    //plan.x_pos(),
                    //plan.y_pos(),
                    //plan.direction(),
                    //plan.control().bot().left_pos(),
                    //plan.control().bot().right_pos(),
                    //control.bot().left_velocity(),
                    //control.bot().right_velocity(),
                    //control.bot().left_power(),
                    //control.bot().right_power(),
                    //plan.control().bot().linear_pos(),
                    //plan.control().bot().spin_pos(),
                    //plan.control().bot().linear_velocity(),
                    //plan.control().bot().spin_velocity(),
                    //plan.control().bot().left_distance(),
                    //plan.control().bot().front_distance(),
                    //plan.control().bot().right_distance(),
                )
                .ignore();
            }

            green_led.toggle();

            if plan.control().is_idle() {
                orange_led.set_low();
            } else {
                orange_led.set_high();
            }

            if plan.is_win() {
                blue_led.set_high();
            } else {
                blue_led.set_low();
            }

            if battery.is_dead() {
                red_led.set_high();
            } else {
                red_led.set_low();
            }

            if left_button.is_low() {
                plan.go();
            }

            if right_button.is_low() {
                plan.stop();
            }

            last_time = now;
        }

        plan.update(now);
        battery.update(now);
    }
}
