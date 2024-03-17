use core::mem;

use num_traits::ToPrimitive;

use crate::*;

struct UiState {
    levels: [u32; 3],
    frame_rate: u64,
}

impl UiState {
    fn show(&self) {
        let names = ["red", "green", "blue"];
        rprintln!();
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        rprintln!("frame rate: {}", self.frame_rate);
    }

	fn with_frame_rate(frame_rate: u64) -> Self {
		Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1],
            frame_rate
		}
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

pub struct Ui {
    knob: Knob,
    _button_a: Button,
    _button_b: Button,
    state: UiState,
}

impl Ui {
    pub fn new(knob: Knob, _button_a: Button, _button_b: Button, frame_rate: u64) -> Self {
        Self {
            knob,
            _button_a,
            _button_b,
            state: UiState::with_frame_rate(frame_rate),
        }
    }

	/// Update `UiState`, according to spec, with a `measurement` assumedly pulled from our
	/// analog input source (knob).
	/// Note that `Knob::measure` scales our input so it changes in steps up to `LEVELS`.
	fn update(&mut self, measurement: u32) {
		let btn_a_pressed = self._button_a.is_low();
		let btn_b_pressed = self._button_b.is_low();
		// Update `UiState` based on button state
		match (btn_a_pressed, btn_b_pressed) {
			(false, false) => { // FRAME_RATE
				// u32 can always fit in u64:
				self.state.frame_rate = measurement.to_u64().unwrap() * 10 
			},
			(true, false) => { // BLUE
				self.state.levels[2] = measurement
			},
			(false, true) => { // GREEN
				self.state.levels[1] = measurement
			},
			(true, true) => { // RED
				self.state.levels[0] = measurement
			}
		}

	}

    pub async fn run(&mut self) -> ! {
		// Take initial knob state measurement and update UI
		let level = self.knob.measure().await;
		self.update(level);
        set_rgb_levels(|rgb| {
            *rgb = self.state.levels;
        })
        .await;
        self.state.show();
        loop {
            let level = self.knob.measure().await;
            if level != self.state.levels[2] {
                self.state.levels[2] = level;
                self.state.show();
                set_rgb_levels(|rgb| {
                    *rgb = self.state.levels;
                })
                .await;
            }
            Timer::after_millis(50).await;
        }
    }
}
