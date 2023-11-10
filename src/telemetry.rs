use ufmt::{uDebug, uDisplay, uWrite, uwrite, Formatter};

pub const FORWARD_TELEMETRY_COLUMN_COUNT: usize = 5;
pub static FORWARD_MOVEMENT_TELEMETRY_HEADERS: [&str; FORWARD_TELEMETRY_COLUMN_COUNT] = [
    "millis", "Left Wheel Counter", "Right Wheel Counter", "Distance", "Target Wheel Tick Count"
];

#[derive(Copy, Clone, Default)]
pub struct ForwardMovementTelemetryRow {
    timestamp: u32,
    left_encoder: u32,
    right_encoder: u32,
    distance: f32,
    target_wheel_tick_count: u32,
}

impl ForwardMovementTelemetryRow {
    pub fn new(timestamp: u32, left_encoder: u32, right_encoder: u32, distance: f32, target_wheel_tick_count: u32) -> Self {
        Self {
            timestamp,
            left_encoder,
            right_encoder,
            distance,
            target_wheel_tick_count,
        }
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
            "{}, {}, {}, {}, {}",
            self.timestamp,
            self.left_encoder,
            self.right_encoder,
            self.distance,
            self.target_wheel_tick_count,
        )?;

        Ok(())
    }
}
