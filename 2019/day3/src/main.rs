// use std::collections::BTreeMap;
use std::error::Error;
use std::io::{self, Read};
// use std::ops::Bound::Included;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let path_1 = &mut parse_input(&input)?[0];
    let path_2 = &mut parse_input(&input)?[1];

    let intersections: Intersections = find_sections([&path_1, &path_2]);

    println!("{:#?}", path_1);
    
    Ok(())
}

#[derive(Debug)]
struct IndexedPoints {
    x: Vec<(i64, usize)>, // Horizontal
    y: Vec<(i64, usize)> // Vertical
}
impl IndexedPoints {
    fn new (path: &Vec<Point>) -> Self {
        let mut points: IndexedPoints = IndexedPoints { x: Vec::new(), y: Vec::new() };
    
        for (i, point) in path[1..].iter().enumerate() {
            match point.direction {
                Direction::Left | Direction::Right => {
                    points.x.push((point.x, i));
                },
                Direction::Down | Direction::Up => {
                    points.y.push((point.y, i));
                },
                _ => unreachable!()
            }
        }
    
        points.sort_by_axis();
    
        points
    }

    fn sort_by_axis (&mut self) {
        self.x.sort_unstable_by_key(|p| p.0);
        self.y.sort_unstable_by_key(|p| p.0);
    }
}


struct StepCount (u64, u64);
type Intersections = Vec<(Point, StepCount)>;

fn find_sections(paths: [&Vec<Point>; 2]) -> Intersections {
    let mut intersections: Intersections = Vec::new();

    let paths = vec![IndexedPoints::new(&paths[0]), IndexedPoints::new(&paths[1])];

    println!("{:#?}", paths);
    intersections
}

fn parse_input(input: &str) -> Result<[Vec<Point>; 2]> {
    let mut input_iter = input.lines();
    
    let mut parsed = || to_points(input_iter.next().unwrap().trim()).unwrap();

    Ok([parsed(), parsed()])
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    direction: Direction,
    steps: u64
}

impl Point {
    fn next(&self, step: &Step) -> Self {
        let Point {x,y, steps, ..} = *self;

        let Step {direction, distance} = *step;

        let steps = steps + distance;

        match direction {
            Direction::Right => Point {x: x + distance as i64, y, direction, steps},
            Direction::Left => Point {x: x - distance as i64, y, direction, steps},
            Direction::Up => Point {x, y: y + distance as i64, direction, steps},
            Direction::Down => Point {x, y: y - distance as i64, direction, steps},
            _ => unreachable!()
        }
    }
}

fn to_points(input: &str) -> Result<Vec<Point>> {
    let mut points = vec![Point { x: 0, y: 0, direction: Direction::Origin, steps: 0 }];

    for step in input.split(',') {
        let step = parse_step(step).unwrap();
        points.push(points.last().unwrap().next(&step))
    }

    Ok(points)
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Origin
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
    distance: u64,
}
fn parse_step(step: &str) -> Result<Step> {
    let chr = step.chars().nth(0).unwrap();
    let direction = Direction::from_chr(chr)?;
    let distance = step[1..].parse::<u64>()?;
    Ok(Step {
        direction,
        distance,
    })
}
