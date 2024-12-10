use itertools::Itertools;
use std::fs::read_to_string;

const TRAILHEAD: char = '0';
// A little lookup that has the convenience that the next number is at the index of the current number.
const ELEVATIONS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    row: usize,
    col: usize,
}

struct Grid {
    grid: Vec<Vec<char>>,
    row_size: usize,
    col_size: usize,
    padding: usize,
}

impl Grid {
    fn get_possible_trailheads(&self) -> Vec<Coordinate> {
        let mut coordinates = Vec::new();
        for row in self.padding..self.row_size - self.padding {
            for col in self.padding..self.col_size - self.padding {
                if self.grid[row][col] == TRAILHEAD {
                    coordinates.push(Coordinate { row, col });
                }
            }
        }
        coordinates
    }

    fn get_possible_next(&self, coordinate: Coordinate) -> Vec<Coordinate> {
        let elevation = self.grid[coordinate.row][coordinate.col];
        let next_elevation = ELEVATIONS[elevation.to_digit(10).unwrap() as usize];

        let mut coordinates = Vec::new();
        if self.grid[coordinate.row - 1][coordinate.col] == next_elevation {
            coordinates.push(Coordinate {
                row: coordinate.row - 1,
                col: coordinate.col,
            });
        }
        if self.grid[coordinate.row + 1][coordinate.col] == next_elevation {
            coordinates.push(Coordinate {
                row: coordinate.row + 1,
                col: coordinate.col,
            });
        }
        if self.grid[coordinate.row][coordinate.col - 1] == next_elevation {
            coordinates.push(Coordinate {
                row: coordinate.row,
                col: coordinate.col - 1,
            });
        }
        if self.grid[coordinate.row][coordinate.col + 1] == next_elevation {
            coordinates.push(Coordinate {
                row: coordinate.row,
                col: coordinate.col + 1,
            });
        }
        coordinates
    }

    // We simply find all possible next steps eight times in a row. Anything remaining is a reachable 9.
    fn walk_trailhead(&self, trailhead: Coordinate) -> Vec<Coordinate> {
        let mut trails = vec![trailhead];

        for _ in 0..9 {
            trails = trails
                .into_iter()
                .flat_map(|coordinate| self.get_possible_next(coordinate))
                .collect();
        }

        trails
    }
}

// Three times a charm, the padded grid is the best thing since sliced bread
impl From<String> for Grid {
    fn from(input: String) -> Self {
        let padding = 1;

        let mut grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| {
                let mut padded_line: Vec<char> = vec!['.'; padding];
                padded_line.extend(line.chars());
                padded_line.extend(vec!['.'; padding]);
                padded_line
            })
            .collect();
        let col_size = grid[0].len();

        let padding_row = vec!['.'; col_size];
        for _ in 0..padding {
            grid.insert(0, padding_row.clone());
            grid.push(padding_row.clone());
        }

        let row_size = grid.len();

        Self {
            grid,
            row_size,
            col_size,
            padding,
        }
    }
}

fn main() -> std::io::Result<()> {
    let grid = Grid::from(read_to_string("input")?);
    let trailheads = grid.get_possible_trailheads();

    // Part A is more work than part B! We need to find the uniques of all the trailheads.
    let part_a: usize = trailheads
        .iter()
        .map(|&trailhead| {
            grid.walk_trailhead(trailhead)
                .iter()
                .unique()
                .collect::<Vec<&Coordinate>>()
                .len()
        })
        .sum();

    println!("Part A: {}", part_a);

    let part_b: usize = trailheads
        .iter()
        .map(|&trailhead| grid.walk_trailhead(trailhead).len())
        .sum();

    println!("Part B: {}", part_b);

    Ok(())
}
