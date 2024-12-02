use std::{
    fs::File,
    io::{self, BufRead},
};

struct Level {
    values: Vec<usize>,
}

impl Level {
    // The window iterator is awesome, it does exactly what we need, sliding over n=2 values at a time with overlap.
    fn is_increasing(&self) -> bool {
        self.values.windows(2).all(|w| w[0] < w[1])
    }

    fn is_decreasing(&self) -> bool {
        self.values.windows(2).all(|w| w[0] > w[1])
    }

    // Found out about abs_diff from yesterday's solution by Fred-Jan \o/
    fn has_max_increment(&self, n: usize) -> bool {
        self.values.windows(2).all(|w| w[0].abs_diff(w[1]) <= n)
    }

    pub fn is_safe(&self) -> bool {
        self.has_max_increment(3) && (self.is_increasing() || self.is_decreasing())
    }

    // Day 2 and we're getting to them lifetimes. Because we're iterating over self.values, we have to make sure that
    // the returned stuff doesn't outlive the original Level or the iterator would break on the next iteration. The
    // annotation says as much as: the iterator (and all its iterations) will live as long as the Level input (self).
    fn generate_sublevels<'a>(&'a self) -> impl Iterator<Item = Level> + 'a {
        (0..self.values.len()).map(move |i| {
            let mut sublevel = self.values.clone();
            sublevel.remove(i);
            Level { values: sublevel }
        })
    }

    pub fn has_safe_sublevel(&self) -> bool {
        self.generate_sublevels().any(|sublevel| sublevel.is_safe())
    }
}

fn main() -> io::Result<()> {
    let reader = io::BufReader::new(File::open("input")?);

    let levels: Vec<Level> = reader
        .lines()
        .filter_map(|line| match line {
            Ok(line) => parse_line(&line).or_else(|| {
                eprintln!("Failed to parse line: {}", line);
                None
            }),
            // Reading a line can also err, if we suddenly lose access to the file or something.
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
                None
            }
        })
        .collect();

    let part_a = levels.iter().filter(|level| level.is_safe()).count();
    println!("Part A: {}", part_a);

    let part_b = levels
        .iter()
        .filter(|level| level.is_safe() || level.has_safe_sublevel())
        .count();

    println!("Part B: {}", part_b);

    Ok(())
}

fn parse_line(line: &str) -> Option<Level> {
    let values: Vec<usize> = line
        .split_whitespace()
        .filter_map(|part| part.parse().ok())
        .collect();

    Some(Level { values })
}
