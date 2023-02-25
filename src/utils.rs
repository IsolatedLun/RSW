pub fn underscorize(text: String) -> String {
    text
    .to_lowercase()
    .split_whitespace()
    .map(|x| x.to_string())
    .collect::<Vec<String>>()
    .join("_")
}

pub enum LogLevel {
    INFO,
    EXIT,
    SUCCESS,
    WARN,
    ERR,
}

pub fn log(level: LogLevel, message: String) {
    match level {
        LogLevel::INFO => println!("[INFO] {}", message),
        LogLevel::SUCCESS => println!("[SUCCESS] {}", message),
        LogLevel::WARN => println!("[WARN] {}", message),
        LogLevel::ERR => println!("[ERR] {}", message),
        LogLevel::EXIT => println!("[EXIT] {}", message),

    }
}