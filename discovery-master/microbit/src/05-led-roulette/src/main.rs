#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::{board::Board, pac::TIMER0};
use microbit::hal::timer::Timer;
use microbit::hal::prelude::*;

// Current section: https://docs.rust-embedded.org/discovery/microbit/05-led-roulette/it-blinks.html#blinking:~:text=from%20your%20MCU.-,Blinking,-Now%20we%27ve%20arrived


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
