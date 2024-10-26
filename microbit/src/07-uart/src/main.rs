#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use heapless::Vec;
use nb::block;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[cfg(feature = "v1")]
use microbit::{
    hal::prelude::*,
    hal::uart,
    hal::uart::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

const ENTER_KEY_ASCII_CODE: u8 = 13;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v1")]
    let mut serial = {
        uart::Uart::new(
            board.UART0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        )
    };

    #[cfg(feature = "v2")]
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        let byte: u8 = nb::block!(serial.read()).unwrap();

        if byte == ENTER_KEY_ASCII_CODE {
            for b in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                nb::block!(serial.write(*b)).unwrap();
            }

            nb::block!(serial.flush()).unwrap();
            buffer.clear();
        } else if buffer.push(byte).is_err() {
            write!(serial, "Error: the buffer is full and will be reset!\r\n").unwrap();
            nb::block!(serial.flush()).unwrap();
            buffer.clear();
        }
    }
}
