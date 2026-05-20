const ECAM_BASE: u64 = 0x3F00_0000;
const ECAM_SIZE: u64 = 256 * 1024 * 1024;

fn ecam_offset(bus: u8, dev: u8, func: u8, reg: u8) -> u64 {
    ECAM_BASE
        + ((bus as u64) << 20)
        + ((dev as u64) << 15)
        + ((func as u64) << 12)
        + (reg as u64)
}

fn probe_ecam() -> bool {
    let addr = ECAM_BASE as *const u32;
    unsafe {
        let val = addr.read_volatile();
        val != 0xFFFFFFFF && val != 0x00000000
    }
}

fn read16(addr: u64) -> u16 {
    unsafe { (addr as *const u16).read_volatile() }
}

fn read32(addr: u64) -> u32 {
    unsafe { (addr as *const u32).read_volatile() }
}

fn read_config16(bus: u8, dev: u8, func: u8, reg: u8) -> u16 {
    read16(ecam_offset(bus, dev, func, reg))
}

fn read_config32(bus: u8, dev: u8, func: u8, reg: u8) -> u32 {
    read32(ecam_offset(bus, dev, func, reg))
}

fn log_pci_device(bus: u8, dev: u8, func: u8, vendor: u16, device: u16) {
    let class = read_config32(bus, dev, func, 8);
    let class_code = (class >> 24) as u8;
    let subclass = (class >> 16) as u8;

    crate::drivers::serial::write_str("  PCI ");
    crate::drivers::serial::write_u64_hex(bus as u64);
    crate::drivers::serial::write_str(":");
    crate::drivers::serial::write_u64_hex(dev as u64);
    crate::drivers::serial::write_str(".");
    crate::drivers::serial::write_u64_hex(func as u64);
    crate::drivers::serial::write_str(" vendor=0x");
    crate::drivers::serial::write_u64_hex(vendor as u64);
    crate::drivers::serial::write_str(" device=0x");
    crate::drivers::serial::write_u64_hex(device as u64);
    crate::drivers::serial::write_str(" class=0x");
    crate::drivers::serial::write_u64_hex(class_code as u64);
    crate::drivers::serial::write_str("/0x");
    crate::drivers::serial::write_u64_hex(subclass as u64);
    crate::drivers::serial::write_str("\n");
}

pub fn scan() {
    if !probe_ecam() {
        crate::drivers::serial::write_str("  PCI: ECAM not available at 0x");
        crate::drivers::serial::write_u64_hex(ECAM_BASE);
        crate::drivers::serial::write_str(" (skipping)\n");
        return;
    }

    let mut found = 0;
    let mut dev = 0;
    while dev < 32 {
        let vendor = read_config16(0, dev, 0, 0);
        if vendor != 0xFFFF {
            log_pci_device(0, dev, 0, vendor, read_config16(0, dev, 0, 2));
            found += 1;

            let header_type = read_config16(0, dev, 0, 0x0E) as u8;
            if header_type & 0x80 != 0 {
                let mut func = 1;
                while func < 8 {
                    let v = read_config16(0, dev, func, 0);
                    if v != 0xFFFF {
                        log_pci_device(0, dev, func, v, read_config16(0, dev, func, 2));
                        found += 1;
                    }
                    func += 1;
                }
            }
        }
        dev += 1;
    }

    crate::drivers::serial::write_str("  PCI scan: ");
    crate::drivers::serial::write_u64_dec(found);
    crate::drivers::serial::write_str(" devices\n");
}
