use crate::{drivers::serial, kernel_init};

#[test]
fn kernel_init_writes_boot_banner_to_serial() {
    serial::init();
    kernel_init();
    assert!(serial::buffer_len() > 0);

    let target = b"[info] kernel boot sequence complete\n";
    let buflen = serial::buffer_len();
    if buflen >= target.len() {
        let mut found = false;
        let mut start = 0;
        while start <= buflen - target.len() {
            let mut match_pos = true;
            let mut j = 0;
            while j < target.len() {
                if serial::buffer_at(start + j) != Some(target[j]) {
                    match_pos = false;
                    break;
                }
                j += 1;
            }
            if match_pos {
                found = true;
                break;
            }
            start += 1;
        }
        assert!(found, "boot banner not found in serial buffer");
    } else {
        panic!(
            "buffer too short: {} < {}",
            buflen,
            target.len()
        );
    }
}
