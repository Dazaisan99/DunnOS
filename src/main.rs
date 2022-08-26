#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dunn_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use dunn_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello DunnOS !\n");

    dunn_os::init();

    fn so() {
        so();
    }

    so();

    #[cfg(test)]
    test_main();

    println!("Didn't Crash ! Yay");

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    dunn_os::test_panic_handler(info)
}
