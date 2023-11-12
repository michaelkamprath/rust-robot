use embedded_hal::digital::v2::OutputPin;
use embedded_hal::PwmPin;

pub struct MotorController<INA1, INA2, INB1, INB2, ENA, ENB> {
    ina1: INA1,
    ina2: INA2,
    inb1: INB1,
    inb2: INB2,
    ena: ENA,
    enb: ENB,
}

#[allow(dead_code)]
impl<INA1, INA2, INB1, INB2, ENA, ENB> MotorController<INA1, INA2, INB1, INB2, ENA, ENB>
where
    INA1: OutputPin,
    INA2: OutputPin,
    INB1: OutputPin,
    INB2: OutputPin,
    ENA: PwmPin,
    ENB: PwmPin,
{
    pub fn new(ina1: INA1, ina2: INA2, inb1: INB1, inb2: INB2, ena: ENA, enb: ENB) -> Self
    where
        INA1: OutputPin,
        INA2: OutputPin,
        INB1: OutputPin,
        INB2: OutputPin,
        ENA: PwmPin,
        ENB: PwmPin,
    {
        Self {
            ina1,
            ina2,
            inb1,
            inb2,
            ena,
            enb,
        }
    }

    pub fn set_duty(&mut self, duty_a: ENA::Duty, duty_b: ENB::Duty) {
        self.ena.set_duty(duty_a);
        self.enb.set_duty(duty_b);
    }

    pub fn set_duty_a(&mut self, duty: ENA::Duty) {
        self.ena.set_duty(duty);
    }

    pub fn set_duty_b(&mut self, duty: ENB::Duty) {
        self.enb.set_duty(duty);
    }

    pub fn get_duty_a(&self) -> ENA::Duty {
        self.ena.get_duty()
    }

    pub fn get_duty_b(&self) -> ENB::Duty {
        self.enb.get_duty()
    }

    pub fn forward(&mut self) {
        self.ina1.set_high().ok();
        self.ina2.set_low().ok();
        self.inb1.set_high().ok();
        self.inb2.set_low().ok();
        self.ena.enable();
        self.enb.enable();
    }

    pub fn forward_a(&mut self) {
        self.ina1.set_high().ok();
        self.ina2.set_low().ok();
        self.ena.enable();
    }

    pub fn forward_b(&mut self) {
        self.inb1.set_high().ok();
        self.inb2.set_low().ok();
        self.enb.enable();
    }

    pub fn reverse(&mut self) {
        self.ina1.set_low().ok();
        self.ina2.set_high().ok();
        self.inb1.set_low().ok();
        self.inb2.set_high().ok();
        self.ena.enable();
        self.enb.enable();
    }

    pub fn reverse_a(&mut self) {
        self.ina1.set_low().ok();
        self.ina2.set_high().ok();
        self.ena.enable();
    }

    pub fn reverse_b(&mut self) {
        self.inb1.set_low().ok();
        self.inb2.set_high().ok();
        self.enb.enable();
    }

    pub fn stop(&mut self) {
        self.ena.disable();
        self.enb.disable();
        self.ina1.set_low().ok();
        self.ina2.set_low().ok();
        self.inb1.set_low().ok();
        self.inb2.set_low().ok();
    }

    pub fn stop_a(&mut self) {
        self.ena.disable();
        self.ina1.set_low().ok();
        self.ina2.set_low().ok();
    }

    pub fn stop_b(&mut self) {
        self.enb.disable();
        self.inb1.set_low().ok();
        self.inb2.set_low().ok();
    }
}
