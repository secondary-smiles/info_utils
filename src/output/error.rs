///# Error
/// Logs errors and then kills the program with exit code 1.
///
///## ⚠WARNING⚠
/// `error!` creates a hook for `panic!` which also silences the typical error message.
/// This hook applies to ALL uses of panic once `error!` has been invoked.
/// The `panic!` macro will be affected for the rest of your program's runtime.
///
/// Examples
///
/// Kill program
/// ```no_run
///# use info_utils::prelude::*;
///# fn main()  {
///     // Something is happening..
///     // Uh oh, an error has occurred.
///     error!("Something unrecoverable went wrong!");
///# }
///```
///
///Kill thread
/// ```rust
///# use std::thread;
///# use info_utils::prelude::*;
///# fn main() {
/// thread::spawn(|| {
///     // Do a calculation
///     // Oh no, an error happened and we need to kill the thread!
///     error!("Noncritical error, thread aborting");
/// }).join().eval_or_default();
///# }
///```
#[macro_export]
macro_rules! error {
    ($($msg:tt)*) => {{
        let binding = format!("{}",format_args!($($msg)*));
        println!("\x1b[0;1;31merror\x1b[0;36m [{:?}]:\x1b[0m {}", std::thread::current().name().unwrap_or("<unknown>"), binding);

            // eprintln!("\x1b[1;31mERR\x1b[0m\r");
            // println!("\x1b[36m[{:?}]:\x1b[39m\r", std::thread::current().name().unwrap_or("<unknown>"));
            // for line in binding.lines() {
            //     // eprint!("  ");
            //     println!("{}\r", line);
            // }
        std::panic::set_hook(Box::new(|_|{}));
        panic!();
    }}
}
