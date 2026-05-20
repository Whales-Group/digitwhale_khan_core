#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[cfg(target_os = "uefi")]
mod uefi_entry {
    use uefi::boot::MemoryType;
    use uefi::mem::memory_map::MemoryMap;
    use uefi::prelude::*;
    use uefi::proto::console::gop::GraphicsOutput;

    use kernel_baremetal::kernel_log;
    use kernel_baremetal::memory::memmap::{self, MemoryRegion};

    #[entry]
    fn efi_main() -> Status {
        kernel_baremetal::drivers::serial::init();
        kernel_log::info("JevKen booting via UEFI on aarch64");

        if let Ok(handle) = boot::get_handle_for_protocol::<GraphicsOutput>() {
            if let Ok(mut gop) = boot::open_protocol_exclusive::<GraphicsOutput>(handle) {
                let mut fb = gop.frame_buffer();
                let base = fb.as_mut_ptr() as u64;
                let size = fb.size() as u64;

                let mode_info = gop.current_mode_info();
                let (width, height) = mode_info.resolution();
                let stride = mode_info.stride();
                let bpp: u32 = 4;
                let pitch = stride as u32 * bpp;

                kernel_baremetal::drivers::framebuffer::set_framebuffer(
                    base, size, width as u32, height as u32, pitch,
                );
                kernel_log::info("framebuffer: GOP captured");
            }
        }

        let mmap = unsafe { uefi::boot::exit_boot_services(MemoryType::LOADER_DATA) };

        let mut regions: [MemoryRegion; 256] = [MemoryRegion::zero(); 256];
        let mut count = 0;
        for desc in mmap.entries() {
            if count >= 256 {
                break;
            }
            let pages: u64 = desc.page_count;
            regions[count] = MemoryRegion {
                phys_start: desc.phys_start,
                phys_end: desc.phys_start + pages * 4096,
                region_type: desc.ty.0,
                attr: desc.att.bits(),
            };
            count += 1;
        }
        memmap::set_memory_map(&regions[..count]);

        kernel_baremetal::kernel_init();

        kernel_baremetal::kernel_log::info("starting scheduler");
        kernel_baremetal::task::schedule();

        loop {
            core::hint::spin_loop();
        }
  }
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
