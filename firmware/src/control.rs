use core::f64;
use core::fmt::Write;

use ignore_result::Ignore;

use pid_control::Controller;
use pid_control::DerivativeMode;
use pid_control::PIDController;

use crate::bot::Bot;
use crate::config::BotConfig;

use crate::uart::Command;
use crate::uart::Uart;

pub struct SpinMove {
    spin_pid: PIDController,
    err: f64,
    settle: u32,
    last_ok: u32,
    last_update: u32,
}

impl SpinMove {
    pub fn new(target: f64, config: &BotConfig) -> SpinMove {
        let mut spin_pid =
            PIDController::new(config.spin_p, config.spin_i, config.spin_d);
        spin_pid.set_limits(-2.0, 2.0);
        spin_pid.d_mode = DerivativeMode::OnMeasurement;
        spin_pid.set_target(target);

        SpinMove {
            spin_pid,
            err: config.spin_err,
            settle: config.spin_settle,
            last_update: 0,
            last_ok: 0,
        }
    }

    /**
     *  Update the spin controller
     *
     *  Returns true if the spin controller is done,
     *  false if it is not done.
     */
    pub fn update(&mut self, now: u32, bot: &mut Bot) -> bool {
        let spin_pos = bot.spin_pos();

        let error = spin_pos - self.spin_pid.target();

        if error > self.err || error < -self.err {
            self.last_ok = now;
        }

        if now - self.last_ok > self.settle {
            bot.change_velocity(0.0, 0.0);
            true
        } else {
            let delta_time = now - self.last_update;
            let spin_vel = self.spin_pid.update(spin_pos, delta_time as f64);
            bot.change_velocity(0.0, spin_vel);
            self.last_update = now;
            false
        }
    }
}

pub struct LinearMove {
    linear_pid: PIDController,
    spin_pid: PIDController,
    linear_target: f64,
    last_linear_ok: bool,
    last_spin_ok: bool,
    err: f64,
    settle: u32,
    last_ok: u32,
    last_update: u32,
}

impl LinearMove {
    pub fn new(target: f64, config: &BotConfig) -> LinearMove {
        let mut linear_pid = PIDController::new(
            config.linear_p,
            config.linear_i,
            config.linear_d,
        );

        linear_pid.set_limits(-2.0, 2.0);
        linear_pid.d_mode = DerivativeMode::OnMeasurement;
        linear_pid.set_target(target);

        let mut spin_pid = PIDController::new(
            config.linear_spin_p,
            config.linear_spin_i,
            config.linear_spin_d,
        );

        spin_pid.set_limits(-2.0, 2.0);
        spin_pid.d_mode = DerivativeMode::OnMeasurement;
        spin_pid.set_target(0.0);

        LinearMove {
            linear_pid,
            spin_pid,
            linear_target: target,
            last_linear_ok: false,
            last_spin_ok: false,
            err: config.linear_err,
            settle: config.linear_settle,
            last_update: 0,
            last_ok: 0,
        }
    }

    /**
     *  Update the linear controller
     *
     *  Returns true if the linear controller is done,
     *  false if it is not done.
     */
    pub fn update(&mut self, now: u32, bot: &mut Bot) -> bool {
        let front_distance = bot.front_distance();

        let (linear_pos, linear_target, linear_err) =
            if front_distance <= bot.config.cell_width {
                (
                    -front_distance * 9.0,
                    -bot.config.front_wall_distance * 9.0,
                    bot.config.linear_front_err * 9.0
                )
            } else {
                (
                    bot.linear_pos(),
                    self.linear_target,
                    bot.config.linear_err
                )
            };

        self.linear_pid.set_target(linear_target);

        let linear_error = linear_pos - self.linear_pid.target();
        let linear_ok = linear_error < linear_err && linear_error > -linear_err;

        if linear_ok && !self.last_linear_ok {
            self.linear_pid.reset();
        }

        self.last_linear_ok = linear_ok;

        let left_distance = bot.left_distance();
        let right_distance = bot.right_distance();

        let width = left_distance + right_distance;

        if !linear_ok {
            let spin_target = bot.config.linear_spin_pos_p *
                if width <= bot.config.cell_width {
                    right_distance - left_distance
                } else if left_distance < right_distance {
                    bot.config.cell_offset - left_distance
                } else if right_distance < left_distance {
                    right_distance - bot.config.cell_offset
                } else {
                    0.0
                };
            self.spin_pid.set_target(spin_target);
        }


        let spin_pos = bot.spin_pos();
        let spin_error = spin_pos - self.spin_pid.target();
        let spin_ok = spin_error < self.err && spin_error > -self.err;

        if spin_ok && !self.last_linear_ok {
            self.spin_pid.reset();
        }

        self.last_spin_ok = spin_ok;

        if !linear_ok || !spin_ok {
            self.last_ok = now;
        }

        if now - self.last_ok > self.settle {
            bot.change_velocity(0.0, 0.0);
            true
        } else {
            let delta_time = now - self.last_update;

            let linear_vel =
                self.linear_pid.update(linear_pos, delta_time as f64);

            let spin_vel = self.spin_pid.update(spin_pos, delta_time as f64);

            bot.change_velocity(linear_vel, spin_vel);
            self.last_update = now;
            false
        }
    }
}

