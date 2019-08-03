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
//pub mod bot;
//pub mod config;
//pub mod control;
pub mod motors;
//pub mod navigate;
//pub mod plan;
pub mod time;
pub mod uart;
pub mod vl6180x;

use cortex_m_rt::entry;
use stm32f4xx_hal as stm32f4;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::stm32 as stm32f405;

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::ToggleableOutputPin;

use ignore_result::Ignore;

use micromouse_lib::CONFIG2019;
use micromouse_lib::msgs::Msg;
use micromouse_lib::msgs::ParseError;
use micromouse_lib::mouse::Mouse;

use crate::battery::Battery;
use crate::time::Time;

use crate::uart::Uart;

use crate::motors::Encoder;

use crate::motors::left::{LeftEncoder, LeftMotor};
use crate::motors::right::{RightEncoder, RightMotor};
use crate::motors::Motor;

// Setup the master clock out
pub fn mco2_setup(rcc: &stm32f405::RCC, gpioc: &stm32f405::GPIOC) {
    rcc.ahb1enr.write(|w| w.gpiocen().set_bit());
    rcc.cfgr.modify(|_, w| w.mco2().sysclk());
    gpioc.moder.write(|w| w.moder9().alternate());
    gpioc.afrh.write(|w| w.afrh9().af0());
}

