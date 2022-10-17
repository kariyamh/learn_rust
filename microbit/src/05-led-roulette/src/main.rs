#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut led_matrix = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let i = 0;
    loop {
        let (x, y) = get_index_of_lit_led(i);
        led_matrix[x][y] = 1; 
        display.show(&mut timer, led_matrix, 500);
        display.clear();
        timer.delay_ms(500_u32);
    }
}

// returns the indicies of the next light-up
fn get_index_of_lit_led(index: u8) -> (usize, usize) {
    match index {
        0 => (0, 0),
        1 => (1, 0),
        2 => (2, 0),
        3 => (3, 0),
        4 => (4, 0),
        5 => (4, 1),
        6 => (4, 2),
        7 => (4, 3),
        8 => (4, 4),
        9 => (3, 4),
        10 => (2, 4),
        11 => (1, 4),
        12 => (0, 4),
        13 => (0, 3),
        14 => (0, 2),
        15 => (0, 1),
        _ => (1, 1)
    }
}