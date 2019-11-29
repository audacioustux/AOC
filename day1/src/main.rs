use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut freq = 0;
    for line in input.lines() {
        let change: i32 = line.parse()?;
        freq += change;
    }
    println!("{}", freq);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut freq = 0;
    let mut seen = HashSet::new();
    seen.insert(0);

    loop {
        for line in input.lines() {
            let change: i32 = line.parse()?;
            freq += change;
            if seen.contains(&freq) {
                println!("{}", freq);
                return Ok(());
            }
            seen.insert(freq);
        }
    }
}
