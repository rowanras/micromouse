use core::fmt::Display;
use core::fmt::Write;
use core::str::FromStr;

use ignore_result::Ignore;

use pid_control::Controller;
use pid_control::PIDController;

use crate::motors::left::LeftEncoder;
use crate::motors::left::LeftMotor;
use crate::motors::right::RightEncoder;
use crate::motors::right::RightMotor;

use crate::motors::Encoder;
use crate::motors::Motor;

use crate::uart::Command;
use crate::uart::Uart;

pub struct BotConfig {
    pub left_p: f64,
    pub left_i: f64,
    pub left_d: f64,

    pub right_p: f64,
    pub right_i: f64,
    pub right_d: f64,

    pub spin_p: f64,
    pub spin_i: f64,
    pub spin_d: f64,
    pub spin_err: f64,
    pub spin_settle: u32,

    pub linear_p: f64,
    pub linear_i: f64,
    pub linear_d: f64,
    pub linear_spin_p: f64,
    pub linear_spin_i: f64,
    pub linear_spin_d: f64,
    pub linear_err: f64,
    pub linear_settle: u32,

    pub ticks_per_spin: f64,
    pub ticks_per_cell: f64,
}

impl Command for BotConfig {
    fn keyword_command(&self) -> &str {
        "config"
    }

    fn handle_command<'a, I: Iterator<Item = &'a str>>(
        &mut self,
        uart: &mut Uart,
        mut args: I,
    ) {
        match args.next() {
            Some("left_p") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.left_p = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "left_p: {}", self.left_p).ignore();
                }
            }
            Some("left_i") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.left_i = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "left_i: {}", self.left_i).ignore();
                }
            }
            Some("left_d") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.left_d = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "left_d: {}", self.left_d).ignore();
                }
            }
            Some("right_p") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.right_p = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "right_p: {}", self.right_p).ignore();
                }
            }
            Some("right_i") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.right_i = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "right_i: {}", self.right_i).ignore();
                }
            }
            Some("right_d") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.right_d = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "right_d: {}", self.right_d).ignore();
                }
            }
            Some("spin_p") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.spin_p = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "spin_p: {}", self.spin_p).ignore();
                }
            }
            Some("spin_i") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.spin_i = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "spin_i: {}", self.spin_i).ignore();
                }
            }
            Some("spin_d") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.spin_d = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "spin_d: {}", self.spin_d).ignore();
                }
            }
            Some("spin_err") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.spin_err = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "spin_err: {}", self.spin_err).ignore();
                }
            }
            Some("spin_settle") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.spin_settle = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "spin_settle: {}", self.spin_settle)
                        .ignore();
                }
            }
            Some("linear_p") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.linear_p = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "linear_p: {}", self.linear_p).ignore();
                }
            }
            Some("linear_i") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.linear_i = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "linear_i: {}", self.linear_i).ignore();
                }
            }
            Some("linear_d") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.linear_d = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "linear_d: {}", self.linear_d).ignore();
                }
            }
            Some("linear_spin_p") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.linear_spin_p = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "linear_spin_p: {}", self.linear_spin_p)
                        .ignore();
                }
            }
            Some("linear_spin_i") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.linear_spin_i = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "linear_spin_i: {}", self.linear_spin_i)
                        .ignore();
                }
            }
            Some("linear_spin_d") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.linear_spin_d = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "linear_spin_d: {}", self.linear_spin_d)
                        .ignore();
                }
            }
            Some("linear_err") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.linear_err = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "linear_err: {}", self.linear_err).ignore();
                }
            }
            Some("linear_settle") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.linear_settle = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "linear_settle: {}", self.linear_settle)
                        .ignore();
                }
            }

            Some(_) => writeln!(uart, "config: unknown key").ignore(),
            None => writeln!(uart, "config: Need a key").ignore(),
        }
    }
}

pub struct Bot {
    left_pid: PIDController,
    left_motor: LeftMotor,
    left_encoder: LeftEncoder,
    left_velocity: f64,
    left_power: f64,
    last_left_pos: f64,

    right_pid: PIDController,
    right_motor: RightMotor,
    right_encoder: RightEncoder,
    right_velocity: f64,
    right_power: f64,
    last_right_pos: f64,

    last_update: u32,

    pub config: BotConfig,
}

