use std::io::{self, Error, Read};

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let input: Vec<usize> = input
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &Vec<usize>) -> Result<usize> {
    Ok(intcode(&input, 12, 2)?)
}

fn part2(input: &Vec<usize>) -> Result<usize> {
    for noun in 0..99 {
        for verb in 0..99 {
            if intcode(input, noun, verb)? == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }

    Err(Error::new(
        std::io::ErrorKind::InvalidInput,
        "NOT POSSIBLE!",
    ))
}

fn intcode(input: &Vec<usize>, noun: usize, verb: usize) -> Result<usize> {
    let mut input = input.clone();
    input[1] = noun;
    input[2] = verb;
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
            _ => {
                return Result::Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("malformed input pos: {}, val: {}", pos, input[pos]),
                ))
            }
        }
        if input[pos + 4] == 99 {
            return Result::Ok(input[0]);
        };
    }
    Result::Err(Error::new(
        std::io::ErrorKind::InvalidInput,
        "NO HALT INSTRUCTION GIVEN!",
    ))
}
