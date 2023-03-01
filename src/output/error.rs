///# Error
/// Logs errors and then kills the program with exit code 1.
///
///## ⚠WARNING⚠
/// `error!` will kill the entire program. Using `error!` inside a thread will not stop that thread, it will stop the entire application.
/// `error!` is only meant to be used in situations where a recovery is impossible.
///
/// Example
/// ```no_run
///# use info_utils::prelude::*;
///# fn main()  {
///     // Something is happening..
///     // Uh oh, and error has occurred.
///     error!("Something unrecoverable went wrong!");
///# }
#[macro_export]
macro_rules! error {
    ($($msg:tt)*) => {{
        let binding = format!("{}",format_args!($($msg)*));

            eprintln!("\x1b[1;31mERR\x1b[0m\r");
            println!("\x1b[36m[{:?}]:\x1b[39m\r", std::thread::current().name().unwrap_or("<unknown>"));
            for line in binding.lines() {
                // eprint!("  ");
                println!("{}\r", line);
            }
        std::process::exit(1);
    }}
}