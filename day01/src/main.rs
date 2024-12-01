use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let reader = io::BufReader::new(File::open("input")?);

    let (mut left, mut right): (Vec<usize>, Vec<usize>) = reader
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
        .unzip();

    left.sort();
    right.sort();

    let part_a_result = part_a(left.clone(), right.clone());
    println!("Part A: {}", part_a_result);

    let part_b_result = part_b(left, right);
    println!("Part B: {}", part_b_result);

    Ok(())
}

// Lists are sorted here, so we just take the difference between pairwise elements and sum them up.
fn part_a(left: Vec<usize>, right: Vec<usize>) -> usize {
    left.iter()
        .zip(right.iter())
        .map(|(&a, &b)| usize::max(a, b) - usize::min(a, b))
        .sum::<usize>()
}

// Not the fastest solution, but it works on what we already have. If there would have been a billion entires we should
// have used a lookup structure for the right side and precount the values.
fn part_b(left: Vec<usize>, right: Vec<usize>) -> usize {
    left.iter()
        .map(|a| {
            let count = right.iter().filter(|&b| a == b).count();
            a * count
        })
        .sum::<usize>()
}

// Extract the two (unsigned) numbers from the line.
fn parse_line(line: &str) -> Option<(usize, usize)> {
    let mut parts = line.split_whitespace();
    let a = parts.next()?.parse().ok()?;
    let b = parts.next()?.parse().ok()?;
    Some((a, b))
}
