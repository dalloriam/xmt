use once_cell::sync::OnceCell;

use crate::XMT;

static INSTANCE: OnceCell<XMT> = OnceCell::new();

/// Initialize the global XMT instance with the provided configuration.
#[allow(unused_must_use)]
pub fn init() {
    INSTANCE.set(XMT::default());
}

pub fn get_instance() -> &'static XMT {
    &INSTANCE.get_or_init(XMT::default)
}
