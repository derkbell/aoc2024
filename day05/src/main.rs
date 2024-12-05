use std::fs::read_to_string;

struct Problem {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

struct Rule {
    left: usize,
    right: usize,
}

struct Update {
    pages: Vec<usize>,
}

impl Problem {
    fn valid_updates(&self) -> Vec<&Update> {
        self.updates
            .iter()
            .filter(|&update| self.verify_update(update))
            .collect()
    }

    fn invalid_updates(&self) -> Vec<&Update> {
        self.updates
            .iter()
            .filter(|&update| !self.verify_update(update))
            .collect()
    }

    fn verify_update(&self, update: &Update) -> bool {
        update.pages.windows(2).all(|window| {
            let left = window[0];
            let right = window[1];

            self.rules
                .iter()
                .any(|rule| rule.left == left && rule.right == right)
        })
    }

    // So I had this epiphany: the set of rules are a directed acyclic graph. Therfore, if we take the subgraph
    // containing only nodes that are part of the update, the in-degree equals the index in the corrected update list:
    //
    // The page that should be first in the update will have no rules where it appears on the right hand side, so it has
    // an in-degree of 0. The page that is after that one will have exactly one rule where it appears on the right hand
    // side, namely the rule that says the first page we already ordered must be before the page we're processing now,
    // so it will have an in-degree of 1.
    fn correct_update(&self, update: &Update) -> Update {
        let mut pages = vec![];

        for page in update.pages.iter() {
            pages.insert(
                self.rules
                    .iter()
                    .filter(|&rule| rule.right == *page && pages.contains(&rule.left))
                    .map(|rule| rule.right)
                    .count(),
                *page,
            );
        }

        Update { pages }
    }
}

impl Update {
    fn middle(&self) -> usize {
        self.pages[self.pages.len() / 2]
    }
}

// This is a neat little trick in rust, you can implement a trait generically for a type. Anything that can be converted
// to a &str can be be converted to a Rule with this implementation. This solves some issues I had while parsing the
// input where I had String, &String and &str types. With this generic implementation, it doesn't matter what type it
// is, as long as the input type S implement AsRef<str>, therefore having the .as_ref method.
impl<S> From<S> for Rule
where
    S: AsRef<str>,
{
    fn from(input: S) -> Self {
        let input = input.as_ref();
        let (left, right) = input.split_once("|").unwrap();
        Self {
            left: left.parse().unwrap(),
            right: right.parse().unwrap(),
        }
    }
}
impl<S> From<S> for Update
where
    S: AsRef<str>,
{
    fn from(input: S) -> Self {
        let pages = input
            .as_ref()
            .split(",")
            .map(|page| page.parse().unwrap())
            .collect();
        Self { pages }
    }
}

// Here we always pass a String (from the read_to_string) so it didn't have to be a generic implementation. I just
// wanted to illustrate the concept a bit more.
impl<S> From<S> for Problem
where
    S: Into<String>,
{
    fn from(input: S) -> Self {
        let input = input.into();
        let (rules_input, updates_input) = input.split_once("\n\n").unwrap();

        let rules = rules_input.lines().map(|line| Rule::from(line)).collect();
        let updates = updates_input
            .lines()
            .map(|line| Update::from(line))
            .collect();

        Self { rules, updates }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let problem = Problem::from(read_to_string("input")?);

    println!(
        "Part A: {}",
        problem
            .valid_updates()
            .iter()
            .map(|update| update.middle())
            .sum::<usize>()
    );

    println!(
        "Part B: {}",
        problem
            .invalid_updates()
            .iter()
            .map(|update| problem.correct_update(update).middle())
            .sum::<usize>()
    );

    Ok(())
}
