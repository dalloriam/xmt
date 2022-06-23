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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Style {
    pub prefix: Option<String>,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputMode {
    Text,
    Tree,
    JSON,
}

impl Default for OutputMode {
    fn default() -> Self {
        Self::Text
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config {
    pub output: OutputMode,
    pub theme: HashMap<Level, Style>,
}

impl Config {
    pub fn with_style(mut self, level: Level, style: Style) -> Self {
        self.theme.insert(level, style);
        self
    }

    pub fn with_json_output(mut self) -> Self {
        self.output = OutputMode::JSON;
        self
    }

    pub fn with_tree_output(mut self) -> Self {
        self.output = OutputMode::Tree;
        self
    }
}
