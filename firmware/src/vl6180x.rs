use ignore_result::Ignore;

use embedded_hal::blocking::i2c;

pub const DEFAULT_ADDRESS: u8 = 0x29;

mod registers {
    #![allow(dead_code)]
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

pub struct VL6180x<I2C>
where
    I2C: i2c::Read + i2c::Write + i2c::WriteRead,
{
    i2c: I2C,

    address: u8,
    scaling: u8,
    ptp_offset: u8,

    range: u8,
}

impl<I2C> VL6180x<I2C>
where
    I2C: i2c::Read + i2c::Write + i2c::WriteRead,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        VL6180x {
            i2c,
            address,
            scaling: 1,
            ptp_offset: 0,
            range: 255,
        }
    }

    fn write_u8(&mut self, reg: u16, data: u8) {
        let buf = [((reg >> 8) & 0xff) as u8, (reg & 0xff) as u8, data];
        self.i2c.write(self.address, &buf).ignore();
    }

    fn write_u16(&mut self, reg: u16, data: u16) {
        let buf = [
            ((reg >> 8) & 0xff) as u8,
            (reg & 0xff) as u8,
            ((data >> 8) & 0xff) as u8,
            (data & 0xff) as u8,
        ];
        self.i2c.write(self.address, &buf).ignore();
    }

    fn read_u8(&mut self, reg: u16) -> u8 {
        let mut buf = [0; 1];
        self.i2c.write(self.address, &reg.to_be_bytes()).ignore();
        self.i2c.read(self.address, &mut buf).ignore();
        u8::from_be_bytes(buf)
    }

    fn read_u16(&mut self, reg: u16) -> u16 {
        let mut buf = [0; 2];
        self.i2c.write(self.address, &reg.to_be_bytes()).ignore();
        self.i2c.read(self.address, &mut buf).ignore();
        u16::from_be_bytes(buf)
    }

    pub fn get_id_bytes(&mut self) -> [u8; 8] {
        let mut buf = [0xde; 8];

        for (reg, byte) in buf.iter_mut().enumerate() {
            *byte = self.read_u8(reg as u16);
        }

        buf
    }

    pub fn init_private_registers(&mut self) {
        // Store part-to-part range offset so it can be adjusted if scaling is changed
        self.ptp_offset =
            self.read_u8(registers::SYSRANGE__PART_TO_PART_RANGE_OFFSET);

        if self.read_u8(registers::SYSTEM__FRESH_OUT_OF_RESET) == 1 {
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

            self.write_u8(registers::SYSTEM__FRESH_OUT_OF_RESET, 0);
        } else {
            // Sensor has already been initialized, so try to get scaling settings by
            // reading registers.

            self.scaling = {
                let s = self.read_u16(registers::RANGE_SCALER);

                if s == registers::SCALAR_VALUES[3] {
                    3
                } else if s == registers::SCALAR_VALUES[2] {
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

    pub fn read_range_status(&mut self) -> u8 {
        self.read_u8(registers::RESULT__RANGE_STATUS) >> 4
    }

    pub fn init_default(&mut self) {
        // "Recommended : Public registers"

        // readout__averaging_sample_period = 48
        self.write_u8(registers::READOUT__AVERAGING_SAMPLE_PERIOD, 0x30);

        // sysals__analogue_gain_light = 6
        // (ALS gain = 1 nominal, actually 1.01 according to Table 14 in datasheet)
        self.write_u8(registers::SYSALS__ANALOGUE_GAIN, 0x46);

        // sysrange__vhv_repeat_rate = 255
        // (auto Very High Voltage temperature recalibration
        // after every 255 range measurements)
        self.write_u8(registers::SYSRANGE__VHV_REPEAT_RATE, 0xFF);

        // sysals__integration_period = 99 (100 ms)
        // AN4545 incorrectly recommends writing to register 0x040;
        // 0x63 should go in the lower byte, which is register 0x041.
        self.write_u16(registers::SYSALS__INTEGRATION_PERIOD, 0x0063);

        // sysrange__vhv_recalibrate = 1 (manually trigger a VHV recalibration)
        self.write_u8(registers::SYSRANGE__VHV_RECALIBRATE, 0x01);

        // "Optional: Public registers"

        // sysrange__intermeasurement_period = 9 (100 ms)
        self.write_u8(registers::SYSRANGE__INTERMEASUREMENT_PERIOD, 0x09);

        // sysals__intermeasurement_period = 49 (500 ms)
        self.write_u8(registers::SYSALS__INTERMEASUREMENT_PERIOD, 0x31);

        // als_int_mode = 4 (ALS new sample ready interrupt);
        // range_int_mode = 4 (range new sample ready interrupt)
        self.write_u8(registers::SYSTEM__INTERRUPT_CONFIG_GPIO, 0x24);

        // Reset other settings to power-on defaults

        // sysrange__max_convergence_time = 49 (49 ms)
        self.write_u8(registers::SYSRANGE__MAX_CONVERGENCE_TIME, 0x31);

        // disable interleaved mode
        self.write_u8(registers::INTERLEAVED_MODE__ENABLE, 0);

        // reset range scaling factor to 1x
        self.set_scaling(1);
    }

    // Implemented using ST's VL6180X API as a reference (STSW-IMG003); see
    // VL6180x_UpscaleSetScaling() in vl6180x_api.c.
    fn set_scaling(&mut self, new_scaling: u8) {
        // default value of SYSRANGE__CROSSTALK_VALID_HEIGHT
        let default_crosstalk_valid_height = 20;

        // do nothing if scaling value is invalid
        if new_scaling < 1 || new_scaling > 3 {
            return;
        }

        self.scaling = new_scaling;

        let scaling = self.scaling;
        let ptp_offset = self.ptp_offset;
        self.write_u16(
            registers::RANGE_SCALER,
            registers::SCALAR_VALUES[scaling as usize],
        );

        // apply scaling on part-to-part offset
        self.write_u8(
            registers::SYSRANGE__PART_TO_PART_RANGE_OFFSET,
            ptp_offset / scaling,
        );

        // apply scaling on CrossTalkValidHeight
        self.write_u8(
            registers::SYSRANGE__CROSSTALK_VALID_HEIGHT,
            default_crosstalk_valid_height / scaling,
        );

        // This function does not apply scaling to RANGE_IGNORE_VALID_HEIGHT.

        // enable early convergence estimate only at 1x scaling
        let rce = self.read_u8(registers::SYSRANGE__RANGE_CHECK_ENABLES);
        self.write_u8(
            registers::SYSRANGE__RANGE_CHECK_ENABLES,
            (rce & 0xFE) | (scaling == 1) as u8,
        );
    }

    // Performs a single-shot ranging measurement
    pub fn read_range_single(&mut self) -> u8 {
        self.write_u8(registers::SYSRANGE__START, 0x01);
        self.read_range_continuous()
    }

    // Starts continuous ranging measurements with the given period in ms
    // (10 ms resolution; defaults to 100 ms if not specified).
    //
    // The period must be greater than the time it takes to perform a
    // measurement. See section 2.4.4 ("Continuous mode limits") in the datasheet
    // for details.
    pub fn start_range_continuous(&mut self, period: u16) {
        let period_reg = (period as i16 / 10) - 1;
        let period_reg = if period_reg < 0 {
            0
        } else if period_reg > 254 {
            254
        } else {
            period_reg
        };

        self.write_u8(
            registers::SYSRANGE__INTERMEASUREMENT_PERIOD,
            period_reg as u8,
        );
        self.write_u8(registers::SYSRANGE__START, 0x03);
    }

    // Returns a range reading when continuous mode is activated
    // (readRangeSingle() also calls this function after starting a single-shot
    // range measurement)
    pub fn read_range_continuous(&mut self) -> u8 {
        while (self.read_u8(registers::RESULT__INTERRUPT_STATUS_GPIO) & 0x04)
            == 0
        {}

        let range = self.read_u8(registers::RESULT__RANGE_VAL);
        self.write_u8(registers::SYSTEM__INTERRUPT_CLEAR, 0x01);

        range
    }

    pub fn start_ranging(&mut self) {
        self.write_u8(registers::SYSRANGE__START, 0x01);
    }

    pub fn update(&mut self) {
        if (self.read_u8(registers::RESULT__INTERRUPT_STATUS_GPIO) * 0x04) != 0
        {
            let range = self.read_u8(registers::RESULT__RANGE_VAL);
            self.write_u8(registers::SYSTEM__INTERRUPT_CLEAR, 0x01);
            self.start_ranging();

            self.range = range;
        }
    }

    pub fn range(&self) -> u8 {
        self.range
    }
}
