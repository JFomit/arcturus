use core::{arch::asm, num::NonZero};

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum WatchpointAction {
    Execute = 0b00,
    Write = 0b01,
    Read = 0b11,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum WatchpointSize {
    Size1, // TODO
    Size2,
    Size4,
}

#[derive(Clone)]
pub struct Watchpoint {
    pub address: u32,
    pub action: WatchpointAction,
    pub global: bool,
    pub size: WatchpointSize,
}

impl Watchpoint {
    fn dr7_mask_for_num(num: u8) -> Option<NonZero<u32>> {
        if num < 4 {
            NonZero::new((0b11u32 << (2 * num)) | 0b1111u32 << (16 + 4 * num))
        } else {
            None
        }
    }
    fn to_dr7_repr(&self, num: u8) -> Option<NonZero<u32>> {
        if num < 4 {
            let locality: u32 = if self.global { 0b10 } else { 0b01 } << (2 * num);
            let size_stop: u32 = (self.size as u32 | self.action as u32) << (16 + 4 * num);
            NonZero::new(locality | size_stop)
        } else {
            None
        }
    }
    fn from_dr7_repr(num: u8, dr7: u32) -> Self {
        todo!()
    }
}

pub fn set_watchpoint(num: u8, watchpoint: &Watchpoint) -> Option<Watchpoint> {
    let mask = !Watchpoint::dr7_mask_for_num(num)?.get();
    let repr = watchpoint.to_dr7_repr(num)?.get();
    let old_dr7: u32;
    unsafe {
        match num {
            0 => asm!("mov  dr0, {0}", in(reg) watchpoint.address),
            1 => asm!("mov  dr1, {0}", in(reg) watchpoint.address),
            2 => asm!("mov  dr2, {0}", in(reg) watchpoint.address),
            3 => asm!("mov  dr3, {0}", in(reg) watchpoint.address),
            _ => unreachable!()
        }
        asm!(
            "mov    {old}, dr7",     // Load DR7
            "mov    {new}, {old}",
            "and    {new}, {mask}",  // Clear bits related to the watchpoint in DR7
            "or     {new}, {val}",   // Set bits to target values
            "mov    dr7,  {new}",
            old = out(reg) old_dr7,
            new = out(reg) _,
            mask = in(reg) mask,
            val = in(reg) repr,
        )
    }
    Some(Watchpoint::from_dr7_repr(num, old_dr7))
}

pub fn get_watchpoint(num: u8) -> Option<Watchpoint> {
    todo!()
}
