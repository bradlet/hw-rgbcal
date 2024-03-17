#![no_std]
#![no_main]

mod knob;
mod rgb;
mod ui;
pub use knob::*;
pub use rgb::*;
pub use ui::*;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use embassy_executor::Spawner;
use embassy_futures::join;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_time::Timer;
use microbit_bsp::{
    embassy_nrf::{
        bind_interrupts,
        gpio::{AnyPin, Level, Output, OutputDrive},
        saadc,
    },
    Button, Microbit,
};
use num_traits::float::FloatCore;

const INITIAL_FRAME_RATE: u64 = 50;
pub const LEVELS: u32 = 16; // count of steps for the various levels in our program

pub static RGB_LEVELS: Mutex<ThreadModeRawMutex, [u32; 3]> = Mutex::new([0; 3]);
pub static FRAME_RATE: Mutex<ThreadModeRawMutex, u64> = Mutex::new(INITIAL_FRAME_RATE);

/// Perform a thread-safe read of our 3 RGB levels held in `RGB_LEVELS`
async fn get_rgb_levels() -> [u32; 3] {
    let rgb_levels = RGB_LEVELS.lock().await;
    *rgb_levels
}

/// Apply `setter` to our `RGB_VALUES` in a thread-safe way -- intended to set those values.
async fn set_rgb_levels<F>(setter: F)
where
    F: FnOnce(&mut [u32; 3]),
{
    let mut rgb_levels = RGB_LEVELS.lock().await;
    setter(&mut rgb_levels);
}

// Note: Essentially same behavior for these two, but used to update global static FRAME_RATE.

async fn get_frame_rate() -> u64 {
    *(FRAME_RATE.lock().await)
}

async fn set_frame_rate<F>(setter: F)
where
    F: FnOnce(&mut u64),
{
    let mut fr = FRAME_RATE.lock().await;
    setter(&mut fr)
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    rtt_init_print!();
    let board = Microbit::default();

    // Setup SAADC to receive interrupts measuring change in analog input value (from our knob)
    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });

    // Setup closure used to get consistent `AnyPin` outputs for our rgb led (all three channels) and build those pins.
    let led_pin = |p| Output::new(p, Level::Low, OutputDrive::Standard);
    let red = led_pin(AnyPin::from(board.p9));
    let green = led_pin(AnyPin::from(board.p8));
    let blue = led_pin(AnyPin::from(board.p16));
    let rgb: Rgb = Rgb::new([red, green, blue], INITIAL_FRAME_RATE);

    // Finish SAADC configuration
    let mut saadc_config = saadc::Config::default();
    saadc_config.resolution = saadc::Resolution::_14BIT; // Step size for analog input sensitivity
    let saadc = saadc::Saadc::new(
        board.saadc,
        Irqs,
        saadc_config,
        [saadc::ChannelConfig::single_ended(board.p2)],
    );
    let knob = Knob::new(saadc).await;
    let mut ui = Ui::new(knob, board.btn_a, board.btn_b);

    // Each component runs its own async main loop -- wait here to catch if both fall off that loop
    join::join(rgb.run(), ui.run()).await;

    panic!("fell off end of main loop");
}
