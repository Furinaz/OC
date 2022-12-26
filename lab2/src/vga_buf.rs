const BUF_ADDR: u32 = 0xb8000;
const BUF_HEIGHT: u32 = 25;
const BUF_WIDTH: u32 = 80;

pub const COLOR_LIGHT_GREEN: u8 = 0xa;
pub const COLOR_BLACK: u8 = 0x0;
pub const COLOR_BLUE: u8 = 0x1;
pub const COLOR_GREEN: u8 = 0x2;
pub const COLOR_LIGHT_BLUE: u8 = 0x3;
pub const COLOR_RED: u8 = 0x4;
pub const COLOR_DARK_RED: u8 = 0x5;
pub const COLOR_DARK_GRAY: u8 = 0x8;
pub const COLOR_LIGHT_RED: u8 = 0xc;

pub struct AsciiChar {
    pub char_byte: u8,
    pub color_byte: u8
}

pub enum Alignment {
    Left, 
    Right, 
    Center
}

pub struct Screen {
    buffer: *mut u8,
    color: u8,
    align: Alignment,
    cursor: u32
}

impl core::fmt::Write for Screen {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}

impl Screen {
    
    pub fn new(char_color: u8, bg_color: u8, align: Alignment) -> Screen {
        let color = (bg_color << 4) | char_color;
        return Screen{
            buffer: BUF_ADDR as *mut u8,
            color,
            align,
            cursor: 0
        }
    }

    pub fn print(&mut self, string: &str) {
        let strings = string.split("\n");
        for string in strings {
            self.write_line(string);
            self.new_line();
            if self.cursor / BUF_WIDTH > BUF_HEIGHT { self.scroll_up(); }
        }
    }

    fn write_line(&mut self, string: &str){
        let offset = match self.align {
                Alignment::Left => 0,
                Alignment::Center => ((BUF_WIDTH - string.len() as u32) / 2) as u32,
                Alignment::Right => (BUF_WIDTH - string.len() as u32) as u32
            };
        self.cursor += offset;
        for byte in string.bytes() {
            self.write_char(self.cursor, AsciiChar{char_byte: byte, color_byte: self.color });
            self.cursor += 1;
        }
    }

    fn new_line(&mut self){
        let current_line_width = self.cursor - BUF_WIDTH * (self.cursor / BUF_WIDTH);
        self.cursor += BUF_WIDTH - current_line_width;
    }

    fn scroll_up(&mut self) {
        for i in BUF_WIDTH..self.cursor {
            let char_to_copy = self.read_char(i);
            self.write_char(i - BUF_WIDTH, char_to_copy);
        }
        self.cursor -= BUF_WIDTH;
    }

    pub fn write_char(&mut self, offset: u32, char: AsciiChar) {
        if char.char_byte == 0x0a { self.new_line(); }
        else {
            unsafe {
                *self.buffer.offset(offset as isize * 2) = char.char_byte;
                *self.buffer.offset(offset as isize * 2 + 1) = char.color_byte;
            }
        }
    }

    pub fn read_char(&self, offset: u32) -> AsciiChar {
        unsafe {
            return AsciiChar{
                char_byte: *self.buffer.offset(offset as isize * 2),
                color_byte: *self.buffer.offset(offset as isize * 2 + 1)
            }
        }
    }
}
