#![no_std]
#![no_main]
use crate::boot::{BASE_REVISION, BootFramebuffer, FRAMEBUFFER_REQUEST};
use core::panic::PanicInfo;
// use limine::framebuffer::Framebuffer;
use x86_64::instructions::hlt;

mod arch;
mod boot;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        hlt();
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());

    let fb = FRAMEBUFFER_REQUEST
        .response()
        .expect("Framebuffer response not found!")
        .framebuffers()
        .first()
        .expect("Framebuffer not found!");
    let framebuffer: BootFramebuffer = BootFramebuffer {
        addr: fb.address() as *mut u32,
        width: fb.width,
        height: fb.height,
        pitch: fb.pitch,
    };

    kmain();
}

fn kmain() -> ! {
    loop {
        hlt();
    }
}
