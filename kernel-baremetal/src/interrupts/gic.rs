#![allow(dead_code)]

const GICD_BASE: u64 = 0x0800_0000;
const GICC_BASE: u64 = 0x0801_0000;

const GICD_CTLR: u64 = 0x0000;
const GICD_TYPER: u64 = 0x0004;
const GICD_IGROUPR: u64 = 0x0080;
const GICD_ISENABLER: u64 = 0x0100;

const GICC_CTLR: u64 = 0x0000;
const GICC_PMR: u64 = 0x0004;

const GICD_CTLR_ENABLE: u32 = 1;
const GICC_CTLR_ENABLE: u32 = 1;
const GICC_PMR_UNMASKED: u32 = 0xFF;

fn read32(base: u64, offset: u64) -> u32 {
    unsafe { ((base + offset) as *const u32).read_volatile() }
}

fn write32(base: u64, offset: u64, val: u32) {
    unsafe { ((base + offset) as *mut u32).write_volatile(val) }
}

pub fn init_gic() {
    #[cfg(target_os = "uefi")]
    {
        let typer = read32(GICD_BASE, GICD_TYPER);
        let int_ids = ((typer >> 16) & 0x1F) * 32;
        let gic_version = typer & 0xF;

        crate::drivers::serial::write_str("  GICD_TYPER=0x");
        crate::drivers::serial::write_u64_hex(typer as u64);
        crate::drivers::serial::write_str(" version=");
        crate::drivers::serial::write_u64_dec(gic_version as u64);
        crate::drivers::serial::write_str("\n");

        write32(GICD_BASE, GICD_CTLR, GICD_CTLR_ENABLE);
        let mut i = 0;
        while i < int_ids {
            let reg = GICD_IGROUPR + (i / 32) as u64 * 4;
            write32(GICD_BASE, reg, 0xFFFFFFFF);
            i += 32;
        }
        i = 0;
        while i < int_ids {
            let reg = GICD_ISENABLER + (i / 32) as u64 * 4;
            write32(GICD_BASE, reg, 0xFFFFFFFF);
            i += 32;
        }

        write32(GICC_BASE, GICC_PMR, GICC_PMR_UNMASKED);
        write32(GICC_BASE, GICC_CTLR, GICC_CTLR_ENABLE);

        crate::kernel_log::info("GIC initialized");
    }
}
