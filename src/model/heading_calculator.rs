use crate::system::millis::millis;
use arduino_hal::I2c;
use mpu6050::Mpu6050;

// CALIBRATION
// ....................	XAccel			YAccel				ZAccel			XGyro			YGyro			ZGyro
// [-2505,-2504] --> [-4,8]	[385,386] --> [-2,14]	[1559,1560] --> [16371,16394]	[82,83] --> [0,5]	[31,31] --> [0,2]	[-49,-48] --> [-1,2]
//  .................... [-2505,-2504] --> [-1,8]	[385,386] --> [-4,14]	[1559,1560] --> [16368,16394]	[82,82] --> [0,1]	[31,31] --> [0,1]	[-49,-48] --> [-1,2]
// -------------- done --------------
 
pub struct HeadingCalculator {
    heading: f32,
    mpu6050: Mpu6050<I2c>,
    last_update_rate: f32,
    last_update_time: u32,
}

impl HeadingCalculator {
    pub fn new(mpu6050: Mpu6050<I2c>) -> Self {
        Self {
            heading: 0.0,
            mpu6050,
            last_update_rate: 0.0,
            last_update_time: millis(),
        }
    }

    pub fn reset(&mut self) {
        self.heading = 0.0;
        self.last_update_rate = 0.0;
        self.last_update_time = millis();
    }

    pub fn update(&mut self) -> f32 {
        let now = millis();
        let delta_time = now - self.last_update_time;
        if delta_time > 50 {
            if let Ok(gyro) = self.mpu6050.get_gyro() {
                // the heding is about the sensor's Z-axis
                let delta_rads = gyro.z * delta_time as f32 / 1000.0;
                self.heading += delta_rads;
                self.last_update_rate = gyro.z;
            }
        }

        self.heading
    }

    pub fn heading(&mut self) -> f32 {
        self.update()
    }
}
