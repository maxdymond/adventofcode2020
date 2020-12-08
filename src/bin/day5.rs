use aoc2020::read_map;
use std::collections::HashSet;

#[derive(Debug)]
struct Seat {
    row: usize,
    col: usize,
}

fn char_search(lower: usize, higher: usize, input: &str) -> usize {
    let mut lower = lower;
    let mut higher = higher;
    for k in input.chars() {
        let mid = (higher - lower + 1) / 2;
        match k {
            'F' => {
                higher = higher - mid;
            }
            'B' => {
                lower = lower + mid;
            }
            'L' => {
                higher = higher - mid;
            }
            'R' => {
                lower = lower + mid;
            }
            _ => unreachable!(),
        }
    }
    assert_eq!(lower, higher);
    lower
}

impl From<String> for Seat {
    fn from(input: String) -> Self {
        let (rowinfo, seatinfo) = input.split_at(7);
        Seat {
            row: char_search(0, 127, rowinfo),
            col: char_search(0, 7, seatinfo),
        }
    }
}

impl Seat {
    fn seat_id(&self) -> usize {
        (self.row * 8) + self.col
    }
}

fn part1(map: &[Seat]) {
    let maxsid = map
        .iter()
        .map(|s| s.seat_id())
        .max()
        .expect("Get minimum value failed");
    println!("Part 1: max seat ID is {}", maxsid);
}

fn part2(map: &[Seat]) {
    let vals = map.iter().map(|s| s.seat_id()).collect::<HashSet<usize>>();
    for ii in 1..(vals.len() - 1) {
        if !vals.contains(&ii) && vals.contains(&(ii - 1)) && vals.contains(&(ii + 1)) {
            println!("Part 2: missing seat ID is {}", ii);
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let map = read_map("./input/5", |v| Ok(Seat::from(v)))?;
    part1(&map);
    part2(&map);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat() {
        let x = Seat::from("FBFBBFFRLR".to_string());
        println!("Seat: {:?}: {}", x, x.seat_id());
    }
}
