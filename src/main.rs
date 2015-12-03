extern crate gollib;

use gollib::Cell;
use gollib::Neighbors;
use gollib::get_neighbors;

fn main() {
	let cell = Cell {x:0, y:1};
	let mut nb: Neighbors = [Cell { x: 10, y: 10 }; 8];

	get_neighbors(&cell, &mut nb);

	for element in nb.iter() {
        println!("{:?}", element);
    }
}
