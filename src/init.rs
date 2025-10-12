use core::mem::MaybeUninit;

use gdbstub::stub::{
    state_machine::GdbStubStateMachine, DisconnectReason, GdbStubBuilder, MultiThreadStopReason,
    SingleThreadStopReason,
};

use crate::bios::com::ComStatusFlags;
use crate::stub::{conn::ComConnection, gdb::DosTarget};

#[no_mangle]
pub static mut DOS_TARGET: DosTarget = unsafe { core::mem::zeroed() };
pub static mut GDB_STATE_MACHINE: Option<GdbStubStateMachine<'static, DosTarget, ComConnection>> = None;
pub static mut BUF: [u8; 1024] = [0; 1024];

pub fn init_dbg() -> Result<(), i32> {
    unsafe { DOS_TARGET = DosTarget::new() };

    let com = ComConnection::new(0);

    let gdb: gdbstub::stub::GdbStub<'_, DosTarget, ComConnection> = GdbStubBuilder::new(com)
        .with_packet_buffer(unsafe { &mut BUF })
        .build()
        .map_err(|_| 1)?;

    println!("Starting GDB session...");

    unsafe {
        GDB_STATE_MACHINE = Some(gdb.run_state_machine(&mut DOS_TARGET).map_err(|_| 2)?);
    }

    Ok(())
}
