use core::mem::swap;

use gdbstub::target;
use gdbstub::target::ext::base::singlethread::SingleThreadBase;
use gdbstub::target::Target;
use gdbstub::target::TargetResult;

pub struct DosTarget {
    break_stack_head: u8,
}

impl DosTarget {
    pub fn new() -> DosTarget {
        DosTarget {
            break_stack_head: 0,
        }
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
}

#[derive(Clone, Copy)]
struct Breakpoint {
    addr: u32,
    opcode: u8,
}

static mut BREAKS: [Breakpoint; 128] = [Breakpoint { addr: 0, opcode: 0 }; 128];

impl Target for DosTarget {
    type Arch = gdbstub_arch::x86::X86_SSE;
    type Error = &'static str;

    #[inline(always)]
    fn base_ops(&mut self) -> target::ext::base::BaseOps<'_, Self::Arch, Self::Error> {
        target::ext::base::BaseOps::SingleThread(self)
    }

    // disable `QStartNoAckMode` in order to save space
    #[inline(always)]
    fn use_no_ack_mode(&self) -> bool {
        false
    }

    // disable X packet optimization in order to save space
    #[inline(always)]
    fn use_x_upcase_packet(&self) -> bool {
        false
    }

    #[inline(always)]
    fn support_breakpoints(
        &mut self,
    ) -> Option<target::ext::breakpoints::BreakpointsOps<'_, Self>> {
        Some(self)
    }
}

// NOTE: to try and make this a marginally more realistic estimate of
// `gdbstub`'s library overhead, non-IDET methods are marked as
// `#[inline(never)]` to prevent the optimizer from too aggressively coalescing
// the stubbed implementations.
//
// EXCEPTION: `list_active_threads` accepts a closure arg, and should be
// be inlined for smaller codegen

impl SingleThreadBase for DosTarget {
    #[inline(never)]
    fn read_registers(
        &mut self,
        _regs: &mut gdbstub_arch::x86::reg::X86CoreRegs,
    ) -> TargetResult<(), Self> {
        println!("> read_registers");
        Ok(())
    }

    #[inline(never)]
    fn write_registers(
        &mut self,
        _regs: &gdbstub_arch::x86::reg::X86CoreRegs,
    ) -> TargetResult<(), Self> {
        println!("> write_registers");
        Ok(())
    }

    #[inline(never)]
    fn read_addrs(&mut self, start_addr: u32, data: &mut [u8]) -> TargetResult<usize, Self> {
        let mut count = 0;
        // let data = &mut *data;
        unsafe {
            let mut address = start_addr as *mut u8;
            for item in data {
                *item = *address;
                address = address.add(1);
                count += 1;
            }
        }
        println!("> read_addrs");
        Ok(count)
    }

    #[inline(never)]
    fn write_addrs(&mut self, _start_addr: u32, _data: &[u8]) -> TargetResult<(), Self> {
        println!("> write_addrs");
        Ok(())
    }
}

impl target::ext::breakpoints::Breakpoints for DosTarget {
    #[inline(always)]
    fn support_sw_breakpoint(
        &mut self,
    ) -> Option<target::ext::breakpoints::SwBreakpointOps<'_, Self>> {
        Some(self)
    }
}

impl target::ext::breakpoints::SwBreakpoint for DosTarget {
    #[inline(never)]
    fn add_sw_breakpoint(&mut self, addr: u32, _kind: usize) -> TargetResult<bool, Self> {
        self.add_breakpoint(addr)
    }

    #[inline(never)]
    fn remove_sw_breakpoint(&mut self, addr: u32, _kind: usize) -> TargetResult<bool, Self> {
        self.remove_breakpoint(addr)
    }
}
