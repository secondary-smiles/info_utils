#[macro_export]
macro_rules! log {
    ($msg:expr) => {
        {
            eprintln!("\x1b[1;34mINFO:\x1b[0m");
            for line in $msg.lines() {
                eprintln!("  {:?} | {}", std::thread::current().name().unwrap_or("unknown"), line);
            }
        }
    }
}