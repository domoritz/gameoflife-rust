extern crate gollib;

use std::io;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

use gollib::Field;

fn main() {
    let mut field = Field::from("......X.\nXX......\n.X...XXX");

    for _ in 0..130 {
        field = field.step();
        print!("\x1b[2J\x1b[1;1H");
        println!("{}", field);
        io::stdout().flush().expect("Could not flush stdout");
        sleep(Duration::from_millis(100));
    }
}
