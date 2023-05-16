#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{display, display_pins};
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

    board.display_pins.col1.set_low().unwrap();
    let mut row1 = board.display_pins.row1;

    board.display_pins.col3.set_low().unwrap();
    let mut row3 = board.display_pins.row3;

    loop {
        row1.set_low().unwrap();
        
        rprintln!("
        DARK!");
        timer.delay_ms(3000u16);

        row1.set_high().unwrap();
        rprintln!("LIGHT!");
        timer.delay_ms(1000u16);


        row3.set_high().unwrap();
        timer.delay_ms(2000u16);

        row3.set_low().unwrap();
        timer.delay_ms(1000u16);
    }
}
