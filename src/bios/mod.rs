use core::arch::asm;

pub mod com;
pub mod mem;

/// Waits for a specified number of microseconds.
pub fn sleep(u_time: u32) {
    unsafe {
        asm!(
            "int   15h",
            in("ax") 0x86 << 8,
            in("cx") ((u_time & 0xffff_0000) >> 16) as u16,
            in("dx") u_time as u16
        );
    }
}
