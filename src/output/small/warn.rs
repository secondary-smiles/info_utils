///# Warn
/// Same as `log!`, but more visually distinct and noticeable.
///
/// Example
/// ```rust
///# use info_utils::output::small::*;
///# fn main() {
///     let data = "important information";
///     warn!("program data: {}", data);
///# }
/// ```
#[macro_export]
macro_rules! warn {
    ($($msg:tt)*) => {{
        let binding = format!("{}",format_args!($($msg)*));
        eprintln!("\x1b[0;1;33mwarn\x1b[0m {}", binding);
    }}
}
