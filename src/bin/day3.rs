use aoc2020::read_map;

fn run_map_vector(map: &Vec<Vec<char>>, x_inc: usize, y_inc: usize) -> u32 {
    let width = map[0].len();
    let mut trees = 0;
    let mut x = 0;

    for line in map.into_iter().step_by(y_inc) {
        if line[x] == '#' {
            trees += 1;
        }

        // Move the X parameter along by x_inc, but modulus by the width of the
        // map to wrap around when we get to the edge.
        x = (x + x_inc) % width;
    }
    trees
}

fn part1(map: &Vec<Vec<char>>) {
    println!("Part 1: encountered {} trees", run_map_vector(map, 3, 1));
}

fn part2(map: &Vec<Vec<char>>) {
    println!(
        "Part 2: solution {}",
        run_map_vector(map, 1, 1)
            * run_map_vector(map, 3, 1)
            * run_map_vector(map, 5, 1)
            * run_map_vector(map, 7, 1)
            * run_map_vector(map, 1, 2)
    );
}

fn main() -> Result<(), anyhow::Error> {
    let map = read_map("./input/3", |v| {
        let vals: Vec<char> = v.chars().collect();
        Ok(vals)
    })?;

    part1(&map);
    part2(&map);

    Ok(())
}
