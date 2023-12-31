use arduino_hal::{delay_ms, I2c};
use micromath::F32Ext;
use ufmt::{uDebug, uDisplay, uWrite, uwrite, Formatter};

use crate::{
    l298n::motor_controller::MotorController,
    model::{
        heading_calculator::HeadingCalculator, motor_calibration::get_lr_motor_power,
        pid_controller::PIDController,
    },
    print_with_fn, println,
    system::{data_logging::log_csv_headers, millis::millis},
    telemetry::{ForwardMovementTelemetryRow, FORWARD_MOVEMENT_TELEMETRY_HEADERS},
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

const WHEEL_CIRCUMFERENCE: f32 = 214.0; // millimeters
const WHEEL_BASE: f32 = 132.5; // millimeters
const WHEEL_ENCODER_TICK_COUNT: u32 = 20;
const CONTROL_LOOP_PERIOD: u32 = 75; // milliseconds

const HEADING_PID_CONTROLLER_KP: f32 = 20.0;
const HEADING_PID_CONTROLLER_KI: f32 = 0.0;
const HEADING_PID_CONTROLLER_KD: f32 = 0.0;

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
    heading_calculator: HeadingCalculator,
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
        i2c: I2c,
    ) -> Self {
        // set up wheel counter interupts
        eicra.modify(|_, w| w.isc2().val_0x03());
        eicra.modify(|_, w| w.isc3().val_0x03());
        eimsk.modify(|r, w| {
            let new_bits = r.bits() | 0b00001100; // INT2 and INT3
            w.bits(new_bits)
        });
        // create self structure
        let heading_calculator = HeadingCalculator::new(i2c);

        println!("Robot initialized");
        Self {
            motors: MotorController::new(ina1_pin, ina2_pin, inb1_pin, inb2_pin, ena_pin, enb_pin),
            button: button_pin,
            button_pressed: false,
            heading_calculator,
        }
        
    }

    /// This function is called in the main loop to allow the robot to handle state updates
    pub fn handle_loop(&mut self) {
        // unset button press if button is not pressed
        if self.button.is_high().ok().unwrap() {
            self.button_pressed = false;
        }

        self.heading_calculator.update();
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
                println!("robot button pressed");
                self.button_pressed = true;
                return true;
            }
        } else {
            self.button_pressed = false;
        }
        false
    }

    pub fn straight(&mut self, distance_mm: u32) -> &mut Self {
        println!("Robot move straight, distance = {}", distance_mm);
        let target_power: u8 = 125;
        let (left_target_power, right_target_power) = get_lr_motor_power(target_power);
        let mut controller = PIDController::new(
            HEADING_PID_CONTROLLER_KP,
            HEADING_PID_CONTROLLER_KI,
            HEADING_PID_CONTROLLER_KD,
        );
        // we want a heading of 0.0 (straight ahead)
        controller.set_setpoint(0.0);
        controller.set_max_control_signal(30.0);
        println!("controller = {}", controller);
        // heading is in radians
        let mut heading: f32 = 0.0;

        self.motors.set_duty(left_target_power, right_target_power);

        let target_wheel_tick_count: u32 =
            1 + ((WHEEL_ENCODER_TICK_COUNT * distance_mm) as f32 / WHEEL_CIRCUMFERENCE) as u32;

        self.reset_wheel_counters();

        println!(
            "Starting robot movement. Target wheel tick count = {}\nData table:\n\n",
            target_wheel_tick_count,
        );

        print_with_fn!(|f| { log_csv_headers(f, &FORWARD_MOVEMENT_TELEMETRY_HEADERS,) });
        let mut last_left_ticks = 0;
        let mut last_right_ticks = 0;
        let mut last_checkin_time = millis();
        controller.reset(last_checkin_time);
        self.heading_calculator.reset();
        self.motors.forward();

        let mut data_row = ForwardMovementTelemetryRow::new(
            last_checkin_time,
            0,
            0,
            0.0,
            0.0,
            0.0,
            self.heading_calculator.heading(),
            0.0,
            controller.integral,
            self.motors.get_duty_a(),
            self.motors.get_duty_b(),
        );

        println!("{}", data_row);

        while (self.get_left_wheel_counter() + self.get_right_wheel_counter()) / 2
            < target_wheel_tick_count
        {
            self.handle_loop();
            if millis() - last_checkin_time > CONTROL_LOOP_PERIOD
            {
                let current_time = millis();
                let left_ticks = self.get_left_wheel_counter();
                let right_ticks = self.get_right_wheel_counter();
                let delta_left_ticks = left_ticks - last_left_ticks;
                let delta_right_ticks = right_ticks - last_right_ticks;
                let distance = ((left_ticks + right_ticks) / 2) as f32 * WHEEL_CIRCUMFERENCE
                    / WHEEL_ENCODER_TICK_COUNT as f32;

                // calculate heading change since last checkin
                // left turn is positive per right hand rule
                let heading_change = (WHEEL_CIRCUMFERENCE / WHEEL_ENCODER_TICK_COUNT as f32)
                    * (delta_right_ticks as f32 - delta_left_ticks as f32)
                    / WHEEL_BASE;
                heading += heading_change;
                let current_heading = self.heading_calculator.heading();

                // get control signal from PID controller
                let control_signal = controller.update(current_heading, current_time);

                // set motor power. positive control signal means turn left, a negative control signal means turn right
                let adjustment = control_signal.abs() as u8;
                if control_signal.is_sign_positive() {
                    self.motors.set_duty(
                        left_target_power - adjustment / 2,
                        right_target_power + adjustment,
                    );
                } else {
                    self.motors.set_duty(
                        left_target_power + adjustment,
                        right_target_power - adjustment / 2,
                    );
                }

                println!(
                    "{}",
                    data_row.update(
                        current_time,
                        left_ticks,
                        right_ticks,
                        distance,
                        heading_change,
                        heading,
                        current_heading,
                        control_signal,
                        controller.integral,
                        self.motors.get_duty_a(),
                        self.motors.get_duty_b(),
                    )
                );

                // update last checkin values
                last_left_ticks = left_ticks;
                last_right_ticks = right_ticks;
                last_checkin_time = current_time;
            }
        }
        self.motors.stop();
        let stop_millis = millis();
        let left_ticks = self.get_left_wheel_counter();
        let right_ticks = self.get_right_wheel_counter();
        let left_power = self.motors.get_duty_a();
        let right_power = self.motors.get_duty_b();
        // ensure a stop by reversing for a short time
        self.motors.set_duty(255, 255);
        self.motors.reverse();
        delay_ms(100);
        self.motors.stop();

        let distance = ((left_ticks + right_ticks) / 2) as f32 * WHEEL_CIRCUMFERENCE
            / WHEEL_ENCODER_TICK_COUNT as f32;
        let heading_change = (WHEEL_CIRCUMFERENCE / WHEEL_ENCODER_TICK_COUNT as f32)
            * (right_ticks as f32 - left_ticks as f32)
            / WHEEL_BASE;
        heading += heading_change;
        println!(
            "{}\n",
            data_row.update(
                stop_millis,
                left_ticks,
                right_ticks,
                distance,
                heading_change,
                heading,
                self.heading_calculator.heading(),
                0.0,
                controller.integral,
                left_power,
                right_power,
            )
        );
        println!(
            "Stop overshoot: left_ticks = {}, right_ticks = {}",
            self.get_left_wheel_counter() - left_ticks,
            self.get_right_wheel_counter() - right_ticks,
        );
        println!("Done with robot movement.");

        // println!("Plotting control signal");
        // print_with_fn!(|f| {
        //     data.plot(f, |row: &ForwardMovementTelemetryRow| {
        //         row.control_signal() as i32
        //     })
        // });

        self
    }

    #[cfg(feature = "calibrate_motors")]
    pub fn calibrate_motors(&mut self) {
        println!("{}", F!("Calibrating motors"));

        const COUNT_TEST_POWER_LEVELS: usize = 12;
        const COUNT_TEST_RUNS: usize = 10;
        const COUNT_TEST_DATA_ROWS: usize = COUNT_TEST_POWER_LEVELS * COUNT_TEST_RUNS;
        let test_power_levels: [u8; COUNT_TEST_POWER_LEVELS] =
            [70, 80, 90, 100, 110, 120, 140, 160, 180, 200, 225, 255];

        let mut data = uDataTable::<MotorCalibrationRow, COUNT_TEST_DATA_ROWS, 5>::new([
            "test_id",
            "power",
            "left_ticks",
            "right_ticks",
            "lr_ratio",
        ]);

        let mut test_id: u16 = 0;
        for test_power in test_power_levels.iter() {
            println!("{}{}", F!("testing power: "), test_power);
            for i in 0..COUNT_TEST_RUNS {
                println!("    run #{}", i);
                test_id += 1;
                let left_power = *test_power;
                let right_power = *test_power;
                self.motors.set_duty(left_power, right_power);
                self.reset_wheel_counters();
                self.motors.forward();
                while self.get_left_wheel_counter() < 200 {
                    self.handle_loop();
                }
                self.motors.stop();
                let left_ticks = self.get_left_wheel_counter();
                let right_ticks = self.get_right_wheel_counter();
                self.motors.set_duty(255, 255);
                self.motors.reverse();
                delay_ms(50);
                self.motors.stop();
                delay_ms(1000);
                let lr_ratio = left_ticks as f32 / right_ticks as f32;
                if let Err(row) = data.append(MotorCalibrationRow {
                    test_id,
                    power: *test_power,
                    left_ticks,
                    right_ticks,
                    lr_ratio,
                }) {
                    println!("{}{}", F!("Error appending row to data table: "), row);
                }

                println!(
                    "        left_ticks: {}, right_ticks: {}, lr_ratio: {}",
                    left_ticks, right_ticks, lr_ratio
                );
            }
        }

        println!(
            "{}{}",
            F!("Done with motor calibration. Data collected:\n"),
            data
        );
    }
}

#[derive(Default, Copy, Clone)]
pub struct MotorCalibrationRow {
    test_id: u16,
    power: u8,
    left_ticks: u32,
    right_ticks: u32,
    lr_ratio: f32,
}

impl uDebug for MotorCalibrationRow {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "MotorCalibrationRow {{ test_id: {}, power: {}, left_ticks: {}, right_ticks: {}, lr_ratio: {} }}", self.test_id, self.power, self.left_ticks, self.right_ticks, self.lr_ratio)
    }
}

impl uDisplay for MotorCalibrationRow {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(
            f,
            "{}, {}, {}, {}, {}",
            self.test_id,
            self.power,
            self.left_ticks,
            self.right_ticks,
            self.lr_ratio
        )
    }
}
