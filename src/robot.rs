use arduino_hal::{simple_pwm::{Timer4Pwm, IntoPwmPin}, port::{mode::{self, Input, Floating}, Pin}, hal::port::{PG5, PE3, PE4, PE5, PH3, PH4}};
use avr_device::interrupt::Mutex;
use avr_device::interrupt;
use avr_device::generic::Reg;
use avr_device::atmega2560::exint::{eicra, eimsk};
use core::cell::Cell;
use crate::{l298n::{motor_controller::MotorController, motor_enable_pins::MotorEnablePin}, println, serial_print};


static LEFT_WHEEL_COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static RIGHT_WHEEL_COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

// INT3 is d18 pin
#[interrupt(atmega2560)]
fn INT3() {
    interrupt::free(|cs| {
        let counter  = LEFT_WHEEL_COUNTER.borrow(cs);
        counter.set(counter.get() + 1);
    });
}

// INT2 is d19 pin
#[interrupt(atmega2560)]
fn INT2() {
    interrupt::free(|cs| {
        let counter  = RIGHT_WHEEL_COUNTER.borrow(cs);
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
                MotorEnablePin<PH4>
            >
}

#[allow(dead_code)]
impl Robot {
    pub fn new(
            timer: &mut Timer4Pwm, 
            ina1: Pin<Input<Floating>, PG5>,
            ina2: Pin<Input<Floating>, PE3>,
            inb1: Pin<Input<Floating>, PE4>,
            inb2: Pin<Input<Floating>, PE5>,
            ena: Pin<Input<Floating>, PH3>,
            enb: Pin<Input<Floating>, PH4>,
            eicra: &Reg<eicra::EICRA_SPEC>,
            eimsk: &Reg<eimsk::EIMSK_SPEC>,
            
        ) -> Self 
    {
        // set up wheel counter interupts
        eicra.modify(|_, w| w.isc2().val_0x03());
        eicra.modify(|_, w| w.isc3().val_0x03());
        eimsk.modify(|r, w| {
            let cur_bits: u8 = r.bits();
            let new_bits = cur_bits|0b00001100;  // INT2 and INT3 
            println!("current bits: {}, new bits {}", cur_bits, new_bits);
            w.bits(new_bits)
        });

        // create self structure
        Self {
            motors: MotorController::new(
                ina1.into_output(),
                ina2.into_output(),
                inb1.into_output(),
                inb2.into_output(),
                MotorEnablePin::new(ena.into_output().into_pwm(timer)),
                MotorEnablePin::new(enb.into_output().into_pwm(timer)),
            )
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
        interrupt::free(|cs| {
            counter = LEFT_WHEEL_COUNTER.borrow(cs).get()
        });
        counter
    }

    pub fn get_right_wheel_counter(&self) -> u32 {
        let mut counter: u32 = 0;
        interrupt::free(|cs| {
            counter = RIGHT_WHEEL_COUNTER.borrow(cs).get()
        });
        counter
    }
}