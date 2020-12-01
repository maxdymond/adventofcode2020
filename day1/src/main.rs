use std::collections::HashSet;
use std::io::{self, BufRead};
use std::path::Path;
use std::{fs::File, num::ParseIntError};

#[derive(thiserror::Error, Debug)]
enum DayError {
    #[error("IO error")]
    IOError(#[from] io::Error),

    #[error("Parse Error")]
    ParseError(#[from] ParseIntError),
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(line_values: &Vec<i32>, target: &i32) {
    let mut values = HashSet::new();
    for value in line_values {
        // Subtract the value from the target. If it exists in the hashset
        // then that's our value!
        let hashcheck = target - value;
        if values.contains(&hashcheck) {
            println!(
                "Part 1: Solution: {} * {} = {}",
                value,
                hashcheck,
                value * hashcheck
            );
            return;
        }

        // otherwise insert the value in the hashset.
        values.insert(value);
    }
}

fn part2(line_values: &Vec<i32>, target: &i32) {
    let num_values = line_values.len();
    for ii in 0..(num_values - 2) {
        for jj in (ii + 1)..(num_values - 1) {
            for kk in (jj + 1)..(num_values) {
                let ii_v = line_values.get(ii).unwrap();
                let jj_v = line_values.get(jj).unwrap();
                let kk_v = line_values.get(kk).unwrap();

                if ii_v + jj_v + kk_v == *target {
                    println!(
                        "Part 2: Solution: {} * {} * {} = {}",
                        ii_v,
                        jj_v,
                        kk_v,
                        ii_v * jj_v * kk_v
                    );
                    return;
                }
            }
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let lines = read_lines("./input.txt")?;
    let values = lines
        .map(|res| {
            res.map_err(DayError::IOError)
                .and_then(|v| v.parse::<i32>().map_err(DayError::ParseError))
        })
        .collect::<Result<Vec<i32>, DayError>>()?;
    let target = 2020;

    // Run part1
    part1(&values, &target);

    // Run part2
    part2(&values, &target);

    Ok(())
}
