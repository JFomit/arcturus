use core::mem::MaybeUninit;

use gdbstub::stub::{
    state_machine::GdbStubStateMachine, DisconnectReason, GdbStubBuilder, MultiThreadStopReason,
    SingleThreadStopReason,
};

use crate::bios::com::ComStatusFlags;
use crate::stub::{conn::ComConnection, gdb::DosTarget};

#[no_mangle]
static mut DOS_TARGET: DosTarget = unsafe { core::mem::zeroed() };
static mut GDB_STATE_MACHINE: Option<GdbStubStateMachine<'static, DosTarget, ComConnection>> = None;
static mut BUF: [u8; 1024] = [0; 1024];

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

    let res = loop {
        unsafe {
            GDB_STATE_MACHINE = match GDB_STATE_MACHINE.take().unwrap() {
                GdbStubStateMachine::Idle(mut gdb) => {
                    let mut byte = gdb.borrow_conn().read();
                    loop {
                        if byte.is_err_and(|_| {
                            let flags = gdb.borrow_conn().status();
                            flags.contains(ComStatusFlags::TimeOutError)
                                | flags.contains(ComStatusFlags::TransmitterHoldingRegisterEmpty)
                        }) {
                            byte = gdb.borrow_conn().read();
                            continue;
                        }
                        break;
                    }

                    match gdb.incoming_data(&mut DOS_TARGET, byte.unwrap()) {
                        Ok(gdb) => Some(gdb),
                        Err(e) => break Err(e),
                    }
                }
                GdbStubStateMachine::Running(gdb) => {
                    match gdb.report_stop(&mut DOS_TARGET, SingleThreadStopReason::DoneStep) {
                        Ok(gdb) => Some(gdb),
                        Err(e) => break Err(e),
                    }
                }
                GdbStubStateMachine::CtrlCInterrupt(gdb) => {
                    match gdb.interrupt_handled(&mut DOS_TARGET, None::<SingleThreadStopReason<u32>>) {
                        Ok(gdb) => Some(gdb),
                        Err(e) => break Err(e),
                    }
                }
                GdbStubStateMachine::Disconnected(gdb) => break Ok(gdb.get_reason()),
            }
        }
    };

    match res {
        Ok(disconnect_reason) => match disconnect_reason {
            DisconnectReason::Disconnect => println!("GDB Disconnected"),
            DisconnectReason::TargetExited(_) => println!("Target exited"),
            DisconnectReason::TargetTerminated(_) => println!("Target halted"),
            DisconnectReason::Kill => println!("GDB sent a kill command"),
        },
        Err(e) => {
            if e.is_target_error() {
                println!("Target raised a fatal error");
            } else {
                println!("{}", e);
            }
        }
    }

    Ok(())
}
