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
    let mut fuel_req = 0;
    for line in input.lines() {
        let mass: i32 = line.parse()?;
        fuel_req += mass / 3 - 2;
    }
    println!("{}", fuel_req);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut fuel_req = 0;
    for line in input.lines() {
        let mass: i32 = line.parse()?;
        let mut fuel = (mass / 3) - 2;
        while fuel > 0 {
            fuel_req += fuel;
            fuel = (fuel / 3) - 2;
        }
    }
    println!("{}", fuel_req);
    Ok(())
}
