use std::io::{self, BufRead};
use std::path::Path;
use std::{fs::File, num::ParseIntError};

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("IO error")]
    IOError(#[from] io::Error),

    #[error("Parse Error")]
    ParseError(#[from] ParseIntError),

    #[error("Generic Error")]
    GenericError(String),
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_numbers<P, T>(filename: P) -> Result<Vec<T>, AocError>
where
    P: AsRef<Path>,
    T: std::str::FromStr,
{
    read_map(filename, |v| {
        v.parse::<T>()
            .map_err(|_| AocError::GenericError("Failed to parse line".to_string()))
    })
}

pub fn read_map<P, T, F>(filename: P, map: F) -> Result<Vec<T>, AocError>
where
    P: AsRef<Path>,
    F: Fn(String) -> Result<T, AocError>,
{
    let lines = read_lines(filename)?;
    lines
        .map(|res| res.map_err(AocError::IOError).and_then(&map))
        .collect::<Result<Vec<T>, AocError>>()
}

pub fn read_split<P>(filename: P, pattern: &str) -> Result<Vec<String>, AocError>
where
    P: AsRef<Path>,
{
    Ok(std::fs::read_to_string(filename)?
        .split(pattern)
        .map(String::from)
        .collect::<Vec<String>>())
}
