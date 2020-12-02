use aoc2020::{read_map, AocError};
use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"(?P<lower>\d+)-(?P<upper>\d+)\s+(?P<letter>[a-z]):\s+(?P<password>[a-z]+)"#)]
struct PassEntry {
    lower: usize,
    upper: usize,
    letter: char,
    password: String,
}

impl PassEntry {
    fn part1_valid(&self) -> bool {
        let num_matches = self.password.matches(self.letter).count();
        (self.lower <= num_matches) && (num_matches <= self.upper)
    }

    fn part2_valid(&self) -> bool {
        (self.password.chars().nth(self.lower - 1).unwrap() == self.letter)
            ^ (self.password.chars().nth(self.upper - 1).unwrap() == self.letter)
    }
}

fn main() -> Result<(), anyhow::Error> {
    let entries = read_map("./input/2", |v| {
        let entry: Result<PassEntry, AocError> = v
            .parse()
            .map_err(|_| AocError::GenericError("Failed to parse line".to_string()));
        entry
    })?;

    // Part 1: count the (part 1) valid pass entries
    let valid_entries = entries.iter().filter(|e| e.part1_valid()).count();
    println!("Part 1: valid entries: {}", valid_entries);

    // Part 2: count the (part 2) valid pass entries
    let valid_entries = entries.iter().filter(|e| e.part2_valid()).count();
    println!("Part 2: valid entries: {}", valid_entries);

    Ok(())
}
