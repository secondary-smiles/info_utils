///# Terror
/// Thread-error macro.
/// Logs error and then panics the thread to abort.
///
///## ⚠WARNING⚠
/// `terror!` creates a hook for `panic!` which also silences the typical error message.
/// This hook applies to ALL uses of panic once `terror!` has been invoked.
/// The `panic!` macro will be affected for the rest of your program's runtime.
///
/// Example
/// ```rust
///# use std::thread;
///# use info_utils::prelude::*;
///# fn main() {
/// thread::spawn(|| {
///     // Do a calculation
///     // Oh no, an error happened but we don't want to kill the entire program
///     terror!("Noncritical error, thread aborting");
/// }).join().eval_or_default();
///# }
#[macro_export]
macro_rules! terror {
    ($($msg:tt)*) => {{
        let binding = format!("{}",format_args!($($msg)*));

            eprintln!("\x1b[1;31mERR\x1b[0m\r");
            println!("\x1b[36m[{:?}]:\x1b[39m\r", std::thread::current().name().unwrap_or("<unknown>"));
            for line in binding.lines() {
                // eprint!("  ");
                println!("{}\r", line);
            }
        std::panic::set_hook(Box::new(|_|{}));
        panic!();
    }}
}
