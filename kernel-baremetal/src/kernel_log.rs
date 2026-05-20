use crate::drivers::serial;

pub fn info(message: &str) {
    serial::write_str("[info] ");
    serial::write_str(message);
    serial::write_str("\n");
}
