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
struct IndexedVectors {
    horizontal: Vec<(i64, usize)>,
    vertical: Vec<(i64, usize)>,
}
impl IndexedVectors {
    fn new(path: &Vec<Vector>) -> Self {
        let mut horizontal = Vec::new();
        let mut vertical = Vec::new();
        for (i, vector) in path[1..].iter().enumerate() {
            match vector.direction {
                Direction::Left | Direction::Right => {
                    horizontal.push((vector.to.x, i));
                }
                Direction::Down | Direction::Up => {
                    vertical.push((vector.to.y, i));
                }
                _ => unreachable!(),
            }
        }
        // sort by axis
        horizontal.sort_unstable_by_key(|p| p.0);
        vertical.sort_unstable_by_key(|p| p.0);
        IndexedVectors {
            horizontal: horizontal,
            vertical: vertical,
        }
    }
    fn find_intersections(&self, paths: PathPair) -> VecIntersections {
        let mut intersections: VecIntersections = Vec::new();
        let PathPair {
            vertical: path_v,
            horizontal: path_h,
        } = paths;
        let Self {
            vertical,
            horizontal,
        } = self;
        // outer loop horizontal vector y value
        let mut v_btree: BTreeMap<i64, usize> = BTreeMap::new();
        let mut h_ivectors = horizontal.iter();
        let mut v_ivectors = vertical.iter().peekable();

        // find intersections of vertical vectors (inner loop) with horizontal vectors (outer loop)
        while let Some(v_ivector) = v_ivectors.next() {
            let v_vector: &Vector = &path_v[v_ivector.1 + 1];
            // populate y values of horizontal vectors (outer loop) in path_v_btree
            while let Some(h_ivector) = h_ivectors.next() {
                if h_ivector.0 < v_vector.to.x {
                    let h_vector: &Vector = &path_h[h_ivector.1];
                    v_btree.entry(h_vector.to.y).or_insert(h_ivector.1 + 1);
                } else {
                    break;
                }
            }

            if let Some(other_v_ivector_next) = v_ivectors.peek() {
                for (&path_h_y, &path_h_i) in v_btree.range(v_ivector.0..other_v_ivector_next.0) {
                    let h_vector = &path_h[path_h_i];

                    let last_v_steps = {
                        match v_vector.direction {
                            Direction::Down => v_ivector.0 - path_h_y,
                            Direction::Up => path_h_y - v_ivector.0,
                            _ => unreachable!(),
                        }
                    } + v_vector.steps as i64;

                    let from_h_vector = &path_h[path_h_i - 1];
                    let last_h_steps = {
                        match h_vector.direction {
                            Direction::Left => from_h_vector.to.x - v_vector.to.x,
                            Direction::Right => v_vector.to.x - from_h_vector.to.x,
                            _ => unreachable!(),
                        }
                    } + from_h_vector.steps as i64;
                    intersections.push((
                        Point {
                            x: v_vector.to.x,
                            y: h_vector.to.y,
                        },
                        StepCount(last_v_steps as u64, last_h_steps as u64),
                    ))
                }
            }
        }
        intersections
    }
}

#[derive(Debug)]
struct PathPair<'a> {
    horizontal: &'a Vec<Vector>,
    vertical: &'a Vec<Vector>,
}

#[derive(Debug)]
struct StepCount(u64, u64); // steps taken to reach intersection by each path

type VecIntersections = Vec<(Point, StepCount)>;

#[derive(Debug)]
struct PathIntersections {
    points: VecIntersections,
    between_wire_i: (usize, usize),
}

fn find_intersections(paths: &Vec<Vec<Vector>>) -> Vec<PathIntersections> {
    let mut intersections_by_paths: Vec<PathIntersections> = Vec::with_capacity(paths.len());

    // sorted indexed vector of all paths
    let mut indexed_vectors_all: Vec<IndexedVectors> = paths
        .iter()
        .map(|path| IndexedVectors::new(&path))
        .collect();

    let (iv, o_iv) = indexed_vectors_all.split_at_mut(1);
    let IndexedVectors {
        vertical: o_vertical,
        horizontal: o_horizontal,
    } = &o_iv[0];

    let IndexedVectors {
        vertical,
        horizontal,
    } = &iv[0];

    let p_h_po_v_pair = IndexedVectors {
        vertical: o_vertical.to_vec(),
        horizontal: horizontal.to_vec(),
    };

    let intersections = p_h_po_v_pair.find_intersections(PathPair {
        vertical: &paths[1],
        horizontal: &paths[0],
    });

    let p_v_po_h_pair = IndexedVectors {
        vertical: vertical.to_vec(),
        horizontal: o_horizontal.to_vec(),
    };

    let intersectionas = p_v_po_h_pair.find_intersections(PathPair {
        vertical: &paths[0],
        horizontal: &paths[1],
    });

    intersections_by_paths.push(PathIntersections {
        points: intersections,
        between_wire_i: (0, 1),
    });
    intersections_by_paths.push(PathIntersections {
        points: intersectionas,
        between_wire_i: (1, 0),
    });
    intersections_by_paths
}

fn parse_input(input: &str) -> Result<Vec<Vec<Vector>>> {
    Ok(input
        .lines()
        .map(|path| to_points(path.trim()).unwrap())
        .collect())
}

#[derive(Debug)]
struct Vector {
    to: Point,
    direction: Direction,
    steps: u64,
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Vector {
    fn next(&self, step: &Step) -> Self {
        let Vector {
            to: Point { x, y },
            steps,
            ..
        } = *self;

        let Step {
            direction,
            distance,
        } = *step;

        let steps = steps + distance;

        match direction {
            Direction::Right => Vector {
                to: Point {
                    x: x + distance as i64,
                    y,
                },
                direction,
                steps,
            },
            Direction::Left => Vector {
                to: Point {
                    x: x - distance as i64,
                    y,
                },
                direction,
                steps,
            },
            Direction::Up => Vector {
                to: Point {
                    x,
                    y: y + distance as i64,
                },
                direction,
                steps,
            },
            Direction::Down => Vector {
                to: Point {
                    x,
                    y: y - distance as i64,
                },
                direction,
                steps,
            },
            _ => unreachable!(),
        }
    }
}

fn to_points(input: &str) -> Result<Vec<Vector>> {
    let mut vector = vec![Vector {
        to: Point { x: 0, y: 0 },
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
