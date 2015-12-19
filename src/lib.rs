use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cell {
    pub x: i64,
    pub y: i64,
}

pub struct Field(HashSet<Cell>);
pub type Counts = HashMap<Cell, u64>;
pub type Neighbors = [Cell; 8];


pub fn get_neighbors(cell: Cell, neighbors: &mut Neighbors) {
    let mut i = 0;
    for x in cell.x - 1..cell.x + 2 {
        for y in cell.y - 1..cell.y + 2 {
            if (x, y) != (cell.x, cell.y) {
                neighbors[i] = Cell { x: x, y: y };
                i = i + 1;
            }
        }
    }
}

impl Field {
    fn empty() -> Field {
        return Field(HashSet::new());
    }

    fn new(desc: &str) -> Field {
        let mut field = Field(HashSet::new());

        for (x, line) in desc.split('\n').enumerate() {
            for (y, elem) in line.chars().enumerate() {
                if elem == 'X' {
                    field.0.insert(Cell {
                        x: x as i64,
                        y: y as i64,
                    });
                }
            }
        }

        return field;
    }

    fn add(&mut self, cell: Cell) {
        self.0.insert(cell);
    }

    fn neighbor_counts(self) -> Counts {
        let mut counts: Counts = HashMap::new();

        let mut neighbors: Neighbors = [Cell { x: 10, y: 10 }; 8];
        for cell in self.0 {
            get_neighbors(cell, &mut neighbors);
            for neighbor in &neighbors {
                if counts.contains_key(neighbor) {
                    if let Some(x) = counts.get_mut(neighbor) {
                        *x += 1;
                    }
                } else {
                    counts.insert(*neighbor, 1);
                }
            }
        }

        return counts;
    }

    fn step(&mut self) {

    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::get_neighbors;
    use super::Cell;
    use super::Counts;
    use super::Neighbors;
    use super::Field;

    #[test]
    fn generate_board() {
        let description = "X.\n.X";
        let mut expected = Field::empty();
        expected.add(Cell { x: 0, y: 0 });
        expected.add(Cell { x: 1, y: 1 });

        let actual = Field::new(description);
        assert_eq!(expected.0, actual.0);
    }

    #[test]
    fn neighbors_works() {
        let cell = Cell { x: 0, y: 1 };
        let mut actual: Neighbors = [Cell { x: 10, y: 10 }; 8];
        let expected: Neighbors = [Cell { x: -1, y: 0 },
                                   Cell { x: -1, y: 1 },
                                   Cell { x: -1, y: 2 },
                                   Cell { x: 0, y: 0 },
                                   Cell { x: 0, y: 2 },
                                   Cell { x: 1, y: 0 },
                                   Cell { x: 1, y: 1 },
                                   Cell { x: 1, y: 2 }];
        get_neighbors(cell, &mut actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn neighbor_counts_work() {
        let mut field = Field::empty();
        field.add(Cell { x: 0, y: 0 });
        field.add(Cell { x: 1, y: 1 });

        let actual = field.neighbor_counts();

        let mut expected: Counts = HashMap::new();

        expected.insert(Cell { x: 2, y: 2 }, 1);
        expected.insert(Cell { x: 2, y: 0 }, 1);
        expected.insert(Cell { x: 0, y: -1 }, 1);
        expected.insert(Cell { x: 1, y: 0 }, 2);
        expected.insert(Cell { x: 0, y: 1 }, 2);
        expected.insert(Cell { x: 0, y: 0 }, 1);
        expected.insert(Cell { x: 1, y: 2 }, 1);
        expected.insert(Cell { x: 1, y: -1 }, 1);
        expected.insert(Cell { x: 2, y: 1 }, 1);
        expected.insert(Cell { x: 1, y: 1 }, 1);
        expected.insert(Cell { x: -1, y: 0 }, 1);
        expected.insert(Cell { x: 0, y: 2 }, 1);
        expected.insert(Cell { x: -1, y: 1 }, 1);
        expected.insert(Cell { x: -1, y: -1 }, 1);

        assert_eq!(14, actual.len());
        assert_eq!(expected, actual);
    }
}
