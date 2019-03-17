use pid_control::Controller;
use pid_control::PIDController;
use pid_control::DerivativeMode;

use crate::motors::Encoder;
use crate::motors::Motor;

pub struct MotorControl<P, M, E>
where
    P: Fn(u32) -> i32,
    M: Motor,
    E: Encoder,
{
    pid: PIDController,
    period: u32,
    target_position: i32,
    current_position: i32,
    calculate_position: P,
    last_time: Option<u32>,
    last_motor_velocity: i32,
    motor: M,
    encoder: E,
}

impl<P, M, E> MotorControl<P, M, E>
where
    P: Fn(u32) -> i32,
    M: Motor,
    E: Encoder,
{
    pub fn new(
        p: f64,
        i: f64,
        d: f64,
        period: u32,
        calculate_position: P,
        motor: M,
        encoder: E,
    ) -> MotorControl<P, M, E> {
        let mut pid = PIDController::new(p, i, d);
        pid.set_limits(-10000.0, 10000.0);
        pid.d_mode = DerivativeMode::OnMeasurement;

        MotorControl {
            pid,
            period,
            target_position: 0,
            current_position: 0,
            calculate_position,
            last_time: None,
            last_motor_velocity: 0,
            motor,
            encoder,
        }
    }

    pub fn update(&mut self, now: u32) {
        let delta_t = match self.last_time {
            Some(last_time) => now - last_time,
            None => now,
        };

        if delta_t >= self.period {

            let new_target_position = (self.calculate_position)(now);
            let new_current_position = self.encoder.count();

            self.pid.set_target(new_target_position as f64);
            let motor_velocity = self
                .pid
                .update(new_current_position as f64, delta_t as f64);

            self.motor.change_velocity(motor_velocity as i32);

            self.last_time = Some(now);
            self.target_position = new_target_position;
            self.current_position = new_current_position;
            self.last_motor_velocity = motor_velocity as i32;
        }
    }

    pub fn position(&self) -> i32 {
        self.current_position
    }

    pub fn error(&self) -> i32 {
        self.target_position - self.current_position
    }

    pub fn target(&self) -> i32 {
        self.target_position
    }

    pub fn motor_velocity(&self) -> i32 {
        self.last_motor_velocity
    }
}
