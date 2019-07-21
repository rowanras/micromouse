#![no_std]

pub mod control;
pub mod msgs;
pub mod mouse;

use core::f32;

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

    linear_motion: MotionControl {
        p: 1.0,
        i: 0.0,
        d: 0.0,
        acc: 1.0,
    },

    angular_motion: MotionControl {
        p: 1.0,
        i: 0.0,
        d: 0.0,
        acc: 1.0,
    },
};

#[derive(Copy, Clone, Debug)]
pub struct MotionControl {
    p: f32,
    i: f32,
    d: f32,
    acc: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Mouse {
    pub wheel_diameter: f32,
    pub gearbox_ratio: f32,
    pub ticks_per_rev: f32,
    pub wheelbase: f32,
    pub width: f32,
    pub length: f32,
    pub front_offset: f32,
}

impl Mouse {
    pub fn ticks_per_mm(&self) -> f32 {
        (self.ticks_per_rev * self.gearbox_ratio)
            / (self.wheel_diameter * f32::consts::PI)
    }

    pub fn ticks_to_mm(&self, ticks: f32) -> f32 {
        ticks / self.ticks_per_mm()
    }

    pub fn mm_to_ticks(&self, mm: f32) -> f32 {
        mm * self.ticks_per_mm()
    }

    pub fn ticks_per_rad(&self) -> f32 {
        self.mm_to_ticks(self.wheelbase / 2.0)
    }

    pub fn ticks_to_rads(&self, ticks: f32) -> f32 {
        ticks / self.ticks_per_rad()
    }

    pub fn rads_to_ticks(&self, rads: f32) -> f32 {
        rads * self.ticks_per_rad()
    }

    pub fn mm_per_rad(&self) -> f32 {
        self.wheelbase / 2.0
    }

    pub fn mm_to_rads(&self, mm: f32) -> f32 {
        mm / self.mm_per_rad()
    }

    pub fn rads_to_mm(&self, rads: f32) -> f32 {
        rads * self.mm_per_rad()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Config {
    pub mouse: Mouse,
    pub linear_motion: MotionControl,
    pub angular_motion: MotionControl,
}
