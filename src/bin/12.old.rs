use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

fn char_to_height(c: char) -> u8 {
    // A-Z => 65 -> 90
    // a-z => 97 -> 122
    // Lowercase item types a through z have height 1 through 26.
    if c.is_lowercase() {
        c as u8 - 96
    } else if c == 'S' {
        1
    } else if c == 'E' {
        26
    } else {
        println!("unknown char -> {}", c);
        0
    }
}

#[derive(Debug)]
enum CellStatus {
    // Can reach the target by usize steps
    Some(usize),

    // Can not reach the target
    None,

    // the status is calculating
    Calculating,
}

#[derive(PartialEq, Clone, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Point({},{})", self.x, self.y)
    }
}

#[derive(Debug)]
struct HeightMap {
    map: Vec<Vec<u8>>,
    step_map: HashMap<Point, CellStatus>,
    start_point: Point,
    target_point: Point,
}

impl HeightMap {
    fn new() -> Self {
        HeightMap {
            map: vec![],
            step_map: HashMap::new(),
            start_point: Point { x: 0, y: 0 },
            target_point: Point { x: 0, y: 0 },
        }
    }

    fn from_input(input: &str) -> Self {
        let mut m = HeightMap::new();

        for (y, line) in input.lines().enumerate() {
            let mut line_vec = vec![];

            for (x, ch) in line.chars().enumerate() {
                if ch == 'S' {
                    m.start_point = Point { x: x, y: y };
                } else if ch == 'E' {
                    m.target_point = Point { x: x, y: y };
                }

                line_vec.push(char_to_height(ch));
            }

            m.map.push(line_vec);
        }

        return m;
    }

    fn get_size(&self) -> (usize, usize) {
        let height = self.map.len();
        let width = self.map.first().unwrap().len();

        (width, height)
    }

    fn try_move(&self, from: &Point, offset: (i32, i32)) -> Option<Point> {
        let (offset_x, offset_y) = offset;
        let (x, y) = (from.x as i32 + offset_x, from.y as i32 + offset_y);
        let (width, height) = self.get_size();

        if x >= 0 && y >= 0 && x < width as i32 && y < height as i32 {
            let from_depth = self.map[from.y][from.x];
            let target_depth = self.map[y as usize][x as usize];

            // println!(
            //     "try move from {:?}({}) to {:?}({})",
            //     from,
            //     from_depth,
            //     (x, y),
            //     target_depth
            // );

            if (target_depth as i32 - from_depth as i32) <= 1 {
                return Some(Point {
                    x: x as usize,
                    y: y as usize,
                });
            }
        }

        None
    }

    fn find_target(&mut self, point: &Point) -> Option<usize> {
        if point.x == self.target_point.x && point.y == self.target_point.y {
            return Some(0);
        }

        if self.step_map.contains_key(&point) {
            let r = self.step_map.get(&point).unwrap();

            // println!("contains key {:?} -> {:?}", &point, r);

            return match r {
                CellStatus::Some(level) => Some(level.clone()),
                _ => None,
            };
        }

        // println!("start calculating {:?}", &point);

        // mark the current cell as calculating
        self.step_map.insert(point.clone(), CellStatus::Calculating);

        let mut min_point: Option<usize> = None;

        for offset in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let target_point = self.try_move(point, offset);

            if target_point.is_some() {
                let target_step = self.find_target(&target_point.unwrap());

                if target_step.is_some() {
                    if min_point.is_some() {
                        min_point = Some(min_point.unwrap().min(target_step.unwrap()))
                    } else {
                        min_point = target_step;
                    }
                }
            }
        }

        min_point = match min_point {
            Some(v) => Some(v + 1),
            _ => None,
        };

        self.step_map.insert(
            point.clone(),
            match min_point {
                Some(v) => CellStatus::Some(v),
                _ => CellStatus::None,
            },
        );

        // println!("{:?} -> {:?}", &point, min_point);

        return min_point;
    }

    fn part_one(&mut self) -> Option<usize> {
        println!("{:?} -> {:?}", &self.start_point, &self.target_point);

        return self.find_target(&self.start_point.clone());

        // return self.find_target(&Point { x: 147, y: 20 })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut m = HeightMap::from_input(input);
    // println!("{:?}", m);

    let r = match m.part_one() {
        Some(u) => Some(u as u32),
        _ => None,
    };

    // println!("{:?}", m.step_map.get(&Point { x: 147, y: 20 }));

    return r;
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}

