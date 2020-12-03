use aoc2020::read_map;

fn run_map_vector(map: &[Vec<usize>], x_inc: usize, y_inc: usize) -> usize {
    map.iter()
        .step_by(y_inc)
        .zip((0..).step_by(x_inc))
        .fold(0, |acc, (row, x)| acc + row[x % row.len()])
}

fn part1(map: &[Vec<usize>]) {
    println!("Part 1: encountered {} trees", run_map_vector(map, 3, 1));
}

fn part2(map: &[Vec<usize>]) {
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
        Ok(v.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
    })?;

    part1(&map);
    part2(&map);

    Ok(())
}
