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

use cortex_m_rt::entry;
use stm32f4xx_hal as stm32f4;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::stm32 as stm32f405;

use ignore_result::Ignore;

use arrayvec::ArrayVec;

use micromouse_lib::CONFIG2019;
use micromouse_lib::msgs::Msg;
use micromouse_lib::msgs::MsgId;
use micromouse_lib::msgs::ParseError;
use micromouse_lib::control::MotionControl;

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

    let left_button = gpioc.pc10.into_pull_up_input();
    let middle_button = gpioc.pc11.into_pull_up_input();
    let right_button = gpioc.pc12.into_pull_up_input();

    orange_led.set_high();
    blue_led.set_low();

    //writeln!(uart, "Initializing").ignore();

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

    //writeln!(uart, "Reading id registers").ignore();

    for _ in 0..2 {
        let _buf = front_distance.get_id_bytes();

        //writeln!(uart, "{:x?}", buf).ignore();

        orange_led.toggle();
    }

    for _ in 0..2 {
        let _buf = left_distance.get_id_bytes();

        //writeln!(uart, "{:x?}", buf).ignore();

        orange_led.toggle();
    }

    for _ in 0..2 {
        let _buf = right_distance.get_id_bytes();

        //writeln!(uart, "{:x?}", buf).ignore();

        orange_led.toggle();
    }

    let mut last_msg_time = 0;
    let mut logged = ArrayVec::new();
    let mut provided = ArrayVec::new();
    let mut last_control_time = 0.0;
    //let mut linear_control = MotionControl::new(1.0, 0.0, 0.0, 1.0);
    //let mut angular_control = MotionControl::new(1.0, 0.0, 0.0, 1.0);

    loop {
        let now: u32 = time.now();

        let msg = Msg::parse_bytes(&mut uart);

        if msg.is_ok() {
            last_msg_time = now;
        } else if now - last_msg_time >= 1000 {
            uart.clear_rx().ignore();
            red_led.set_low();
            last_msg_time = now;
        }

        match msg {
            // Core
            Ok(Msg::Time(t)) => {},
            Ok(Msg::Logged(m)) => logged = m,
            Ok(Msg::Provided(m)) => provided = m,

            // Raw in/out
            Ok(Msg::LeftPos(p)) => {},
            Ok(Msg::RightPos(p)) => {},
            Ok(Msg::LeftPower(p)) => left_motor.change_power((p * 10000.0) as i32),
            Ok(Msg::RightPower(p)) => right_motor.change_power((p * 10000.0) as i32),
            Ok(Msg::Battery(v)) => {},

            // Calculated
            Ok(Msg::LinearPos(p)) => {},
            Ok(Msg::AngularPos(p)) => {},
            Ok(Msg::LinearSet(s)) => {},
            Ok(Msg::AngularSet(s)) => {},
            Ok(Msg::AddLinear(v, d)) => {},
            Ok(Msg::AddAngular(v, d)) => {},

            // Config
            Ok(Msg::LinearP(p)) => {},
            Ok(Msg::LinearI(i)) => {},
            Ok(Msg::LinearD(d)) => {},
            Ok(Msg::LinearAcc(a)) => {},
            Ok(Msg::AngularP(p)) => {},
            Ok(Msg::AngularI(i)) => {},
            Ok(Msg::AngularD(d)) => {},
            Ok(Msg::AngularAcc(a)) => {},
            Err(ParseError::UnknownMsg(_)) => red_led.set_high(),
            Err(_) => {},
        }

        if uart.tx_len() == Ok(0) {
            orange_led.set_low();
        } else {
            orange_led.set_high();
        }

        if uart.rx_len() == Ok(0) {
            blue_led.set_low();
        } else {
            blue_led.set_high();
        }

        if logged.len() > 0 {
            green_led.set_high();
        } else {
            green_led.set_low();
        }

        /*
        let now = now as f64 / 1000.0;
        if now - last_control_time > 0.01 {
            let linear_position = config.mouse.ticks_to_mm((left_encoder.count() + right_encoder.count()) as f64 / 2.0);
            let angular_position = config.mouse.ticks_to_rads((left_encoder.count() - right_encoder.count()) as f64 / 2.0);

            let linear_power = linear_control.update(now, linear_position);
            let angular_power = angular_control.update(now, angular_position);

            left_motor.change_power(((linear_power + angular_power) * 10000.0) as i32);
            right_motor.change_power((( linear_power - angular_power) * 10000.0) as i32);
        }
        */

        if uart.tx_len() == Ok(0) {
            for log in logged.iter() {
                let msg = match log {
                    // Core
                    MsgId::Time => Msg::Time(now as f32 / 1000.0),
                    MsgId::Logged => Msg::Logged(logged.clone()),
                    MsgId::Provided => Msg::Provided(provided.clone()),

                    // Raw in/out
                    MsgId::LeftPos => Msg::LeftPos(config.mouse.ticks_to_mm(left_encoder.count() as f32)),
                    MsgId::RightPos => Msg::RightPos(config.mouse.ticks_to_mm(right_encoder.count() as f32)),
                    MsgId::LeftPower => Msg::LeftPower(0.0),
                    MsgId::RightPower => Msg::RightPower(0.0),
                    MsgId::Battery => Msg::Battery(battery.raw() as f32),

                    // Calculated
                    MsgId::LinearPos => Msg::LinearPos(0.0),
                    MsgId::AngularPos => Msg::AngularPos(0.0),
                    MsgId::LinearSet => Msg::LinearSet(0.0),
                    MsgId::AngularSet => Msg::AngularSet(0.0),
                    MsgId::AddLinear => Msg::AddLinear(0.0, 0.0),
                    MsgId::AddAngular => Msg::AddAngular(0.0, 0.0),

                    // Config
                    MsgId::LinearP => Msg::LinearP(0.0),
                    MsgId::LinearI => Msg::LinearI(0.0),
                    MsgId::LinearD => Msg::LinearD(0.0),
                    MsgId::LinearAcc => Msg::LinearAcc(0.0),
                    MsgId::AngularP => Msg::AngularP(0.0),
                    MsgId::AngularI => Msg::AngularI(0.0),
                    MsgId::AngularD => Msg::AngularD(0.0),
                    MsgId::AngularAcc => Msg::AngularAcc(0.0),
                };

                msg.generate_bytes(&mut uart).ignore();
            }
        }

        battery.update(now);
    }
}

