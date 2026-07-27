#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        use colored::Colorize;
        eprintln!("{}: {}", "error".red(), format!($($arg)*).red());
    }};
}
