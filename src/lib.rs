#![no_std]
#![feature(alloc_error_handler)]
#![feature(asm_goto_with_outputs)]
#![no_main]

use crate::init::init_dbg;

#[macro_use]
pub mod dos;
pub mod bios;
pub mod dpkey;
mod init;
mod local_cell;
mod stub;

extern crate rlibc;

#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    init_dbg().unwrap();

    unsafe {
        main();
    }
    dos::exit(0);
}

unsafe extern "C" {
    unsafe fn main();
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub fn __main() -> () {
            // type check the given path
            let f: fn() -> () = $path;
            f()
        }
    };
}
