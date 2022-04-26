/// Different output levels supported by the library.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Level {
    /// Normal logging level.
    Normal,

    /// Level used for printing formatting that is not critical to the output.
    ///
    /// Omitted when stdout is not a TTY.
    Detail,

    /// Quiet logging level. Only displayed when in verbose mode.
    Quiet,

    /// Warnings. Printed to stdout.
    Warn,

    /// Errors. Printed to stderr.
    Error,
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config {}
