///# Logs
/// Logs noncritical information to the console.
///
/// Example
/// ```rust
///# use info_utils::prelude::*;
///# fn main() {
///     let data = "important information";
///     logs!("program data: {}", data);
///# }
/// ```
#[macro_export]
macro_rules! logs {
    ($($msg:tt)*) => {{
        let binding = format!("{}",format_args!($($msg)*));
        eprintln!("\x1b[0;1;34minfo\x1b[0m {}", binding);
    }}
}
