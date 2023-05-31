///# Log
/// Logs noncritical information to the console.
///
/// Example
/// ```rust
///# use info_utils::output::small::*;
///# fn main() {
///     let data = "important information";
///     log!("program data: {}", data);
///# }
/// ```
#[macro_export]
macro_rules! log {
    ($($msg:tt)*) => {{
        let binding = format!("{}",format_args!($($msg)*));
        eprintln!("\x1b[0;1;34minfo :\x1b[0m {}", binding);
    }}
}
