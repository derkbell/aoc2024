use itertools::{repeat_n, Itertools};
use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

struct Equation {
    result: usize,
    values: Vec<usize>,
}

struct Problem {
    equations: Vec<Equation>,
}

impl Equation {
    // We could optimize this further by halting early if the intermediate results is larger than the result we are
    // looking for.
    fn test_with_operators(&self, operators: Vec<&Operator>) -> bool {
        let result = self.values.iter().skip(1).zip(operators.iter()).fold(
            self.values[0],
            |result, (value, operator)| match operator {
                Operator::Add => result + value,
                Operator::Multiply => result * value,
                Operator::Concatenate => {
                    let mut result = result.to_string();
                    result.push_str(&value.to_string());
                    result.parse().unwrap()
                }
                _ => unreachable!(),
            },
        );

        result == self.result
    }

    // The bulk of work is done by the itertools crate that has the repeat_n and multi_cartesian_product functions.
    // Together these create all possible combinations of operators of length n from a set of options. We simply test
    // each possiblility to see if it is a valid result.
    fn has_valid_result(&self, operators: &Vec<Operator>) -> bool {
        repeat_n(operators.into_iter(), self.values.len() - 1)
            .multi_cartesian_product()
            .any(|operators| self.test_with_operators(operators))
    }
}

impl Problem {
    fn sum_valid_results(&self, operators: &Vec<Operator>) -> usize {
        self.equations
            .iter()
            .filter(|equation| equation.has_valid_result(operators))
            .map(|equation| equation.result)
            .sum()
    }
}

impl From<String> for Problem {
    fn from(input: String) -> Self {
        let equations = input.lines().map(Equation::from).collect();

        Problem { equations }
    }
}

impl<S> From<S> for Equation
where
    S: AsRef<str>,
{
    fn from(input: S) -> Self {
        let input = input.as_ref();
        let mut parts = input.split(":");
        let result = parts.next().unwrap().trim().parse().unwrap();
        let values = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|value| value.parse().unwrap())
            .collect();

        Equation { result, values }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let problem = Problem::from(read_to_string("input")?);

    println!(
        "Part A: {}",
        problem.sum_valid_results(&vec![Operator::Add, Operator::Multiply])
    );

    println!(
        "Part B: {}",
        problem.sum_valid_results(&vec![
            Operator::Add,
            Operator::Multiply,
            Operator::Concatenate
        ])
    );

    Ok(())
}
