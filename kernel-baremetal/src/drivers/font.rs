pub const FONT_WIDTH: u32 = 8;
pub const FONT_HEIGHT: u32 = 16;
pub const FONT_FIRST_CHAR: u8 = 0x20;
pub const FONT_LAST_CHAR: u8 = 0x7E;

static FONT_DATA: &[u8] = include_bytes!("font.bin");

pub fn get_char_rows(c: u8) -> &'static [u8; 16] {
    if c < FONT_FIRST_CHAR || c > FONT_LAST_CHAR {
        // return space for out-of-range
        let idx = (b' ' - FONT_FIRST_CHAR) as usize;
        let start = idx * 16;
        unsafe { &*(FONT_DATA.as_ptr().add(start) as *const [u8; 16]) }
    } else {
        let idx = (c - FONT_FIRST_CHAR) as usize;
        let start = idx * 16;
        unsafe { &*(FONT_DATA.as_ptr().add(start) as *const [u8; 16]) }
    }
}
