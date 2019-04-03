use stm32f4::stm32f405;

mod VL6180x {
    pub const SLAVE_ADDR: u8 = 0x29;

    pub const IDENTIFICATION__MODEL_ID: u16 = 0x000;
    pub const IDENTIFICATION__MODEL_REV_MAJOR: u16 = 0x001;
    pub const IDENTIFICATION__MODEL_REV_MINOR: u16 = 0x002;
    pub const IDENTIFICATION__MODULE_REV_MAJOR: u16 = 0x003;
    pub const IDENTIFICATION__MODULE_REV_MINOR: u16 = 0x004;
    pub const IDENTIFICATION__DATE_HI: u16 = 0x006;
    pub const IDENTIFICATION__DATE_LO: u16 = 0x007;
    pub const IDENTIFICATION__TIME: u16 = 0x008; // 16-bit
    pub const SYSTEM__MODE_GPIO0: u16 = 0x010;
    pub const SYSTEM__MODE_GPIO1: u16 = 0x011;
    pub const SYSTEM__HISTORY_CTRL: u16 = 0x012;
    pub const SYSTEM__INTERRUPT_CONFIG_GPIO: u16 = 0x014;
    pub const SYSTEM__INTERRUPT_CLEAR: u16 = 0x015;
    pub const SYSTEM__FRESH_OUT_OF_RESET: u16 = 0x016;
    pub const SYSTEM__GROUPED_PARAMETER_HOLD: u16 = 0x017;
    pub const SYSRANGE__START: u16 = 0x018;
    pub const SYSRANGE__THRESH_HIGH: u16 = 0x019;
    pub const SYSRANGE__THRESH_LOW: u16 = 0x01A;
    pub const SYSRANGE__INTERMEASUREMENT_PERIOD: u16 = 0x01B;
    pub const SYSRANGE__MAX_CONVERGENCE_TIME: u16 = 0x01C;
    pub const SYSRANGE__CROSSTALK_COMPENSATION_RATE: u16 = 0x01E; // 16-bit
    pub const SYSRANGE__CROSSTALK_VALID_HEIGHT: u16 = 0x021;
    pub const SYSRANGE__EARLY_CONVERGENCE_ESTIMATE: u16 = 0x022; // 16-bit
    pub const SYSRANGE__PART_TO_PART_RANGE_OFFSET: u16 = 0x024;
    pub const SYSRANGE__RANGE_IGNORE_VALID_HEIGHT: u16 = 0x025;
    pub const SYSRANGE__RANGE_IGNORE_THRESHOLD: u16 = 0x026; // 16-bit
    pub const SYSRANGE__MAX_AMBIENT_LEVEL_MULT: u16 = 0x02C;
    pub const SYSRANGE__RANGE_CHECK_ENABLES: u16 = 0x02D;
    pub const SYSRANGE__VHV_RECALIBRATE: u16 = 0x02E;
    pub const SYSRANGE__VHV_REPEAT_RATE: u16 = 0x031;
    pub const SYSALS__START: u16 = 0x038;
    pub const SYSALS__THRESH_HIGH: u16 = 0x03A;
    pub const SYSALS__THRESH_LOW: u16 = 0x03C;
    pub const SYSALS__INTERMEASUREMENT_PERIOD: u16 = 0x03E;
    pub const SYSALS__ANALOGUE_GAIN: u16 = 0x03F;
    pub const SYSALS__INTEGRATION_PERIOD: u16 = 0x040;
    pub const RESULT__RANGE_STATUS: u16 = 0x04D;
    pub const RESULT__ALS_STATUS: u16 = 0x04E;
    pub const RESULT__INTERRUPT_STATUS_GPIO: u16 = 0x04F;
    pub const RESULT__ALS_VAL: u16 = 0x050; // 16-bit
    pub const RESULT__HISTORY_BUFFER_0: u16 = 0x052; // 16-bit
    pub const RESULT__HISTORY_BUFFER_1: u16 = 0x054; // 16-bit
    pub const RESULT__HISTORY_BUFFER_2: u16 = 0x056; // 16-bit
    pub const RESULT__HISTORY_BUFFER_3: u16 = 0x058; // 16-bit
    pub const RESULT__HISTORY_BUFFER_4: u16 = 0x05A; // 16-bit
    pub const RESULT__HISTORY_BUFFER_5: u16 = 0x05C; // 16-bit
    pub const RESULT__HISTORY_BUFFER_6: u16 = 0x05E; // 16-bit
    pub const RESULT__HISTORY_BUFFER_7: u16 = 0x060; // 16-bit
    pub const RESULT__RANGE_VAL: u16 = 0x062;
    pub const RESULT__RANGE_RAW: u16 = 0x064;
    pub const RESULT__RANGE_RETURN_RATE: u16 = 0x066; // 16-bit
    pub const RESULT__RANGE_REFERENCE_RATE: u16 = 0x068; // 16-bit
    pub const RESULT__RANGE_RETURN_SIGNAL_COUNT: u16 = 0x06C; // 32-bit
    pub const RESULT__RANGE_REFERENCE_SIGNAL_COUNT: u16 = 0x070; // 32-bit
    pub const RESULT__RANGE_RETURN_AMB_COUNT: u16 = 0x074; // 32-bit
    pub const RESULT__RANGE_REFERENCE_AMB_COUNT: u16 = 0x078; // 32-bit
    pub const RESULT__RANGE_RETURN_CONV_TIME: u16 = 0x07C; // 32-bit
    pub const RESULT__RANGE_REFERENCE_CONV_TIME: u16 = 0x080; // 32-bit
    pub const RANGE_SCALER: u16 = 0x096; // 16-bit - see STSW-IMG003 core/inc/vl6180x_def.h
    pub const READOUT__AVERAGING_SAMPLE_PERIOD: u16 = 0x10A;
    pub const FIRMWARE__BOOTUP: u16 = 0x119;
    pub const FIRMWARE__RESULT_SCALER: u16 = 0x120;
    pub const I2C_SLAVE__DEVICE_ADDRESS: u16 = 0x212;
    pub const INTERLEAVED_MODE__ENABLE: u16 = 0x2A3;

