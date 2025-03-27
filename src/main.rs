#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::{hlt_loop, println};

#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    // panic!("asd");

    blog_os::init();
    // x86_64::instructions::interrupts::int3();

    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // };

    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }

    // // trigger a stack overflow
    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    // loop {
    //     use blog_os::print;
    //     print!("-");        // new
    // }

    hlt_loop()
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
