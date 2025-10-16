use core::cell::SyncUnsafeCell;

use gdbstub::stub::{
    state_machine::GdbStubStateMachine, GdbStubBuilder,
};

use crate::stub::{conn::ComConnection, gdb::DosTarget};

#[no_mangle]
#[link_section = ".data"]
pub static DOS_TARGET: SyncUnsafeCell<DosTarget> = unsafe { core::mem::zeroed() };
#[link_section = ".data"]
pub static mut BUF: [u8; 1024] = [0; 1024];
#[link_section = ".data"]
pub static GDB_STATE_MACHINE: SyncUnsafeCell<
    Option<GdbStubStateMachine<'static, DosTarget, ComConnection>>,
> = SyncUnsafeCell::new(None);

pub fn init_dbg() -> Result<(), i32> {
    unsafe {
        DOS_TARGET.get().write(DosTarget::new());
        GDB_STATE_MACHINE.get().write(None);
    };

    let com = ComConnection::new(0);

    let gdb: gdbstub::stub::GdbStub<'_, DosTarget, ComConnection> = GdbStubBuilder::new(com)
        .with_packet_buffer(unsafe { &mut BUF })
        .build()
        .map_err(|_| 1)?;

    unsafe {
        GDB_STATE_MACHINE.get().write(Some(
            gdb.run_state_machine(DOS_TARGET.get().as_mut_unchecked())
                .map_err(|_| 2)?,
        ));
    }

    // println!("> waiting for debugger...");

    Ok(())
}
