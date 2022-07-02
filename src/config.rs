use std::collections::HashMap;

use colored::Color;

/// Different output levels supported by the library.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Level {
    /// Normal logging level.
    Normal,

    /// Prompt logging level.
    ///
    /// Omitted when stdout is not a TTY.
    Prompt,

    // Normal logging level.
    Success,

    /// Level used for printing formatting that is not critical to the output.
    ///
    /// Omitted when stdout is not a TTY.
    Detail,

    /// Warnings. Printed to stdout.
    Warn,

    /// Errors. Printed to stderr.
    Error,
}

/// A style for a given level.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Style {
    /// The prefix to use for the level.
    pub prefix: Option<String>,

    /// The color to use for the level.
    pub color: Color,
}

impl Style {
    pub fn new(color: Color) -> Self {
        Self {
            prefix: None,
            color,
        }
    }

    pub fn with_prefix(mut self, prefix: String) -> Self {
        self.prefix = Some(prefix);
        self
    }
}

/// The different output modes supported by the library.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputMode {
    /// Prints to stdout using [Display](std::fmt::Display).
    ///
    /// Default.
    Text,

    /// Prints a tree to stdout using [Serialize](serde::Serialize).
    Tree,

    /// Prints a JSON representation of the object to stdout using [Serialize](serde::Serialize).
    ///
    /// Default when stdout is not a TTY.
    JSON,
}

impl Default for OutputMode {
    fn default() -> Self {
        Self::Text
    }
}

/// Configuration for the XMT logger.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config {
    /// The output mode.
    ///
    /// Used to determine how to print values emitted with [xmt::out!](crate::out).
    pub output: OutputMode,

    /// The theme to use for the output.
    ///
    /// The theme is not taken into account when not outputing to a TTY.
    pub theme: HashMap<Level, Style>,
}

impl Config {
    /// Set the style for a given log level.
    ///
    /// Overwrites any previously set style for the given level.
    ///
    /// # Examples
    /// ```rust
    /// use xmt::{Color, Config, Level, Style};
    ///
    /// Config::default().with_style(Level::Normal, Style::new(Color::Red));
    /// ```
    pub fn with_style(mut self, level: Level, style: Style) -> Self {
        self.theme.insert(level, style);
        self
    }

    /// Enables JSON output
    ///
    /// Mutually exclusive with [xmt::Config::with_tree_output](crate::Config::with_tree_output).
    pub fn with_json_output(mut self) -> Self {
        self.output = OutputMode::JSON;
        self
    }

    /// Enables tree output
    ///
    /// Mutually exclusive with [xmt::Config::with_json_output](crate::Config::with_json_output).
    pub fn with_tree_output(mut self) -> Self {
        self.output = OutputMode::Tree;
        self
    }
}
