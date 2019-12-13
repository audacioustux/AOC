use std::collections::BTreeMap;
use std::error::Error;
use std::io::{self, Read};
use std::ops::Bound::Included;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let path_1 = &mut parse_input(&input)[0];
    let path_2 = &mut parse_input(&input)[1];

    let mut section_points: Vec<Point> =
        intersections(&mut path_1.h_segments, &mut path_2.v_segments);

    section_points.append(&mut intersections(
        &mut path_2.h_segments,
        &mut path_1.v_segments,
    ));

    println!("{}", part1(&mut section_points));

    Ok(())
}

fn part1(points: &mut Vec<Point>) -> isize {
    let manhattan_distances: Vec<isize> = points
        .iter()
        .map(|point| manhattan_distance(&point))
        .collect();

    *manhattan_distances.iter().min().unwrap()
}

fn manhattan_distance(point: &Point) -> isize {
    point.x.abs() + point.y.abs()
}

fn intersections(h_segments: &mut Vec<HSegment>, v_segments: &mut Vec<VSegment>) -> Vec<Point> {
    let mut section_points: Vec<Point> = Vec::new();

    let mut y_btree_map = BTreeMap::new();
    for (i, h_seg) in h_segments.iter().enumerate() {
        y_btree_map.insert(h_seg.y, i);
    }
    for v_seg in v_segments {
        for (&y, &h_seg_i) in y_btree_map.range((
            Included(&(v_seg.start_y - v_seg.len)),
            Included(&v_seg.start_y),
        )) {
            let h_seg_range = std::ops::Range {
                start: h_segments[h_seg_i].start_x,
                end: h_segments[h_seg_i].start_x + h_segments[h_seg_i].len,
            };
            if h_seg_range.contains(&v_seg.x) {
                section_points.push(Point { y: y, x: v_seg.x });
            }
        }
    }
    section_points
}

#[derive(Debug)]
struct Segments {
    h_segments: Vec<HSegment>,
    v_segments: Vec<VSegment>,
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

fn parse_input(input: &str) -> Vec<Segments> {
    input
        .lines()
        .map(|path| to_segments(path.trim()).unwrap())
        .collect()
}

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}
fn to_segments(input: &str) -> Result<Segments> {
    let mut h_segments: Vec<HSegment> = Vec::new();
    let mut v_segments: Vec<VSegment> = Vec::new();

    let mut curr_pos = Point { x: 0, y: 0 };
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
                curr_pos.y += step.distance;
                v_segments.push(VSegment {
                    x: curr_pos.x,
                    start_y: curr_pos.y,
                    len: step.distance,
                });
            }
            Direction::Down => {
                v_segments.push(VSegment {
                    x: curr_pos.x,
                    start_y: curr_pos.y,
                    len: step.distance,
                });
                curr_pos.y -= step.distance;
            }
        };
    }
    Ok(Segments {
        h_segments,
        v_segments,
    })
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
