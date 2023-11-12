use micromath::F32Ext;

#[derive(Default, Clone)]
pub struct PIDController {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub setpoint: f32,
    pub integral: f32,
    pub last_error: f32,
    pub last_millis: u32,
    pub max_control_signal: f32,
}

#[allow(dead_code)]
impl PIDController {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        PIDController {
            kp,
            ki,
            kd,
            setpoint: 0.0,
            integral: 0.0,
            last_error: 0.0,
            last_millis: 0,
            max_control_signal: 0.0,
        }
    }

    pub fn set_setpoint(&mut self, setpoint: f32) {
        self.setpoint = setpoint;
    }

    pub fn set_max_control_signal(&mut self, max_control_signal: f32) {
        self.max_control_signal = max_control_signal;
    }

    pub fn update(&mut self, measurement: f32, measurement_millis: u32) -> f32 {
        let dt = (measurement_millis - self.last_millis) as f32;
        let error = self.setpoint - measurement;
        self.integral += error * dt;
        let derivative = (error - self.last_error) / dt;
        let mut control_signal = self.kp * error + self.ki * self.integral + self.kd * derivative;
        self.last_error = error;
        self.last_millis = measurement_millis;
        if self.max_control_signal > 0.0 && control_signal.abs() > self.max_control_signal {
            control_signal = control_signal.signum() * self.max_control_signal;
        }
        control_signal
    }

    pub fn reset(&mut self, start_millis: u32) {
        self.integral = 0.0;
        self.last_error = 0.0;
        self.last_millis = start_millis;
    }
}
