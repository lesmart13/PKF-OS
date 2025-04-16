fn main() {
    println!("Hello, world!");
}
#![no_std]
#![no_main]
#![feature(panic_info_static_ref)]

mod vga;
mod wanbel;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use vga::print;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // Initialize VGA
    vga::init();

    // Print boot message
    print("Paradise Kumul Fly OS\n");

    // Run Wanbel code
    let wanbel_code = "say('hello, this is from Paradise Kumul Fly')";
    wanbel::run(wanbel_code);

    // Halt
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print("Kernel Panic!\n");
    loop {
        x86_64::instructions::hlt();
    }
}