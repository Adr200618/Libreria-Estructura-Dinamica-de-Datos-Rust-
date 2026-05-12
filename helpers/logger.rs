use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
            LogLevel::Debug => "DEBUG",
        }
    }
}

pub struct Logger {
    log_file: Option<String>,
    enabled_levels: Vec<LogLevel>,
    console_output: bool,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            log_file: None,
            enabled_levels: vec![LogLevel::Info, LogLevel::Warning, LogLevel::Error],
            console_output: true,
        }
    }

    pub fn with_file(mut self, file_path: &str) -> Self {
        self.log_file = Some(file_path.to_string());
        self
    }

    pub fn with_levels(mut self, levels: Vec<LogLevel>) -> Self {
        self.enabled_levels = levels;
        self
    }

    pub fn without_console(mut self) -> Self {
        self.console_output = false;
        self
    }

    pub fn enable_debug(&mut self) {
        if !self.enabled_levels.contains(&LogLevel::Debug) {
            self.enabled_levels.push(LogLevel::Debug);
        }
    }

    fn log(&self, level: LogLevel, message: &str) {
        if !self.enabled_levels.contains(&level) {
            return;
        }

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_message = format!("[{}] [{}] {}", timestamp, level.as_str(), message);

        if self.console_output {
            match level {
                LogLevel::Error => eprintln!("{}", log_message),
                _ => println!("{}", log_message),
            }
        }

        if let Some(file_path) = &self.log_file {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path) {
                let _ = writeln!(file, "{}", log_message);
            }
        }
    }

    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    pub fn warning(&self, message: &str) {
        self.log(LogLevel::Warning, message);
    }

    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    pub fn success(&self, message: &str) {
        if self.console_output {
            println!("✅ {}", message);
        }
        self.info(&format!("SUCCESS: {}", message));
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}