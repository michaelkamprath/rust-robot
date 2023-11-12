use ufmt::{uDebug, uDisplay, uWrite, uwrite, Formatter};

pub const FORWARD_TELEMETRY_COLUMN_COUNT: usize = 11;
pub static FORWARD_MOVEMENT_TELEMETRY_HEADERS: [&str; FORWARD_TELEMETRY_COLUMN_COUNT] = [
    "millis",
    "Left Wheel Counter",
    "Right Wheel Counter",
    "Distance",
    "Target Wheel Tick Count",
    "Delta Heading",
    "Current Heading",
    "Control Signal",
    "Control Error Integral",
    "Updated Left Power",
    "Updated Right Power",
];

#[derive(Copy, Clone, Default)]
pub struct ForwardMovementTelemetryRow {
    timestamp: u32,
    left_encoder: u32,
    right_encoder: u32,
    distance: f32,
    target_wheel_tick_count: u32,
    delta_heading: f32,
    current_heading: f32,
    control_signal: f32,
    control_error_integral: f32,
    updated_left_power: u8,
    updated_right_power: u8,
}

#[allow(dead_code)]
impl ForwardMovementTelemetryRow {
    pub fn new(
        timestamp: u32,
        left_encoder: u32,
        right_encoder: u32,
        distance: f32,
        target_wheel_tick_count: u32,
        delta_heading: f32,
        current_heading: f32,
        control_signal: f32,
        control_error_integral: f32,
        updated_left_power: u8,
        updated_right_power: u8,
    ) -> Self {
        Self {
            timestamp,
            left_encoder,
            right_encoder,
            distance,
            target_wheel_tick_count,
            delta_heading,
            current_heading,
            control_signal,
            control_error_integral,
            updated_left_power,
            updated_right_power,
        }
    }

    pub fn timestamp(&self) -> u32 {
        self.timestamp
    }

    pub fn control_signal(&self) -> f32 {
        self.control_signal
    }
}

impl uDebug for ForwardMovementTelemetryRow {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(
            f,
            "ForwardMovementTelemetryRow<timestamp: {}, left_encoder: {}, right_encoder: {}, distance: {}>",
            self.timestamp,
            self.left_encoder,
            self.right_encoder,
            self.distance,
        )?;

        Ok(())
    }
}

impl uDisplay for ForwardMovementTelemetryRow {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(
            f,
            "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
            self.timestamp,
            self.left_encoder,
            self.right_encoder,
            self.distance,
            self.target_wheel_tick_count,
            self.delta_heading,
            self.current_heading,
            self.control_signal,
            self.control_error_integral,
            self.updated_left_power,
            self.updated_right_power,
        )?;

        Ok(())
    }
}
