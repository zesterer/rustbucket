//number of columns and rows in the console
const COLUMNS: usize = 80;
const ROWS: usize = 25;

//enum for the 16 basic VGA colours (RGBI)
#[allow(dead_code)] //disable unused code warnings
#[repr(u8)] //represent as u8 integers
#[derive(Debug, Clone, Copy)]
pub enum VGA_COL {
    BLACK = 0,
    BLUE = 1,
    GREEN = 2,
    CYAN = 3,
    RED = 4,
    MAGENTA = 5,
    BROWN = 6,
    LIGHT_GRAY = 7,
    GRAY = 8,
    LIGHT_BLUE = 9,
    LIGHT_GREEN = 10,
    LIGHT_CYAN = 11,
    LIGHT_RED = 12,
    LIGHT_MAGENTA = 13,
    YELLOW = 14,
    WHITE = 15,
}

//colour code byte
#[derive(Debug, Clone, Copy)]
struct ColorCode {
    ccode: u8,
}

#[repr(C)] //ensure struct is ordered
#[derive(Debug, Clone, Copy)]
struct TermChar {
    ascii: u8,
    color_code: u8,
}

//array to store characters in the buffer
use volatile::Volatile;
struct VGABuffer {
    chars: [[Volatile<TermChar>; COLUMNS]; ROWS],
}

//store buffer
use core::ptr::Unique;
pub struct Writer {
    buffer: Unique<VGABuffer>,
}

impl Writer {
    //initialise/clear terminal
    pub fn terminal_init(&mut self) {
        for col in 0..COLUMNS {
            for row in 0..ROWS {
                let mut character_color = ColorCode { ccode: (0 as u8) << 4 | (15 as u8) };
                self.buffer().chars[row][col].write(TermChar {
                    ascii: 32,
                    color_code: character_color.ccode,
                });
            }
        }
    }

    fn buffer(&mut self) -> &mut VGABuffer {
        unsafe {
            self.buffer.as_mut()
        }
    }
}

use spin::Mutex;
pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
});
