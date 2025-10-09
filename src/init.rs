use core::arch::asm;

use gdbstub::stub::{
    state_machine::GdbStubStateMachine, DisconnectReason, GdbStubBuilder, MultiThreadStopReason, SingleThreadStopReason,
};

use crate::stub::{
    conn::{ComConnection},
    gdb::DosTarget,
};
use crate::bios::com::ComStatusFlags;

pub fn init_dbg() -> Result<(), i32> {
    let mut target = DosTarget::new();

    let com = ComConnection::new(0);

    let mut buf = [0; 1024];
    let gdb: gdbstub::stub::GdbStub<'_, DosTarget, ComConnection> = GdbStubBuilder::new(com)
        .with_packet_buffer(&mut buf)
        .build()
        .map_err(|_| 1)?;

    println!("Starting GDB session...");

    let mut gdb = gdb.run_state_machine(&mut target).map_err(|_| 2)?;

    let res = loop {
        gdb = match gdb {
            GdbStubStateMachine::Idle(mut gdb) => {
                let mut byte = gdb.borrow_conn().read();
                loop {
                    unsafe { asm!("pause"); }

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

                match gdb.incoming_data(&mut target, byte.unwrap()) {
                    Ok(gdb) => gdb,
                    Err(e) => break Err(e),
                }
            }
            GdbStubStateMachine::Running(gdb) => {
                match gdb.report_stop(&mut target, MultiThreadStopReason::DoneStep) {
                    Ok(gdb) => gdb,
                    Err(e) => break Err(e),
                }
            }
            GdbStubStateMachine::CtrlCInterrupt(gdb) => {
                match gdb.interrupt_handled(&mut target, None::<SingleThreadStopReason<u32>>) {
                    Ok(gdb) => gdb,
                    Err(e) => break Err(e),
                }
            }
            GdbStubStateMachine::Disconnected(gdb) => break Ok(gdb.get_reason()),
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
