use core::fmt::Write;

use ignore_result::Ignore;

use pid_control::Controller;
use pid_control::DerivativeMode;
use pid_control::PIDController;

use crate::uart::Command;
use crate::uart::Uart;

use crate::motors::Encoder;
use crate::motors::Motor;

enum DriveMode {
    Idle,
    Spin(i32),
}

pub struct MotorControl<'a, LM, LE, RM, RE>
where
    LM: Motor,
    LE: Encoder,
    RM: Motor,
    RE: Encoder,
{
    pid: PIDController,
    period: u32,
    last_time: u32,
    left_motor: LM,
    left_encoder: LE,
    right_motor: RM,
    right_encoder: RE,
    command: &'a str,
}

impl<'a, M, E> MotorControl<'a, M, E>
where
    M: Motor,
    E: Encoder,
{
    pub fn new(
        p: f64,
        i: f64,
        d: f64,
        period: u32,
        motor: M,
        encoder: E,
        command: &'a str,
    ) -> MotorControl<'a, M, E> {
        let mut pid = PIDController::new(p, i, d);
        pid.set_limits(-10000.0, 10000.0);
        pid.d_mode = DerivativeMode::OnMeasurement;

        MotorControl {
            pid,
            period,
            target_position: 0,
            current_position: 0,
            last_time: None,
            last_motor_velocity: 0,
            motor,
            encoder,
            command,
        }
    }

    pub fn update(&mut self, now: u32, target: i32) {
        let delta_t = match self.last_time {
            Some(last_time) => now - last_time,
            None => now,
        };

        if delta_t >= self.period {
            let new_current_position = self.encoder.count();

            self.pid.set_target(target as f64);
            let motor_velocity =
                self.pid.update(new_current_position as f64, delta_t as f64);

            self.motor.change_velocity(motor_velocity as i32);

            self.last_time = Some(now);
            self.target_position = target;
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

impl<'a, M, E> Command for MotorControl<'a, M, E>
where
    M: Motor,
    E: Encoder,
{
    fn keyword_command(&self) -> &str {
        self.command
    }

    fn handle_command<'b, I: Iterator<Item = &'b str>>(
        &mut self,
        uart: &mut Uart,
        mut args: I,
    ) {
        match args.next() {
            Some("p") => {
                if let Some(Ok(p)) = args.next().map(|p| p.parse()) {
                    self.pid.p_gain = p;
                    writeln!(uart, "Set P Gain to {}", p).ignore();
                } else {
                    writeln!(uart, "Value for P required").ignore();
                }
            }
            Some("i") => {
                if let Some(Ok(i)) = args.next().map(|i| i.parse()) {
                    self.pid.i_gain = i;
                    writeln!(uart, "Set I Gain to {}", i).ignore();
                } else {
                    writeln!(uart, "Value for I required").ignore();
                }
            }
            Some("d") => {
                if let Some(Ok(d)) = args.next().map(|d| d.parse()) {
                    self.pid.d_gain = d;
                    writeln!(uart, "Set D Gain to {}", d).ignore();
                } else {
                    writeln!(uart, "Value for D required").ignore();
                }
            }
            Some("pid") => writeln!(
                uart,
                "P: {}, I: {}, D: {}",
                self.pid.p_gain, self.pid.i_gain, self.pid.d_gain
            )
            .ignore(),
            None => writeln!(uart, "Missing motor control command!").ignore(),
            _ => writeln!(uart, "Invalid motor control command!").ignore(),
        };
    }
}
