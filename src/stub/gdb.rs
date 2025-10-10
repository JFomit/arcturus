use core::mem::swap;

use gdbstub::stub::state_machine::GdbStubStateMachine;
use gdbstub::stub::{DisconnectReason, GdbStubBuilder, SingleThreadStopReason};
use gdbstub::target::TargetResult;

use crate::bios::com::ComStatusFlags;
use crate::stub::conn::ComConnection;

pub struct DosTarget {
    break_stack_head: u8,
    gdb: GdbStubStateMachine<'static, DosTarget, ComConnection>,
}

impl DosTarget {
    pub fn new() -> Result<DosTarget, &'static str> {
        let mut target = DosTarget {
            break_stack_head: 0,
            gdb: unsafe { core::mem::zeroed() },
        };

        unsafe { Self::init_dbg(&mut target)? };
        Ok(target)
    }

    pub fn add_breakpoint(&mut self, addr: u32) -> TargetResult<bool, Self> {
        unsafe {
            let buf = &raw mut BREAKS;
            let len = (*buf).len();
            let opcode_ptr = addr as *mut u8;
            println!("Break at {}, opcode is {}", addr, *opcode_ptr);
            if self.break_stack_head as usize > len {
                Ok(false)
            } else {
                (*buf)[self.break_stack_head as usize] = Breakpoint {
                    addr: addr,
                    opcode: *opcode_ptr,
                };
                self.break_stack_head += 1;
                *opcode_ptr = 0xCC;

                Ok(true)
            }
        }
    }
    pub fn remove_breakpoint(&mut self, addr: u32) -> TargetResult<bool, Self> {
        unsafe {
            let buf = &raw mut BREAKS;
            let breakpoint = (*buf).iter_mut().find(|b| b.addr == addr);

            if let Some(b) = breakpoint {
                let to_fill = b.opcode;
                // the breakpoint is the last one
                if self.break_stack_head == 1 {
                    self.break_stack_head = 0;
                } else {
                    let top = &mut (*buf)[(self.break_stack_head - 1) as usize];
                    swap(b, top);
                    self.break_stack_head -= 1;
                }

                *(addr as *mut u8) = to_fill;

                return Ok(true);
            }

            Ok(false) // not found
        }
    }

    unsafe fn init_dbg(target: &mut DosTarget) -> Result<(), &'static str> {
        let com = ComConnection::new(0);

        let gdb: gdbstub::stub::GdbStub<'_, DosTarget, ComConnection> = GdbStubBuilder::new(com)
            .with_packet_buffer(&mut PACKETS)
            .build()
            .map_err(|_| "Failed to construct gdb stub.")?;

        target.gdb = gdb
            .run_state_machine(target)
            .map_err(|_| "State machine failure")?;
        Ok(())
    }

    fn gdb_loop(&mut self) -> Result<(), &'static str> {
        let mut gdb = self.gdb;
        let res = loop {
            gdb = match gdb {
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

                    match gdb.incoming_data(self, byte.unwrap()) {
                        Ok(gdb) => gdb,
                        Err(e) => break Err(e),
                    }
                }
                GdbStubStateMachine::Running(gdb) => {
                    match gdb.report_stop(self, SingleThreadStopReason::DoneStep) {
                        Ok(gdb) => gdb,
                        Err(e) => break Err(e),
                    }
                }
                GdbStubStateMachine::CtrlCInterrupt(gdb) => {
                    match gdb.interrupt_handled(self, None::<SingleThreadStopReason<u32>>) {
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
                    Err("Target raised a fatal error.")
                } else {
                    println!("{}", e);
                    Err("Gdb stub error.")
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Breakpoint {
    addr: u32,
    opcode: u8,
}

static mut BREAKS: [Breakpoint; 128] = [Breakpoint { addr: 0, opcode: 0 }; 128];
static mut PACKETS: [u8; 1024] = [0; 1024];
