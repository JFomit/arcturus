pub(crate) mod gdb;
pub(crate) mod conn;
pub(crate) mod handlers;

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TargetRegisters {
    /// Accumulator
    pub eax: u32, // 0
    /// Count register
    pub ecx: u32, // 4
    /// Data register
    pub edx: u32, // 8
    /// Base register
    pub ebx: u32, // 12
    /// Stack pointer
    pub esp: u32, // 16
    /// Base pointer
    pub ebp: u32, // 20
    /// Source index
    pub esi: u32, // 24
    /// Destination index
    pub edi: u32, // 28
    /// Instruction pointer
    pub eip: u32, // 32
    /// Status register
    pub eflags: u32, // 36

    /// Code segment
    pub cs: u16, // 40
    /// Stack segment
    pub ss: u16, // 42
    /// Data segment
    pub ds: u16, // 44
    /// Extra segment
    pub es: u16, // 46
    /// General-purpose FS segment
    pub fs: u16, // 48
    /// General-purpose GS segment
    pub gs: u16, // 50

    // TODO: fpu and sse registers
    
    // /// FPU registers: ST0 through ST7
    // pub st: [F80; 8],
    // /// FPU internal registers
    // pub fpu: X87FpuInternalRegs,
    // /// SIMD Registers: XMM0 through XMM7
    // pub xmm: [u128; 8],
    // /// SSE Status/Control Register
    // pub mxcsr: u32,
}
