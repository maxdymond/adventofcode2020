use aoc2020::read_split;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

struct Passport {
    map: HashMap<String, String>,
}

impl From<String> for Passport {
    fn from(input: String) -> Self {
        Self {
            map: input.split_whitespace().map(colon_tuple).collect(),
        }
    }
}

enum Height {
    Unknown(String),
    Cm(usize),
    In(usize),
}

impl From<&String> for Height {
    fn from(input: &String) -> Self {
        let pos = input.len() - 2;
        let (height, units) = input.split_at(pos);
        match height.parse::<usize>() {
            Ok(height) => match units {
                "cm" => Self::Cm(height),
                "in" => Self::In(height),
                _ => Self::Unknown(input.to_owned()),
            },
            Err(_) => Self::Unknown(input.to_owned()),
        }
    }
}

impl Height {
    fn valid(&self) -> bool {
        match self {
            Self::Unknown(_) => false,
            Self::Cm(height) => (height >= &150) && (height <= &193),
            Self::In(height) => (height >= &59) && (height <= &76),
        }
    }
}

enum Eyes {
    Unknown(String),
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

impl From<&String> for Eyes {
    fn from(input: &String) -> Self {
        match &input[..] {
            "amb" => Self::Amb,
            "blu" => Self::Blu,
            "brn" => Self::Brn,
            "gry" => Self::Gry,
            "grn" => Self::Grn,
            "hzl" => Self::Hzl,
            "oth" => Self::Oth,
            _ => Self::Unknown(input.to_owned()),
        }
    }
}

impl Eyes {
    fn valid(&self) -> bool {
        !matches!(self, Self::Unknown(_))
    }
}

impl Passport {
    fn part1_valid(&self) -> bool {
        self.map.contains_key("byr")
            && self.map.contains_key("iyr")
            && self.map.contains_key("eyr")
            && self.map.contains_key("hgt")
            && self.map.contains_key("hcl")
            && self.map.contains_key("ecl")
            && self.map.contains_key("pid")
    }

    fn valid_year(&self, key: &str, lower: u16, upper: u16) -> bool {
        self.map
            .get(key)
            .and_then(|v| v.parse::<u16>().ok())
            .filter(|yr| yr >= &lower && yr <= &upper)
            .is_some()
    }

    fn valid_height(&self) -> bool {
        self.map
            .get("hgt")
            .map(Height::from)
            .filter(|hgt| hgt.valid())
            .is_some()
    }

    fn valid_hair(&self) -> bool {
        self.map
            .get("hcl")
            .filter(|hair| {
                let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                re.is_match(hair)
            })
            .is_some()
    }

    fn valid_eyes(&self) -> bool {
        self.map
            .get("ecl")
            .map(Eyes::from)
            .filter(|eyes| eyes.valid())
            .is_some()
    }

    fn valid_passportid(&self) -> bool {
        self.map
            .get("pid")
            .filter(|pid| {
                let re = Regex::new(r"^[0-9]{9}$").unwrap();
                re.is_match(pid)
            })
            .is_some()
    }

    fn part2_valid(&self) -> bool {
        self.valid_year("byr", 1920, 2002)
            && self.valid_year("iyr", 2010, 2020)
            && self.valid_year("eyr", 2020, 2030)
            && self.valid_height()
            && self.valid_hair()
            && self.valid_eyes()
            && self.valid_passportid()
    }
}

fn colon_tuple(input: &str) -> (String, String) {
    let (key, value) = input.split(':').next_tuple().expect("Expected tuple");
    (key.to_owned(), value.to_owned())
}

fn part1(passports: &[Passport]) {
    println!(
        "Part 1: counted {} valid passports",
        passports.iter().filter(|p| p.part1_valid()).count()
    );
}

fn part2(passports: &[Passport]) {
    println!(
        "Part 2: counted {} valid passports",
        passports.iter().filter(|p| p.part2_valid()).count()
    );
}

fn main() -> Result<(), anyhow::Error> {
    let chunks = read_split("./input/4", "\n\n")?;
    let passports = chunks
        .into_iter()
        .map(Passport::from)
        .collect::<Vec<Passport>>();

    part1(&passports);
    part2(&passports);

    Ok(())
}
