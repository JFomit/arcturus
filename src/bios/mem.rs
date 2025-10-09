use core::arch::asm;

pub struct MemorySizes {
    /// Extended memory from 1MiB to 16MiB in KiB.
    pub extended1: u16,
    /// Extended memory from 16MiB in 64KiB blocks.
    pub extended2: u16,
}

// source: https://wiki.osdev.org/Detecting_Memory_(x86)#Detecting_Upper_Memory
pub fn request_upper_memory_size() -> Result<MemorySizes, ()> {
    let mut lower: u16;
    let mut upper: u16;

    unsafe {
        asm!(
            "xor    cx, cx",
            "xor    dx, dx",
            // mov ax, 0xe801
            "int   0x15",      // request upper memory size
            "jc     {0}",
            "cmp    ah, 0x86", // unsupported function
            "je     {0}",
            "cmp    ah, 0x80", // invalid command
            "je     {0}",
            "jcxz   2f",        // was the cx register invalid?
            "mov    ax, cx",
            "mov    bx, dx",
            "2:",
            label {
                return Err(())
            },

            inout("ax") 0xe801u16 => lower,
            out("bx") upper,

            clobber_abi("C")
        );
    }

    Ok(MemorySizes {
        extended1: lower,
        extended2: upper,
    })
}
