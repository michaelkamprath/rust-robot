// This code taken from the example code in the avr-hal crate, which is licensed under the MIT license:
//      https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-println.rs
//
use avr_device::interrupt;
use core::cell::RefCell;

type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
pub static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
    interrupt::Mutex::new(RefCell::new(None));

#[macro_export]
macro_rules! print {
    ($($t:tt)*) => {
        avr_device::interrupt::free(
            |cs| {
                if let Some(console) = $crate::system::serial_print::CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwrite!(console, $($t)*);
                }
            },
        )
    };
}

#[macro_export]
macro_rules! println {
    ($($t:tt)*) => {
        avr_device::interrupt::free(
            |cs| {
                if let Some(console) = $crate::system::serial_print::CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwriteln!(console, $($t)*);
                }
            },
        )
    };
}

#[macro_export]
macro_rules! print_with_fn {
    ($print_fn:expr) => {
        avr_device::interrupt::free(|cs| {
            if let Some(console) = $crate::system::serial_print::CONSOLE
                .borrow(cs)
                .borrow_mut()
                .as_mut()
            {
                let _ = $print_fn(console);
            }
        })
    };
}

pub fn put_console(console: Console) {
    interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}
