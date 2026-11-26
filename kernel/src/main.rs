#![ o_std]
#![ o_main]
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
    unsafe { asm!("out dx, al", in("dx") port, in("al") value, options( omem, nostack, preserves_flags)); }
}
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe { asm!("in al, dx", in("dx") port, out("al") value, options( omem, nostack, preserves_flags)); }
    value
}
fn serial_init() {
    unsafe {
        outb(COM1 + 1, 0);
        outb(COM1 + 3, 128);
        outb(COM1, 1);
        outb(COM1 + 1, 0);
        outb(COM1 + 3, 3);
        outb(COM1 + 2, 199);
        outb(COM1 + 4, 11);
    }
}
fn serial_write(message: &str) {
    for byte in message.bytes() {
        let mut spins = 0_u32;
        while unsafe { inb(COM1 + 5) } & 32 == 0 {
 spins = spins.wrapping_add(1);
            if spins == u32::MAX { break; }
        }
        unsafe { outb(COM1, byte); }
    }
}
fn halt() -> ! {
    loop { unsafe { asm!("cli; hlt", options( omem, nostack)); } }
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
            let pixel = (red * 65536) | (green * 256) | blue;
            let offset = y * pitch + x * 4;
            unsafe { framebuffer.addr().add(offset as usize).cast().write_volatile(pixel); }
        }
    }
    true
}
#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    serial_init();
    serial_write("MyOS kernel starting");
    if !BASE_REVISION.is_supported() {
        serial_write("ERROR: unsupported Limine base revision");
        halt();
    }
    if !paint_framebuffer() {
        serial_write("ERROR: no usable framebuffer response");
        halt();
    }
    serial_write("MyOS framebuffer initialized");
    serial_write("MYOS_BOOT_OK");
    halt()
}
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    serial_write("MYOS_PANIC");
    halt()
}
