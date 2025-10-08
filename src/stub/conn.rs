use core::arch::asm;
use gdbstub::conn::Connection;

pub struct ComConnection {
    port: u8,
}

impl ComConnection {
    pub fn write(&mut self, b: u8) {
        unsafe {
            // TODO: error-handling
            asm!(
                "mov    ah, 0x1",
                "int    0x14",
                in("al") b,
                in("dx") self.port as u16,

                clobber_abi("C")
            );
        }
    }
    
    pub fn new(port: u8) -> Self {
        const COM_OPTS: u16 = 0b0000_0000_1110_0011; 

        unsafe {
            asm!(
	            "int	0x14",

                in("dx") port as u16,
                in("ax") COM_OPTS,
                
                clobber_abi("C")
            )
        }

        Self { port }
    }
    
    pub fn read(&mut self) -> Result<u8, ()> {
        let mut res: u8;
        let mut err: u8;
        unsafe {
            asm!(
                "mov    ah, 0x2",
                "int    0x14",

                in("dx") self.port as u16,
                out("al") res,
                out("ah") err,

                clobber_abi("C")
            )
        }

        if err & 0x80 != 0 {
            Err(())
        } else {
            Ok(res)
        }
    }
}

impl Connection for ComConnection {
    type Error = &'static str;

    fn write(&mut self, b: u8) -> Result<(), &'static str> {
        self.write(b);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), &'static str> {
        // Our Com ports are unbuffered
        Ok(())
    }
}