    pub const SCALAR_VALUES: [u16; 4] = [0, 253, 127, 84];
}

pub struct LeftDistance {
    i2c: stm32f405::I2C2,
    slave_addr: u8,
    scaling: u8,
    ptp_offset: u8,
}

impl LeftDistance {
    pub fn setup(
        rcc: &stm32f405::RCC,
        gpio: &stm32f405::GPIOB,
        i2c: stm32f405::I2C2,
    ) -> LeftDistance {

        /*
        // Enable clock for i2c
        rcc.apb1enr.modify(|_, w| w.i2c2en().set_bit());
        rcc.ahb1enr.modify(|_, w| w.gpioben().set_bit());
        /*

        // Set pins to alternate function
        gpio.moder
            .modify(|_, w| w.moder10().alternate().moder11().alternate());

        // Set open drain pull up
        gpio.otyper.modify(|_, w| w.ot10().open_drain().ot11().open_drain());
        gpio.pupdr.modify(|_, w| w.pupdr10().pull_up().pupdr11().pull_up());

        // Set the alternate function to i2c
        gpio.afrh.modify(|_, w| w.afrh10().af4().afrh11().af4());

        // Apparently, bit 14 "Should always be kept at 1 by software."
        // According to the datasheet
        i2c.oar1.write(|w| unsafe { w.bits(0x4000) });

        // Set the clocks for i2c
        // http://tath.eu/projects/stm32/stm32-i2c-calculating-clock-control-register/
        i2c.cr2.write(|w| unsafe { w.freq().bits(16) });
        i2c.ccr.write(|w| unsafe { w.f_s().standard().ccr().bits(80) });
        i2c.trise.write(|w| unsafe { w.trise().bits(17) });
        i2c.cr1.write(|w| w.pe().set_bit());
        */

        // STM32F101 I2C bug workaround
        // 1. Disable the I2C peripheral by clearing the PE bit in I2Cx_CR1 register
        i2c.cr1.modify(|_, w| w.pe().clear_bit());

        // 2. Configure the SCL and SDA I/Os as General Purpose Output Open-Drain,
        // High level (Write 1 to GPIOx_ODR).
        gpio.moder.modify(|_, w| w.moder10().output().moder11().output());
        gpio.otyper.modify(|_, w| w.ot10().open_drain().ot11().open_drain());
        gpio.odr.modify(|_, w| w.odr10().set_bit().odr11().set_bit());

        // 3. Check SCL and SDA High level in GPIOx_IDR
        while gpio.idr.read().idr10().bit_is_clear() {}
        while gpio.idr.read().idr11().bit_is_clear() {}

        // 4. Configure the SDA I/O as General Purpose Output Open-Drain,
        // Low level (Write 0 to GPIOx_ODR).
        gpio.moder.modify(|_, w| w.moder11().output());
        gpio.otyper.modify(|_, w| w.ot11().open_drain());
        gpio.odr.modify(|_, w| w.odr11().clear_bit());

        // 5. Check SDA Low level in GPIOx_IDR.
        while gpio.idr.read().idr11().bit_is_set() {}

        // 6. Configure the SCL I/O as General Purpose Output Open-Drain,
        // Low level (Write 0 to GPIOx_ODR).
        gpio.moder.modify(|_, w| w.moder10().output());
        gpio.otyper.modify(|_, w| w.ot10().open_drain());
        gpio.odr.modify(|_, w| w.odr10().clear_bit());

        // 7. Check SCL Low level in GPIOx_IDR.
        while gpio.idr.read().idr10().bit_is_set() {}

        // 8. Configure the SCL I/O as General Purpose Output Open-Drain,
        // High level (Write 1 to GPIOx_ODR).
        gpio.moder.modify(|_, w| w.moder10().output());
        gpio.otyper.modify(|_, w| w.ot10().open_drain());
        gpio.odr.modify(|_, w| w.odr10().set_bit());

        // 9. Check SCL High level in GPIOx_IDR.
        while gpio.idr.read().idr10().bit_is_clear() {}

        // 10. Configure the SDA I/O as General Purpose Output Open-Drain,
        // High level (Write 1 to GPIOx_ODR).
        gpio.moder.modify(|_, w| w.moder11().output());
        gpio.otyper.modify(|_, w| w.ot11().open_drain());
        gpio.odr.modify(|_, w| w.odr11().set_bit());

        // 11. Check   SDA   High   level in GPIOx_IDR
        while gpio.idr.read().idr11().bit_is_clear() {}

        // 12. Configure the SCL and SDA I/Os as Alternate function Open-Drain.
        gpio.afrh.modify(|_, w| w.afrh10().af4().afrh11().af4());
        gpio.otyper.modify(|_, w| w.ot10().open_drain().ot11().open_drain());

        // 13. Set SWRST bit in I2Cx_CR1 register.
        i2c.cr1.modify(|_, w| w.swrst().set_bit());

        // 14. Clear SWRST bit in I2Cx_CR1 register.
        i2c.cr1.modify(|_, w| w.swrst().clear_bit());

        // 15. Enable the I2C peripheral by setting the PE bit in I2Cx_CR1 register.

        rcc.apb1rstr.modify(|_, w| w.i2c2rst().set_bit());

        let mut i = 10000;
        while i > 0 {
            i -= 1;
        }

        rcc.apb1rstr.modify(|_, w| w.i2c2rst().clear_bit());

        // Apparently, bit 14 "Should always be kept at 1 by software."
        // According to the datasheet
        i2c.oar1.modify(|_, w| unsafe { w.bits(0x4000) });

        // Set the clocks for i2c
        // http://tath.eu/projects/stm32/stm32-i2c-calculating-clock-control-register/
        i2c.cr2.modify(|_, w| unsafe { w.freq().bits(16) });
        i2c.ccr.modify(|_, w| unsafe { w.f_s().standard().ccr().bits(80) });
        i2c.trise.modify(|_, w| unsafe { w.trise().bits(17) });
        i2c.cr1.modify(|_, w| w.pe().set_bit());
        i2c.cr1.modify(|_, w| w.stop().set_bit());
        */

        // Enable clock for I2C2
        rcc.apb1enr.modify(|_, w| w.i2c2en().set_bit());

        // Reset I2C2
        rcc.apb1rstr.modify(|_, w| w.i2c2rst().set_bit());
        rcc.apb1rstr.modify(|_, w| w.i2c2rst().clear_bit());

        // Make sure the I2C unit is disabled so we can configure it
        i2c.cr1.modify(|_, w| w.pe().clear_bit());

        // Configure bus frequency into I2C peripheral
        i2c.cr2.write(|w| unsafe { w.freq().bits(16 as u8) });

        // Configure correct rise times
        i2c.trise.write(|w| w.trise().bits(17 as u8));

        // Set clock to standard mode with appropriate parameters for selected speed
        i2c.ccr.write(|w| unsafe {
            w.f_s()
                .clear_bit()
                .duty()
                .clear_bit()
                .ccr()
                .bits(80 as u16)
        });

        // Enable the I2C processing
        i2c.cr1.modify(|_, w| w.pe().set_bit());

        let mut left_distance = LeftDistance {
            i2c,
            scaling: 1,
            slave_addr: VL6180x::SLAVE_ADDR,
            ptp_offset: 0,
        };

        left_distance.init_private();
        left_distance.init_default();

        left_distance
    }

