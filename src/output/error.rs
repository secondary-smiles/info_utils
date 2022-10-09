#[macro_export]
macro_rules! error {
    ($msg: expr) => {
        {
            let binding = format!("{}", $msg.clone());
            eprintln!("\x1b[1;31mERR:\x1b[0m");
            for line in binding.lines() {
                eprintln!("  {}", line);
            }
            std::process::exit(1);
        }
    };
    ($msg:expr, $code:expr) => {
        {
            let binding = format!("{}", $msg.clone());
            eprintln!("\x1b[1;31mErr:\x1b[0m");
            for line in binding.lines() {
                eprintln!("  {}", line);
            }
            std::process::exit($code);
        }
    };

}