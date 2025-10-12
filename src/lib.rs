#![no_std]
#![feature(alloc_error_handler)]
#![feature(asm_goto_with_outputs)]
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

#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    unsafe { set_interrupt_handlers() };
    init_dbg().unwrap();
    unsafe { asm!("int3") };

    unsafe {
        let rt = main();

        match GDB_STATE_MACHINE.take().unwrap() {
            GdbStubStateMachine::Running(gdb) => {
                match gdb.report_stop(&mut DOS_TARGET, SingleThreadStopReason::Exited(rt)) {
                    Ok(_) => (),
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                }
            },
            _ => panic!("Stub left in an invalid state.")
        }

        _exit(rt);
    }
}

#[no_mangle]
fn _exit(rt: u8) -> ! {
    unsafe { remove_interrupt_handlers() };
    dos::exit(rt);
}

unsafe extern "C" {
    unsafe fn main() -> u8;
}

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
