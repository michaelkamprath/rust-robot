use ufmt::{uDebug, uDisplay, uWrite, uwrite, Formatter};

#[derive(Debug, Copy, Clone, Default)]
pub struct ForwardMovementTelemetryRow {
    timestamp: u32,
    left_encoder: u32,
    right_encoder: u32,
}

impl ForwardMovementTelemetryRow {
    pub fn new(timestamp: u32, left_encoder: u32, right_encoder: u32) -> Self {
        Self {
            timestamp,
            left_encoder,
            right_encoder,
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
            "ForwardMovementTelemetryRow<timestamp: {}, left_encoder: {}, right_encoder: {}>",
            self.timestamp,
            self.left_encoder,
            self.right_encoder
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
            "{}, {}, {}",
            self.timestamp,
            self.left_encoder,
            self.right_encoder
        )?;

        Ok(())
    }
}
