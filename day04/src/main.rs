use std::fs::read_to_string;

struct Grid {
    grid: Vec<Vec<char>>,
    row_size: usize,
    col_size: usize,
}

impl Grid {
    // This little helper can check for MAS in any direction by iterating over the pattern and grid simultaneously,
    // using the pattern index to calculate the next position in the specified direction.
    fn has_mas_in_direction(
        &self,
        row: usize,
        col: usize,
        row_direction: isize,
        col_direction: isize,
    ) -> bool {
        let pattern = ['M', 'A', 'S'];
        pattern.iter().enumerate().all(|(i, &c)| {
            let row = row as isize + row_direction * (i + 1) as isize;
            let col = col as isize + col_direction * (i + 1) as isize;
            self.grid[row as usize][col as usize] == c
        })
    }

    // If the current position is an X, we count in all directions if it is the start of an XMAS pattern.
    fn count_xmas_at(&self, row: usize, col: usize) -> usize {
        if self.grid[row][col] != 'X' {
            return 0;
        }

        vec![
            self.has_mas_in_direction(row, col, 0, 1),
            self.has_mas_in_direction(row, col, 0, -1),
            self.has_mas_in_direction(row, col, 1, 0),
            self.has_mas_in_direction(row, col, -1, 0),
            self.has_mas_in_direction(row, col, 1, 1),
            self.has_mas_in_direction(row, col, 1, -1),
            self.has_mas_in_direction(row, col, -1, 1),
            self.has_mas_in_direction(row, col, -1, -1),
        ]
        .into_iter()
        .filter(|&x| x == true)
        .count()
    }

    // The x-mas pattern is a lot easier to find than the xmas pattern. We can just check the diagonals and there are
    // only four variations.
    fn count_x_mas_at(&self, row: usize, col: usize) -> usize {
        if self.grid[row][col] != 'A' {
            return 0;
        }

        let has_principal_mas = false
            || self.grid[row - 1][col - 1] == 'M' && self.grid[row + 1][col + 1] == 'S'
            || self.grid[row - 1][col - 1] == 'S' && self.grid[row + 1][col + 1] == 'M';

        let has_secondary_mas = false
            || self.grid[row - 1][col + 1] == 'M' && self.grid[row + 1][col - 1] == 'S'
            || self.grid[row - 1][col + 1] == 'S' && self.grid[row + 1][col - 1] == 'M';

        if has_principal_mas && has_secondary_mas {
            1
        } else {
            0
        }
    }

    // Just because I wanted to show how nice rust is with generics and closures. This method loops over the grid
    // without the padding and applies the closure to each position.
    fn count<F>(&self, f: F) -> usize
    where
        F: Fn(usize, usize) -> usize,
    {
        let mut count = 0;
        for row in 3..self.row_size - 3 {
            for col in 3..self.col_size - 3 {
                count += f(row, col);
            }
        }
        count
    }

    // Now the counting is straightforward. We just loop over the grid and apply the closure to each position.
    pub fn count_xmas(&self) -> usize {
        self.count(|row, col| self.count_xmas_at(row, col))
    }

    pub fn count_x_mas(&self) -> usize {
        self.count(|row, col| self.count_x_mas_at(row, col))
    }
}

// Convert the input into a grid with padding.
impl From<String> for Grid {
    fn from(input: String) -> Self {
        let padding = 3;

        // Create rows with padding
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

        // Add the padding rows
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
        }
    }
}

fn main() -> std::io::Result<()> {
    let input = read_to_string("input")?;
    let grid = Grid::from(input);

    println!("Part A: {}", grid.count_xmas());
    println!("Part B: {}", grid.count_x_mas());
    Ok(())
}
