use stm32f4::stm32f405;

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
