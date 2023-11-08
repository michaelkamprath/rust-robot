use crate::{
    l298n::{motor_controller::MotorController, motor_enable_pins::MotorEnablePin},
    println,
};
use arduino_hal::{
    hal::port::{PA4, PE3, PE4, PE5, PG5, PH3, PH4},
    port::{
        mode::{self, Floating, Input},
        Pin,
    },
    simple_pwm::{IntoPwmPin, Timer4Pwm},
};
use avr_device::atmega2560::exint::{eicra, eimsk};
use avr_device::generic::Reg;
use avr_device::interrupt;
use avr_device::interrupt::Mutex;
use core::cell::Cell;

static LEFT_WHEEL_COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static RIGHT_WHEEL_COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

// INT3 is d18 pin
#[interrupt(atmega2560)]
fn INT3() {
    interrupt::free(|cs| {
        let counter = LEFT_WHEEL_COUNTER.borrow(cs);
        counter.set(counter.get() + 1);
    });
}

// INT2 is d19 pin
#[interrupt(atmega2560)]
fn INT2() {
    interrupt::free(|cs| {
        let counter = RIGHT_WHEEL_COUNTER.borrow(cs);
        counter.set(counter.get() + 1);
    });
}

pub struct Robot {
    motors: MotorController<
        Pin<mode::Output, PG5>,
        Pin<mode::Output, PE3>,
        Pin<mode::Output, PE4>,
        Pin<mode::Output, PE5>,
        MotorEnablePin<PH3>,
        MotorEnablePin<PH4>,
    >,
    button: Pin<Input<Floating>, PA4>,
    button_pressed: bool,
}

#[allow(dead_code)]
impl Robot {
    pub fn new(
        timer: &mut Timer4Pwm,
        ina1_pin: Pin<Input<Floating>, PG5>,
        ina2_pin: Pin<Input<Floating>, PE3>,
        inb1_pin: Pin<Input<Floating>, PE4>,
        inb2_pin: Pin<Input<Floating>, PE5>,
        ena_pin: Pin<Input<Floating>, PH3>,
        enb_pin: Pin<Input<Floating>, PH4>,
        button_pin: Pin<Input<Floating>, PA4>,
        eicra: &Reg<eicra::EICRA_SPEC>,
        eimsk: &Reg<eimsk::EIMSK_SPEC>,
    ) -> Self {
        // set up wheel counter interupts
        eicra.modify(|_, w| w.isc2().val_0x03());
        eicra.modify(|_, w| w.isc3().val_0x03());
        eimsk.modify(|r, w| {
            let new_bits = r.bits() | 0b00001100; // INT2 and INT3
            w.bits(new_bits)
        });
        println!("   wheel counter interrupts set up");
        // set up button
        let button = button_pin.into_floating_input();
        println!("   button set up");

        // create self structure
        Self {
            motors: MotorController::new(
                ina1_pin.into_output(),
                ina2_pin.into_output(),
                inb1_pin.into_output(),
                inb2_pin.into_output(),
                MotorEnablePin::new(ena_pin.into_output().into_pwm(timer)),
                MotorEnablePin::new(enb_pin.into_output().into_pwm(timer)),
            ),
            button,
            button_pressed: false,
        }
    }

    /// This function is called in the main loop to allow the robot to handle state updates
    pub fn handle_loop(&mut self) {
        // unset button press if button is not pressed
        if self.button.is_high() {
            self.button_pressed = false;
        }
    }

    pub fn forward(&mut self) {
        self.motors.set_duty(75, 75);
        self.motors.forward();
    }

    pub fn stop(&mut self) {
        self.motors.stop();
    }

    pub fn reset_left_wheel_counter(&mut self) {
        interrupt::free(|cs| {
            LEFT_WHEEL_COUNTER.borrow(cs).set(0);
        });
    }

    pub fn reset_right_wheel_counter(&mut self) {
        interrupt::free(|cs| {
            RIGHT_WHEEL_COUNTER.borrow(cs).set(0);
        });
    }

    pub fn get_left_wheel_counter(&self) -> u32 {
        let mut counter: u32 = 0;
        interrupt::free(|cs| counter = LEFT_WHEEL_COUNTER.borrow(cs).get());
        counter
    }

    pub fn get_right_wheel_counter(&self) -> u32 {
        let mut counter: u32 = 0;
        interrupt::free(|cs| counter = RIGHT_WHEEL_COUNTER.borrow(cs).get());
        counter
    }

    /// returns true if the button is newly pressed
    pub fn button_pressed(&mut self) -> bool {
        // the button is active low
        if self.button.is_low() {
            if !self.button_pressed {
                println!("robot button pressed");
                self.button_pressed = true;
                return true;
            }
        } else {
            self.button_pressed = false;
        }
        false
    }
}
