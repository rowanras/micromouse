use core::fmt::Write;

use ignore_result::Ignore;

use arrayvec::ArrayVec;

use crate::uart::Command;
use crate::uart::Uart;

use crate::motors::control::MotorControl;
use crate::motors::Encoder;
use crate::motors::Motor;

pub enum PlanError {
    BufferFull,
}

#[derive(Clone)]
pub enum StopCondition {
    LinearDistance(i32),
    TurnDistance(i32),
}

#[derive(Clone)]
pub struct Move {
    linear_acceleration: i64,
    turn_velocity: i64,
    stop_condition: StopCondition,
}

pub static STRAIGHT: [Move; 3] = [
    Move {
        linear_acceleration: 4,
        turn_velocity: 0,
        stop_condition: StopCondition::LinearDistance(500),
    },
    Move {
        linear_acceleration: 0,
        turn_velocity: 0,
        stop_condition: StopCondition::LinearDistance(1000),
    },
    Move {
        linear_acceleration: -4,
        turn_velocity: 0,
        stop_condition: StopCondition::LinearDistance(500),
    },
];

pub struct Plan<'a, LM, LE, RM, RE>
where
    LM: Motor,
    LE: Encoder,
    RM: Motor,
    RE: Encoder,
{
    move_buffer: ArrayVec<[Move; 16]>,

    move_start_linear_position: i64,
    move_start_turn_position: i64,

    linear_velocity: i64,

    linear_position: i64,
    turn_position: i64,

    left_control: MotorControl<'a, LM, LE>,
    right_control: MotorControl<'a, RM, RE>,

    period: u32,
    last_update_time: u32,
    last_delta_time: u32,

    command: &'a str,
}

impl<'a, LM, LE, RM, RE> Plan<'a, LM, LE, RM, RE>
where
    LM: Motor,
    LE: Encoder,
    RM: Motor,
    RE: Encoder,
{
    pub fn new(
        left_control: MotorControl<'a, LM, LE>,
        right_control: MotorControl<'a, RM, RE>,
        period: u32,
        command: &'a str,
    ) -> Plan<'a, LM, LE, RM, RE> {
        Plan {
            move_buffer: ArrayVec::new(),
            move_start_linear_position: 0,
            move_start_turn_position: 0,
            linear_velocity: 0,
            linear_position: 0,
            turn_position: 0,
            left_control,
            right_control,
            period,
            last_update_time: 0,
            last_delta_time: 0,
            command,
        }
    }

    pub fn add_moves(&mut self, moves: &[Move]) -> Result<usize, PlanError> {
        for m in moves.into_iter() {
            self.move_buffer
                .try_push(m.clone())
                .map_err(|_| PlanError::BufferFull)?;
        }

        Ok(self.move_buffer.len())
    }

    pub fn update(&mut self, now: u32) {
        let delta_time = now - self.last_update_time;

        if delta_time >= self.period {
            // If there is a move to execute, execute it
            // There is lots of math in here
            if let Some(current_move) = self.move_buffer.first() {
                let delta_time = delta_time as i64;

                let linear_velocity = self.linear_velocity
                    + current_move.linear_acceleration * delta_time;

                let turn_velocity = current_move.turn_velocity;

                match current_move.stop_condition {
                    StopCondition::LinearDistance(stop_distance) => {
                        let stop_distance = stop_distance as i64 * 1000;

                        let current_move_distance = self.linear_position
                            - self.move_start_linear_position;

                        let next_move_distance = current_move_distance
                            + linear_velocity * delta_time;

                        if next_move_distance >= stop_distance {
                            // We would go beyond the current move this cycle,
                            // so clip it and go to next move

                            // The turn distance needs to be interpolated based
                            // on how far the linear distance needs to go
                            let needed_delta_time = (stop_distance
                                - self.linear_position)
                                / linear_velocity;

                            self.turn_position +=
                                turn_velocity * needed_delta_time;

                            self.linear_position +=
                                linear_velocity * needed_delta_time;

                            self.move_buffer.pop_at(0);

                            self.move_start_linear_position = self.linear_position;
                            self.move_start_turn_position = self.turn_position;
                        } else {
                            // We are still within this move this cycle
                            self.linear_position +=
                                linear_velocity * delta_time;
                            self.turn_position += turn_velocity * delta_time;
                        }
                    }

                    StopCondition::TurnDistance(stop_distance) => {
                        let stop_distance = stop_distance as i64 * 1000;

                        let current_move_distance =
                            self.turn_position - self.move_start_turn_position;

                        let next_move_distance =
                            current_move_distance + turn_velocity * delta_time;

                        if next_move_distance >= stop_distance {
                            // We would go beyond the current move this cycle.
                            // so clip it and go to next move

                            // The linear distance needs to be interpolated based
                            // on how far the turn distance needs to go
                            let needed_delta_time = (stop_distance
                                - self.turn_position)
                                / turn_velocity;

                            self.turn_position +=
                                turn_velocity * needed_delta_time;
                            self.linear_position +=
                                linear_velocity * needed_delta_time;

                            self.move_buffer.pop_at(0);

                            self.move_start_linear_position = self.linear_position;
                            self.move_start_turn_position = self.turn_position;
                        } else {
                            // We are still within this move this cycle
                            self.linear_position +=
                                linear_velocity * delta_time;
                            self.turn_position +=
                                turn_velocity * delta_time;
                        }
                    }
                }

                self.linear_velocity = linear_velocity;
            }

            self.last_update_time = now;
            self.last_delta_time = delta_time;
        }

        self.left_control.update(
            now,
            ((self.linear_position + self.turn_position) / 1000) as i32,
        );
        self.right_control.update(
            now,
            ((self.linear_position - self.turn_position) / 1000) as i32,
        );
    }

    pub fn linear_acceleration(&self) -> i64 {
        if let Some(current_move) = self.move_buffer.first() {
            current_move.linear_acceleration
        } else {
            0
        }
    }

    pub fn linear_velocity(&self) -> i64 {
        self.linear_velocity
    }

    pub fn turn_velocity(&self) -> i64 {
        if let Some(current_move) = self.move_buffer.first() {
            current_move.turn_velocity
        } else {
            0
        }
    }

    pub fn linear_position(&self) -> i64 {
        self.linear_position
    }

    pub fn turn_position(&self) -> i64 {
        self.turn_position
    }

    pub fn left_control(&self) -> &MotorControl<'a, LM, LE> {
        &self.left_control
    }

    pub fn right_control(&self) -> &MotorControl<'a, RM, RE> {
        &self.right_control
    }

    pub fn delta_time(&self) -> u32 {
        self.last_delta_time
    }
}

impl<'a, LM, LE, RM, RE> Command for Plan<'a, LM, LE, RM, RE>
where
    LM: Motor,
    LE: Encoder,
    RM: Motor,
    RE: Encoder,
{
    fn keyword_command(&self) -> &str {
        self.command
    }

    fn handle_command<'b, I: Iterator<Item = &'b str>>(
        &mut self,
        uart: &mut Uart,
        mut args: I,
    ) {
        let command = args.next();

        if command == Some(self.left_control.keyword_command()) {
            self.left_control.handle_command(uart, args);
        } else if command == Some(self.right_control.keyword_command()) {
            self.right_control.handle_command(uart, args);
        } else if command == Some("straight") {
            match self.add_moves(&STRAIGHT) {
                Ok(n) => {
                    writeln!(uart, "{} moves in queue", n).ignore();
                }

                Err(_) => {
                    writeln!(uart, "Buffer is full").ignore();
                }
            }
        } else {
            writeln!(uart, "{}: Unknown command", self.command).ignore();
        }
    }
}