    fn i2c_send_addr(&mut self, addr: u8, write: bool) {
        self.i2c.cr1.modify(|_, w| w.start().set_bit());

        //while self.i2c.sr1.read().tx_e().bit_is_clear() {}
        //while self.i2c.sr1.read().sb().bit_is_clear() {}

        let write = if write { 0 } else { 1 };
        let write_addr = (addr << 1) | write as u8;

        self.i2c.dr.write(|w| w.dr().bits(write_addr));

        while self.i2c.sr1.read().addr().bit_is_clear() {}

        let _sr2 = self.i2c.sr2.read().bits();
    }

    fn i2c_send_byte(&mut self, byte: u8) {
        self.i2c.cr1.modify(|_, w| w.start().set_bit());
        self.i2c.dr.write(|w| w.dr().bits(byte));
        while self.i2c.sr1.read().tx_e().bit_is_clear() {}
    }
    fn i2c_receive_byte(&mut self) -> u8 {
        self.i2c.cr1.modify(|_, w| w.start().set_bit());
        while self.i2c.sr1.read().rx_ne().bit_is_clear() {}
        self.i2c.dr.read().dr().bits()
    }

    fn i2c_stop(&mut self) {
        self.i2c.cr1.write(|w| w.stop().set_bit());
    }

    fn write_bytes(&mut self, reg_addr: u16, data: &[u8]) {
        self.i2c_send_addr(self.slave_addr, true);
        self.i2c_send_byte(((reg_addr >> 8) & 0xff) as u8);
        self.i2c_send_byte((reg_addr & 0xff) as u8);
        for byte in data {
            self.i2c_send_byte(*byte);
        }
        self.i2c_stop();
    }

