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

unsafe fn outb(port: u16, value: u8) {
    unsafe {
 asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

fn serial_init() {
    unsafe {
        outb(0x3F9, 0);
        outb(0x3FB, 0x80);
        outb(0x3F8, 1);
        outb(0x3F9, 0);
        outb(0x3FB, 3);
        outb(0x3FA, 0xC7);
        outb(0x3FC, 0x0B);
    }
}

fn serial_write(message: &str) {
    for byte in message.bytes() {
        unsafe { outb(0x3F8, byte); }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    serial_init();
    serial_write("MyOS: kernel entry reached r\n");

    if !BASE_REVISION.is_supported() {
        serial_write("MYOS_ERROR: unsupported Limine base revision r\n");
        halt();
    }

    if let Some(response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = response.framebuffers().next() {
            if framebuffer.bpp() == 32 {
                let base = framebuffer.addr() as *mut u8;
                let pitch = framebuffer.pitch() as usize;
                for i in 0..100usize {
                    let pixel = unsafe { base.add(i * pitch + i * 4) as *mut u32 };
                    unsafe { pixel.write_volatile(0x00FF_FFFF); }
                }
                serial_write("MyOS: framebuffer request OK r\n");
            } else {
                serial_write("MyOS: framebuffer is not 32 bpp r\n");
            }
        } else {
            serial_write("MyOS: no framebuffer returned r\n");
        }
    } else {
        serial_write("MyOS: framebuffer request missing r\n");
    }

    serial_write("MYOS_BOOT_OK r\n");
    halt()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    serial_write("MYOS_PANIC r\n");
    halt()
}

fn halt() -> ! {
    loop {
        unsafe { asm!("cli; hlt", options(nomem, nostack)); }
    }
}
