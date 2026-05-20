use super::font;

#[repr(C)]
pub struct FramebufferInfo {
    pub base: u64,
    pub size: u64,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bpp: u8,
}

static mut FB: FramebufferInfo = FramebufferInfo {
    base: 0,
    size: 0,
    width: 0,
    height: 0,
    pitch: 0,
    bpp: 0,
};

pub fn set_framebuffer(base: u64, size: u64, width: u32, height: u32, pitch: u32) {
    unsafe {
        FB = FramebufferInfo { base, size, width, height, pitch, bpp: 4 };
    }
}

pub fn init() {
    if unsafe { FB.base == 0 } {
        return;
    }
}

pub fn is_available() -> bool {
    unsafe { FB.base != 0 }
}

fn r() -> *const FramebufferInfo {
    &raw const FB
}

pub fn clear(color: u32) {
    let f = unsafe { &*r() };
    if f.base == 0 {
        return;
    }
    let ptr = f.base as *mut u32;
    let total = (f.pitch * f.height) / 4;
    let mut i: u32 = 0;
    while i < total {
        unsafe { ptr.add(i as usize).write_volatile(color) };
        i += 1;
    }
}

pub fn put_pixel(x: u32, y: u32, color: u32) {
    let f = unsafe { &*r() };
    if f.base == 0 || x >= f.width || y >= f.height {
        return;
    }
    let bpp = f.bpp as u32;
    let offset = (y * f.pitch + x * bpp) as isize;
    unsafe {
        (f.base as *mut u8).offset(offset).cast::<u32>().write_volatile(color);
    }
}

pub fn fill_rect(x: u32, y: u32, w: u32, h: u32, color: u32) {
    let f = unsafe { &*r() };
    if f.base == 0 {
        return;
    }
    let bpp = f.bpp as u32;
    let max_y = (y + h).min(f.height);
    let max_x = (x + w).min(f.width);
    let mut row = y;
    while row < max_y {
        let mut col = x;
        while col < max_x {
            let offset = (row * f.pitch + col * bpp) as isize;
            unsafe {
                (f.base as *mut u8).offset(offset).cast::<u32>().write_volatile(color);
            }
            col += 1;
        }
        row += 1;
    }
}

fn write_pixel_at(x: u32, y: u32, color: u32) {
    let f = unsafe { &*r() };
    if f.base == 0 || x >= f.width || y >= f.height {
        return;
    }
    let offset = (y * f.pitch + x * 4) as isize;
    unsafe {
        (f.base as *mut u8).offset(offset).cast::<u32>().write_volatile(color);
    }
}

pub fn put_char(c: u8, x: u32, y: u32, fg: u32, bg: u32) {
    let rows = font::get_char_rows(c);
    let mut row = 0;
    while row < 16 {
        let bits = rows[row as usize];
        let mut col = 0;
        while col < 8 {
            let color = if bits & (1 << (7 - col)) != 0 { fg } else { bg };
            write_pixel_at(x + col, y + row as u32, color);
            col += 1;
        }
        row += 1;
    }
}

pub fn write_str(s: &str, x: u32, y: u32, fg: u32, bg: u32) {
    let mut cx = x;
    for &b in s.as_bytes() {
        if b == b'\n' {
            cx = x;
        } else {
            put_char(b, cx, y, fg, bg);
            cx += font::FONT_WIDTH;
        }
    }
}
