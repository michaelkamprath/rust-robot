use crate::{
    system::millis::millis,
    println,
};
use arduino_hal::{Delay, I2c};
use mpu6050::{Mpu6050, Mpu6050Error};

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

const MPU6050_RA_XG_OFFS_USRH: u8 = 0x13;
const MPU6050_RA_XG_OFFS_USRL: u8 = 0x14;
const MPU6050_RA_YG_OFFS_USRH: u8 = 0x15;
const MPU6050_RA_YG_OFFS_USRL: u8 = 0x16;
const MPU6050_RA_ZG_OFFS_USRH: u8 = 0x17;
const MPU6050_RA_ZG_OFFS_USRL: u8 = 0x18;

const MPU6050_GYRO_X_OFFSET: i16 = 82;
const MPU6050_GYRO_Y_OFFSET: i16 = 31;
const MPU6050_GYRO_Z_OFFSET: i16 = -49;


impl HeadingCalculator  {
    pub fn new(i2c: I2c) -> Self {
        let mut mpu6050 = Mpu6050::new(i2c);
        let mut delay = Delay::new();
        match mpu6050.init(&mut delay) {
            Ok(()) => println!("MPU6050 initialized"),
            Err(Mpu6050Error::InvalidChipId(id)) => {
                println!("Error initializing MPU6050: InvalidChipId = {}", id)
            }
            Err(Mpu6050Error::I2c(_error)) => {
                println!("Error initializing MPU6050: I2cError ")
            }
        }
        if let Err(_error) = mpu6050.set_gyro_range(mpu6050::device::GyroRange::D250) {
            println!("Error setting gyro range");
        }

        // set the mpu6050 offsets. These were determined by running the calibration code in the
        // Arduino C++ library and then converting the results to Rust. The calibration code is here:
        //      https://github.com/ElectronicCats/mpu6050/blob/master/examples/IMU_Zero/IMU_Zero.ino
        //
        if let Err(_e) = mpu6050.write_byte(MPU6050_RA_XG_OFFS_USRH, (MPU6050_GYRO_X_OFFSET >> 8) as u8) {
            // todo: handle error
        }
        if let Err(_e) = mpu6050.write_byte(MPU6050_RA_XG_OFFS_USRL, (MPU6050_GYRO_X_OFFSET & 0xFF) as u8) {
           // todo: handle error
        }
        if let Err(_e) = mpu6050.write_byte(MPU6050_RA_YG_OFFS_USRH, (MPU6050_GYRO_Y_OFFSET >> 8) as u8) {
            // todo: handle error
        }
        if let Err(_e) = mpu6050.write_byte(MPU6050_RA_YG_OFFS_USRL, (MPU6050_GYRO_Y_OFFSET & 0xFF) as u8) {
            // todo: handle error
        }
        if let Err(_e) = mpu6050.write_byte(MPU6050_RA_ZG_OFFS_USRH, (MPU6050_GYRO_Z_OFFSET >> 8) as u8) {
            // todo: handle error
        }
        if let Err(_e) = mpu6050.write_byte(MPU6050_RA_ZG_OFFS_USRL, (MPU6050_GYRO_Z_OFFSET & 0xFF) as u8) {
           // todo: handle error
        }
        println!("Gyro offsets set");

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

    /// updates the heading value with the latest gyro measurement, then returns the current heading in degrees
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

    /// returns the current heading in degrees
    pub fn heading(&mut self) -> f32 {
        self.update()
    }
}
