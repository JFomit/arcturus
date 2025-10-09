use core::arch::asm;

use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    pub struct ComStatusFlags : u16 {
        const TimeOutError = 1 << 15;
        const TransmitterShiftRegisterEmpty = 1 << 14;
        const TransmitterHoldingRegisterEmpty = 1 << 13;
        const BreakDetectionError = 1 << 12;
        const FramingError = 1 << 11;
        const ParityError = 1 << 10;
        const OverrunError = 1 << 9;
        const DataAvailable = 1 << 8;
        const ReceiveLineSignalDetect = 1 << 7;
        const RingIndicator = 1 << 6;
        const DataSetReady = 1 << 5;
        const ClearToSend = 1 << 4;
        const DeltaReceiveLineSignalDetect = 1 << 3;
        const TrailingEdgeRingDetector = 1 << 2;
        const DeltaDataSetReady = 1 << 1;
        const DeltaClearToSend = 1 << 0;

        const _ = !0;
    }
}

pub const COM_OPTS: u16 = 0b0000_0000_1110_0011;

bitflags! {
    pub struct ComConfiguration : u8 {
        const Ascii7 = 0b10;
        const Ascii8 = 0b11;

        const OneStopBit = 0b000;
        const TwoStopBits = 0b100;

        const EvenParity = 0b11000;
        const OddParity = 0b01000;
        const NoParity = 0b00000;

        const Baud110 = 0b000_00000;
        const Baud150 = 0b001_00000;
        const Baud300 = 0b010_00000;
        const Baud600 = 0b011_00000;
        const Baud1200 = 0b100_00000;
        const Baud2400 = 0b101_00000;
        const Baud4800 = 0b110_00000;
        const Baud9600 = 0b111_00000;

        const _ = !0;
    }
}

pub fn com_init(port: u8, flags: ComConfiguration) {
    unsafe {
        asm!(
            "int	0x14",

            in("dx") port as u16,
            in("ax") flags.bits() as u16,

            clobber_abi("C")
        )
    }
}

pub fn com_write(port: u8, b: u8) {
    unsafe {
        // TODO: error-handling
        asm!(
            "mov    ah, 0x1",
            "int    0x14",
            in("al") b,
            in("dx") port as u16,

            clobber_abi("C")
        );
    }
}

pub fn com_read(port: u8) -> Result<u8, ()> {
    let mut res: u8;
    let mut err: u8;
    unsafe {
        asm!(
            "mov    ah, 0x2",
            "int    0x14",

            in("dx") port as u16,
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

pub fn com_status(port: u8) -> ComStatusFlags {
    let mut flags: u16;

    unsafe {
        asm!(
            "mov    ah, 0x3",
            "int    0x14",
            in("dx") port as u16,
            out("ax") flags,
            clobber_abi("C")
        );
    }

    ComStatusFlags::from_bits_truncate(flags)
}
