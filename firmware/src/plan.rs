use core::fmt::Write;

use ignore_result::Ignore;

use crate::control::Control;

use crate::uart::Uart;
use crate::uart::Command;

struct Plan {
    control: Control,
    move_done: bool,
}

impl Plan {
    pub fn new(control: Control) -> Plan {
        Plan {
            control,
            move_done: true,
        }
    }

    pub fn update(&mut self, now: u32) {
        self.control.update(now);
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
                _ => writeln!(uart, "plan: unknown command").ignore(),
            }
        }
    }
}

