/// Forwards to the [XMT::print](crate::XMT::print) method of the global instance.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::global::get_instance().lock().print(&format!($($arg)*));
    }
}

/// Forwards to the [XMT::detail](crate::XMT::detail) method of the global instance.
#[macro_export]
macro_rules! detail {
    ($($arg:tt)*) => {
        $crate::global::get_instance().lock().detail(&format!($($arg)*));
    }
}

/// Forwards to the [XMT::success](crate::XMT::success) method of the global instance.
#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        $crate::global::get_instance().lock().success(&format!($($arg)*));
    }
}

/// Forwards to the [XMT::warn](crate::XMT::warn) method of the global instance.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::global::get_instance().lock().warn(&format!($($arg)*));
    }
}

/// Forwards to the [XMT::error](crate::XMT::error) method of the global instance.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::global::get_instance().lock().error(&format!($($arg)*));
    }
}

/// Forwards to the [XMT::out](crate::XMT::out) method of the global instance.
#[macro_export]
macro_rules! out {
    ($arg: expr) => {
        $crate::global::get_instance().lock().out($arg);
    };
}

/// Forwards to the [XMT::prompt](crate::XMT::prompt) method of the global instance.
#[macro_export]
macro_rules! prompt {
    ($($arg:tt)*) => {
        $crate::global::get_instance().lock().prompt(&format!($($arg)*))
    }
}

/// Forwards to the [XMT::prompt_yn](crate::XMT::prompt_yn) method of the global instance.
#[macro_export]
macro_rules! prompt_yn {
    ($($arg:tt)*) => {
        $crate::global::get_instance().lock().prompt_yn(&format!($($arg)*), false)
    }
}
