use aoc2020::read_map;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Bag {
    pub name: String,
    pub contents: HashMap<String, usize>,
}

impl From<String> for Bag {
    fn from(input: String) -> Self {
        // Split on " bags contain  "
        let parts = input.split(" bags contain ").collect::<Vec<&str>>();

        // Find subbags
        let re = Regex::new(r#"(\d+) ([a-z]+ [a-z]+) bag"#).unwrap();

        Self {
            name: parts[0].to_string(),
            contents: re
                .captures_iter(parts[1])
                .map(|x| {
                    let quantity = x.get(1).unwrap().as_str().parse().unwrap();
                    let bag_type = x.get(2).unwrap().as_str().to_owned();
                    (bag_type, quantity)
                })
                .collect(),
        }
    }
}

fn part1(bags: &[Bag]) {
    let mut revmap = HashMap::new();

    for bag in bags {
        // Iterate over the contents and insert a reverse mapping.
        for (subbag, _) in bag.contents.iter() {
            revmap
                .entry(subbag.as_str())
                .or_insert(Vec::new())
                .push(bag.name.clone());
        }
    }

    let mut revset = HashSet::new();

    fn visit_bags(bag_type: &str, hmap: &HashMap<&str, Vec<String>>, set: &mut HashSet<String>) {
        if let Some(v) = hmap.get(bag_type) {
            // println!("Found {} : {:?}", bag_type, v);
            for sbt in v {
                // println!("Inserting {}", sbt);
                set.insert(sbt.to_owned());
                visit_bags(sbt, hmap, set);
            }
        } else {
            // println!("{} not found in map", bag_type);
        }
    };
    // println!("Revmap {:?}", revmap);

    visit_bags("shiny gold", &revmap, &mut revset);

    println!("Part 1: Shiny Gold revset total: {}", revset.len());
}

fn part2(bags: &[Bag]) {
    let mut fwdmap = HashMap::new();
    for bag in bags {
        // Iterate over the contents and insert a mapping.
        for subbagcontents in bag.contents.iter() {
            fwdmap
                .entry(bag.name.as_str())
                .or_insert(Vec::new())
                .push(subbagcontents);
        }
    }

    fn get_bag_size(
        bag_type: &str,
        hmap: &HashMap<&str, Vec<(&String, &usize)>>,
        depth: usize,
    ) -> usize {
        let indent = " ".repeat(depth);

        if let Some(v) = hmap.get(bag_type) {
            println!("{}{} bags contain...", indent, bag_type);
            let res = v
                .iter()
                .map(|&(sbt, quantity)| {
                    println!("{}- {} {} bags", indent, quantity, sbt);
                    quantity * get_bag_size(sbt.as_str(), hmap, depth + 2)
                })
                .sum::<usize>()
                + 1;
            println!("{}{} bags have {} bags in total", indent, bag_type, res);
            res
        } else {
            println!("{}{} bags have no internal bags", indent, bag_type);
            1
        }
    };

    let num_bags = get_bag_size("shiny gold", &fwdmap, 0) - 1;
    println!("Part 2: Shiny Gold total bags: {}", num_bags);
}

fn main() -> Result<(), anyhow::Error> {
    let bags = read_map("./input/7", |v| Ok(Bag::from(v)))?;

    part1(&bags);
    part2(&bags);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part2() {
        let bags = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
            .lines()
            .map(|v| Bag::from(v.to_owned()))
            .collect::<Vec<Bag>>();

        println!("Bags: {:?}", bags);
        part2(&bags);
    }
}
