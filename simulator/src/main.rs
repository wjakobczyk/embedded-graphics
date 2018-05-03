extern crate embedded_graphics;
extern crate simulator;

use std::time::Duration;
use std::thread;

use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line};
use embedded_graphics::fonts::Font6x8;

use simulator::Display;

fn main() {
    let mut display = Display::new();

    // Outline
    display.draw(Circle::new((64, 64), 63, 1).into_iter());

    // Clock hands
    display.draw(Line::new((64, 64), (0, 64), 1).into_iter());
    display.draw(Line::new((64, 64), (80, 80), 1).into_iter());

    display.draw(Font6x8::render_str("Hello World!").translate((5, 50)).into_iter());

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}