///# Log
/// Logs noncritical information to the console.
///
/// Example
/// ```rust
///# use info_utils::prelude::*;
///# fn main() {
///     let data = "important information";
///     log!("program data: {}", data);
///# }
/// ```
#[macro_export]
macro_rules! log {
    ($($msg:tt)*) => {{
        let binding = format!("{}",format_args!($($msg)*));
        println!("\x1b[0;1;34minfo\x1b[0;36m  [{:?}]:\x1b[0m {}", std::thread::current().name().unwrap_or("<unknown>"), binding);

        // eprintln!("\x1b[1;34mINFO\x1b[0m\r");
        // println!("\x1b[36m[{:?}]:\x1b[39m\r", std::thread::current().name().unwrap_or("<unknown>"));
        // for line in binding.lines() {
        //     // eprint!("  ");
        //     println!("{}\r", line);
        // }
    }}
}