#[entry]
fn main() -> ! {

    let config = CONFIG2019;

    let p = stm32f4::stm32::Peripherals::take().unwrap();
    let mut cp = stm32f405::CorePeripherals::take().unwrap();

    // Init non-hal things
    let mut time = Time::setup(&p.RCC, p.TIM1);

    while time.now() < 10000 {}

    let mut battery = Battery::setup(&p.RCC, &p.GPIOB, p.ADC1);

    let mut uart = Uart::setup(&p.RCC, &mut cp.NVIC, p.USART1, &p.GPIOA);

    let mut left_motor = LeftMotor::setup(&p.RCC, p.TIM3, &p.GPIOA);
    let left_encoder = LeftEncoder::setup(&p.RCC, &p.GPIOA, &p.GPIOB, p.TIM2);

    let mut right_motor = RightMotor::setup(&p.RCC, p.TIM4, &p.GPIOB);
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

    let _left_button = gpioc.pc10.into_pull_up_input();
    let _middle_button = gpioc.pc11.into_pull_up_input();
    let _right_button = gpioc.pc12.into_pull_up_input();

    orange_led.set_high().ignore();
    blue_led.set_low().ignore();

    //writeln!(uart, "Initializing").ignore();

    let mut front_distance = {
        let scl = gpiob.pb8.into_open_drain_output().into_alternate_af4();
        let sda = gpiob.pb9.into_open_drain_output().into_alternate_af4();

        let mut gpio0 = gpioc.pc0.into_open_drain_output();
        gpio0.set_high().ignore();

        let mut gpio1 = gpioc.pc1.into_open_drain_output();
        gpio1.set_high().ignore();

        let i2c =
            stm32f4::i2c::I2c::i2c1(p.I2C1, (scl, sda), 100.khz(), clocks);

        time.delay(10000);

        let mut distance = vl6180x::VL6180x::new(i2c, 0x29);
        distance.init_private_registers();
        distance.init_default();
        distance
    };

    orange_led.set_low().ignore();
    blue_led.set_high().ignore();

    let mut left_distance = {
        let scl = gpiob.pb10.into_open_drain_output().into_alternate_af4();
        let sda = gpiob.pb11.into_open_drain_output().into_alternate_af4();

        let mut gpio0 = gpioc.pc2.into_open_drain_output();
        gpio0.set_high().ignore();

        let mut gpio1 = gpioc.pc3.into_open_drain_output();
        gpio1.set_high().ignore();

        let i2c =
            stm32f4::i2c::I2c::i2c2(p.I2C2, (scl, sda), 100.khz(), clocks);

        time.delay(1000);

        let mut distance = vl6180x::VL6180x::new(i2c, 0x29);
        distance.init_private_registers();
        distance.init_default();
        distance
    };

    orange_led.set_high().ignore();
    blue_led.set_high().ignore();

    let mut right_distance = {
        let scl = gpioa.pa8.into_open_drain_output().into_alternate_af4();
        let sda = gpioc.pc9.into_open_drain_output().into_alternate_af4();

        let mut gpio0 = gpioc.pc4.into_open_drain_output();
        gpio0.set_high().ignore();

        let mut gpio1 = gpioc.pc5.into_open_drain_output();
        gpio1.set_high().ignore();

        let i2c =
            stm32f4::i2c::I2c::i2c3(p.I2C3, (scl, sda), 100.khz(), clocks);

        time.delay(1000);

        let mut distance = vl6180x::VL6180x::new(i2c, 0x29);
        distance.init_private_registers();
        distance.init_default();
        distance
    };

    blue_led.set_low().ignore();
    orange_led.set_low().ignore();

    //writeln!(uart, "Reading id registers").ignore();

    for _ in 0..2 {
        let _buf = front_distance.get_id_bytes();
        //uart.add_bytes(&_buf).ignore();
        orange_led.toggle().ignore();
    }

    for _ in 0..2 {
        let _buf = left_distance.get_id_bytes();
        //uart.add_bytes(&_buf).ignore();
        orange_led.toggle().ignore();
    }

    for _ in 0..2 {
        let _buf = right_distance.get_id_bytes();
        //uart.add_bytes(&_buf).ignore();
        orange_led.toggle().ignore();
    }

    left_distance.start_ranging();
    front_distance.start_ranging();
    right_distance.start_ranging();

    let mut mouse = Mouse::new(config);

    let mut last_msg_time = 0.0;

    let mut last_time = 0;

    loop {

        let now = time.now();

        battery.update(now);
        left_distance.update();
        front_distance.update();
        right_distance.update();

        if now - last_time >= 10 {

            mouse.time = now as f32 / 1000.0;
            mouse.battery = battery.raw() as f32;
            mouse.left_distance = left_distance.range();
            mouse.front_distance = front_distance.range();
            mouse.right_distance = right_distance.range();
            mouse.left_pos = mouse.mouse_config.ticks_to_mm(left_encoder.count() as f32);
            mouse.right_pos = mouse.mouse_config.ticks_to_mm(right_encoder.count() as f32);

            let msg = Msg::parse_bytes(&mut uart);

            if let Ok(msg) = msg {
                last_msg_time = mouse.time;
                mouse.update(msg);
            } else if mouse.time - last_msg_time >= 1.0{
                uart.clear_rx().ignore();
                red_led.set_low().ignore();
                last_msg_time = mouse.time;
                if let Err(ParseError::UnknownMsg(_)) = msg {
                    red_led.set_high().ignore();
                }
            }

            if uart.tx_len() == Ok(0) {
                orange_led.set_low().ignore();
            } else {
                orange_led.set_high().ignore();
            }

            if uart.rx_len() == Ok(0) {
                blue_led.set_low().ignore();
            } else {
                blue_led.set_high().ignore();
            }

            if mouse.logged.len() > 0 {
                green_led.set_high().ignore();
            } else {
                green_led.set_low().ignore();
            }

            mouse.linear_pos = (mouse.left_pos + mouse.right_pos) / 2.0;
            mouse.angular_pos = mouse.mouse_config.mm_to_rads((mouse.left_pos - mouse.right_pos) / 2.0);

            //mouse.linear_power = mouse.linear_control.update(mouse.time as f64, mouse.linear_pos as f64) as f32;
            //mouse.angular_power = mouse.angular_control.update(mouse.time as f64, mouse.angular_pos as f64) as f32;

            mouse.left_power = mouse.linear_power + mouse.angular_power;
            mouse.right_power = mouse.linear_power - mouse.angular_power;

            left_motor.change_power((mouse.left_power * 10000.0) as i32);
            right_motor.change_power((mouse.right_power * 10000.0) as i32);

            if uart.tx_len() == Ok(0) {
                for &log in mouse.logged.iter() {
                    let msg = mouse.msg(log);
                    msg.generate_bytes(&mut uart).ignore();
                }
            }

            last_time = now;
        }
    }
}

