use atty::Stream;

use crate::Config;

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
    pub fn print(&self, msg: &str) {
        if self.stdout_tty {
            // TODO: Actually pretty-print
            println!("{}", msg)
        } else {
            println!("{}", msg)
        }
    }
}
