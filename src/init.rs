use gdbstub::stub::{
    state_machine::GdbStubStateMachine, DisconnectReason, GdbStubBuilder, MultiThreadStopReason,
    SingleThreadStopReason,
};

use crate::bios::com::ComStatusFlags;
use crate::local_cell::LocalCell;
use crate::stub::{conn::ComConnection, gdb::DosTarget};

static GDB_STATE_MACHINE: LocalCell<
    Option<GdbStubStateMachine<'static, DosTarget, ComConnection>>,
> = LocalCell::new(None);
static mut BUF: [u8; 1024] = [0; 1024];

pub fn init_dbg() -> Result<(), i32> {
    let mut target = DosTarget::new();

    let com = ComConnection::new(0);

    let gdb: gdbstub::stub::GdbStub<'_, DosTarget, ComConnection> = GdbStubBuilder::new(com)
        .with_packet_buffer(unsafe { &mut BUF })
        .build()
        .map_err(|_| 1)?;

    println!("Starting GDB session...");

    GDB_STATE_MACHINE.replace(Some(gdb.run_state_machine(&mut target).map_err(|_| 2)?));

    let res = loop {
        let mut r#break = None;
        GDB_STATE_MACHINE.replace_with(|opt| opt.take().map(|gdb_state_machine|
            match gdb_state_machine {
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

                    match gdb.incoming_data(&mut target, byte.unwrap()) {
                        Ok(gdb) => Some(gdb),
                        Err(e) => {
                            r#break = Some(Err(e));
                            None
                        }
                    }
                }
                GdbStubStateMachine::Running(gdb) => {
                    match gdb.report_stop(&mut target, MultiThreadStopReason::DoneStep) {
                        Ok(gdb) => Some(gdb),
                        Err(e) => {r#break = Some(Err(e)); None},
                    }
                }
                GdbStubStateMachine::CtrlCInterrupt(gdb) => {
                    match gdb.interrupt_handled(&mut target, None::<SingleThreadStopReason<u32>>) {
                        Ok(gdb) => Some(gdb),
                        Err(e) => {r#break = Some(Err(e)); None},
                    }
                }
                GdbStubStateMachine::Disconnected(gdb) => {r#break = Some(Ok(gdb.get_reason())); None},
            }).flatten());
        if let Some(result) = r#break {
            break result;
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
