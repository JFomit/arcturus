use core::panic::PanicInfo;

use crate::_exit;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("> debugger panicked:\r\n> {}", _info);
    _exit(1);
}
