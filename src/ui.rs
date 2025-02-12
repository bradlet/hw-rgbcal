use num_traits::ToPrimitive;

use crate::*;

struct UiState {
    levels: [u32; 3],
    frame_rate: u64,
}

impl UiState {
    /// Printout logs in consol updating current calibration values for the
    /// benefit of the calibrator.
    fn show(&self) {
        let names = ["red", "green", "blue"];
        rprintln!();
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        rprintln!("frame rate: {}", self.frame_rate);
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1],
            frame_rate: 100,
        }
    }
}

/// `Ui` holds the source of truth `UiState` for our program.
/// Downstream updates occur to our `RGB_LEVELS` in order to
/// stay in sync with this source of truth.
pub struct Ui {
    knob: Knob,
    _button_a: Button,
    _button_b: Button,
    state: UiState,
}

impl Ui {
    pub fn new(knob: Knob, _button_a: Button, _button_b: Button) -> Self {
        Self {
            knob,
            _button_a,
            _button_b,
            state: UiState::default(),
        }
    }

    /// Update `UiState`, according to spec, with a `measurement` assumedly pulled from our
    /// analog input source (knob).
    /// Note that `Knob::measure` scales our input so it changes in steps up to `LEVELS`.
    ///
    /// Returns: a bool indiciating whether a value update occurred.
    fn update(&mut self, measurement: u32) -> bool {
        let btn_a_pressed = self._button_a.is_low();
        let btn_b_pressed = self._button_b.is_low();
        // Update `UiState` based on button state and return the pre-update
        // value so we can report back if an actual change in value occurred.
        let previous_level = match (btn_a_pressed, btn_b_pressed) {
            (false, false) => {
                // FRAME_RATE
                // Assume our frame rate will always stick into u32
                let prev = ((self.state.frame_rate - 10) / 10).to_u32().unwrap();
                // Note: Have to subtract 10 above because of the following + 10 we have
                // 	to do to match the spec...
                // u32 can always fit in u64:
                self.state.frame_rate = (measurement.to_u64().unwrap() * 10) + 10;
                prev
            }
            (true, false) => {
                // BLUE
                let prev = self.state.levels[2];
                self.state.levels[2] = measurement;
                prev
            }
            (false, true) => {
                // GREEN
                let prev = self.state.levels[1];
                self.state.levels[1] = measurement;
                prev
            }
            (true, true) => {
                // RED
                let prev = self.state.levels[0];
                self.state.levels[0] = measurement;
                prev
            }
        };
        // Return true if the measurement is not the same as the old value, indicating
        // that an actual update occurred above.
        previous_level != measurement
    }

    /// Update global statics shared with `Rgb` so that rgb level match
    /// the logged calibration output.
    async fn update_rgb(&self) {
        set_rgb_levels(|rgb| {
            *rgb = self.state.levels;
        })
        .await;
        set_frame_rate(|fr| {
            *fr = self.state.frame_rate;
        })
        .await;
    }

    pub async fn run(&mut self) -> ! {
        // Take initial knob state measurement and update UI
        let level = self.knob.measure().await;
        self.update(level);
        self.update_rgb().await;
        self.state.show();
        loop {
            // At each step: take an analog measurement, check for updates, if the
            // measurement has changed, update in the UI and then set our global
            // static RGB_LEVELS state so that the RGB led updates to match the UI.
            let level = self.knob.measure().await;
            let updated = self.update(level);
            if updated {
                self.state.show();
                self.update_rgb().await;
            }
            Timer::after_millis(50).await;
        }
    }
}
