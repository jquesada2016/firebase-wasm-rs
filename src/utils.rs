#[allow(unused_macros)]
macro_rules! console_log {
    ($($tt:tt)*) => {
        #[cfg(debug_assertions)]
        gloo::console::log!($($tt)*);
    };
}
