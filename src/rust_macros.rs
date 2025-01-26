// Macros for Rust, not for Keyboard/Mouse :D

#[macro_export]
macro_rules! boxed {
    ($expr:expr) => {
        Box::new($expr)
    };
}
