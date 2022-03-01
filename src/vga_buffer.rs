/// Represent different colors using an enum
#[allow(dead_code)] // To disable warnings for unused variant of Color enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // To enable copy semantics for the type and make it printable and comparable
#[repr(u8)] // So that each enum variant is stored as an u8. 4 bits would be sufficient, but Rust doesn’t have an u4 type.
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// Full color code that specifies foreground and background color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // To ensure that the ColorCode has the exact same data layout as an u8
struct ColorCode(u8);

impl ColorCode {
    fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

/// Represents a screen character
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // It guarantees that the struct’s fields are laid out exactly like in a C struct and thus guarantees the correct field ordering.
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

/// Height of text buffer
const BUFFER_HEIGHT: usize = 25;

/// Width of text buffer
const BUFFER_WIDTH: usize = 80;

/// Represents a VGA text buffer
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Allows writing ASCII chars to `Buffer`
struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    /// Writes ASCII byte to `Buffer` and wraps lines
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_char: byte,
                    color_code,
                };
                self.column_position += 1;
            }
        }
    }
    /// Writes ASCII string to `Buffer`
    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // For non-printable ASCII range, print ■ (0xfe)
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// Something
    fn new_line(&mut self) {}
}

pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("World!");
}
