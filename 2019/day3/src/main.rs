use std::collections::BTreeMap;
use std::error::Error;
use std::io::{self, Read};
// use std::ops::Range;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let paths = &mut parse_input(&input)?;

    let intersections = find_intersections(&paths);
    // find_intersections(&paths);
    println!("{:#?}", intersections);
    Ok(())
}

#[derive(Debug)]
struct IndexedPoints {
    x: Vec<(i64, usize)>, // Horizontal
    y: Vec<(i64, usize)>, // Vertical
}
impl IndexedPoints {
    fn new(path: &[Point]) -> Self {
        let mut vector: IndexedPoints = IndexedPoints {
            x: Vec::new(),
            y: Vec::new(),
        };
        for (i, point) in path[1..].iter().enumerate() {
            match point.direction {
                Direction::Left | Direction::Right => {
                    vector.x.push((point.x, i));
                }
                Direction::Down | Direction::Up => {
                    vector.y.push((point.y, i));
                }
                _ => unreachable!(),
            }
        }
        // sort by axis
        vector.x.sort_unstable_by_key(|p| p.0);
        vector.y.sort_unstable_by_key(|p| p.0);
        vector
    }
}

#[derive(Debug)]
struct StepCount(u64, u64); // steps taken to reach intersection by each path

#[derive(Debug)]
struct Intersections {
    vector: Vec<(Point, StepCount)>,
    between_wire_i: (usize, usize),
}

fn find_intersections(paths: &[Vec<Point>]) -> Vec<Intersections> {
    let mut intersections_by_paths: Vec<Intersections> = Vec::with_capacity(paths.len());

    // sorted indexed vector of all paths
    let paths_indexed_points: Vec<IndexedPoints> =
        paths.iter().map(|path| IndexedPoints::new(&path)).collect();

    for (path_no, path) in paths_indexed_points[..paths_indexed_points.len()]
        .iter()
        .enumerate()
    {
        for (path_other_no, path_other) in paths_indexed_points[path_no + 1..].iter().enumerate() {
            let mut intersections: Vec<(Point, StepCount)> = Vec::new();
            // let mut x_btree: BTreeMap<i64, usize> = BTreeMap::new();
            let mut y_btree: BTreeMap<i64, usize> = BTreeMap::new();
            let mut path_x_ipoints = path.x.iter();
            let mut path_other_y_ipoints = path_other.y.iter().peekable();

            let path_other_no = path_other_no + 1;
            let path_other_points = &paths[path_other_no];
            while let Some(other_y_ipoint) = path_other_y_ipoints.next() {
                let y_point: &Point = &path_other_points[other_y_ipoint.1 + 1];
                while let Some(x_ipoint) = path_x_ipoints.next() {
                    if x_ipoint.0 < y_point.x {
                        let x_point: &Point = &paths[path_no][x_ipoint.1];
                        y_btree.entry(x_point.y).or_insert(x_ipoint.1);
                    } else {
                        break;
                    }
                }

                if let Some(other_y_ipoint_next) = path_other_y_ipoints.peek() {
                    for (&path_x_y, &path_x_i) in
                        y_btree.range(other_y_ipoint.0..other_y_ipoint_next.0)
                    {
                        let x_point = &paths[path_no][path_x_i + 1];
                        let from_y_point = &path_other_points[other_y_ipoint.1];
                        let last_y_steps = {
                            match y_point.direction {
                                Direction::Down => path_x_y - from_y_point.y,
                                Direction::Up => from_y_point.y - path_x_y,
                                _ => unreachable!(),
                            }
                        } + y_point.steps as i64;

                        let from_x_point = &paths[path_no][path_x_i];
                        let last_x_steps = {
                            match x_point.direction {
                                Direction::Left => from_x_point.x - y_point.x,
                                Direction::Right => y_point.x - from_y_point.y,
                                _ => unreachable!(),
                            }
                        } + from_x_point.steps as i64;
                        intersections.push((
                            Point {
                                x: y_point.x,
                                y: x_point.y,
                                direction: Direction::Unit,
                                steps: (last_y_steps + last_x_steps) as u64,
                            },
                            StepCount(last_y_steps as u64, last_x_steps as u64),
                        ))
                    }
                }
            }
            intersections_by_paths.push(Intersections {
                vector: intersections,
                between_wire_i: (path_no, path_other_no),
            });
            // println!("{:#?}", intersections);
        }
    }

    intersections_by_paths
}

fn parse_input(input: &str) -> Result<Vec<Vec<Point>>> {
    Ok(input
        .lines()
        .map(|path| to_points(path.trim()).unwrap())
        .collect())
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    direction: Direction,
    steps: u64,
}

impl Point {
    fn next(&self, step: &Step) -> Self {
        let Point { x, y, steps, .. } = *self;

        let Step {
            direction,
            distance,
        } = *step;

        let steps = steps + distance;

        match direction {
            Direction::Right => Point {
                x: x + distance as i64,
                y,
                direction,
                steps,
            },
            Direction::Left => Point {
                x: x - distance as i64,
                y,
                direction,
                steps,
            },
            Direction::Up => Point {
                x,
                y: y + distance as i64,
                direction,
                steps,
            },
            Direction::Down => Point {
                x,
                y: y - distance as i64,
                direction,
                steps,
            },
            _ => unreachable!(),
        }
    }
}

fn to_points(input: &str) -> Result<Vec<Point>> {
    let mut vector = vec![Point {
        x: 0,
        y: 0,
        direction: Direction::Unit,
        steps: 0,
    }];

    for step in input.split(',') {
        let step = parse_step(step).unwrap();
        vector.push(vector.last().unwrap().next(&step))
    }

    Ok(vector)
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Unit,
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
