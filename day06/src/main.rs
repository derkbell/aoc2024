use std::{
    error::Error,
    fmt::{Display, Formatter},
    fs::read_to_string,
    thread::sleep,
    time::Duration,
};

const GUARD_START: char = '^';
const OBSTACLE: char = '#';
const EMPTY: char = '.';
const VISITED_UP_DOWN: char = '|';
const VISITED_LEFT_RIGHT: char = '-';
const VISITED_UP_DOWN_LEFT_RIGHT: char = '+';
const PADDING: char = '@';

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Guard {
    row: usize,
    col: usize,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<char>>,
    row_size: usize,
    col_size: usize,
    padding: usize,
    guard: Guard,
    steps: usize,
    max_steps: usize,
    exited: bool,
}

impl Grid {
    // Look at whats in front of the guard
    fn look(&self) -> char {
        match self.guard.direction {
            Direction::Up => self.grid[self.guard.row - 1][self.guard.col],
            Direction::Down => self.grid[self.guard.row + 1][self.guard.col],
            Direction::Left => self.grid[self.guard.row][self.guard.col - 1],
            Direction::Right => self.grid[self.guard.row][self.guard.col + 1],
        }
    }

    // Move the guard in the direction it is facing and update the grid
    fn move_guard(&mut self) {
        match self.guard.direction {
            Direction::Up => self.guard.row -= 1,
            Direction::Down => self.guard.row += 1,
            Direction::Left => self.guard.col -= 1,
            Direction::Right => self.guard.col += 1,
        };
        if self.grid[self.guard.row][self.guard.col] == EMPTY {
            if self.guard.direction == Direction::Up || self.guard.direction == Direction::Down {
                self.grid[self.guard.row][self.guard.col] = VISITED_UP_DOWN;
            } else if self.guard.direction == Direction::Left
                || self.guard.direction == Direction::Right
            {
                self.grid[self.guard.row][self.guard.col] = VISITED_LEFT_RIGHT;
            }
        } else {
            self.grid[self.guard.row][self.guard.col] = VISITED_UP_DOWN_LEFT_RIGHT;
        }
    }

    // Rotate the guard 90 degrees clockwise
    fn rotate_guard(&mut self) {
        self.guard.direction = match self.guard.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    // Perform the next step for the guard
    fn step(&mut self) {
        match self.look() {
            VISITED_UP_DOWN | VISITED_LEFT_RIGHT | VISITED_UP_DOWN_LEFT_RIGHT | EMPTY => {
                self.move_guard();
            }
            OBSTACLE => {
                self.rotate_guard();
            }
            PADDING => self.exited = true,
            _ => {
                println!("{:?}", self);
                panic!("a cannae step");
            }
        }
        self.steps += 1;
    }

    fn run(&mut self) -> bool {
        while !self.is_done() {
            self.step();
        }
        self.exited
    }

    fn is_done(&self) -> bool {
        self.exited || self.steps >= self.max_steps
    }

    fn sum_visited(&self) -> usize {
        self.grid
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|&&c| {
                        c == VISITED_UP_DOWN
                            || c == VISITED_LEFT_RIGHT
                            || c == VISITED_UP_DOWN_LEFT_RIGHT
                    })
                    .count()
            })
            .sum()
    }

    // Ok, so this is the even uglier part. We're running a brute force algorithm to find the loops by adding an
    // obstruction to the grid at each point and then checking if the guard loops. Instead of properly checking if the
    // field in front of the guard is a field already visited in the same direction the guard is facing we just run the
    // moving forward stuff for "a while".
    //
    // I can imagine some clever mind can make up a grid that has obstacles in such a way it runs the guard in a spiral
    // outward in and maybe even find a way to make the guard run outwards again but this would never be more than twice
    // the number of grid cells so that's a sane upper "a while".
    //
    // Another thing is that we're not checking all the fields, it only makes sense to test with a field where the guard
    // actually passes in the part A grid. So we just pass the part A grid and take only the visited fields into
    // concideration.
    fn find_loops(&mut self, part_a_grid: Grid) -> usize {
        let mut loops = 0;
        for row in self.padding..self.row_size - self.padding {
            for col in self.padding..self.col_size - self.padding {
                if self.grid[row][col] == EMPTY
                    && (part_a_grid.grid[row][col] == VISITED_UP_DOWN
                        || part_a_grid.grid[row][col] == VISITED_LEFT_RIGHT
                        || part_a_grid.grid[row][col] == VISITED_UP_DOWN_LEFT_RIGHT)
                {
                    let mut grid_with_obstruction = self.clone();
                    grid_with_obstruction.grid[row][col] = OBSTACLE;

                    if grid_with_obstruction.run() == false {
                        loops += 1;
                    }
                }
            }
        }
        loops
    }
}

// The Dispaly trait allows the grid to be printed in a regular {} block.
impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for row in &self.grid {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// We reuse our padded grid from day04 so we can look without bounds check. Yes, I hate doing bounds checks.
impl From<String> for Grid {
    fn from(input: String) -> Self {
        let padding = 1;

        let mut grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| {
                let mut padded_line: Vec<char> = vec![PADDING; padding];
                padded_line.extend(line.chars());
                padded_line.extend(vec![PADDING; padding]);
                padded_line
            })
            .collect();
        let col_size = grid[0].len();

        let padding_row = vec![PADDING; col_size];
        for _ in 0..padding {
            grid.insert(0, padding_row.clone());
            grid.push(padding_row.clone());
        }

        let row_size = grid.len();

        let (row, col) = grid
            .iter()
            .enumerate()
            .find(|(_index, row)| row.contains(&GUARD_START))
            .map(|(index, row)| {
                let col = row.iter().position(|&c| c == GUARD_START).unwrap();
                (index, col)
            })
            .unwrap();

        let guard = Guard {
            row,
            col,
            direction: Direction::Up,
        };
        grid[row][col] = VISITED_UP_DOWN;

        Self {
            grid,
            row_size,
            col_size,
            padding,
            guard,
            exited: false,
            max_steps: row_size * col_size * 2,
            steps: 0,
        }
    }
}

// You can run the program as `cargo run visual` to see the guard move around the grid (currently just the part A grid).
// Note that the debug version (`cargo run`) is way slower than the releaes version (`cargo run --release`).
fn main() -> Result<(), Box<dyn Error>> {
    let mut grid = Grid::from(read_to_string("input")?);

    if std::env::args().nth(1) == Some("visual".to_string()) {
        while !grid.is_done() {
            print!("\x1b[1;1H");
            grid.step();
            println!("{}", grid);
            sleep(Duration::from_millis(10));
        }
    } else {
        let mut part_a_grid = grid.clone();
        part_a_grid.run();

        println!("Part A: {}", part_a_grid.sum_visited());
        println!("Part B: {}", grid.find_loops(part_a_grid));
    }

    Ok(())
}
