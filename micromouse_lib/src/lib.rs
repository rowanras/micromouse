#![no_std]

use core::f64;

pub const CONFIG2019: Config = Config {
    mouse: Mouse {
        wheel_diameter: 32.0,
        gearbox_ratio: 75.0,
        ticks_per_rev: 12.0,
        wheelbase: 72.0,
        width: 64.0,
        length: 88.0,
        front_offset: 48.0,
    },
};

#[derive(Copy, Clone, Debug)]
pub struct Mouse {
    pub wheel_diameter: f64,
    pub gearbox_ratio: f64,
    pub ticks_per_rev: f64,
    pub wheelbase: f64,
    pub width: f64,
    pub length: f64,
    pub front_offset: f64,
}

impl Mouse {
    pub fn ticks_per_mm(&self) -> f64 {
        (self.ticks_per_rev * self.gearbox_ratio) / (self.wheel_diameter * f64::consts::PI)
    }

    pub fn ticks_to_mm(&self, ticks: f64) -> f64 {
        ticks / self.ticks_per_mm()
    }

    pub fn mm_to_ticks(&self, mm: f64) -> f64 {
        mm * self.ticks_per_mm()
    }

    pub fn ticks_per_rad(&self) -> f64 {
        self.mm_to_ticks(self.wheelbase / 2.0)
    }

    pub fn ticks_to_rads(&self, ticks: f64) -> f64 {
        ticks / self.ticks_per_rad()
    }

    pub fn rads_to_ticks(&self, rads: f64) -> f64 {
        rads * self.ticks_per_rad()
    }

    pub fn mm_per_rad(&self) -> f64 {
        self.wheelbase / 2.0
    }

    pub fn mm_to_rads(&self, mm: f64) -> f64 {
        mm / self.mm_per_rad()
    }

    pub fn rads_to_mm(&self, rads: f64) -> f64 {
        rads * self.mm_per_rad()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Config {
    pub mouse: Mouse,
}

