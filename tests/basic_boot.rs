#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
use blog_os::println;
use blog_os::test_panic_handler;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    blog_os::init();
    
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

// #[test_case]
// fn test_panic() {
//     panic!("panic!");
// }
