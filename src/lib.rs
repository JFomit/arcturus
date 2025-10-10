#![no_std]
#![feature(alloc_error_handler)]
#![feature(asm_goto_with_outputs)]
#![feature(trivial_bounds)]
#![feature(abi_x86_interrupt)]
#![no_main]

use crate::{stub::gdb::DosTarget};

#[macro_use]
pub mod dos;
pub mod bios;
pub mod dpkey;
mod helpers;
mod init;
mod stub;

extern crate rlibc;

static mut DOS: DosTarget = unsafe { core::mem::zeroed::<DosTarget>() };

#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    unsafe {
        DOS = DosTarget::new();
        set_handlers();
        main();
    }

    _exit(0);
}

#[used]
#[no_mangle]
pub static mut OLD_INT3_HANDLER: u32 = 0;

#[repr(C)]
pub struct InterruptStackFrame {
    ip: usize,
    cs: usize,
    flags: usize,
    sp: usize,
    ss: usize,
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "x86-interrupt" fn int3_handler(_stack_frame: InterruptStackFrame) {
    println!("int3");
    _exit(2);
}

unsafe extern "C" {
    unsafe fn main();
    unsafe fn set_handlers();
    unsafe fn clear_handlers();
}

#[no_mangle]
fn _exit(rt: u8) -> ! {
    unsafe {
        clear_handlers();
    }
    dos::exit(rt);
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
