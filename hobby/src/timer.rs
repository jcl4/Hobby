use std::time::{Duration, Instant};

pub struct FrameTimer {
    start: Instant,
    num_frames: u32,
    last_frame_time: Instant,

    max_frame_time: Duration,
    min_frame_time: Duration,

    last_disp_time: Instant,
    disp_interval: Duration,
    num_frames_disp_interval: u32,
}

impl FrameTimer {
    pub fn new(disp_interval: Duration) -> FrameTimer {
        FrameTimer {
            start: Instant::now(),
            num_frames: 0,
            last_frame_time: Instant::now(),
            max_frame_time: Duration::from_secs(0),
            min_frame_time: Duration::from_secs(u64::MAX),
            last_disp_time: Instant::now(),
            disp_interval,
            num_frames_disp_interval: 0,
        }
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
        self.num_frames = 0;
        self.last_frame_time = Instant::now();
        self.max_frame_time = Duration::from_secs(0);
        self.min_frame_time = Duration::from_secs(u64::MAX);
        self.last_disp_time = Instant::now();
        self.num_frames_disp_interval = 0;
    }

    pub fn tic(&mut self) -> Duration {
        let now = Instant::now();
        self.num_frames += 1;
        self.num_frames_disp_interval += 1;

        let disp_interval = now - self.last_disp_time;
        let frame_time = now - self.last_frame_time;
        if frame_time > self.max_frame_time {
            self.max_frame_time = frame_time;
        } else if frame_time < self.min_frame_time {
            self.min_frame_time = frame_time;
        }

        if disp_interval > self.disp_interval {
            let dt = duration_as_fractional_sec(&disp_interval);
            println!("Dislay interval time: {:#?} s", disp_interval);
            println!("Num Frames {}", self.num_frames_disp_interval);
            println!("Last Frame Time: {:#?}", frame_time);
            println!("Avg Frame Time: {}", self.num_frames as f64 / dt);
            println!(
                "Max Frame Time: {} s, Min Frame Time: {} s",
                duration_as_fractional_sec(&self.max_frame_time),
                duration_as_fractional_sec(&self.min_frame_time)
            )
        }

        self.last_frame_time = Instant::now();

        frame_time
    }

    pub fn log_current_stats(&mut self) {
        let now = Instant::now();
        let total_run_time = now - self.start;
        let fps = self.num_frames as f64 / duration_as_fractional_sec(&total_run_time);

        log::info!(
            "Total Run Time: {} s",
            duration_as_fractional_sec(&total_run_time)
        );
        log::info!("Number of Frames: {} s", self.num_frames);
        log::info!("Average Frames Per Second: {} FPS", fps);
        log::info!("Average Frame Time: {} s", 1.0 / fps);
        log::info!(
            "Max Frame Time: {} s, Min Frame Time: {}s",
            duration_as_fractional_sec(&self.max_frame_time),
            duration_as_fractional_sec(&self.min_frame_time)
        );
    }
}

fn duration_as_fractional_sec(dur: &Duration) -> f64 {
    dur.as_secs() as f64 + dur.subsec_nanos() as f64 / 1_000_000_000.0
}

pub struct PhysicsTimer {}
