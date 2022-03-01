#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";
// 'HeLlo'

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // The text screen video memory for colour monitors resides at 0xB8000
    //The VGA text buffer is accessible via memory-mapped I/O to the address 0xb8000. This means that reads and writes to that address don’t access the RAM, but directly the text buffer on the VGA hardware. This means that we can read and write it through normal memory operations to that address.
    // Text mode memory takes two bytes for every "character" on screen. One is the ASCII code byte, the other the attribute byte.
    // Bit(s)    Value
    // 0-7	     ASCII code point
    // 8-11	     Foreground color
    // 12-14	 Background color
    // 15	     Blink

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
