#[macro_export]
macro_rules! log {
    ($msg:expr) => {
        {
            eprintln!("\x1b[1;34mLog: {{\x1b[0m");
            for line in $msg.lines() {
                eprintln!("  {}", line);
            }
            eprintln!("\x1b[1;34m}}\x1b[0m");
        }
    }
}