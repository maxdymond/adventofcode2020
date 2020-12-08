use aoc2020::read_split;
use std::collections::{HashMap, HashSet};

fn part1(groups: &[String]) {
    println!(
        "Part 1: questions that anyone answered: {}",
        groups
            .iter()
            .map(|v| v
                .chars()
                .filter(|c| c != &'\n')
                .collect::<HashSet<char>>()
                .len())
            .sum::<usize>()
    );
}

fn part2(groups: &[String]) {
    println!(
        "Part 2: questions that everyone answered: {}",
        groups
            .iter()
            .map(|v| {
                let people = v.split('\n').filter(|s| !s.is_empty()).count();
                v.chars()
                    .filter(|c| c != &'\n')
                    .fold(HashMap::new(), |mut acc, c| {
                        *acc.entry(c).or_insert(0) += 1;
                        acc
                    })
                    .values()
                    .filter(|&count| count >= &people)
                    .count()
            })
            .sum::<usize>()
    );
}

fn main() -> Result<(), anyhow::Error> {
    let chunks = read_split("./input/6", "\n\n")?;
    part1(&chunks);
    part2(&chunks);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part2() {
        let chunks = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb"
            .split("\n\n")
            .map(String::from)
            .collect::<Vec<String>>();
        println!("Chunks: {:?}", &chunks);
        part2(&chunks);
    }
}
