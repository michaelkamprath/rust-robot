use arduino_hal::hal::port::{PH3, PH4, PH5};
use arduino_hal::port::mode::PwmOutput;
use arduino_hal::port::Pin;
use arduino_hal::simple_pwm::Timer4Pwm;
use embedded_hal::PwmPin;

/// This is just glue code so that we can use the abstract PwmPin trait in the l298n struct
pub struct MotorEnablePin<PIN>(Pin<PwmOutput<Timer4Pwm>, PIN>);
impl<PIN> MotorEnablePin<PIN> {
    pub fn new(pin: Pin<PwmOutput<Timer4Pwm>, PIN>) -> Self {
        Self(pin)
    }
}

impl PwmPin for MotorEnablePin<PH3> {
    type Duty = u8;

    fn disable(&mut self) {
        self.0.disable();
    }

    fn enable(&mut self) {
        self.0.enable();
    }

    fn get_duty(&self) -> Self::Duty {
        self.0.get_duty()
    }

    fn get_max_duty(&self) -> Self::Duty {
        self.0.get_max_duty()
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        self.0.set_duty(duty);
    }
}

impl PwmPin for MotorEnablePin<PH4> {
    type Duty = u8;

    fn disable(&mut self) {
        self.0.disable();
    }

    fn enable(&mut self) {
        self.0.enable();
    }

    fn get_duty(&self) -> Self::Duty {
        self.0.get_duty()
    }

    fn get_max_duty(&self) -> Self::Duty {
        self.0.get_max_duty()
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        self.0.set_duty(duty);
    }
}

impl PwmPin for MotorEnablePin<PH5> {
    type Duty = u8;

    fn disable(&mut self) {
        self.0.disable();
    }

    fn enable(&mut self) {
        self.0.enable();
    }

    fn get_duty(&self) -> Self::Duty {
        self.0.get_duty()
    }

    fn get_max_duty(&self) -> Self::Duty {
        self.0.get_max_duty()
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        self.0.set_duty(duty);
    }
}
