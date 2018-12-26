use std::time::Duration;

pub trait DurationHelper {
    fn dur_as_f32(&self) -> f32;
    fn as_ms(&self) -> f32;
}

impl DurationHelper for Duration {
    fn dur_as_f32(&self) -> f32 {
        let secs = self.as_secs();
        let nanos = self.subsec_nanos();
        let nanos_frac = nanos as f32 / 1_000_000_000.0;

        secs as f32 + nanos_frac
    }

    fn as_ms(&self) -> f32 {
        let time = self.dur_as_f32();
        time * 1000.0
    }
}
