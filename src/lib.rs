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
        let mut rt: u16;
        asm!(
            "call   main",
            out("ax") rt
        );

        println!("Stopping gdb session...");
        asm!("int3");

        _exit(rt as u8);
    }
}

#[no_mangle]
fn _exit(rt: u8) -> ! {
    let machine = GDB_STATE_MACHINE.take().unwrap();
    match machine {
        GdbStubStateMachine::Running(gdb) => {
            let _ = gdb.report_stop(
                unsafe { &mut DOS_TARGET },
                SingleThreadStopReason::Exited(rt),
            );
        }
        
        _ => println!("Stub left in an invalid state."),
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
