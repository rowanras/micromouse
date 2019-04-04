use core::fmt::Write;

use ignore_result::Ignore;

use crate::uart::Uart;
use crate::uart::Command;

#[derive(Debug)]
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
    pub linear_spin_pos_p: f64,
    pub linear_err: f64,
    pub linear_settle: u32,

    pub ticks_per_spin: f64,
    pub ticks_per_cell: f64,

    pub cell_width: f64,
    pub cell_offset: f64,
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
            Some("linear_spin_pos_p") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.linear_spin_pos_p = v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "linear_spin_pos_p: {}", self.linear_spin_pos_p)
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
            Some("cell_width") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.cell_width= v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "cell_width:: {}", self.cell_width)
                        .ignore();
                }
            }
            Some("cell_offset") => {
                if let Some(arg) = args.next() {
                    if let Ok(v) = arg.parse() {
                        self.cell_offset= v;
                    } else {
                        writeln!(uart, "invalid value").ignore();
                    }
                } else {
                    writeln!(uart, "cell_offset:: {}", self.cell_offset)
                        .ignore();
                }
            }

            Some(_) => writeln!(uart, "config: unknown key").ignore(),
            None => writeln!(uart, "{:#?}", &self).ignore(),
        }
    }
}
