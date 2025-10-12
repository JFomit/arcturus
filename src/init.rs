use gdbstub::stub::{
    state_machine::GdbStubStateMachine, DisconnectReason, GdbStubBuilder, MultiThreadStopReason,
    SingleThreadStopReason,
};

use crate::bios::com::ComStatusFlags;
use crate::local_cell::LocalCell;
use crate::stub::{conn::ComConnection, gdb::DosTarget};

#[no_mangle]
pub static mut DOS_TARGET: DosTarget = unsafe { core::mem::zeroed() };
pub static mut BUF: [u8; 1024] = [0; 1024];
pub static GDB_STATE_MACHINE: LocalCell<
    Option<GdbStubStateMachine<'static, DosTarget, ComConnection>>,
> = LocalCell::new(None);

pub fn init_dbg() -> Result<(), i32> {
    unsafe { DOS_TARGET = DosTarget::new() };

    let com = ComConnection::new(0);

    let gdb: gdbstub::stub::GdbStub<'_, DosTarget, ComConnection> = GdbStubBuilder::new(com)
        .with_packet_buffer(unsafe { &mut BUF })
        .build()
        .map_err(|_| 1)?;

    println!("Starting GDB session...");

    unsafe {
        GDB_STATE_MACHINE.replace(Some(gdb.run_state_machine(&mut DOS_TARGET).map_err(|_| 2)?));
    }

    Ok(())
}
