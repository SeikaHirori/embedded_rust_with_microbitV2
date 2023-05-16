#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::{board::Board, pac::TIMER0};
use microbit::hal::timer::Timer;
use microbit::hal::prelude::*;
use rtt_target::{rtt_init_print, rprintln};

// Current section:     // https://docs.rust-embedded.org/discovery/microbit/05-led-roulette/it-blinks.html#:~:text=Inside%20our%20MCU%2C%20several%20so%2Dcalled%20%22timers%22%20exist.%20They%20can%20do%20various%20things%20regarding%20time%20for%20us%2C%20including%20simply%20pausing%20the%20execution%20of%20our%20program%20for%20a%20fixed%20amount%20of%20time.%20A%20very%20simple%20delay%2Dbased%20program%20that%20prints%20something%20every%20second%20might%20for%20example%20look%20like%20this%3A


#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board: Board = Board::take().unwrap();

    let mut timer: Timer<TIMER0> = Timer::new(board.TIMER0);

    loop {
        timer.delay_ms(1000u16);
        rprintln!("1000 ms passed!")
    }
}
