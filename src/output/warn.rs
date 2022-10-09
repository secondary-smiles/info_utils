#[macro_export]
macro_rules! warn {
    ($($msg:tt)*) => {{
        let binding = format!("{}",format_args!($($msg)*));

        eprintln!("\x1b[1;33mWARN\x1b[39m");
        eprintln!("\x1b[36m[{:?}]:\x1b[39m", std::thread::current().name().unwrap_or("<unknown>"));
        for line in binding.lines() {
            eprintln!("  {}", line);
        }
    }}
}