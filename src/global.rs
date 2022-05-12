use once_cell::sync::OnceCell;

use parking_lot::Mutex;

use crate::{Config, XMT};

static INSTANCE: OnceCell<Mutex<XMT>> = OnceCell::new();

/// Initialize the global XMT instance with the provided configuration.
#[allow(unused_must_use)]
pub fn init(cfg: Config) {
    let mut instance = XMT::new(cfg);
    let mtx = get_instance();
    let mut guard = mtx.lock();
    std::mem::swap(&mut *guard, &mut instance);
}

/// Initialize the global XMT instance with the default configuration.
pub fn init_default() {
    init(Config::default())
}

pub fn get_instance() -> &'static Mutex<XMT> {
    &INSTANCE.get_or_init(|| Mutex::new(XMT::default()))
}

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
