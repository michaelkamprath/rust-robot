#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod l298n;
mod robot;
mod serial_print;

use arduino_hal::simple_pwm::{Timer4Pwm, Prescaler};
use panic_halt as _;

use arduino_hal::pac::Peripherals;
use arduino_hal::delay_ms;

use robot::Robot;
use serial_print::put_console;


#[arduino_hal::entry]
fn main() -> ! {
    

    let dp: Peripherals = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    put_console(serial);
    println!("Starting the Rust Robot! :D");

    let mut timer4: Timer4Pwm = Timer4Pwm::new(dp.TC4, Prescaler::Prescale64);

    let mut robot = Robot::new(
        &mut timer4,
        pins.d4,
        pins.d5,
        pins.d2,
        pins.d3,
        pins.d6,
        pins.d7,
        &dp.EXINT.eicra,
        &dp.EXINT.eimsk,
    );

    let mut led = pins.d13.into_output();
    println!("Enabling interrupts! :D");
    unsafe { avr_device::interrupt::enable() };
    println!("Interrupts enabled! :D");

    robot.reset_left_wheel_counter();
    loop {
        led.toggle();
        if led.is_set_high() {
            robot.forward();
        } else {
            robot.stop();
            println!("Left wheel counter: {}, right wheel counter {}", robot.get_left_wheel_counter(), robot.get_right_wheel_counter());
        }
        delay_ms(1000);
    }
}
