use core::fmt::Write;

use ignore_result::Ignore;

use arrayvec::ArrayVec;

use crate::control::Control;

use crate::navigate::Navigate;

use crate::uart::Command;
use crate::uart::Uart;

#[derive(Copy, Clone)]
pub enum Move {
    TurnLeft,
    TurnRight,
    TurnAround,
    Forward,
}

pub struct MoveOptions {
    pub forward: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    pub fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}


pub struct Plan<N>
where
    N: Navigate,
{
    control: Control,
    move_buffer: ArrayVec<[Move; 32]>,
    going: bool,
    navigate: N,
    x_pos: i32,
    y_pos: i32,
    direction: Direction,
}

impl<N> Plan<N>
where
    N: Navigate,
{
    pub fn new(control: Control, navigate: N) -> Plan<N> {
        Plan {
            control,
            move_buffer: ArrayVec::new(),
            going: false,
            navigate,
            x_pos: 0,
            y_pos: 0,
            direction: Direction::Up,
        }
    }

    pub fn update(&mut self, now: u32) {
        if self.control.is_idle() {
            if let Some(next_move) = self.move_buffer.pop_at(0) {
                let ticks_per_spin = self.control.bot().config.ticks_per_spin;
                let ticks_per_cell = self.control.bot().config.ticks_per_cell;
                match next_move {

                    Move::TurnLeft => {
                        self.control.spin(-ticks_per_spin / 4.0);
                        self.direction = self.direction.turn_left();
                    }

                    Move::TurnRight => {
                        self.control.spin(ticks_per_spin / 4.0);
                        self.direction = self.direction.turn_right();
                    }

                    Move::TurnAround => {
                        self.control.spin(ticks_per_spin / 2.0);
                        self.direction().turn_right();
                        self.direction().turn_right();
                    }

                    Move::Forward => {
                        self.control.linear(ticks_per_cell);
                        let (dx, dy) = match self.direction {
                            Direction::Up => (0, 1),
                            Direction::Down => (0, -1),
                            Direction::Left => (-1, 0),
                            Direction::Right => (1, 0),
                        };
                        self.x_pos += dx;
                        self.y_pos += dy;
                    }
                }
            } else {
                if self.going {
                    let threshold = self.control.bot().config.wall_threshold;
                    let move_options = MoveOptions {
                        left: self.control.bot().left_distance() > threshold,
                        forward: self.control.bot().front_distance()
                            > threshold,
                        right: self.control.bot().right_distance() > threshold,
                    };

                    let next_moves = self.navigate.navigate(self.x_pos, self.y_pos, self.direction, move_options);

                    self.add_moves(&next_moves);
                }
            }
        }

        self.control.update(now);
    }

    pub fn add_moves(&mut self, next_moves: &[Option<Move>]) {
        for &next_move in next_moves {
            if let Some(m) = next_move {
                self.move_buffer.try_push(m);
            }
        }
    }

    pub fn control(&mut self) -> &mut Control {
        &mut self.control
    }

    pub fn go(&mut self) {
        self.going = true;
    }

    pub fn stop(&mut self) {
        self.going = false;
        self.control.stop();
    }

    pub fn x_pos(&self) -> i32 {
        self.x_pos
    }

    pub fn y_pos(&self) -> i32 {
        self.y_pos
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }
}

impl<N> Command for Plan<N>
where
    N: Navigate,
{
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
        } else if command == Some(self.navigate.keyword_command()) {
            self.navigate.handle_command(uart, args);
        } else {
            match command {
                Some("left") => self.add_moves(&[Some(Move::TurnLeft)]),
                Some("right") => self.add_moves(&[Some(Move::TurnRight)]),
                Some("around") => self.add_moves(&[Some(Move::TurnAround)]),
                Some("forward") => self.add_moves(&[Some(Move::Forward)]),
                Some("go") => self.go(),
                Some("stop") => self.stop(),
                _ => writeln!(uart, "plan: unknown command").ignore(),
            }
        }
    }
}
