#[macro_export]
macro_rules! error {
    ($msg:expr, $code:expr) => {
        {
            eprintln!("\x1b[1;31mErr: {{\x1b[0m");
            for line in $msg.lines() {
                eprintln!("  {}", line);
            }
            eprintln!("\x1b[1;31m}}\x1b[0m");
            std::process::exit($code);
        }
    };
    ($msg: expr) => {
        {
            eprintln!("\x1b[1;31mErr: {{\x1b[0m");
            for line in $msg.lines() {
                eprintln!("  {}", line);
            }
            eprintln!("\x1b[1;31m}}\x1b[0m");
            std::process::exit(1);
        }
    }
}