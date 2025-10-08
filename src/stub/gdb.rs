use gdbstub::target;
use gdbstub::target::ext::base::singlethread::SingleThreadBase;
use gdbstub::target::Target;
use gdbstub::target::TargetResult;

pub struct DummyTarget {}

impl DummyTarget {
    pub fn new() -> DummyTarget {
        DummyTarget {}
    }
}

impl Target for DummyTarget {
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

impl SingleThreadBase for DummyTarget {
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
    fn read_addrs(&mut self, _start_addr: u32, data: &mut [u8]) -> TargetResult<usize, Self> {
        println!("> read_addrs");
        data.iter_mut().for_each(|b| *b = 0x55);
        Ok(data.len())
    }

    #[inline(never)]
    fn write_addrs(&mut self, _start_addr: u32, _data: &[u8]) -> TargetResult<(), Self> {
        println!("> write_addrs");
        Ok(())
    }
}

impl target::ext::breakpoints::Breakpoints for DummyTarget {
    #[inline(always)]
    fn support_sw_breakpoint(
        &mut self,
    ) -> Option<target::ext::breakpoints::SwBreakpointOps<'_, Self>> {
        Some(self)
    }
}

impl target::ext::breakpoints::SwBreakpoint for DummyTarget {
    #[inline(never)]
    fn add_sw_breakpoint(&mut self, _addr: u32, _kind: usize) -> TargetResult<bool, Self> {
        Ok(true)
    }

    #[inline(never)]
    fn remove_sw_breakpoint(&mut self, _addr: u32, _kind: usize) -> TargetResult<bool, Self> {
        Ok(true)
    }
}
