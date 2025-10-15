#![no_std]
#![feature(alloc_error_handler)]
#![feature(asm_goto_with_outputs)]
#![feature(sync_unsafe_cell)]
#![feature(ptr_as_ref_unchecked)]
#![no_main]

use core::arch::asm;

use gdbstub::stub::{state_machine::GdbStubStateMachine, SingleThreadStopReason};

use crate::init::{init_dbg, DOS_TARGET, GDB_STATE_MACHINE};

#[macro_use]
pub mod dos;
pub mod bios;
pub mod dpkey;
mod init;
mod local_cell;
mod stub;

extern crate rlibc;

#[inline(never)]
unsafe extern "C" fn call_main() -> u16 {
    let mut rt: u16;
    asm!(
        "push   ebp",
        "mov    ebp,    esp",
        "xor    eax,    eax",
        "call   main",
        "pop    ebp",
        out("ax") rt,
        clobber_abi("C")
    );
    rt
}

#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    unsafe { set_interrupt_handlers() };
    init_dbg().unwrap();
    unsafe { asm!("int3") };

    unsafe {
        let rt = call_main();

        println!("> stopping gdb session...");
        asm!("int3");

        _exit(rt as u8);
    }
}

#[no_mangle]
extern "C" fn _exit(rt: u8) -> ! {
    let machine = unsafe { GDB_STATE_MACHINE.get().read().unwrap() };
    match machine {
        GdbStubStateMachine::Running(gdb) => {
            let _ = gdb.report_stop(
                unsafe { DOS_TARGET.get().as_mut_unchecked() },
                SingleThreadStopReason::Exited(rt),
            );
        }

        _ => println!("> stub was left in an invalid state"),
    }

    unsafe { remove_interrupt_handlers() };
    dos::exit(rt);
}

// unsafe extern "C" {
//     unsafe fn main() -> u8;
// }

#[link(name = "dbrt", kind = "static")]
unsafe extern "C" {
    unsafe fn set_interrupt_handlers();
    unsafe fn remove_interrupt_handlers();
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
