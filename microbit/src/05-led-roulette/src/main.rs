#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, timer::Timer},
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut i = 0;
    let mut j = 0;

    loop {
        let mut matrix_display = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];

        matrix_display[i][j] = 1;

        // Show light_it_all for 1000 ms
        display.show(&mut timer, matrix_display, 25);

        // Clear the display again
        display.clear();

        if (i == 0) && (j < 4) {
            j += 1;
        } else if (j == 4) && (i < 4) {
            i += 1;
        } else if (i == 4) && (j >= 1) {
            j -= 1;
        } else if (j == 0) && (i >= 1) {
            i -= 1;
        }
    }
}
