use aoc2020::read_numbers;
use std::collections::HashSet;

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
    let values: Vec<i32> = read_numbers("./input/1")?;
    let target = 2020;

    // Run part1
    part1(&values, &target);

    // Run part2
    part2(&values, &target);

    Ok(())
}
