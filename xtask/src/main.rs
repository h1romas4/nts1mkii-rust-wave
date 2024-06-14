// For thumbv7em-none-eabihf, do not build xtask.
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]
#[cfg(target_os = "none")]
use core::panic::PanicInfo;
#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }

///
/// xtask main
///
#[cfg(not(target_os = "none"))]
fn main() {
    println!("Hello, world!");
}
