extern crate gollib;

use std::thread::sleep_ms;
use std::io::prelude::*;
use std::io;

use gollib::Field;

fn main() {
    let mut field = Field::from("......X.\nXX......\n.X...XXX");

    for _ in 0..130 {
        field = field.step();
        print!("\x1b[2J\x1b[1;1H");
        print!("{}\n", field);
        io::stdout().flush().ok().expect("Could not flush stdout");
        sleep_ms(100);
    }
}
