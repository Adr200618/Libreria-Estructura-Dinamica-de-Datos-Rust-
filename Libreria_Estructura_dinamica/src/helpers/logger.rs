pub struct Logger;

impl Logger {
    pub fn log(msg: &str) {
        println!("[LOG]: {}", msg);
    }
}