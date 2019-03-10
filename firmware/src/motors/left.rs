use stm32f4::stm32f405;

use crate::motors::Direction;
use crate::motors::Motor;

pub struct LeftMotor {
    timer: stm32f405::TIM4,
}

impl LeftMotor {
    pub fn setup(
        rcc: &stm32f405::RCC,
        timer: stm32f405::TIM4,
        gpio: &stm32f405::GPIOB,
    ) -> LeftMotor{
        // Enable clock for gpio b
        rcc.ahb1enr.modify(|_, w| w.gpioben().set_bit());

        // Enable clock for timer 4
        rcc.apb1enr.modify(|_, w| w.tim4en().set_bit());

        // Set pins to alternate function
        gpio
            .moder
            .modify(|_, w| w.moder6().alternate().moder7().alternate());

        // Set the alternate function to timer 4 channel 1 and 2
        gpio.afrl.modify(|_, w| w.afrl6().af2().afrl7().af2());

        // setup the timer
        timer.psc.write(|w| unsafe { w.psc().bits(10u16) });
        timer.cr1.write(|w| w.arpe().set_bit());
        timer.arr.write(|w| w.arr().bits(10000u32));
        timer.ccr1.write(|w| w.ccr1().bits(5000u32));
        timer.ccr2.write(|w| w.ccr2().bits(5000u32));
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
        timer
            .ccer
            .write(|w| w.cc1e().set_bit().cc2e().clear_bit());
        timer.cr1.modify(|_, w| w.cen().set_bit());

        LeftMotor { timer }
    }

}

impl Motor for LeftMotor {
    fn change_speed(&mut self, speed: u32) {
        self.timer.ccr1.write(|w| w.ccr1().bits(speed));
        self.timer.ccr2.write(|w| w.ccr2().bits(speed));
    }

    fn change_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Forward => self.timer
                .ccer
                .write(|w| w.cc1e().set_bit().cc2e().clear_bit()),
            Direction::Backward => self.timer
                .ccer
                .write(|w| w.cc1e().clear_bit().cc2e().set_bit()),
        }
    }
}

pub struct LeftEncoder {
    timer: stm32f405::TIM2,
}

impl LeftEncoder {
    pub fn setup(
        rcc: &stm32f405::RCC,
        gpioa: &stm32f405::GPIOA,
        gpiob: &stm32f405::GPIOB,
        timer: stm32f405::TIM2,
    ) -> LeftEncoder {
        rcc.ahb1enr
            .modify(|_, w| w.gpioaen().set_bit().gpioben().set_bit());
        rcc.apb1enr.modify(|_, w| w.tim2en().set_bit());

        gpioa.moder.modify(|_, w| w.moder5().alternate());
        gpiob.moder.modify(|_, w| w.moder3().alternate());

        gpioa.afrl.modify(|_, w| w.afrl5().af1());
        gpiob.afrl.modify(|_, w| w.afrl3().af1());

        timer.ccmr1_output.write(|w| unsafe { w.cc1s().bits(0b01).cc2s().bits(0b01) });
        timer.smcr.write(|w| unsafe { w.sms().bits(0b011) });
        timer.ccer.write(|w| w.cc1e().set_bit().cc2e().set_bit());
        timer.cr1.write(|w| w.cen().set_bit());

        LeftEncoder { timer }
    }

    pub fn count(&self) -> u32 {
        self.timer.cnt.read().cnt().bits()
    }
}
