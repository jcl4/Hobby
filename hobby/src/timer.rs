use std::time::{Duration, Instant};

pub struct FrameTimer {
    start: Instant,
    num_frames: u32,
    last_frame_time: Instant,

    last_disp_time: Instant,
    disp_interval: Duration,
    num_frames_disp_interval: u32,
}

impl FrameTimer {
    pub fn new(disp_interval: f32) -> FrameTimer {
        FrameTimer {
            start: Instant::now(),
            num_frames: 0,
            last_frame_time: Instant::now(),
            last_disp_time: Instant::now(),
            disp_interval: Duration::from_secs_f32(disp_interval),
            num_frames_disp_interval: 0,
        }
    }

    pub fn tic(&mut self) -> Duration {
        let now = Instant::now();
        self.num_frames += 1;
        self.num_frames_disp_interval += 1;

        let disp_interval = now - self.last_disp_time;
        let frame_time = now - self.last_frame_time;

        if disp_interval > self.disp_interval {
            println!("Dislay interval time: {} s", disp_interval.as_secs());
        }

        frame_time
    }
}

pub struct PhysicsTimer {}
