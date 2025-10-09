use core::arch::asm;

pub fn inb(port: usize) -> u8 {
    let mut ret: u8;
    unsafe {
        asm!("in al, dx", out("al") ret, in("dx") port);
    }
    ret
}

pub fn inw(port: usize) -> u16 {
    let mut ret: u16;
    unsafe { asm!("in ax, dx", out("ax") ret, in("dx") port) }
    ret
}

pub fn outb(data: u8, port: usize) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") data);
    }
}

pub fn outw(data: u16, port: usize) {
    unsafe {
        asm!("out dx, ax", in("dx") port, in("ax") data);
    }
}

pub fn com0_read() -> u8 {
    let mut result: u8;
    unsafe {
        asm!(
            "int   0x21",
            in("ah") 0x3u8,
            out("al") result,
            
            clobber_abi("C")
        );
    }

    result
}

pub fn com0_write(b: u8) {
    unsafe {
        asm!(
            "int    0x21",
            in("ah") 0x4u8,
            in("dl") b,

            clobber_abi("C")
        )
    }
}
