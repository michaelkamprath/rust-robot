use crate::{
    l298n::motor_controller::MotorController,
    model::motor_calibration::get_lr_motor_power,
    telemetry::{ForwardMovementTelemetryRow, FORWARD_MOVEMENT_TELEMETRY_HEADERS, FORWARD_TELEMETRY_COLUMN_COUNT},
    system::{
        data_table::DataTable,
        millis::millis,
    }, println,
};
use avr_device::atmega2560::exint::{eicra, eimsk};
use avr_device::generic::Reg;
use avr_device::interrupt;
use avr_device::interrupt::Mutex;
use core::cell::Cell;
use embedded_hal::{
    digital::v2::{InputPin, OutputPin},
    PwmPin,
};


const WHEEL_CIRCUMFERENCE: f32 = 214.0;     // millimeters
const WHEEL_BASE: f32 = 132.5;              // millimeters
const WHEEL_ENCODER_TICK_COUNT: u32 = 20;

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

/// This is the main hardware abstractions for the robot. It is repsponsible for setting up
/// and providing access to the robot's hardware.
pub struct Robot<
    INA1: OutputPin,
    INA2: OutputPin,
    INB1: OutputPin,
    INB2: OutputPin,
    ENA: PwmPin<Duty = u8>,
    ENB: PwmPin<Duty = u8>,
    BUTT1: InputPin,
> {
    motors: MotorController<INA1, INA2, INB1, INB2, ENA, ENB>,
    button: BUTT1,
    button_pressed: bool,
}

#[allow(dead_code)]
impl<
        INA1: OutputPin,
        INA2: OutputPin,
        INB1: OutputPin,
        INB2: OutputPin,
        ENA: PwmPin<Duty = u8>,
        ENB: PwmPin<Duty = u8>,
        BUTT1: InputPin,
    > Robot<INA1, INA2, INB1, INB2, ENA, ENB, BUTT1>
{
    pub fn new(
        ina1_pin: INA1,
        ina2_pin: INA2,
        inb1_pin: INB1,
        inb2_pin: INB2,
        ena_pin: ENA,
        enb_pin: ENB,
        button_pin: BUTT1,
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
        // println!("   wheel counter interrupts set up");

        // create self structure
        Self {
            motors: MotorController::new(ina1_pin, ina2_pin, inb1_pin, inb2_pin, ena_pin, enb_pin),
            button: button_pin,
            button_pressed: false,
        }
    }

    /// This function is called in the main loop to allow the robot to handle state updates
    pub fn handle_loop(&mut self) {
        // unset button press if button is not pressed
        if self.button.is_high().ok().unwrap() {
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

    /// Resets the wheel counters to 0
    pub fn reset_wheel_counters(&mut self) {
        self.reset_left_wheel_counter();
        self.reset_right_wheel_counter();
    }

    /// Resets the left wheel counters to 0
    pub fn reset_left_wheel_counter(&mut self) {
        interrupt::free(|cs| {
            LEFT_WHEEL_COUNTER.borrow(cs).set(0);
        });
    }

    /// Resets the right wheel counters to 0
    pub fn reset_right_wheel_counter(&mut self) {
        interrupt::free(|cs| {
            RIGHT_WHEEL_COUNTER.borrow(cs).set(0);
        });
    }

    /// Returns the number of wheel ticks on the left wheel since the last reset
    pub fn get_left_wheel_counter(&self) -> u32 {
        let mut counter: u32 = 0;
        interrupt::free(|cs| counter = LEFT_WHEEL_COUNTER.borrow(cs).get());
        counter
    }

    /// Returns the number of wheel ticks on the right wheel since the last reset
    pub fn get_right_wheel_counter(&self) -> u32 {
        let mut counter: u32 = 0;
        interrupt::free(|cs| counter = RIGHT_WHEEL_COUNTER.borrow(cs).get());
        counter
    }

    /// returns true if the button is newly pressed
    pub fn button_pressed(&mut self) -> bool {
        // the button is active low
        if self.button.is_low().ok().unwrap() {
            if !self.button_pressed {
                // println!("robot button pressed");
                self.button_pressed = true;
                return true;
            }
        } else {
            self.button_pressed = false;
        }
        false
    }

    pub fn straight(&mut self, distance_mm: u32) -> &mut Self {
        println!("Robot::straight({})", distance_mm);
        let (left_power, right_power) = get_lr_motor_power(100);
        println!("left_power: {}, right_power: {}", left_power, right_power);

        let mut data = DataTable::<ForwardMovementTelemetryRow, 100, FORWARD_TELEMETRY_COLUMN_COUNT>::new(FORWARD_MOVEMENT_TELEMETRY_HEADERS);

        self.motors.set_duty(left_power, right_power);

        let target_wheel_tick_count: u32 = 1+((WHEEL_ENCODER_TICK_COUNT*distance_mm) as f32 / WHEEL_CIRCUMFERENCE ) as u32;

        self.reset_wheel_counters();
        let mut last_checkin_time = millis();
        self.forward();
        data.append(ForwardMovementTelemetryRow::new(
            last_checkin_time,
            0,
            0,
            0.0,
            target_wheel_tick_count,
        )).ok();

        while (self.get_left_wheel_counter()+self.get_right_wheel_counter())/2 < target_wheel_tick_count {
            self.handle_loop();
            if millis() - last_checkin_time > 100 {
                last_checkin_time = millis();
                let left_ticks = self.get_left_wheel_counter();
                let right_ticks = self.get_right_wheel_counter();
                let distance = ((left_ticks+right_ticks)/2) as f32 * WHEEL_CIRCUMFERENCE / WHEEL_ENCODER_TICK_COUNT as f32;
                data.append(ForwardMovementTelemetryRow::new(
                    last_checkin_time,
                    left_ticks,
                    right_ticks,
                    distance,
                    target_wheel_tick_count,
                )).ok();
            }
        }
        self.stop();
        let left_ticks = self.get_left_wheel_counter();
        let right_ticks = self.get_right_wheel_counter();
        let distance = ((left_ticks+right_ticks)/2) as f32 * WHEEL_CIRCUMFERENCE / WHEEL_ENCODER_TICK_COUNT as f32;
        data.append(ForwardMovementTelemetryRow::new(
            millis(),
            left_ticks,
            right_ticks,
            distance,
            target_wheel_tick_count,
        )).ok();

        println!("Done with robot movement. Wheel counter data collected:\n{}", data);

        self
    }
}
