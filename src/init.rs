use core::cell::SyncUnsafeCell;

use gdbstub::stub::{
    state_machine::GdbStubStateMachine, DisconnectReason, GdbStubBuilder, MultiThreadStopReason,
    SingleThreadStopReason,
};

use crate::bios::com::ComStatusFlags;
use crate::local_cell::LocalCell;
use crate::stub::{conn::ComConnection, gdb::DosTarget};

#[no_mangle]
#[link_section = ".data"]
pub static DOS_TARGET: SyncUnsafeCell<DosTarget> = unsafe { core::mem::zeroed() };
#[link_section = ".data"]
pub static mut BUF: [u8; 1024] = [0; 1024];
#[link_section = ".data"]
pub static GDB_STATE_MACHINE: LocalCell<
    Option<GdbStubStateMachine<'static, DosTarget, ComConnection>>,
> = LocalCell::new(None);

pub fn init_dbg() -> Result<(), i32> {
    unsafe { DOS_TARGET.get().write(DosTarget::new()); };
    GDB_STATE_MACHINE.replace(None);

    let com = ComConnection::new(0);

    let gdb: gdbstub::stub::GdbStub<'_, DosTarget, ComConnection> = GdbStubBuilder::new(com)
        .with_packet_buffer(unsafe { &mut BUF })
        .build()
        .map_err(|_| 1)?;

    println!("Starting GDB session...");

    unsafe {
        GDB_STATE_MACHINE.replace(Some(gdb.run_state_machine(DOS_TARGET.get().as_mut_unchecked()).map_err(|_| 2)?));
    }

    Ok(())
}
