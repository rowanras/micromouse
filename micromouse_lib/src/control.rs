use micromath::F32Ext;

use pid_control::Controller;
use pid_control::DerivativeMode;
use pid_control::PIDController;

use arrayvec::ArrayVec;

use crate::MotionControl as MotionControlConfig;

pub const TARGET_BUFFER_SIZE: usize = 64;

#[derive(Copy, Clone, Debug, Default)]
pub struct Target {
    pub velocity: f32,
    pub distance: f32,
}

#[derive(Clone, Debug)]
pub struct MotionControl {
    pub pid: PIDController,
    pub target_buffer: ArrayVec<[Target; TARGET_BUFFER_SIZE]>,
    pub target: Target,
    pub position: f32,
    pub velocity: f32,
    pub acceleration: f32,
    pub last_time: f32,

    pub acceleration_time: f32,
    pub deceleration_time: f32,
    pub travel_time: f32,
}

impl MotionControl {
    pub fn new(config: MotionControlConfig) -> MotionControl {
        let mut pid = PIDController::new(config.p as f64, config.i as f64, config.d as f64);
        pid.d_mode = DerivativeMode::OnMeasurement;

        MotionControl {
            pid,
            target_buffer: ArrayVec::new(),
            target: Target::default(),
            position: 0.0,
            velocity: 0.0,
            acceleration: config.acc as f32,
            last_time: 0.0,
            acceleration_time: 0.0,
            deceleration_time: 0.0,
            travel_time: 0.0,
        }
    }

    pub fn queue_target(&mut self, target: Target) -> bool {
        self.target_buffer.try_insert(0, target).is_ok()
    }

    pub fn target(&self) -> Target {
        self.target
    }

    pub fn target_buffer(&self) -> &ArrayVec<[Target; TARGET_BUFFER_SIZE]> {
        &self.target_buffer
    }

    pub fn update(&mut self, now: f32, position: f32) -> (u8, f32) {
        let delta_time = now - self.last_time;

        let mut direction: f32 = if self.target.velocity > 0.0 { 1.0 } else { -1.0 };
        let mut debug = 0;

        if now < self.acceleration_time {
            self.velocity += self.acceleration * delta_time;
            debug = 1;
        } else if now < self.travel_time {
            debug = 2;
        } else if now < self.deceleration_time {
            self.velocity -= self.acceleration * delta_time;
            debug = 3;
        } else {
            self.target = self.target_buffer.pop().unwrap_or_default();
            let next_target = self.target_buffer.first().cloned().unwrap_or_default();

            // https://www.desmos.com/calculator/0otxgkkqor

            let vi: f64 = self.velocity as f64;
            let vm: f64 = self.target.velocity as f64;
            let vf: f64 = (next_target.velocity as f64).min(vm);
            let a: f64 = self.acceleration as f64;
            let d: f64 = self.target.distance as f64;

            direction = if self.target.velocity > 0.0 { 1.0 } else { -1.0 };

            let v: f64 = (0.5 * ((2.0 * (2.0 * d * a + vf * vf + vi * vi)) as f32).sqrt() as f64 ).min(vm);

            let t1: f64 = (v - vi) / a;
            let t3: f64 = (v - vf) / a;
            let t2: f64 = (d - (vi * t1 + 0.5 * (v - vi) * t1 + vf * t3 + 0.5 * (v - vf) * t3)) / v;

            let i1: f64 = t1;
            let i2: f64 = i1 + t2;
            let i3: f64 = i2 + t3;

            self.acceleration_time = now + i1 as f32;
            self.travel_time = now + i2 as f32;
            self.deceleration_time = now + i3 as f32;

            //self.velocity += self.acceleration * delta_time;

            debug = 4;
        }

        self.position += self.velocity * delta_time * direction;

        self.pid.set_target(self.position as f64);
        (debug, self.pid.update(position as f64, delta_time as f64) as f32)
    }
}
