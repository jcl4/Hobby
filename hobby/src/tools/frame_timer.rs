use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Duration, Instant};

use crate::Result;

pub struct FrameTimer {
    num_frames: u32,
    game_start: Instant,
    frame_start: Instant,
    update_duration: Duration,
    last_update: Instant,

    min_frame_time: f32,
    max_frame_time: f32,
    average_frame_time: f32,

    app_name: String,
}

impl FrameTimer {
    pub fn new(update_duration: Duration, app_name: &str) -> FrameTimer {
        FrameTimer {
            num_frames: 0,
            game_start: Instant::now(),
            frame_start: Instant::now(),
            update_duration,
            last_update: Instant::now(),
            min_frame_time: 0.0,
            max_frame_time: 0.0,
            average_frame_time: 0.0,
            app_name: app_name.to_string(),
        }
    }

    pub fn start(&mut self) {
        self.game_start = Instant::now();
        self.frame_start = Instant::now();
        self.last_update = Instant::now();
    }

    pub fn kick(&mut self) {
        self.num_frames += 1;
        let now = Instant::now();

        let frame_time = as_ms(now.duration_since(self.frame_start));

        if self.min_frame_time == 0.0 {
            self.min_frame_time = frame_time;
        } else {
            self.min_frame_time = self.min_frame_time.min(frame_time);
        }

        self.max_frame_time = self.max_frame_time.max(frame_time);

        if now.duration_since(self.last_update) >= self.update_duration {
            self.average_frame_time =
                as_ms(now.duration_since(self.game_start)) / self.num_frames as f32;
            println!("Num Frames: {}", self.num_frames);
            println!("Average Frame Time: {} ms", self.average_frame_time);
            self.last_update = Instant::now();
        }

        self.frame_start = Instant::now();
    }

    pub fn stop(&self) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("./logs/frame_time.csv")?;

        let dt = Local::now();
        let dt_str = dt.format("%Y-%m-%d %H:%M:%S").to_string();

        write!(
            file,
            "{}, {:.2}, {:.2}, {:.2}, {},\n",
            dt_str,
            self.min_frame_time,
            self.average_frame_time,
            self.max_frame_time,
            self.app_name
        )?;

        Ok(())
    }
}

fn dur_as_f32(duration: Duration) -> f32 {
    let secs = duration.as_secs();
    let nanos = duration.subsec_nanos();
    let nanos_frac = nanos as f32 / 1_000_000_000.0;

    secs as f32 + nanos_frac
}

fn as_ms(duration: Duration) -> f32 {
    let time = dur_as_f32(duration);
    time * 1000.0
}
