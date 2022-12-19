// head

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn is_touching(&self, other: &Self) -> bool {
        // in fact, the head (H) and tail (T) must always be touching
        // (diagonally adjacent and even overlapping both count as touching):
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn keep_up(&mut self, other: &Self) {
        let diff_x = other.x - self.x;
        let diff_y = other.y - self.y;

        if diff_x.abs() > diff_y.abs() {
            self.y = other.y;
            self.x = if diff_x > 0 { other.x - 1 } else { other.x + 1 };
        } else if diff_x.abs() < diff_y.abs() {
            self.x = other.x;
            self.y = if diff_y > 0 { other.y - 1 } else { other.y + 1 };
        } else {
            self.x = if diff_x > 0 { other.x - 1 } else { other.x + 1 };
            self.y = if diff_y > 0 { other.y - 1 } else { other.y + 1 };
            // println!("diff {}, {}", diff_x, diff_y);
        }
    }
}

struct Rope {
    // head: Position,
    // tail: Position,
    knots: Vec<Position>,
    tail_visited: HashSet<Position>,
}

impl Rope {
    fn init(knots_count: usize) -> Self {
        let mut tail_visited = HashSet::new();
        tail_visited.insert(Position { x: 0, y: 0 });

        let mut knots: Vec<Position> = Vec::with_capacity(knots_count);
        for _i in 0..knots_count {
            knots.push(Position { x: 0, y: 0 });
        }

        Rope {
            knots: knots,
            tail_visited: tail_visited,
        }
    }

    fn read_input(&mut self, input: &str) {
        for line in input.lines() {
            let mut iter = line.split(' ');
            let command = iter.next().unwrap();
            let steps: usize = iter.next().unwrap().parse().unwrap();

            // println!("{:?} {:?}", command, steps);

            match command {
                "L" => self.move_left(steps),
                "R" => self.move_right(steps),
                "U" => self.move_up(steps),
                "D" => self.move_down(steps),
                _ => println!("Unknown command: {:?}", command),
            }

            // println!("{:?}", self.knots);
        }
    }

    fn head(&mut self) -> &mut Position {
        self.knots.first_mut().unwrap()
    }

    fn post_move(&mut self) {
        // calculate the position of head and tail, then try to follow
        let mut n = 1;
        let len = self.knots.len();
        // println!("len is {}", len);

        loop {
            if n > (len - 1) {
                break;
            }

            let first = self.knots.get(n - 1).unwrap().clone();
            let second = self.knots.get_mut(n).unwrap();

            if !first.is_touching(second) {
                second.keep_up(&first);
                if n == (len - 1) {
                    println!("tail {:?} visit {:?}", n, second.clone());
                    self.tail_visited.insert(second.clone());
                }
            }

            n += 1;
        }
    }

    fn move_left(&mut self, times: usize) {
        for _i in 0..times {
            self.head().x -= 1;
            self.post_move();
        }
    }

    fn move_right(&mut self, times: usize) {
        for _i in 0..times {
            self.head().x += 1;
            self.post_move();
        }
    }

    fn move_up(&mut self, times: usize) {
        for _i in 0..times {
            self.head().y += 1;
            self.post_move();
        }
    }

    fn move_down(&mut self, times: usize) {
        for _i in 0..times {
            self.head().y -= 1;
            self.post_move();
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rope = Rope::init(2);
    // println!("Knots: {:?}", rope.knots);
    rope.read_input(input);
    Some(rope.tail_visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rope = Rope::init(10);
    rope.read_input(input);
    Some(rope.tail_visited.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    // 6470
    advent_of_code::solve!(1, part_one, input);

    // 2616 too low, 2658
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = String::from(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
        );

        assert_eq!(part_two(&input), Some(36));

        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
