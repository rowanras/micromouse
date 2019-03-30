use stm32f4::stm32f405;

use crate::motors::{Encoder, Motor};

const FORWARD_DEADBAND: i32 = 0;
const BACKWARD_DEADBAND: i32 = 0;

pub struct RightMotor {
    timer: stm32f405::TIM4,
}

impl RightMotor {
    pub fn setup(
        rcc: &stm32f405::RCC,
        timer: stm32f405::TIM4,
        gpio: &stm32f405::GPIOB,
    ) -> RightMotor {
        // Enable clock for gpio b
        rcc.ahb1enr.modify(|_, w| w.gpioben().set_bit());

        // Enable clock for timer 4
        rcc.apb1enr.modify(|_, w| w.tim4en().set_bit());

        // Set pins to alternate function
        gpio.moder
            .modify(|_, w| w.moder6().alternate().moder7().alternate());

        // Set the alternate function to timer 4 channel 1 and 2
        gpio.afrl.modify(|_, w| w.afrl6().af2().afrl7().af2());

        // setup the timer
        timer.psc.write(|w| unsafe { w.psc().bits(10u16) });
        timer.cr1.write(|w| w.arpe().set_bit());
        timer.arr.write(|w| w.arr().bits(10000u32));
        timer.ccr1.write(|w| w.ccr1().bits(0u32));
        timer.ccr2.write(|w| w.ccr2().bits(0u32));
        timer.ccmr1_output.write(|w| unsafe {
            w.oc1m()
                .bits(0b110)
                .oc1pe()
                .set_bit()
                .oc2m()
                .bits(0b110)
                .oc2pe()
                .set_bit()
        });
        timer.egr.write(|w| w.ug().set_bit());
        timer.ccer.write(|w| w.cc1e().clear_bit().cc2e().set_bit());
        timer.cr1.modify(|_, w| w.cen().set_bit());

        RightMotor { timer }
    }
}

impl Motor for RightMotor {
    fn change_power(&mut self, power: i32) {
        self.timer.ccer.write(|w| {
            if power > 0 {
                let speed = (power.abs() + FORWARD_DEADBAND) as u32;
                self.timer.ccr1.write(|w| w.ccr1().bits(speed));
                self.timer.ccr2.write(|w| w.ccr2().bits(speed));
                w.cc1e().clear_bit().cc2e().set_bit()
            } else {
                let speed = (power.abs() + BACKWARD_DEADBAND) as u32;
                self.timer.ccr1.write(|w| w.ccr1().bits(speed));
                self.timer.ccr2.write(|w| w.ccr2().bits(speed));
                w.cc1e().set_bit().cc2e().clear_bit()
            }
        });
    }
}

pub struct RightEncoder {
    timer: stm32f405::TIM5,
}

impl RightEncoder {
    pub fn setup(
        rcc: &stm32f405::RCC,
        gpioa: &stm32f405::GPIOA,
        timer: stm32f405::TIM5,
    ) -> RightEncoder {
        rcc.ahb1enr.modify(|_, w| w.gpioaen().set_bit());
        rcc.apb1enr.modify(|_, w| w.tim5en().set_bit());

        gpioa
            .moder
            .modify(|_, w| w.moder0().alternate().moder1().alternate());

        gpioa.afrl.modify(|_, w| w.afrl0().af2().afrl1().af2());

        timer
            .ccmr1_output
            .write(|w| unsafe { w.cc1s().bits(0b01).cc2s().bits(0b01) });
        timer.smcr.write(|w| unsafe { w.sms().bits(0b011) });
        timer.ccer.write(|w| {
            w.cc1e()
                .set_bit()
                .cc1p()
                .set_bit()
                .cc2e()
                .set_bit()
                .cc2p()
                .clear_bit()
        });
        timer.cr1.write(|w| w.cen().set_bit());

        RightEncoder { timer }
    }
}

impl Encoder for RightEncoder {
    fn count(&self) -> i32 {
        ((self.timer.cnt.read().cnt_h().bits() as i32) << 16)
            + self.timer.cnt.read().cnt_l().bits() as i32
    }

    fn reset(&mut self) {
        self.timer
            .cnt
            .write(|w| unsafe { w.cnt_h().bits(0).cnt_l().bits(0) });
    }
}