enum CurrentMove {
    Idle,
    SpinMove(SpinMove),
    LinearMove(LinearMove),
}

impl CurrentMove {
    pub fn is_idle(&self) -> bool {
        match self {
            CurrentMove::Idle => true,
            _ => false,
        }
    }
}

pub struct Control {
    bot: Bot,
    current_move: CurrentMove,

    last_update: u32,
}

impl Control {
    pub fn new(bot: Bot) -> Control {
        Control {
            bot,
            current_move: CurrentMove::Idle,
            last_update: 0,
        }
    }

    pub fn spin(&mut self, spin_target: f64) {
        if self.current_move.is_idle() {
            let spin_move = SpinMove::new(spin_target, &self.bot.config);
            self.current_move = CurrentMove::SpinMove(spin_move);
        }
    }

    pub fn linear(&mut self, linear_target: f64) {
        if self.current_move.is_idle() {
            let linear_move = LinearMove::new(linear_target, &self.bot.config);
            self.current_move = CurrentMove::LinearMove(linear_move);
        }
    }

    pub fn update(&mut self, now: u32) {
        let delta_time = now - self.last_update;

        if delta_time >= 10 {
            let is_done = match self.current_move {
                CurrentMove::SpinMove(ref mut spin_move) => {
                    spin_move.update(now, &mut self.bot)
                }
                CurrentMove::LinearMove(ref mut linear_move) => {
                    linear_move.update(now, &mut self.bot)
                }
                CurrentMove::Idle => false,
            };

            if is_done {
                self.current_move = CurrentMove::Idle;
                self.bot.reset();
            }

            self.last_update = now;
        }

        self.bot.update(now);
    }

    pub fn is_idle(&self) -> bool {
        self.current_move.is_idle()
    }

    pub fn bot(&self) -> &Bot {
        &self.bot
    }

    pub fn current_move_name(&self) -> &str {
        match self.current_move {
            CurrentMove::SpinMove(_) => "spin",
            CurrentMove::LinearMove(_) => "linear",
            CurrentMove::Idle => "idle",
        }
    }

    pub fn stop(&mut self) {
        self.bot.change_velocity(0.0, 0.0);
        self.bot.reset();
        self.current_move = CurrentMove::Idle;
    }
}

impl Command for Control {
    fn keyword_command(&self) -> &str {
        "control"
    }

    fn handle_command<'a, I: Iterator<Item = &'a str>>(
        &mut self,
        uart: &mut Uart,
        mut args: I,
    ) {
        let command = args.next();

        if command == Some(self.bot.keyword_command()) {
            self.bot.handle_command(uart, args);
        } else {
            match command {
                Some("stop") => {
                    self.bot.change_velocity(0.0, 0.0);
                    self.bot.reset();
                    self.current_move = CurrentMove::Idle;
                }

                Some("spin") => {
                    if let Some(spin_pos) =
                        args.next().and_then(|s| s.parse().ok())
                    {
                        self.spin(spin_pos);
                    } else {
                        writeln!(uart, "No target!").ignore();
                    }
                }

                Some("linear") => {
                    if let Some(linear_pos) =
                        args.next().and_then(|s| s.parse().ok())
                    {
                        self.linear(linear_pos);
                    } else {
                        writeln!(uart, "No target!").ignore();
                    }
                }

                Some("turn") => match args.next() {
                    Some("left") => {
                        self.spin(-self.bot.config.ticks_per_spin / 4.0)
                    }
                    Some("right") => {
                        self.spin(self.bot.config.ticks_per_spin / 4.0)
                    }
                    Some("around") => {
                        self.spin(self.bot.config.ticks_per_spin / 2.0)
                    }
                    _ => writeln!(uart, "control: unknown turn!").ignore(),
                },

                _ => writeln!(uart, "control: unknown command").ignore(),
            }
        }
    }
}
