#![no_std]
#![no_main]

use uefi::prelude::*;
use uefi::table::boot::MemoryType;

#[no_mangle]
pub extern "C" fn efi_main(
    image_handle: uefi::Handle,
    mut system_table: SystemTable<Boot>,
) -> ! {
    // Initialize UEFI
    uefi::init(&mut system_table).expect("Failed to initialize UEFI");

    // Load kernel
    let kernel_entry = 0x100000;
    let kernel_bytes = include_bytes!("../../target/x86_64-paradise-kumul-fly/debug/paradise-kumul-fly");
    let kernel_pages = (kernel_bytes.len() + 0xfff) / 0x1000;
    let kernel_memory = system_table
        .boot_services()
        .allocate_pages(
            uefi::table::boot::AllocateType::Address(kernel_entry),
            MemoryType::LOADER_CODE,
            kernel_pages,
        )
        .expect("Failed to allocate kernel memory");

    unsafe {
        core::ptr::copy_nonoverlapping(
            kernel_bytes.as_ptr(),
            kernel_memory as *mut u8,
            kernel_bytes.len(),
        );
    }

    // Exit boot services
    let (_system_table, memory_map) = system_table
        .exit_boot_services(image_handle, &mut [0u8; 1024])
        .expect("Failed to exit boot services");

    // Jump to kernel
    let entry: extern "C" fn(*const u8) -> ! = unsafe { core::mem::transmute(kernel_entry) };
    entry(memory_map.as_ptr());
}