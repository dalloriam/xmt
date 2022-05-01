#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::global::get_instance().lock().print(&format!($($arg)*));
    }
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        $crate::global::get_instance().lock().success(&format!($($arg)*));
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::global::get_instance().lock().warn(&format!($($arg)*));
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::global::get_instance().lock().error(&format!($($arg)*));
    }
}

#[macro_export]
macro_rules! out {
    ($arg: tt) => {
        $crate::global::get_instance().lock().out($arg);
    };
}

#[macro_export]
macro_rules! prompt {
    ($($arg:tt)*) => {
        $crate::global::get_instance().lock().prompt(&format!($($arg)*))
    }
}

#[macro_export]
macro_rules! prompt_yn {
    ($($arg:tt)*) => {
        $crate::global::get_instance().lock().prompt_yn(&format!($($arg)*), false)
    }
}
