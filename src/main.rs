#![no_std]
#![no_main]
use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr::write_volatile;

use limine::framebuffer::Framebuffer;

use crate::boot::FRAMEBUFFER_REQUEST;

mod boot;

#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
    let Some(response) = FRAMEBUFFER_REQUEST.response() else {
        panic!("Framebuffer request response not recieved.")
    };
    let Some(framebuffer) = response.framebuffers().first() else {
        panic!("Framebuffer not found.")
    };
    draw(*framebuffer);
    idle();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    idle();
}

fn idle() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

fn draw(fb: &Framebuffer) {
    let base: *mut u8 = fb.address() as *mut u8;
    let bytes_per_pixel: usize = (fb.bpp / 8) as usize;

    let width: usize = fb.width as usize;
    let height: usize = fb.height as usize;
    let pitch: usize = fb.pitch as usize;

    for y in 0..height {
        for x in 0..width {
            let offset = y * pitch + x * bytes_per_pixel;
            unsafe {
                write_volatile(base.add(offset), (x ^ y) as u8); // blue
                write_volatile(base.add(offset + 1), (x * 255 / width) as u8); // green
                write_volatile(base.add(offset + 2), (y * 255 / height) as u8); // red
            }
        }
    }
}
