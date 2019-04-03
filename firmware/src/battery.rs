use stm32f4xx_hal::stm32 as stm32f405;

const DEAD_VOLTAGE: u16 = 2000;
const DEAD_TIME: u32 = 5000;

pub struct Battery {
    adc: stm32f405::ADC1,
    last_alive: Option<u32>,
    last_update: Option<u32>,
}

impl Battery {
    pub fn setup(
        rcc: &stm32f405::RCC,
        gpiob: &stm32f405::GPIOB,
        adc: stm32f405::ADC1,
    ) -> Battery {
        rcc.apb2enr.modify(|_, w| w.adc1en().set_bit());
        rcc.ahb1enr.write(|w| w.gpioben().set_bit());

        gpiob.moder.modify(|_, w| w.moder1().analog());

        adc.sqr1.write(|w| w.l().bits(0));
        adc.sqr3.write(|w| unsafe { w.sq1().bits(9) });
        adc.cr2.write(|w| w.swstart().set_bit().cont().set_bit());

        adc.cr2.modify(|_, w| w.adon().set_bit());

        Battery {
            adc,
            last_alive: None,
            last_update: None,
        }
    }

    pub fn raw(&self) -> u16 {
        let raw = self.adc.dr.read().data().bits();
        self.adc.cr2.modify(|_, w| w.swstart().set_bit());
        raw
    }

    pub fn update(&mut self, now: u32) {
        if self.raw() > DEAD_VOLTAGE {
            self.last_alive = Some(now);
        }

        self.last_update = Some(now);
    }

    pub fn is_dead(&self) -> bool {
        match (self.last_alive, self.last_update) {
            (Some(alive), Some(update)) => update - alive > DEAD_TIME,
            _ => true,
        }
    }
}
