use stm32f4::stm32f405;

static OVERFLOW_VALUE: u32 = 65535;

pub struct Time {
    timer: stm32f405::TIM1,
    last_time: u32,
    overflows: u32,
}

impl Time {
    pub fn setup(rcc: &stm32f405::RCC, timer: stm32f405::TIM1) -> Time {
        // Enable clock for timer 4
        rcc.apb2enr.modify(|_, w| w.tim1en().set_bit());

        // setup the timer
        timer.psc.write(|w| unsafe { w.psc().bits(16000) });
        timer.cr1.modify(|_, w| w.cen().set_bit());

        Time {
            timer,
            last_time: 0,
            overflows: 0,
        }
    }

    #[inline(always)]
    pub fn now(&mut self) -> u32 {
        let current_time = self.timer.cnt.read().cnt().bits() as u32;

        if current_time < self.last_time {
            self.overflows += 1;
        }

        self.last_time = current_time;

        current_time + self.overflows * OVERFLOW_VALUE
    }
}
