#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        use colored::Colorize;
        eprintln!("{}: {}", "Error".red(), format!($($arg)*).red());
    };
}
