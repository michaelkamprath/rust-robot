use micromath::F32Ext;

use crate::println;

#[derive(Default, Clone)]
pub struct PIDController {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub setpoint: f32,
    pub integral: f32,
    pub last_error: f32,
    pub last_time: u32,
    pub max_control_signal: f32,
}

#[allow(dead_code)]
impl PIDController {

    /// Create a new PIDController with the given gains.
    /// `kp` is the propotial can in the same units as the error.
    /// `ki` is the integral gain in the same units as the error per unit time.
    /// `kd` is the derivative gain in the same units as the error per unit time.
    /// Note that if you are using millis for time, then your constants will be in units of
    /// error per millisecond.
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        PIDController {
            kp,
            ki,
            kd,
            setpoint: 0.0,
            integral: 0.0,
            last_error: 0.0,
            last_time: 0,
            max_control_signal: 0.0,
        }
    }

    /// The setpoint is the desired value of the measurement.
    pub fn set_setpoint(&mut self, setpoint: f32) {
        self.setpoint = setpoint;
    }

    /// The max control signal is the maximum absolute value that the controller will output.
    pub fn set_max_control_signal(&mut self, max_control_signal: f32) {
        self.max_control_signal = max_control_signal;
    }

    /// Update the controller with a new measurement and the time of the measurement.
    pub fn update(&mut self, measurement: f32, measurement_time: u32) -> f32 {
        if self.last_time > measurement_time {
            println!("Time went backwards! {} -> {}", self.last_time, measurement_time);
            return 0.0;
        } else if self.last_time == measurement_time {
            println!("Time didn't change! {}", measurement_time);
            return 0.0;
        }

        let dt = (measurement_time - self.last_time) as f32;
        let error = self.setpoint - measurement;
        self.integral += error * dt;
        let derivative = (error - self.last_error) / dt;
        let mut control_signal = self.kp * error + self.ki * self.integral + self.kd * derivative;
        self.last_error = error;
        self.last_time = measurement_time;
        if self.max_control_signal > 0.0 && control_signal.abs() > self.max_control_signal {
            control_signal = control_signal.signum() * self.max_control_signal;
        }
        control_signal
    }

    /// Reset the controller to its initial state.
    pub fn reset(&mut self, start_time: u32) {
        self.integral = 0.0;
        self.last_error = 0.0;
        self.last_time = start_time;
    }
}