    fn write_u8(&mut self, reg_addr: u16, data: u8) {
        self.write_bytes(reg_addr, &data.to_be_bytes());
    }

    fn write_u16(&mut self, reg_addr: u16, data: u16) {
        self.write_bytes(reg_addr, &data.to_be_bytes());
    }

    fn write_u32(&mut self, reg_addr: u16, data: u32) {
        self.write_bytes(reg_addr, &data.to_be_bytes());
    }

    fn read_bytes(&mut self, reg_addr: u16, buf: &mut [u8]) {
        self.i2c_send_addr(self.slave_addr, true);
        self.i2c_send_byte(((reg_addr >> 8) & 0xff) as u8);
        self.i2c_send_byte((reg_addr & 0xff) as u8);
        self.i2c_stop();

        self.i2c_send_addr(self.slave_addr, false);
        for byte in buf.iter_mut() {
            *byte = self.i2c_receive_byte();
        }
        self.i2c_stop();
    }

    fn read_u8(&mut self, reg_addr: u16) -> u8 {
        let mut buf = [0; 1];
        self.read_bytes(reg_addr, &mut buf);
        u8::from_be_bytes(buf)
    }

    fn read_u16(&mut self, reg_addr: u16) -> u16 {
        let mut buf = [0; 2];
        self.read_bytes(reg_addr, &mut buf);
        u16::from_be_bytes(buf)
    }

