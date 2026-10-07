#![no_std]
#![no_main]
use core::arch::asm;
use core::panic::PanicInfo;

use limine::framebuffer::Framebuffer;

use crate::boot::FRAMEBUFFER_REQUEST;

mod arch;
mod boot;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    idle();
}

#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
    let Some(response) = FRAMEBUFFER_REQUEST.response() else {
        panic!("Framebuffer request response not received.")
    };
    let Some(framebuffer) = response.framebuffers().first() else {
        panic!("Framebuffer not found.")
    };
    draw_char('A', framebuffer);
    idle();
}

fn idle() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

fn draw_char(char: char, fb: &FrameBuffer) {
    let base: *mut u8 = fb.address() as *mut u8;
    let bytes_per_pixel: usize = (fb.bpp / 8) as usize;

    let width: usize = fb.width as usize;
    let height: usize = fb.height as usize;
    let pitch: usize = fb.pitch as usize;
}
