use gdbstub_arch::x86::reg::X86CoreRegs;


#[struct_layout::explicit(size = 576, align = 16, check(Default))]
#[derive(Copy, Clone, Debug, Default)]
pub struct X86Registers {
    #[field(offset = 0)]
    eax: u32,
    #[field(offset = 4)]
    ecx: u32,
    #[field(offset = 8)]
    edx: u32,
    #[field(offset = 12)]
    ebx: u32,
    #[field(offset = 16)]
    esp: u32,
    #[field(offset = 20)]
    ebp: u32,
    #[field(offset = 24)]
    esi: u32,
    #[field(offset = 28)]
    edi: u32,
    #[field(offset = 32)]
    eip: u32,
    #[field(offset = 36)]
    efalgs: u32,

    #[field(offset = 40)]
    cs: u16,
    #[field(offset = 42)]
    ss: u16,
    #[field(offset = 44)]
    ds: u16,
    #[field(offset = 46)]
    es: u16,
    #[field(offset = 48)]
    fs: u16,
    #[field(offset = 50)]
    gs: u16,

    #[field(offset = 64)]
    float_state: X86FpuSseState
}

#[struct_layout::explicit(size = 512, align = 16)]
#[derive(Clone, Copy, Debug)]
pub struct X86FpuSseState {
    #[field(offset = 0)]
    pub fcw: u8,
    #[field(offset = 1)]
    pub fsw: u8,
    #[field(offset = 2)]
    pub ftw: u8,
    // #[field(offset = 3)]
    // rsvd: u8,
    #[field(offset = 4)]
    pub fop: u32,
    #[field(offset = 8)]
    pub fip: u32,
    #[field(offset = 12)]
    pub fcs: u16,
    // #[field(offset = 14)]
    // rsvd: u16,

    #[field(offset = 16)]
    pub fdp: u16,
    #[field(offset = 18)]
    pub fds: u16,
    
    // #[field(offset = 20)]
    // fds_rsvd: u32
    #[field(offset = 24)]
    pub mxcsr: u32,
    #[field(offset = 28)]
    pub mxcsr_mask: u32,

    #[field(offset = 32)]
    pub st0: FP80,
    #[field(offset = 48)]
    pub st1: FP80,
    #[field(offset = 64)]
    pub st2: FP80,
     #[field(offset = 80)]
    pub st3: FP80,
     #[field(offset = 96)]
    pub st4: FP80,
     #[field(offset = 112)]
    pub st5: FP80,
     #[field(offset = 128)]
    pub st6: FP80,
     #[field(offset = 144)]
    pub st7: FP80,

    #[field(offset = 160)]
    xmm0: u128,
    #[field(offset = 176)]
    xmm1: u128,
    #[field(offset = 192)]
    xmm2: u128,
    #[field(offset = 208)]
    xmm3: u128,
    #[field(offset = 224)]
    xmm4: u128,
    #[field(offset = 240)]
    xmm5: u128,
    #[field(offset = 256)]
    xmm6: u128,
    #[field(offset = 272)]
    xmm7: u128,

    // reserved
}

type FP80 = [u8; 10];

impl Default for X86FpuSseState {
    fn default() -> Self {
        Self([0u8; 512])
    }
}

impl X86Registers {
    pub fn as_slice_mut(&mut self) -> &mut [u8; 576] {
        &mut self.0
    }
    pub fn write_gdb(&mut self, regs: &mut X86CoreRegs) {
        regs.eax = self.eax();
        regs.ebp = self.ebp();
        regs.ebx = self.ebx();
        regs.ecx = self.ecx();
        regs.edi = self.edi();
        regs.edx = self.edx();
        regs.eflags = self.efalgs();
        regs.eip = self.eip();
        regs.esi = self.esi();
        regs.esp = self.esp();

        regs.st[0] = self.float_state().st0();
        regs.st[1] = self.float_state().st1();
        regs.st[2] = self.float_state().st2();
        regs.st[3] = self.float_state().st3();
        regs.st[4] = self.float_state().st4();
        regs.st[5] = self.float_state().st5();
        regs.st[6] = self.float_state().st6();
        regs.st[7] = self.float_state().st7();

        regs.xmm[0] = self.float_state().xmm0();
        regs.xmm[1] = self.float_state().xmm1();
        regs.xmm[2] = self.float_state().xmm2();
        regs.xmm[3] = self.float_state().xmm3();
        regs.xmm[4] = self.float_state().xmm4();
        regs.xmm[5] = self.float_state().xmm5();
        regs.xmm[6] = self.float_state().xmm6();
        regs.xmm[7] = self.float_state().xmm7();
        
        regs.mxcsr = self.float_state().mxcsr();
        regs.segments.cs = self.cs() as u32;
        regs.segments.ds = self.ds() as u32;
        regs.segments.es = self.es() as u32;
        regs.segments.fs = self.fs() as u32;
        regs.segments.gs = self.gs() as u32;
        regs.segments.ss = self.ss() as u32;

        regs.fpu.fctrl = self.float_state().fcw() as u32;
        regs.fpu.fstat = self.float_state().fsw() as u32;
        regs.fpu.fop = self.float_state().fop();
        // TODO: unpacking abridge info
        regs.fpu.ftag = 0xffffffff;
        regs.fpu.foseg = self.float_state().fds() as u32;
        regs.fpu.fioff = self.float_state().fip();
        regs.fpu.fooff = self.float_state().fdp() as u32;
    }
}