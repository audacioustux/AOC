use std::error::Error;
use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let input = parse_input(&input);

    println!("{:#?}", input);
    Ok(())
}

#[derive(Debug)]
struct HSegment {
    y: isize,
    start_x: isize,
    len: isize,
}
#[derive(Debug)]
struct VSegment {
    x: isize,
    start_y: isize,
    len: isize,
}

fn parse_input(input: &str) -> Vec<(Vec<HSegment>, Vec<VSegment>)> {
    input
        .lines()
        .map(|path| to_segments(path.trim()).unwrap())
        .collect()
}

struct Position {
    x: isize,
    y: isize,
}
fn to_segments(input: &str) -> Result<(Vec<HSegment>, Vec<VSegment>)> {
    let mut h_segments: Vec<HSegment> = Vec::new();
    let mut v_segments: Vec<VSegment> = Vec::new();

    let mut curr_pos = Position { x: 0, y: 0 };
    for step in input.split(',') {
        let step = parse_step(step).unwrap();
        match step.direction {
            Direction::Right => {
                h_segments.push(HSegment {
                    y: curr_pos.y,
                    start_x: curr_pos.x,
                    len: step.distance,
                });
                curr_pos.x += step.distance;
            }
            Direction::Left => {
                curr_pos.x -= step.distance;
                h_segments.push(HSegment {
                    y: curr_pos.y,
                    start_x: curr_pos.x,
                    len: step.distance,
                })
            }
            Direction::Up => {
                v_segments.push(VSegment {
                    x: curr_pos.x,
                    start_y: curr_pos.y,
                    len: step.distance,
                });
                curr_pos.y += step.distance;
            }
            Direction::Down => {
                curr_pos.y -= step.distance;
                v_segments.push(VSegment {
                    x: curr_pos.x,
                    start_y: curr_pos.y,
                    len: step.distance,
                });
            }
        };
    }
    Ok((h_segments, v_segments))
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn from_chr(chr: char) -> Result<Direction> {
        let result = match chr {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => {
                return Result::Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Direction Character: {}", chr),
                )))
            }
        };
        Ok(result)
    }
}

struct Step {
    direction: Direction,
    distance: isize,
}
fn parse_step(step: &str) -> Result<Step> {
    let chr = step.chars().nth(0).unwrap();
    let direction = Direction::from_chr(chr)?;
    let distance = step[1..].parse::<isize>()?;
    Ok(Step {
        direction,
        distance,
    })
}
