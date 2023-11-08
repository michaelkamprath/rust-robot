#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod l298n;
mod robot;
mod system;

use arduino_hal::simple_pwm::{Prescaler, Timer4Pwm};
use panic_halt as _;

use arduino_hal::pac::Peripherals;

use robot::Robot;
use system::{
    millis::{millis, millis_init},
    serial_print::put_console,
};

#[arduino_hal::entry]
fn main() -> ! {
    let dp: Peripherals = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    put_console(arduino_hal::default_serial!(dp, pins, 57600));
    println!("Starting the Rust Robot! :D");
    millis_init(dp.TC0);
    println!("Millis initialized");

    let mut timer4: Timer4Pwm = Timer4Pwm::new(dp.TC4, Prescaler::Prescale64);

    let mut robot = Robot::new(
        &mut timer4,
        pins.d4,
        pins.d5,
        pins.d2,
        pins.d3,
        pins.d6,
        pins.d7,
        pins.d26,
        &dp.EXINT.eicra,
        &dp.EXINT.eimsk,
    );
    println!("Robot initialized");
    let mut led = pins.d13.into_output();
    println!("LED initialized");
    unsafe { avr_device::interrupt::enable() };
    println!("Interrupts enabled");

    robot.reset_wheel_counters();
    let mut led_blink_time = millis();
    loop {
        if robot.button_pressed() {
            led.set_high();
            let start_time = millis();
            led_blink_time = start_time;
            robot.forward();
            while millis() - start_time < 1000 {
                robot.handle_loop();
                
                if millis() - led_blink_time > 50 {
                    led_blink_time = millis();
                    led.toggle();
                }
            }
            robot.stop();
            led.set_low();
            led_blink_time = millis();
            println!(
                "Left wheel counter: {}, right wheel counter {}",
                robot.get_left_wheel_counter(),
                robot.get_right_wheel_counter()
            );
        }
        if millis() - led_blink_time > 1000 {
            led_blink_time = millis();
            led.toggle();
        }
        robot.handle_loop();
    }
}
