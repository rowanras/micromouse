use pid_control::Controller;
use pid_control::DerivativeMode;
use pid_control::PIDController;

use arrayvec::ArrayVec;

use crate::MotionControl as MotionControlConfig;

const TARGET_BUFFER_SIZE: usize = 64;

#[derive(Copy, Clone, Debug)]
pub struct Target {
    pub velocity: f64,
    pub distance: f64,
}

pub struct MotionControl {
    pid: PIDController,
    target_buffer: ArrayVec<[Target; TARGET_BUFFER_SIZE]>,
    target: Option<Target>,
    position: f64,
    start_postition: f64,
    velocity: f64,
    acceleration: f64,
    last_time: f64,
}

impl MotionControl {
    pub fn new(config: MotionControlConfig) -> MotionControl {
        let mut pid = PIDController::new(config.p as f64, config.i as f64, config.d as f64);
        pid.d_mode = DerivativeMode::OnMeasurement;

        MotionControl {
            pid,
            target_buffer: ArrayVec::new(),
            target: None,
            position: 0.0,
            start_postition: 0.0,
            velocity: 0.0,
            acceleration: config.acc as f64,
            last_time: 0.0,
        }
    }

    pub fn queue_target(&mut self, target: Target) -> bool {
        self.target_buffer.try_insert(0, target).is_ok()
    }

    pub fn update(&mut self, now: f64, position: f64) -> f64 {
        let delta_time = now - self.last_time;

        self.velocity += self.acceleration * delta_time;

        let target_velocity = self.target.map(|t| t.velocity).unwrap_or(0.0);

        if (target_velocity >= 0.0 && self.velocity >= target_velocity)
            || (target_velocity <= 0.0 && self.velocity <= target_velocity)
        {
            self.velocity = target_velocity;
        }

        self.position += self.velocity * delta_time;

        let target_position = self.target.map(|t| t.distance).unwrap_or(0.0)
            + self.start_postition;

        if (target_position >= 0.0 && self.position >= target_position)
            || (target_position <= 0.0 && self.position <= target_position)
        {
            self.position = target_position;
            self.target = self.target_buffer.pop();
            self.start_postition = target_position;
        }

        self.pid.set_target(self.position);
        self.pid.update(position, delta_time)
    }

    pub fn position(&self) -> f64 {
        self.position
    }
}
