#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod l298n;
mod model;
mod robot;
mod system;
mod telemetry;

use panic_halt as _;

use avr_progmem::progmem_str as F;
use arduino_hal;

use arduino_hal::{
        pac::Peripherals,
        simple_pwm::{IntoPwmPin, Prescaler, Timer4Pwm},
};

use robot::Robot;
use system::{
    millis::{millis, millis_init},
    serial_print::put_console,
};

use crate::l298n::motor_enable_pins::MotorEnablePin;


#[arduino_hal::entry]
fn main() -> ! {
    let dp: Peripherals = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    put_console(serial);
    println!("{}",F!("Starting the Rust Robot! :D"));
    millis_init(dp.TC0);
    println!("{}",F!("Millis initialized"));


    let timer4: Timer4Pwm = Timer4Pwm::new(dp.TC4, Prescaler::Prescale64);

    let mut robot = Robot::new(
        pins.d4.into_output(),
        pins.d5.into_output(),
        pins.d2.into_output(),
        pins.d3.into_output(),
        MotorEnablePin::new(pins.d6.into_output().into_pwm(&timer4)),
        MotorEnablePin::new(pins.d7.into_output().into_pwm(&timer4)),
        pins.d26.into_floating_input(),
        &dp.EXINT.eicra,
        &dp.EXINT.eimsk,
    );
    println!("{}",F!("Robot initialized"));
    let mut led = pins.d13.into_output();
    println!("{}",F!("LED initialized"));
    unsafe { avr_device::interrupt::enable() };
    println!("{}",F!("Interrupts enabled"));

    robot.reset_wheel_counters();
    let mut led_blink_time = millis();
    loop {
        if robot.button_pressed() {
            println!("{}",F!("Button pressed, calibrating motors"));
            led.set_high();
            robot.calibrate_motors();
            led.set_low();
            led_blink_time = millis();
        }
        if millis() - led_blink_time > 1000 {
            led_blink_time = millis();
            led.toggle();
        }
        robot.handle_loop();
    }
}