    fn read_u32(&mut self, reg_addr: u16) -> u32 {
        let mut buf = [0; 4];
        self.read_bytes(reg_addr, &mut buf);
        u32::from_be_bytes(buf)
    }

    fn init_private(&mut self) {
        // Store part-to-part range offset so it can be adjusted if scaling is changed
        self.ptp_offset =
            self.read_u8(VL6180x::SYSRANGE__PART_TO_PART_RANGE_OFFSET);

        if self.read_u8(VL6180x::SYSTEM__FRESH_OUT_OF_RESET) == 1 {
            self.scaling = 1;

            self.write_u8(0x207, 0x01);
            self.write_u8(0x208, 0x01);
            self.write_u8(0x096, 0x00);
            self.write_u8(0x097, 0xFD); // RANGE_SCALER = 253
            self.write_u8(0x0E3, 0x00);
            self.write_u8(0x0E4, 0x04);
            self.write_u8(0x0E5, 0x02);
            self.write_u8(0x0E6, 0x01);
            self.write_u8(0x0E7, 0x03);
            self.write_u8(0x0F5, 0x02);
            self.write_u8(0x0D9, 0x05);
            self.write_u8(0x0DB, 0xCE);
            self.write_u8(0x0DC, 0x03);
            self.write_u8(0x0DD, 0xF8);
            self.write_u8(0x09F, 0x00);
            self.write_u8(0x0A3, 0x3C);
            self.write_u8(0x0B7, 0x00);
            self.write_u8(0x0BB, 0x3C);
            self.write_u8(0x0B2, 0x09);
            self.write_u8(0x0CA, 0x09);
            self.write_u8(0x198, 0x01);
            self.write_u8(0x1B0, 0x17);
            self.write_u8(0x1AD, 0x00);
            self.write_u8(0x0FF, 0x05);
            self.write_u8(0x100, 0x05);
            self.write_u8(0x199, 0x05);
            self.write_u8(0x1A6, 0x1B);
            self.write_u8(0x1AC, 0x3E);
            self.write_u8(0x1A7, 0x1F);
            self.write_u8(0x030, 0x00);

            self.write_u8(VL6180x::SYSTEM__FRESH_OUT_OF_RESET, 0);
        } else {
            // Sensor has already been initialized, so try to get scaling settings by
            // reading registers.

            self.scaling = {
                let s = self.read_u16(VL6180x::RANGE_SCALER);

                if s == VL6180x::SCALAR_VALUES[3] {
                    3
                } else if s == VL6180x::SCALAR_VALUES[2] {
                    2
                } else {
                    1
                }
            };

            // Adjust the part-to-part range offset value read earlier to account for
            // existing scaling. If the sensor was already in 2x or 3x scaling mode,
            // precision will be lost calculating the original (1x) offset, but this can
            // be resolved by resetting the sensor and Arduino again.
            self.ptp_offset *= self.scaling;
        }
    }

