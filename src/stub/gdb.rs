use core::mem::swap;

use gdbstub::target;
use gdbstub::target::ext::base::singlethread::SingleThreadBase;
use gdbstub::target::Target;
use gdbstub::target::TargetResult;

use crate::bios;
use crate::stub::Eflags;
use crate::stub::TargetRegisters;

#[repr(C)]
pub struct DosTarget {
    registers: TargetRegisters,

    break_stack_head: u8,
}

#[link_section = ".data"]
static mut BREAKS: [Breakpoint; 128] = [Breakpoint {
    addr: 0xdeadbeef,
    opcode: 0,
}; 128];

impl DosTarget {
    pub fn new() -> DosTarget {
        DosTarget {
            registers: Default::default(),
            break_stack_head: 0,
        }
    }
    pub fn registers(&mut self) -> &mut TargetRegisters {
        &mut self.registers
    }

    pub fn add_breakpoint(&mut self, addr: u32) -> TargetResult<bool, Self> {
        unsafe {
            let buf = &raw mut BREAKS;
            let len = buf.read().len();
            let opcode_ptr = addr as *mut u8;
            // println!("Break at {:X}, was {:X}, set to CC", addr, opcode_ptr.read());
            if self.break_stack_head as usize > len {
                Ok(false)
            } else {
                (*buf)[self.break_stack_head as usize] = Breakpoint {
                    addr: addr,
                    opcode: opcode_ptr.read(),
                };
                self.break_stack_head += 1;
                opcode_ptr.write(0xCC);

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
                let ptr = addr as *mut u8;
                // println!("Removed break at {:X}, was {:X}, set to {:X}", addr, ptr.read(), to_fill);
                ptr.write(to_fill);

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

macro_rules! min {
    () => ();
    ($a: expr, $b: expr $(,)?) => {
        $a.min($b)
    };
    ($a: expr, $($tail: expr),* $(,)?) => {
        $a.min(min!($($tail,)*))
    };
}

// NOTE: to try and make this a marginally more realistic estimate of
// `gdbstub`'s library overhead, non-IDET methods are marked as
// `#[inline(never)]` to prevent the optimizer from too aggressively coalescing
// the stubbed implementations.
//
// EXCEPTION: `list_active_threads` accepts a closure arg, and should be
// be inlined for smaller codegen

impl SingleThreadBase for DosTarget {
    #[inline(always)]
    fn support_resume(
        &mut self,
    ) -> Option<target::ext::base::singlethread::SingleThreadResumeOps<'_, Self>> {
        Some(self)
    }

    #[inline(always)]
    fn read_registers(
        &mut self,
        regs: &mut gdbstub_arch::x86::reg::X86CoreRegs,
    ) -> TargetResult<(), Self> {
        // println!("> read_registers");
        let registers = self.registers();
        // TODO: is it UB when DosTarget itself is static mut?
        regs.eax = registers.eax;
        regs.ebx = registers.ebx;
        regs.ecx = registers.ecx;
        regs.edx = registers.edx;
        regs.esi = registers.esi;
        regs.edi = registers.edi;
        regs.esp = registers.esp + 6;
        regs.ebp = registers.ebp;
        regs.eip = registers.eip; // adjustment handled in assembly
        regs.eflags = registers.eflags;
        regs.segments.cs = registers.cs as u32;
        regs.segments.ds = registers.ds as u32;
        regs.segments.es = registers.es as u32;
        regs.segments.ss = registers.ss as u32;
        regs.segments.fs = registers.fs as u32;
        regs.segments.gs = registers.gs as u32;

        Ok(())
    }

    #[inline(always)]
    fn write_registers(
        &mut self,
        regs: &gdbstub_arch::x86::reg::X86CoreRegs,
    ) -> TargetResult<(), Self> {
        // println!("> write_registers");
        let registers = self.registers();
        registers.eax = regs.eax;
        registers.ebx = regs.ebx;
        registers.ecx = regs.ecx;
        registers.edx = regs.edx;
        registers.esi = regs.esi;
        registers.edi = regs.edi;
        registers.esp = regs.esp;
        registers.ebp = regs.ebp;
        registers.eip = regs.eip;
        registers.eflags = regs.eflags;
        registers.cs = regs.segments.cs as u16;
        registers.ds = regs.segments.ds as u16;
        registers.es = regs.segments.es as u16;
        registers.ss = regs.segments.ss as u16;
        registers.fs = regs.segments.fs as u16;
        registers.gs = regs.segments.gs as u16;
        Ok(())
    }

    #[inline(always)]
    fn read_addrs(&mut self, start_addr: u32, data: &mut [u8]) -> TargetResult<usize, Self> {
        let read_ptr = start_addr as *const u8;

        let sizes = bios::mem::request_upper_memory_size()?;
        let total_mem_size = (sizes.extended1 as u32) * 1024 + (sizes.extended2 as u32) * 64 * 1024;

        // TODO: switch to unreal mode to enable support for reading ta offsets greater
        // that one segment size
        let size = min!(
            total_mem_size.saturating_sub(start_addr),
            data.len() as u32,
            0x1_00_00u32.saturating_sub(start_addr)
        ) as usize;
        // SAFETY: the previous line ensures read_ptr..read_ptr+size are within segment limits
        let source = unsafe { core::slice::from_raw_parts(read_ptr, size) };

        data[..size].copy_from_slice(source);
        // println!("> read_addrs");
        Ok(size)
    }

    #[inline(always)]
    fn write_addrs(&mut self, start_addr: u32, data: &[u8]) -> TargetResult<(), Self> {
        // println!("> write_addrs");
        let write_ptr = start_addr as *mut u8;

        let sizes = bios::mem::request_upper_memory_size()?;
        let total_mem_size = (sizes.extended1 as u32) * 1024 + (sizes.extended2 as u32) * 64 * 1024;

        // TODO: switch to unreal mode to enable support for reading ta offsets greater
        // that one segment size
        let size = min!(
            total_mem_size.saturating_sub(start_addr),
            data.len() as u32,
            0x1_00_00u32.saturating_sub(start_addr)
        ) as usize;
        // SAFETY: the previous line ensures write_ptr..write_ptr+size are within segment limits
        let destination = unsafe { core::slice::from_raw_parts_mut(write_ptr, size as usize) };

        destination.copy_from_slice(&data[..size]);
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
    #[inline(always)]
    fn add_sw_breakpoint(&mut self, addr: u32, _kind: usize) -> TargetResult<bool, Self> {
        self.add_breakpoint(addr)
    }

    #[inline(always)]
    fn remove_sw_breakpoint(&mut self, addr: u32, _kind: usize) -> TargetResult<bool, Self> {
        self.remove_breakpoint(addr)
    }
}

impl target::ext::base::singlethread::SingleThreadResume for DosTarget {
    #[inline(always)]
    fn resume(&mut self, _signal: Option<gdbstub::common::Signal>) -> Result<(), Self::Error> {
        // println!("> resume");
        Ok(())
    }
    #[inline(always)]
    fn support_single_step(
        &mut self,
    ) -> Option<target::ext::base::singlethread::SingleThreadSingleStepOps<'_, Self>> {
        Some(self)
    }
}

impl target::ext::base::singlethread::SingleThreadSingleStep for DosTarget {
    #[inline(always)]
    fn step(&mut self, _signal: Option<gdbstub::common::Signal>) -> Result<(), Self::Error> {
        // Set EFLAGS
        // println!("> step");
        self.registers().eflags |= Eflags::TRAP.bits();
        Ok(())
    }
}
