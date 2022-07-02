use std::fmt::Display;
use std::io;

use once_cell::sync::OnceCell;

use parking_lot::Mutex;

use crate::{Config, XMT};

static INSTANCE: OnceCell<Mutex<XMT>> = OnceCell::new();

/// Initialize the global XMT instance with the provided configuration.
///
/// # Example
/// ```rust
/// xmt::init(xmt::Config::default().with_json_output());
/// ```
#[allow(unused_must_use)]
pub fn init(cfg: Config) {
    let mut instance = XMT::new(cfg);
    let mtx = get_instance();
    let mut guard = mtx.lock();
    std::mem::swap(&mut *guard, &mut instance);
}

/// Initialize the global XMT instance with the default configuration.
///
/// # Example
/// ```rust
/// xmt::init_default();
/// ```
pub fn init_default() {
    init(Config::default())
}

pub fn get_instance() -> &'static Mutex<XMT> {
    INSTANCE.get_or_init(|| Mutex::new(XMT::default()))
}

/// Execute the provided closure in a nested scope within the global XMT instance.
///
/// # Example
/// ```rust
/// xmt::init_default();
/// xmt::print!("Hello");
/// xmt::nest("Begin nested scope", || {
///     xmt::print!("Within scope");
/// });
///
/// // Prints:
/// // Hello
/// // Begin nested scope
/// //   Within scope
/// ```
#[allow(unused_must_use)]
pub fn nest<T, F: FnOnce() -> T>(message: &str, func: F) -> T {
    let mtx = get_instance();

    let orig = {
        let mut guard = mtx.lock();
        let orig = guard.clone();
        orig.print(message);

        let nested = orig.nest();
        *guard = nested;
        orig
    };

    let ret_val = func();

    {
        let mut guard = mtx.lock();
        *guard = orig;
    }

    ret_val
}

/// Prompt the user to select an item from a list.
///
/// # Errors
/// Returns an [io::Error](std::io::Error) error if stdout is not a TTY or if reading from stdin failed.
///
/// # Example
/// ```no_run
///
/// # fn main() -> std::io::Result<()> {
/// xmt::init_default();
/// let choices = vec!["foo", "bar", "baz"];
/// let pick = xmt::pick("Pick one", &choices)?;
/// println!("You picked: {}", pick);
/// # Ok(())
/// # }
/// ```
///
/// # Returns
/// A reference to the item selected by the user.
pub fn pick<'a, E: Display>(msg: &str, items: &'a [E]) -> io::Result<&'a E> {
    let mtx = get_instance();
    mtx.lock().pick(msg, items)
}
