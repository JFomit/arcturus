use gdbstub::stub::{state_machine::GdbStubStateMachine, DisconnectReason, SingleThreadStopReason};

use crate::{
    _exit, bios::com::ComStatusFlags, init::{DOS_TARGET, GDB_STATE_MACHINE}
};

#[no_mangle]
pub unsafe extern "C" fn break_handler() {
    DOS_TARGET.registers().eip -= 1;
    send_stop();
    let r = gdb_handler_loop();

    match r {
        Ok(true) => return,
        Ok(false) => _exit(1),
        Err(str) => panic!("{:?}", str),
    }
}

#[no_mangle]
pub unsafe extern "C" fn step_over_handler() {}

fn send_stop() {
    unsafe {
        GDB_STATE_MACHINE.replace(match GDB_STATE_MACHINE.take().unwrap() {
            GdbStubStateMachine::Running(gdb) => {
                match gdb.report_stop(&mut DOS_TARGET, SingleThreadStopReason::SwBreak(())) {
                    Ok(gdb) => Some(gdb),
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                }
            }

            gdb => Some(gdb),
        });
    }
}

fn gdb_handler_loop() -> Result<bool, &'static str> {
    let res = loop {
        unsafe {
            GDB_STATE_MACHINE.replace(match GDB_STATE_MACHINE.take().unwrap() {
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
                    // match gdb.report_stop(&mut DOS_TARGET, SingleThreadStopReason::SwBreak(())) {
                    //     Ok(gdb) => Some(gdb),
                    //     Err(e) => break Err(e),
                    // }
                    println!("> running");
                    GDB_STATE_MACHINE.replace(Some(gdb.into()));
                    return Ok(true);
                }
                GdbStubStateMachine::CtrlCInterrupt(gdb) => {
                    match gdb
                        .interrupt_handled(&mut DOS_TARGET, None::<SingleThreadStopReason<u32>>)
                    {
                        Ok(gdb) => Some(gdb),
                        Err(e) => break Err(e),
                    }
                }
                GdbStubStateMachine::Disconnected(gdb) => break Ok(gdb.get_reason()),
            });
        }
    };

    match res {
        Ok(disconnect_reason) => {
            match disconnect_reason {
                DisconnectReason::Disconnect => println!("GDB Disconnected"),
                DisconnectReason::TargetExited(_) => println!("Target exited"),
                DisconnectReason::TargetTerminated(_) => println!("Target halted"),
                DisconnectReason::Kill => println!("GDB sent a kill command"),
            }

            Ok(false)
        }
        Err(e) => {
            if e.is_target_error() {
                Err("Target raised a fatal error")
            } else {
                panic!("{:?}", e)
            }
        }
    }
}
