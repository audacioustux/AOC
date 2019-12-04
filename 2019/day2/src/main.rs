use std::error::Error;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut input: Vec<usize> = input
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    input[1] = 12;
    input[2] = 2;

    part1(&mut input)?;
    // part2(&input)?;
    Ok(())
}

fn part1(input: &mut Vec<usize>) -> Result<()> {
    let mut intcode;
    for pos in (0..input.len()).step_by(4) {
        intcode = (
            input[pos + 0],
            input[pos + 1],
            input[pos + 2],
            input[pos + 3],
        );
        match intcode.0 {
            1 => input[intcode.3] = input[intcode.1] + input[intcode.2],
            2 => input[intcode.3] = input[intcode.1] * input[intcode.2],
            _ => println!("malformed input pos: {}, val: {}", pos, input[pos]),
        }
        if input[pos + 4] == 99 {
            println!("{}", input[0]);
            break;
        };
    }
    Ok(())
}
