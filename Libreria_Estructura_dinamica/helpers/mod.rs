mod file_handler;
mod logger;

pub use file_handler::{read_file, write_file, append_to_file, FileError};
pub use logger::{Logger, LogLevel};