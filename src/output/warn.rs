#[macro_export]
macro_rules! warn {
    ($msg:expr) => {
        let binding = format!("{}", $msg.clone());
        {
            eprintln!("\x1b[1;33mWARN:\x1b[0m");
            for line in binding.lines() {
                eprintln!("  {}", line);
            }
        }
    }
}