    fn init_default(&mut self) {
        // "Recommended : Public registers"

        // readout__averaging_sample_period = 48
        self.write_u8(VL6180x::READOUT__AVERAGING_SAMPLE_PERIOD, 0x30);

        // sysals__analogue_gain_light = 6
        // (ALS gain = 1 nominal, actually 1.01 according to Table 14 in datasheet)
        self.write_u8(VL6180x::SYSALS__ANALOGUE_GAIN, 0x46);

        // sysrange__vhv_repeat_rate = 255
        // (auto Very High Voltage temperature recalibration
        // after every 255 range measurements)
        self.write_u8(VL6180x::SYSRANGE__VHV_REPEAT_RATE, 0xFF);

        // sysals__integration_period = 99 (100 ms)
        // AN4545 incorrectly recommends writing to register 0x040;
        // 0x63 should go in the lower byte, which is register 0x041.
        self.write_u16(VL6180x::SYSALS__INTEGRATION_PERIOD, 0x0063);

        // sysrange__vhv_recalibrate = 1 (manually trigger a VHV recalibration)
        self.write_u8(VL6180x::SYSRANGE__VHV_RECALIBRATE, 0x01);

        // "Optional: Public registers"

        // sysrange__intermeasurement_period = 9 (100 ms)
        self.write_u8(VL6180x::SYSRANGE__INTERMEASUREMENT_PERIOD, 0x09);

        // sysals__intermeasurement_period = 49 (500 ms)
        self.write_u8(VL6180x::SYSALS__INTERMEASUREMENT_PERIOD, 0x31);

        // als_int_mode = 4 (ALS new sample ready interrupt);
        // range_int_mode = 4 (range new sample ready interrupt)
        self.write_u8(VL6180x::SYSTEM__INTERRUPT_CONFIG_GPIO, 0x24);

        // Reset other settings to power-on defaults

        // sysrange__max_convergence_time = 49 (49 ms)
        self.write_u8(VL6180x::SYSRANGE__MAX_CONVERGENCE_TIME, 0x31);

        // disable interleaved mode
        self.write_u8(VL6180x::INTERLEAVED_MODE__ENABLE, 0);

        // reset range scaling factor to 1x
        self.set_scaling(1);
    }

    // Implemented using ST's VL6180X API as a reference (STSW-IMG003); see
    // VL6180x_UpscaleSetScaling() in vl6180x_api.c.
    fn set_scaling(&mut self, new_scaling: u8) {
        // default value of SYSRANGE__CROSSTALK_VALID_HEIGHT
        let DefaultCrosstalkValidHeight = 20;

        // do nothing if scaling value is invalid
        if new_scaling < 1 || new_scaling > 3 {
            return;
        }

        self.scaling = new_scaling;
        self.write_u16(
            VL6180x::RANGE_SCALER,
            VL6180x::SCALAR_VALUES[self.scaling as usize],
        );

        // apply scaling on part-to-part offset
        self.write_u8(
            VL6180x::SYSRANGE__PART_TO_PART_RANGE_OFFSET,
            self.ptp_offset / self.scaling,
        );

        // apply scaling on CrossTalkValidHeight
        self.write_u8(
            VL6180x::SYSRANGE__CROSSTALK_VALID_HEIGHT,
            DefaultCrosstalkValidHeight / self.scaling,
        );

        // This function does not apply scaling to RANGE_IGNORE_VALID_HEIGHT.

        // enable early convergence estimate only at 1x scaling
        let rce = self.read_u8(VL6180x::SYSRANGE__RANGE_CHECK_ENABLES);
        self.write_u8(
            VL6180x::SYSRANGE__RANGE_CHECK_ENABLES,
            (rce & 0xFE) | (self.scaling == 1) as u8,
        );
    }

    // Performs a single-shot ranging measurement
    pub fn read_range_single(&mut self) -> u8 {
        self.write_u8(VL6180x::SYSRANGE__START, 0x01);
        self.read_range_continuous()
    }

    // Returns a range reading when continuous mode is activated
    // (readRangeSingle() also calls this function after starting a single-shot
    // range measurement)
    fn read_range_continuous(&mut self) -> u8 {
        while (self.read_u8(VL6180x::RESULT__INTERRUPT_STATUS_GPIO) & 0x04) == 0
        {
        }

        let range = self.read_u8(VL6180x::RESULT__RANGE_VAL);
        self.write_u8(VL6180x::SYSTEM__INTERRUPT_CLEAR, 0x01);

        range
    }
}
