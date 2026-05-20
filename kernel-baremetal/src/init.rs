use crate::drivers::serial;
use crate::task;

#[unsafe(no_mangle)]
pub extern "C" fn init_main(_pid: u64) -> ! {
    serial::write_str("[init] starting as pid 1\n");

    loop {
        serial::write_str("[init] idle tick\n");
        task::block_current();
    }
}

pub fn spawn_init() -> Option<u32> {
    let entry = init_main as *const () as usize as u64;
    let pid = task::spawn(entry, 10, false);

    if let Some(p) = pid {
        serial::write_str("[init] spawned with pid ");
        serial::write_u64_dec(p as u64);
        serial::write_str("\n");
    }

    pid
}
