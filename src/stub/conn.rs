use gdbstub::conn::Connection;

use crate::{
    bios::com::{com_init, com_read, com_status, com_write, ComConfiguration, ComStatusFlags},
    dos::io::{com0_read, com0_write},
};

pub struct ComConnection {
    port: u8,
}

impl ComConnection {
    pub fn write(&mut self, b: u8) {
        com_write(self.port, b);
    }

    pub fn status(&self) -> ComStatusFlags {
        com_status(self.port)
    }

    pub fn new(port: u8) -> Self {
        com_init(
            port,
            ComConfiguration::Baud9600
                | ComConfiguration::NoParity
                | ComConfiguration::OneStopBit
                | ComConfiguration::Ascii8,
        );

        Self { port: port }
    }

    pub fn read(&mut self) -> Result<u8, ()> {
        com_read(self.port)
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

pub struct SimpleComConnection;
impl SimpleComConnection {
    pub fn new() -> Self {
        com_init(
            0,
            ComConfiguration::Baud9600
                | ComConfiguration::NoParity
                | ComConfiguration::OneStopBit
                | ComConfiguration::Ascii8,
        );
        
        Self {}
    }
    pub fn read(&mut self) -> u8 {
        com0_read()
    }
    pub fn write(&mut self, b: u8) {
        com0_write(b);
    }
}

impl Connection for SimpleComConnection {
    type Error = &'static str;

    fn write(&mut self, byte: u8) -> Result<(), Self::Error> {
        self.write(byte);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
