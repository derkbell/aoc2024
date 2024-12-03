use regex::Regex;
use std::{
    fs::{self, File},
    io::{self, BufRead},
};

struct Mul {
    x: usize,
    y: usize,
}

impl Mul {
    fn value(&self) -> usize {
        self.x * self.y
    }
}

fn main() -> io::Result<()> {
    let reader = io::BufReader::new(File::open("input")?);

    let muls: Vec<Mul> = reader
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
        .flatten()
        .collect();

    let part_a = muls.iter().map(|mul| mul.value()).sum::<usize>();
    println!("Part A: {}", part_a);

    let input = fs::read_to_string("input")?;

    let part_b = parse_input(&input)
        .iter()
        .flatten()
        .map(|mul| mul.value())
        .sum::<usize>();

    println!("Part B: {}", part_b);

    Ok(())
}

// Oh yeah how little did we know here when we decided a regex was better than building a parser. And rust has such nice
// parser builder libraries (peg, pest, nom).
fn parse_line(line: &str) -> Option<Vec<Mul>> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let muls = re
        .captures_iter(line)
        .filter_map(|capture| {
            if let (Some(x), Some(y)) = (capture.get(1), capture.get(2)) {
                let x = x.as_str().parse().unwrap();
                let y = y.as_str().parse().unwrap();
                Some(Mul { x, y })
            } else {
                None
            }
        })
        .collect();

    Some(muls)
}

fn parse_input(input: &str) -> Option<Vec<Mul>> {
    let mul_regex = r"(mul)\((\d{1,3}),(\d{1,3})\)";
    let do_regex = r"(do)\(\)";
    let dont_regex = r"(don't)\(\)";

    // Here we construct a regex for all the different operations we want to parse and the (?m) flag makes it multiline.
    // We need to understand that the capture group counting is over the entire regex, so the first group matches a
    // possible "do", the second a possible "don't", the thrid a possible "mul" and the fourth and fifth the two numbers
    // we want to multiply.
    let parser = Regex::new(&format!(r"(?m){}|{}|{}", do_regex, dont_regex, mul_regex)).unwrap();

    // We start in the activated state with an empty mul list
    let mut activated = true;
    let mut result = vec![];

    // We iterate over all the captures in the input string
    parser.captures_iter(input).for_each(|capture| {
        // To find the operation we need to skip over the first item, which is the entire match and then find the first
        // capture group that is not None. No, I'm not proud of this code, but it works. It would completely fall apart
        // if we had another operation with arguments though.
        let operation = capture.iter().skip(1).flatten().next().map(|m| m.as_str());

        // This however is slightly more elegant, we match the operation and either (de)activate or process the mul if
        // we're activated. Notice how the mul match statement itself is conditional on the activated state.
        match operation {
            Some("do") => activated = true,
            Some("don't") => activated = false,
            Some("mul") if activated => {
                if let (Some(x), Some(y)) = (capture.get(4), capture.get(5)) {
                    let x = x.as_str().parse().unwrap();
                    let y = y.as_str().parse().unwrap();
                    result.push(Mul { x, y });
                }
            }
            _ => {}
        };
    });

    Some(result)
}
