#[macro_export]
macro_rules! error {
    ($msg:expr, $code:expr) => {
        {
            eprintln!("\x1b[1;31mErr:\x1b[0m");
            for line in $msg.lines() {
                eprintln!("  {}", line);
            }
            std::process::exit($code);
        }
    };
    ($msg: expr) => {
        {
            eprintln!("\x1b[1;31mERR\x1b[0m");
            for line in $msg.lines() {
                eprintln!("  {}", line);
            }
            std::process::exit(1);
        }
    }
}