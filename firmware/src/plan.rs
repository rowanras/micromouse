use core::fmt::Write;

use ignore_result::Ignore;

use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;

use arrayvec::ArrayVec;

use crate::control::Control;

use crate::uart::Uart;
use crate::uart::Command;

#[derive(Copy, Clone)]
pub enum Move {
    TurnLeft,
    TurnRight,
    TurnAround,
    Forward,
}

pub struct Plan {
    control: Control,
    move_buffer: ArrayVec<[Move; 32]>,
    rng: SmallRng,
    going: bool,
}

impl Plan {
    pub fn new(control: Control) -> Plan {
        Plan {
            control,
            move_buffer: ArrayVec::new(),
            rng: SmallRng::from_seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
            going: false,
        }
    }

    pub fn update(&mut self, now: u32) {
        if self.control.is_idle() {
            if let Some(next_move) = self.move_buffer.pop_at(0) {
                let ticks_per_spin = self.control.bot().config.ticks_per_spin;
                let ticks_per_cell = self.control.bot().config.ticks_per_cell;
                match next_move {
                    Move::TurnLeft => self.control.spin(-ticks_per_spin/4.0),
                    Move::TurnRight => self.control.spin(ticks_per_spin/4.0),
                    Move::TurnAround => self.control.spin(ticks_per_spin/2.0),
                    Move::Forward => self.control.linear(ticks_per_cell),
                }
            } else {
                if self.going {
                    let threshold = self.control.bot().config.wall_threshold;
                    let possible_directions = (
                        self.control.bot().left_distance() > threshold,
                        self.control.bot().front_distance() > threshold,
                        self.control.bot().right_distance() > threshold,
                    );

                    let next_moves: &[Move] = match possible_directions {
                        (true, true, true) => {
                            match self.rng.gen_range(0, 3) {
                                0 => &[Move::TurnLeft, Move::Forward],
                                1 => &[Move::TurnRight, Move::Forward],
                                _ => &[Move::Forward],
                            }
                        }

                        (true, false, true) => {
                            match self.rng.gen_range(0, 2) {
                                0 => &[Move::TurnLeft, Move::Forward],
                                _ => &[Move::TurnRight, Move::Forward],
                            }
                        }

                        (false, true, true) => {
                            match self.rng.gen_range(0, 2) {
                                0 => &[Move::TurnRight, Move::Forward],
                                _ => &[Move::Forward],
                            }
                        }

                        (true, true, false) => {
                            match self.rng.gen_range(0, 2) {
                                0 => &[Move::TurnLeft, Move::Forward],
                                _ => &[Move::Forward],
                            }
                        }

                        (false, true, false) => {
                            &[Move::Forward]
                        }

                        (true, false, false) => {
                            &[Move::TurnLeft, Move::Forward]
                        }

                        (false, false, true) => {
                            &[Move::TurnRight, Move::Forward]
                        }

                        (false, false, false) => {
                            &[Move::TurnAround, Move::Forward]
                        }
                    };

                    self.add_moves(next_moves);
                }
            }
        }

        self.control.update(now);
    }

    pub fn add_moves(&mut self, next_moves: &[Move]) {
        for &next_move in next_moves {
            self.move_buffer.try_push(next_move);
        }
    }

    pub fn control(&mut self) -> &mut Control {
        &mut self.control
    }

    pub fn go(&mut self) {
        self.going = true;
    }

    pub fn stop(&mut self) {
        self.going  = false;
        self.control.stop();
    }
}

impl Command for Plan {
    fn keyword_command(&self) -> &str {
        "plan"
    }

    fn handle_command<'a, I: Iterator<Item = &'a str>>(
        &mut self,
        uart: &mut Uart,
        mut args: I,
    ) {
        let command = args.next();

        if command == Some(self.control.keyword_command()) {
            self.control.handle_command(uart, args);
        } else {
            match command {
                Some("left") => self.add_moves(&[Move::TurnLeft]),
                Some("right") => self.add_moves(&[Move::TurnRight]),
                Some("around") => self.add_moves(&[Move::TurnAround]),
                Some("forward") => self.add_moves(&[Move::Forward]),
                Some("go") => self.go(),
                Some("stop") => self.stop(),
                _ => writeln!(uart, "plan: unknown command").ignore(),
            }
        }
    }
}
