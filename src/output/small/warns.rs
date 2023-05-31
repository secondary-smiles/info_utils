///# Warns
/// Same as `logs!`, but more visually distinct and noticeable.
///
/// Example
/// ```rust
///# use info_utils::prelude::*;
///# fn main() {
///     let data = "important information";
///     warns!("program data: {}", data);
///# }
/// ```
#[macro_export]
macro_rules! warns {
    ($($msg:tt)*) => {{
        let binding = format!("{}",format_args!($($msg)*));
        eprintln!("\x1b[0;1;33mwarn\x1b[0m {}", binding);
    }}
}
