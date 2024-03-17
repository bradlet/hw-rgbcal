use crate::*;

type RgbPins = [Output<'static, AnyPin>; 3];

pub struct Rgb {
    rgb: RgbPins,
    // Shadow variables to minimize lock contention.
    levels: [u32; 3],
    tick_time: u64,
}

impl Rgb {
    fn frame_tick_time(frame_rate: u64) -> u64 {
        1_000_000 / (3 * frame_rate * LEVELS as u64)
    }

    pub fn new(rgb: RgbPins, initial_frame_rate: u64) -> Self {
        let tick_time = Self::frame_tick_time(initial_frame_rate);
        Self {
            rgb,
            levels: [0; 3],
            tick_time,
        }
    }

    /// Scan through all rgb pins adjusting wavelength by the
    /// `level` in our current state corresponding to that pin.
    async fn step(&mut self, led: usize) {
        let level = self.levels[led];
        if level > 0 {
            self.rgb[led].set_high();
            let on_time = level as u64 * self.tick_time;
            Timer::after_micros(on_time).await;
            self.rgb[led].set_low();
        }
        let level = LEVELS - level;
        if level > 0 {
            let off_time = level as u64 * self.tick_time;
            Timer::after_micros(off_time).await;
        }
    }

    /// Our main RGB loop: Constantly read current global static
    /// `RGB_LEVELS` and `FRAME_RATE` values and scan through our
    /// rgb pins with those updated values.
    pub async fn run(mut self) -> ! {
        loop {
            self.levels = get_rgb_levels().await;
            self.tick_time = Self::frame_tick_time(get_frame_rate().await);

            for led in 0..3 {
                self.step(led).await;
            }
        }
    }
}
