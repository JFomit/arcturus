use core::panic::PanicInfo;

use crate::_exit;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{:?}", _info);
    _exit(1);
}
