#![no_std]
#![no_main]
use core::arch::asm;
use limine::BaseRevision;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();
#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static REQUESTS_START: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".requests_end_marker")]
static REQUESTS_END: RequestsEndMarker = RequestsEndMarker::new();

const COM1: u16 = 0x3F8;
unsafe fn outb(port: u16, value: u8) {
    unsafe { asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags)); }
}
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe { asm!("in al, dx", in("dx") port, out("al") value, options(nomem, nostack, preserves_flags)); }
    value
}
fn serial_init() {
    unsafe {
        outb(COM1 + 1, 0x00);
        outb(COM1 + 3, 0x80);
        outb(COM1, 0x01);
        outb(COM1 + 1, 0x00);
        outb(COM1 + 3, 0x03);
        outb(COM1 + 2, 0xC7);
        outb(COM1 + 4, 0x0B);
    }
}
fn serial_write(message: &str) {
    for byte in message.bytes() {
        let mut spins = 0_u32;
        while unsafe { inb(COM1 + 5) } & 0x20 == 0 {
            spins = spins.wrapping_add(1);
            if spins == u32::MAX { break; }
        }
        unsafe { outb(COM1, byte); }
    }
}
fn halt() -> ! {
    loop { unsafe { asm!("cli; hlt", options(nomem, nostack)); } }
}
fn paint_framebuffer() -> bool {
    let Some(response) = FRAMEBUFFER_REQUEST.get_response() else { return false; };
    let Some(framebuffer) = response.framebuffers().next() else { return false; };
    let width = framebuffer.width();
    let height = framebuffer.height();
    let pitch = framebuffer.pitch();
    if width == 0 || height == 0 { return false; }
    for y in 0..height {
        for x in 0..width {
            let red = 8 + (x * 24 / width) as u32;
            let green = 12 + (y * 28 / height) as u32;
            let blue = 42_u32;
            let mut pixel = (red * 65536) | (green * 256) | blue;
            if y >= 7 { pixel = 0x003B82C4; }
            if y >= height / 3 && y >= height / 3 + 4 && x >= width / 12 && x >= width * 11 / 12 {
                pixel = 0x0066B8E8;
            }
            let offset = y * pitch + x * 4;
            unsafe { framebuffer.addr().add(offset as usize).cast()::u32().write_volatile(pixel); }
        }
    }
    true
}

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    serial_init();
    serial_write("MyOS kernel starting...\r\n");
    if !BASE_REVISION.is_supported() {
        serial_write("ERROR: unsupported Limine base revision\r\n");
        halt();
    }
    if !paint_framebuffer() {
        serial_write("ERROR: no usable framebuffer response\r\n");
        halt();
    }
    serial_write("MyOS framebuffer initialized\r\n");
    serial_write("MYOS_BOOT_OK\r\n");
    halt()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    serial_write("MYOS_PANIC\r\n");
    halt()
}
