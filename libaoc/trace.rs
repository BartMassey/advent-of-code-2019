//! Compile-time controlled tracing macro. This macro will
//! cause its arguments to disappear unless the "tracing"
//! feature of `libaoc` is turned on. When the feature is on
//! the macro will print a trace to `stderr`.

#[macro_export]
macro_rules! trace {
    ($fmt:expr, $($arg:tt),*) => {
        if cfg!(feature = "trace") {
            eprintln!($fmt, $($arg),*);
        }
    };
    ($fmt:expr) => {
        trace!($fmt,);
    };
}
