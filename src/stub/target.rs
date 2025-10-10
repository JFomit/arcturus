use core::ffi::c_void;

use gdbstub::target::{self, ext::base::singlethread::SingleThreadBase, Target, TargetResult};

use crate::{bios, stub::gdb::DosTarget};

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

extern "cdecl" {
    fn save_registers(target: *mut u8) -> c_void;
    fn restore_ergisters(target: *mut u8) -> !;
}

impl SingleThreadBase for DosTarget {
    fn read_registers(
        &mut self,
        gdb_regs: &mut gdbstub_arch::x86::reg::X86CoreRegs,
    ) -> TargetResult<(), Self> {
        // unsafe {
        //     save_registers(self.regs.as_slice_mut() as *mut u8);
        // }
        // self.regs.write_gdb(gdb_regs);
        println!("> read_registers");
        Ok(())
    }

    fn write_registers(
        &mut self,
        _regs: &gdbstub_arch::x86::reg::X86CoreRegs,
    ) -> TargetResult<(), Self> {
        println!("> write_registers");
        Ok(())
    }

    fn read_addrs(&mut self, start_addr: u32, data: &mut [u8]) -> TargetResult<usize, Self> {
        let mut count = 0;
        unsafe {
            let mut address = start_addr as *mut u8;
            let sizes = bios::mem::request_upper_memory_size()?;
            let total_size = (sizes.extended1 as u32) * 1024 + (sizes.extended2 as u32) * 64 * 1024;

            for item in data {
                if address as u32 >= total_size {
                    break;
                }

                *item = *address;
                address = address.add(1);
                count += 1;
            }
        }
        println!("> read_addrs");
        Ok(count)
    }

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
    fn add_sw_breakpoint(&mut self, addr: u32, _kind: usize) -> TargetResult<bool, Self> {
        self.add_breakpoint(addr)
    }

    fn remove_sw_breakpoint(&mut self, addr: u32, _kind: usize) -> TargetResult<bool, Self> {
        self.remove_breakpoint(addr)
    }
}
