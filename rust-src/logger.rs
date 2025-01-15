use std::time::SystemTime;
use std::fs::{OpenOptions, File};
use std::io::Write;
use colored::*;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Info,
    Log,
    Warning,
    Error,
}

pub struct Logger {
    log_file: Option<File>,
    console_output: bool,
}

impl Logger {
    pub fn new(log_file_path: Option<&str>, console_output: bool) -> Result<Self, std::io::Error> {
        let log_file = match log_file_path {
            Some(path) => Some(OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)?),
            None => None,
        };

        Ok(Logger {
            log_file,
            console_output,
        })
    }

    fn get_timestamp(&self) -> String {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string()
    }

    fn format_message(&self, level: LogLevel, msg: &str) -> String {
        format!("[{}] [{}] {}\n", self.get_timestamp(), format!("{:?}", level), msg)
    }

    pub fn log(&mut self, level: LogLevel, msg: &str) -> Result<(), std::io::Error> {
        let formatted_msg = self.format_message(level, msg);

        // Write to file
        if let Some(file) = &mut self.log_file {
            file.write_all(formatted_msg.as_bytes())?;
        }

        // Console output
        if self.console_output {
            match level {
                LogLevel::Info => println!("{}", formatted_msg.blue()),
                LogLevel::Log => println!("{}", formatted_msg.green()),
                LogLevel::Warning => println!("{}", formatted_msg.yellow()),
                LogLevel::Error => println!("{}", formatted_msg.red()),
            }
        }

        Ok(())
    }

    pub fn info(&mut self, msg: &str) -> Result<(), std::io::Error> {
        self.log(LogLevel::Info, msg)
    }

    pub fn log_msg(&mut self, msg: &str) -> Result<(), std::io::Error> {
        self.log(LogLevel::Log, msg)
    }

    pub fn warning(&mut self, msg: &str) -> Result<(), std::io::Error> {
        self.log(LogLevel::Warning, msg)
    }

    pub fn error(&mut self, msg: &str) -> Result<(), std::io::Error> {
        self.log(LogLevel::Error, msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        let mut logger = Logger::new(Some("test.log"), true).unwrap();
        logger.info("This is an info message").unwrap();
        logger.log_msg("This is a log message").unwrap();
        logger.warning("This is a warning message").unwrap();
        logger.error("This is an error message").unwrap();
    }
}
