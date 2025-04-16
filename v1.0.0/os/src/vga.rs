use volatile::Volatile;
use core::fmt::Write;
use spin::Mutex;

static VGA_BUFFER: Mutex<VgaBuffer> = Mutex::new(VgaBuffer {
    buffer: unsafe { &mut *(0xb8000 as *mut [[Volatile<u16>; 80]; 25]) },
});

struct VgaBuffer {
    buffer: &'static mut [[Volatile<u16>; 80]; 25],
}

pub fn init() {
    // Clear the screen
    let mut vga = VGA_BUFFER.lock();
    for y in 0..25 {
        for x in 0..80 {
            vga.buffer[y][x].write(0x0F00); // Black background, white foreground, empty char
        }
    }
}

pub fn print(s: &str) {
    let mut vga = VGA_BUFFER.lock();
    static mut ROW: usize = 0;
    static mut COL: usize = 0;

    for c in s.chars() {
        if c == '\n' {
            unsafe {
                ROW += 1;
                COL = 0;
            }
            if unsafe { ROW } >= 25 {
                // Scroll up
                for y in 0..24 {
                    for x in 0..80 {
                        vga.buffer[y][x].write(vga.buffer[y + 1][x].read());
                    }
                }
                for x in 0..80 {
                    vga.buffer[24][x].write(0x0F00);
                }
                unsafe { ROW = 24; }
            }
            continue;
        }

        if unsafe { COL } < 80 {
            vga.buffer[unsafe { ROW }][unsafe { COL }].write(0x0F00 | (c as u16));
            unsafe { COL += 1; }
        }
    }
}

pub struct VgaWriter;

impl Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        print(s);
        Ok(())
    }
}