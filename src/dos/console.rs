use core::arch::asm;
use core::fmt::{self, Write};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::dos::console::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => {
        crate::print!(concat!($fmt, "\r\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        crate::print!(concat!($fmt, "\r\n"), $($arg)*)
    };
}

pub fn _print(args: fmt::Arguments) {
    let mut writer = DosWriter {};
    writer.write_fmt(args).unwrap();
}

pub fn read_no_echo() -> u8 {
    let mut c: u8;
    unsafe {
        asm!(
            "mov    ah, 8",
            "int    0x21",
            out("al") c
        );
    }
    c
}

struct DosWriter;

impl Write for DosWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            printc(c);
        }
        Ok(())
    }
}

fn printc(ch: u8) {
    unsafe { asm!("int 0x21", in("ah") 0x02_u8, in("dl") ch) }
}
