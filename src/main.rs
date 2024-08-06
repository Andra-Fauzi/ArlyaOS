#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;
use crate::vga_buffer::{ColorCode, Writer,Color,Buffer};
use core::panic::PanicInfo;

/// This function is called on panic.

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}",info);
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("halo{}","!");
    loop {}
}

/*
 fn print_something() {
    use core::fmt::Write;
    let mut writer: Writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    write!(writer,"halo").unwrap();
}
*/
