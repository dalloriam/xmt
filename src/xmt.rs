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

    /// Print a message.
    ///
    /// If stdout is a TTY, the message will be printed with the style defined by the config for [Level::Normal](crate::Level::Normal).
    /// If stdout is not a TTY, the message is printed with no formatting.
    ///
    /// If the output mode is JSON, the message is not printed.
    ///
    /// # Example
    /// ```rust
    /// use xmt::XMT;
    ///
    /// let xmt = XMT::default();
    /// xmt.print("hello world");
    /// ```
    pub fn print(&self, msg: &str) {
        let style = self
            .cfg
            .theme
            .get(&Level::Normal)
            .unwrap_or(&DEFAULT_PRINT_STYLE);
        self.print_stdout(msg, &style.prefix, style.color);
    }

    /// Print a message.
    ///
    /// If stdout is a TTY, the message will be printed with the style defined by the config for [Level::Detail](crate::Level::Detail).
    /// If stdout is not a TTY or if the output mode is JSON, the message is not printed.
    ///
    /// # Example
    /// ```rust
    /// use xmt::XMT;
    ///
    /// let xmt = XMT::default();
    /// xmt.detail("hello world");
    /// ```
    pub fn detail(&self, msg: &str) {
        let style = self
            .cfg
            .theme
            .get(&Level::Detail)
            .unwrap_or(&DEFAULT_PRINT_STYLE);

        if self.is_json_output() || !self.stdout_tty {
            return;
        }

        let padding = self.make_padding();
        let cs_str = if let Some(mkr) = &style.prefix {
            format!("{padding}{mkr} {msg}")
        } else {
            format!("{padding} {msg}")
        };

        println!("{}", cs_str.color(style.color));
    }

    /// Print a success message.
    ///
    /// If stdout is a TTY, the message will be printed with the style defined by the config for [Level::Success](crate::Level::Success).
    /// If stdout is not a TTY, the message is printed with no formatting.
    ///
    /// # Example
    /// ```rust
    /// use xmt::XMT;
    ///
    /// let xmt = XMT::default();
    /// xmt.success("we did it");
    /// ```
    pub fn success(&self, msg: &str) {
        let style = self
            .cfg
            .theme
            .get(&Level::Success)
            .unwrap_or(&DEFAULT_SUCCESS_STYLE);
        self.print_stdout(msg, &style.prefix, style.color);
    }

    /// Output a structure.
    ///
    /// If output mode is JSON or if stdout is not a TTY, the structure is serialized to JSON and printed to stdout.
    /// If output mode is Tree, the structure is serialized to a tree and printed to stdout.
    /// If output mode is Text, the structure is printed to stdout using [fmt::Display](std::fmt::Display).
    ///
    /// # Example
    /// ```rust
    /// use serde::Serialize;
    /// use xmt::XMT;
    ///
    /// #[derive(Serialize)]
    /// struct Thing {
    ///     name: String,
    /// }
    ///
    /// impl std::fmt::Display for Thing {
    ///    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    ///        write!(f, "{}", self.name)
    ///    }
    /// }
    ///
    /// let xmt = XMT::default();
    /// xmt.out(Thing{name: "thing".to_string()});
    /// ```
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

    /// Print a warning.
    ///
    /// If stdout is a TTY, the message will be printed with the style defined by the config for [Level::Warn](crate::Level::Warn).
    /// If stdout is not a TTY, the message is printed with no formatting.
    ///
    /// If the output mode is set to JSON, the warning is not printed.
    ///
    /// # Example
    /// ```rust
    /// use xmt::XMT;
    ///
    /// let xmt = XMT::default();
    /// xmt.warn("something strange happened");
    /// ```
    pub fn warn(&self, msg: &str) {
        let style = self
            .cfg
            .theme
            .get(&Level::Warn)
            .unwrap_or(&DEFAULT_WARN_STYLE);
        self.print_stdout(msg, &style.prefix, style.color);
    }

    /// Print an error.
    ///
    /// If error is a TTY, the message will be printed with the style defined by the config for [Level::Error](crate::Level::Error).
    /// If error is not a TTY, the message is printed with no formatting.
    ///
    /// If the output mode is set to JSON, the error is not printed.
    ///
    /// # Example
    /// ```rust
    /// use xmt::XMT;
    ///
    /// let xmt = XMT::default();
    /// xmt.error("something bad happened");
    /// ```
    pub fn error(&self, msg: &str) {
        let style = self
            .cfg
            .theme
            .get(&Level::Error)
            .unwrap_or(&DEFAULT_ERR_STYLE);
        self.print_stderr(msg, &style.prefix, style.color);
    }

    /// Execute the provided closure in a nested scope within the global XMT instance.
    ///
    /// # Example
    /// ```rust
    /// use xmt::XMT;
    ///
    /// let xmt = XMT::default();
    ///
    /// xmt.print("Hello");
    /// xmt.nest().print("Within scope");
    ///
    /// // Prints:
    /// // Hello
    /// //   Within scope
    /// ```
    pub fn nest(&self) -> Self {
        let mut copy = self.clone();
        copy.indent_level += 1;
        copy
    }

    /// Prompt the user for a yes/no answer.
    ///
    /// # Errors
    /// Returns an [io::Error](std::io::Error) error if stdout is not a TTY or if reading from stdin failed.
    ///
    /// # Example
    /// ```no_run
    /// use xmt::XMT;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let xmt = XMT::default();
    /// if xmt.prompt_yn("Are you sure?", false)? {
    ///     // do the thing
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Returns
    /// `true` if the user answered yes, `false` if the user answered no.
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

    /// Prompt the user for input.
    ///
    /// # Errors
    /// Returns an [io::Error](std::io::Error) error if stdout is not a TTY or if reading from stdin failed.
    ///
    /// # Example
    /// ```no_run
    /// use xmt::XMT;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let xmt = XMT::default();
    /// let name = xmt.prompt("What is your name?")?;
    /// println!("Hello, {}!", name);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Returns
    /// The text entered by the user.
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

    /// Prompt the user to select an item from a list.
    ///
    /// # Errors
    /// Returns an [io::Error](std::io::Error) error if stdout is not a TTY or if reading from stdin failed.
    ///
    /// # Example
    /// ```no_run
    /// use xmt::XMT;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let xmt = XMT::default();
    ///
    /// let choices = vec!["foo", "bar", "baz"];
    /// let pick = xmt.pick("Pick one", &choices)?;
    /// println!("You picked: {}", pick);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Returns
    /// A reference to the item selected by the user.
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
