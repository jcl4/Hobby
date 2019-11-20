// Frame Timer
// Needed Functionality
// - prints to command line number of frames and average frame rate
// - interval for printing is configurable - defined in ApplicationSettings
// - prints number of frames and average frame rate to command line at end of program

use std::time::{Duration, Instant};

pub(crate) struct FrameTimer {
    display_interval: Duration,
    timer_start: Instant,
    total_frame_count: u32,
    interval_start: Instant,
    interval_frame_count: u32,
}

impl FrameTimer {
    pub(crate) fn new(display_interval: Duration) -> Self {
        FrameTimer {
            display_interval,
            timer_start: Instant::now(),
            total_frame_count: 0,
            interval_start: Instant::now(),
            interval_frame_count: 0,
        }
    }

    pub(crate) fn tic(&mut self) {
        self.total_frame_count += 1;
        self.interval_frame_count += 1;
        let time_now = Instant::now();

        let interval_time = time_now.duration_since(self.interval_start);
        if interval_time >= self.display_interval {
            print_output(self.interval_frame_count, interval_time, "Interval");
            self.interval_frame_count = 0;
            self.interval_start = time_now;
        }
    }
}

impl Drop for FrameTimer {
    fn drop(&mut self) {
        print_output(
            self.total_frame_count,
            Instant::now().duration_since(self.timer_start),
            "Total",
        );
    }
}

fn print_output(num_frames: u32, inteval_time: Duration, prelude: &str) {
    let interval = inteval_time.as_secs_f32();
    println!("{}", prelude);
    println!("\tTime: {:?} sec", interval);
    println!("\tNum Frames: {:?}", num_frames);
    println!(
        "\tAverage Frame Time {:?} sec/frame",
        interval / num_frames as f32
    );
}
