use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinates {
    x: isize,
    y: isize,
}
#[derive(Debug)]
struct Antenna {
    coordinates: Coordinates,
}

#[derive(Debug)]
struct AntennaGroup {
    antennas: Vec<Antenna>,
}

#[derive(Debug)]
struct Problem {
    antenna_groups: HashMap<char, AntennaGroup>,
    width: isize,
    height: isize,
}

impl AntennaGroup {
    fn calculate_antinodes(&self, resonance: bool) -> Vec<Coordinates> {
        self.antennas
            .iter()
            .enumerate()
            .map(|(a, antenna)| {
                self.antennas
                    .iter()
                    .enumerate()
                    .map(move |(b, other_antenna)| {
                        if a == b {
                            return vec![];
                        }

                        let dx = antenna.coordinates.x - other_antenna.coordinates.x;
                        let dy = antenna.coordinates.y - other_antenna.coordinates.y;

                        match resonance {
                            false => {
                                // For part A, just return the first antinode.
                                vec![Coordinates {
                                    x: other_antenna.coordinates.x - dx,
                                    y: other_antenna.coordinates.y - dy,
                                }]
                            }
                            true => {
                                // Yeah yeah yeah, it's ugly but it works. We could do something clever with the width
                                // and height of the grid, but I'll admit I'm just lazy. It's sunday after all.
                                (0..50)
                                    .map(|i| Coordinates {
                                        x: other_antenna.coordinates.x - (i * dx),
                                        y: other_antenna.coordinates.y - (i * dy),
                                    })
                                    .collect()
                            }
                        }
                    })
            })
            // Did I tell you I was lazy yet?
            .flatten()
            .flatten()
            .collect()
    }
}

impl Problem {
    fn count_all_antinodes(&self, resonance: bool) -> usize {
        self.antenna_groups
            .values()
            .map(|group| AntennaGroup::calculate_antinodes(group, resonance))
            .map(|v| {
                v.into_iter()
                    .filter(|coordinates| {
                        // But Derk, I though you hated bounds checking?
                        coordinates.x >= 0
                            && coordinates.x < self.width
                            && coordinates.y >= 0
                            && coordinates.y < self.height
                    })
                    .collect::<Vec<Coordinates>>()
            })
            .flatten()
            // Lazy again
            .unique()
            .count()
    }
}

impl From<String> for Problem {
    fn from(input: String) -> Self {
        let mut antenna_groups = HashMap::new();
        input.lines().enumerate().for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_x, c)| *c != '.')
                .for_each(|(x, c)| {
                    let entry = antenna_groups.entry(c).or_insert(AntennaGroup {
                        antennas: Vec::new(),
                    });
                    entry.antennas.push(Antenna {
                        coordinates: Coordinates {
                            x: x as isize,
                            y: y as isize,
                        },
                    });
                });
        });

        let height = input.lines().count() as isize;
        let width = input.lines().next().unwrap().len() as isize;

        Problem {
            antenna_groups,
            width,
            height,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let problem = Problem::from(read_to_string("input")?);

    println!("Part A: {}", problem.count_all_antinodes(false));
    println!("Part B: {}", problem.count_all_antinodes(true));
    Ok(())
}
