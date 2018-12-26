use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Duration, Instant};

use crate::tools::DurationHelper;
use crate::Result;

pub struct FrameTimer {
    num_frames: u32,
    game_start: Instant,
    frame_start: Instant,
    update_duration: Duration,
    last_update: Instant,

    frame_time: f32,
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
            frame_time: 0.0,
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

    pub fn frame_time(&self) -> f32 {
        self.frame_time
    }

    pub fn kick(&mut self) -> bool {
        let mut update_debug = false;
        self.num_frames += 1;
        let now = Instant::now();

        self.frame_time = now.duration_since(self.frame_start).as_ms();
        // println!(
        //     "Frame: {}, Frame Time: {:.2} ms",
        //     self.num_frames, frame_time
        // );

        if self.min_frame_time == 0.0 {
            self.min_frame_time = self.frame_time;
        } else {
            self.min_frame_time = self.min_frame_time.min(self.frame_time);
        }

        if self.num_frames >= 10 {
            // ignore first few frames for reporting max frame time
            self.max_frame_time = self.max_frame_time.max(self.frame_time);
        }

        if now.duration_since(self.last_update) >= self.update_duration {
            self.average_frame_time =
                now.duration_since(self.game_start).as_ms() / self.num_frames as f32;
            println!("Num Frames: {}", self.num_frames);
            println!("Average Frame Time: {} ms", self.average_frame_time);
            println!(
                "Elapsed Time: {} s",
                now.duration_since(self.game_start).dur_as_f32()
            );
            self.last_update = Instant::now();
            update_debug = true;
        }

        self.frame_start = Instant::now();
        update_debug
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
            "{}, {}, {:.2}, {:.2}, {:.2}, {},\n",
            dt_str,
            self.num_frames,
            self.min_frame_time,
            self.average_frame_time,
            self.max_frame_time,
            self.app_name
        )?;

        Ok(())
    }
}
