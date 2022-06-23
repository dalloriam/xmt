use std::fmt::Display;
use std::io::{self, Write};

use atty::Stream;

use colored::{Color, Colorize};

use once_cell::sync::Lazy;

use serde::Serialize;

use crate::{Config, Level, OutputMode, Style};

static DEFAULT_PRINT_STYLE: Lazy<Style> = Lazy::new(|| Style {
    prefix: Some(String::from("+")),
    color: Color::White,
});

static DEFAULT_PROMPT_STYLE: Lazy<Style> = Lazy::new(|| Style {
    prefix: Some(String::from("+")),
    color: Color::White,
});

static DEFAULT_SUCCESS_STYLE: Lazy<Style> = Lazy::new(|| Style {
    prefix: Some(String::from("✔")),
    color: Color::Green,
});

static DEFAULT_WARN_STYLE: Lazy<Style> = Lazy::new(|| Style {
    prefix: Some(String::from("!")),
    color: Color::Yellow,
});

static DEFAULT_ERR_STYLE: Lazy<Style> = Lazy::new(|| Style {
    prefix: Some(String::from("!")),
    color: Color::Red,
});

/// Root formatter struct.
#[derive(Clone, PartialEq, Eq)]
pub struct XMT {
    cfg: Config,

    indent_level: usize,

    stdout_tty: bool,
    stderr_tty: bool,
}

impl Default for XMT {
    fn default() -> Self {
        Self {
            cfg: Config::default(),
            indent_level: 0,
            stdout_tty: atty::is(Stream::Stdout),
            stderr_tty: atty::is(Stream::Stderr),
        }
    }
}

impl XMT {
    pub fn new(cfg: Config) -> Self {
        Self {
            cfg,
            ..Default::default()
        }
    }

    fn make_padding(&self) -> String {
        let mut pad = String::new();

        if self.indent_level > 0 {
            pad = String::new();
            for _ in 0..self.indent_level {
                pad += "|   ";
            }
        }
        pad
    }
}

impl XMT {
    #[inline]
    fn is_json_output(&self) -> bool {
        self.cfg.output == OutputMode::JSON
    }

    fn print_sameline(&self, msg: &str, prefix_marker: &Option<String>, color: Color) {
        if self.is_json_output() {
            return;
        }

        if self.stdout_tty {
            let padding = self.make_padding();
            let cs_str = if let Some(mkr) = prefix_marker {
                format!("{padding}{mkr} {msg}")
            } else {
                format!("{padding} {msg}")
            };

            std::print!("{}", cs_str.color(color));
        } else {
            std::print!("{msg}");
        }
    }

    fn print_stdout(&self, msg: &str, prefix_marker: &Option<String>, color: Color) {
        if self.is_json_output() {
            return;
        }

        if self.stdout_tty {
            let padding = self.make_padding();
            let cs_str = if let Some(mkr) = prefix_marker {
                format!("{padding}{mkr} {msg}")
            } else {
                format!("{padding} {msg}")
            };

            println!("{}", cs_str.color(color));
        } else {
            println!("{msg}");
        }
    }

    fn print_stderr(&self, msg: &str, prefix_marker: &Option<String>, color: Color) {
        if self.is_json_output() {
            return;
        }

        if self.stderr_tty {
            let padding = self.make_padding();
            let cs_str = if let Some(mkr) = prefix_marker {
                format!("{padding}{mkr} {msg}")
            } else {
                format!("{padding} {msg}")
            };
            eprintln!("{}", cs_str.color(color));
        } else {
            eprintln!("{msg}");
        }
    }

    pub fn print(&self, msg: &str) {
        let style = self
            .cfg
            .theme
            .get(&Level::Normal)
            .unwrap_or(&DEFAULT_PRINT_STYLE);
        self.print_stdout(msg, &style.prefix, style.color);
    }

    pub fn success(&self, msg: &str) {
        let style = self
            .cfg
            .theme
            .get(&Level::Success)
            .unwrap_or(&DEFAULT_SUCCESS_STYLE);
        self.print_stdout(msg, &style.prefix, style.color);
    }

    pub fn out<S: Serialize + Display>(&self, obj: S) {
        if self.is_json_output() || !self.stdout_tty {
            let out = if self.stdout_tty {
                serde_json::to_string_pretty(&obj)
            } else {
                serde_json::to_string(&obj)
            }
            .expect("value serialization must not fail");
            println!("{}", out);
        } else {
            match self.cfg.output {
                OutputMode::Tree => {
                    let value = serde_value::to_value(&obj).unwrap();
                    ptree::print_tree(&value).unwrap();
                }
                OutputMode::Text => {
                    println!("{}", obj);
                }
                _ => {
                    unreachable!("unreachable because of is_json_output");
                }
            }
        }
    }

    pub fn warn(&self, msg: &str) {
        let style = self
            .cfg
            .theme
            .get(&Level::Warn)
            .unwrap_or(&DEFAULT_WARN_STYLE);
        self.print_stdout(msg, &style.prefix, style.color);
    }

    pub fn error(&self, msg: &str) {
        let style = self
            .cfg
            .theme
            .get(&Level::Error)
            .unwrap_or(&DEFAULT_ERR_STYLE);
        self.print_stderr(msg, &style.prefix, style.color);
    }

    pub fn nest(&self) -> Self {
        let mut copy = self.clone();
        copy.indent_level += 1;
        copy
    }

    pub fn prompt_yn(&self, msg: &str, default: bool) -> io::Result<bool> {
        if !self.stdout_tty {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "interactive features are disabled when not in TTY mode",
            ));
        }

        let style = self
            .cfg
            .theme
            .get(&Level::Prompt)
            .unwrap_or(&DEFAULT_PROMPT_STYLE);

        if default {
            self.print_sameline(&format!("{msg} [Y/n] - "), &style.prefix, style.color);
        } else {
            self.print_sameline(&format!("{msg} [y/N] - "), &style.prefix, style.color);
        }
        let mut user_input = String::new();
        io::stdout().flush()?;
        io::stdin().read_line(&mut user_input)?;

        let user_pick = user_input.trim().to_ascii_lowercase();
        if default {
            Ok(&user_pick != "n")
        } else {
            Ok(&user_pick == "y")
        }
    }

    pub fn prompt(&self, msg: &str) -> io::Result<String> {
        if !self.stdout_tty {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "interactive features are disabled when not in TTY mode",
            ));
        }

        let style = self
            .cfg
            .theme
            .get(&Level::Prompt)
            .unwrap_or(&DEFAULT_PROMPT_STYLE);

        self.print_sameline(msg, &style.prefix, style.color);

        let mut user_input = String::new();
        io::stdout().flush()?;
        io::stdin().read_line(&mut user_input)?;
        Ok(String::from(user_input.trim()))
    }

    pub fn pick<'a, E: Display>(&self, msg: &str, items: &'a [E]) -> io::Result<&'a E> {
        if self.stdout_tty {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "interactive features are disabled when not in TTY mode",
            ));
        }

        self.print(msg);
        for (i, itm) in items.iter().enumerate() {
            self.print(&format!("[{}] - {}", i + 1, itm));
        }

        let pick_idx = loop {
            let pick = self.prompt("Enter your pick: ")?;
            match pick.parse::<usize>() {
                Ok(idx) => {
                    let idx = idx - 1;
                    if idx >= items.len() {
                        self.error("pick is out of bounds");
                    } else {
                        break idx;
                    }
                }
                Err(_) => {
                    self.error("pick must be a positive integer");
                }
            }
        };

        Ok(&items[pick_idx])
    }
}
