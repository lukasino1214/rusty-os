#![no_std]
#![no_main]
#![warn(stable_features)]
#![feature(custom_test_frameworks, const_mut_refs)]
#![test_runner(rusty_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use rusty_os::task::{executor::Executor, keyboard, Task};
use rusty_os::{print, println, sys};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rusty_os::allocator;
    use rusty_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    println!("Welcome to rusty-os!");
    rusty_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    print!("lukas&rusty-os:-$ ");

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rusty_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rusty_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}