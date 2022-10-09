#[macro_export]
macro_rules! warn {
    ($msg:expr) => {
        {
            eprintln!("\x1b[1;33mWarn: {{\x1b[0m");
            for line in $msg.lines() {
                eprintln!("  {}", line);
            }
            eprintln!("\x1b[1;33m}}\x1b[0m");
        }
    }
}