impl Bot {
    pub fn new(
        left_motor: LeftMotor,
        left_encoder: LeftEncoder,
        right_motor: RightMotor,
        right_encoder: RightEncoder,
        config: BotConfig,
    ) -> Bot {
        let mut left_pid =
            PIDController::new(config.left_p, config.left_i, config.left_d);
        left_pid.set_limits(-5000.0, 5000.0);

        let mut right_pid =
            PIDController::new(config.right_p, config.right_i, config.right_d);
        right_pid.set_limits(-5000.0, 5000.0);

        Bot {
            left_pid,
            left_motor,
            left_encoder,
            left_velocity: 0.0,
            left_power: 0.0,
            last_left_pos: 0.0,
            right_pid,
            right_motor,
            right_encoder,
            right_velocity: 0.0,
            right_power: 0.0,
            last_right_pos: 0.0,
            last_update: 0,
            config,
        }
    }

    pub fn change_velocity(
        &mut self,
        linear_velocity: f64,
        rotational_velocity: f64,
    ) {
        self.left_pid
            .set_target(linear_velocity + rotational_velocity / 2.0);
        self.right_pid
            .set_target(linear_velocity - rotational_velocity / 2.0);

        if linear_velocity == 0.0 && rotational_velocity == 0.0 {
            self.left_pid.reset();
            self.right_pid.reset();
        }
    }

    pub fn update(&mut self, now: u32) {
        let delta_time = now - self.last_update;

        if delta_time > 10 {
            self.left_pid.p_gain = self.config.left_p;
            self.left_pid.i_gain = self.config.left_i;
            self.left_pid.d_gain = self.config.left_d;

            let left_pos = self.left_pos();

            self.left_velocity =
                (left_pos - self.last_left_pos) / delta_time as f64;

            self.left_power =
                self.left_pid.update(self.left_velocity, delta_time as f64);

            self.left_motor.change_power(self.left_power as i32);

            self.last_left_pos = left_pos;

            self.right_pid.p_gain = self.config.right_p;
            self.right_pid.i_gain = self.config.right_i;
            self.right_pid.d_gain = self.config.right_d;

            let right_pos = self.right_pos();

            self.right_velocity =
                (right_pos - self.last_right_pos) / delta_time as f64;

            self.right_power = self
                .right_pid
                .update(self.right_velocity, delta_time as f64);

            self.right_motor.change_power(self.right_power as i32);

            self.last_right_pos = right_pos;

            self.last_update = now;
        }
    }

    pub fn reset(&mut self) {
        self.last_left_pos = 0.0;
        self.left_encoder.reset();

        self.last_right_pos = 0.0;
        self.right_encoder.reset();

        self.left_pid.reset();
        self.right_pid.reset();
    }

    pub fn linear_pos(&self) -> f64 {
        (self.left_pos() + self.right_pos()) / 2.0
    }

    pub fn spin_pos(&self) -> f64 {
        (self.left_pos() - self.right_pos()) / 2.0
    }

    pub fn linear_velocity(&self) -> f64 {
        (self.left_velocity + self.right_velocity) / 2.0
    }

    pub fn spin_velocity(&self) -> f64 {
        self.left_velocity - self.right_velocity
    }

    pub fn left_pos(&self) -> f64 {
        self.left_encoder.count() as f64
    }

    pub fn right_pos(&self) -> f64 {
        self.right_encoder.count() as f64
    }

    pub fn left_velocity(&self) -> f64 {
        self.left_velocity
    }

    pub fn right_velocity(&self) -> f64 {
        self.right_velocity
    }

    pub fn left_target(&self) -> f64 {
        self.left_pid.target()
    }

    pub fn right_target(&self) -> f64 {
        self.right_pid.target()
    }

    pub fn left_power(&self) -> f64 {
        self.left_power
    }

    pub fn right_power(&self) -> f64 {
        self.right_power
    }
}

impl Command for Bot {
    fn keyword_command(&self) -> &str {
        "bot"
    }

    fn handle_command<'a, I: Iterator<Item = &'a str>>(
        &mut self,
        uart: &mut Uart,
        mut args: I,
    ) {
        let command = args.next();

        if command == Some(self.config.keyword_command()) {
            self.config.handle_command(uart, args);
        } else {
            match command {
                Some("spin") => {
                    if let Some(spin_vel) =
                        args.next().and_then(|s| s.parse().ok())
                    {
                        self.change_velocity(0.0, spin_vel);
                    } else {
                        writeln!(uart, "bot: value needed").ignore();
                    }
                }
                Some("linear") => {
                    if let Some(linear_vel) =
                        args.next().and_then(|s| s.parse().ok())
                    {
                        self.change_velocity(linear_vel, 0.0);
                    } else {
                        writeln!(uart, "bot: value needed").ignore();
                    }
                }
                Some(c) => {
                    writeln!(uart, "bot: unknown command: {}", c).ignore()
                }
                None => writeln!(uart, "bot: no command").ignore(),
            }
        }
    }
}
