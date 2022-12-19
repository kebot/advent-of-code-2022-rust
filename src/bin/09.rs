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

    fn keep_up(&self, other: &Self) -> Self {
        let mut result = Position { x: 0, y: 0 };

        let diff_x = other.x - self.x;
        let diff_y = other.y - self.y;

        if diff_x.abs() > diff_y.abs() {
            result.y = other.y;
            result.x = if diff_x > 0 { other.x - 1 } else { other.x + 1 }
        } else {
            result.x = other.x;
            result.y = if diff_y > 0 { other.y - 1 } else { other.y + 1 }
        }

        result
    }
}

struct Rope {
    head: Position,
    tail: Position,
    tail_visited: HashSet<Position>,
}

impl Rope {
    fn init() -> Self {
        let mut tail_visited = HashSet::new();
        tail_visited.insert(Position { x: 0, y: 0 });

        Rope {
            head: Position { x: 0, y: 0 },
            tail: Position { x: 0, y: 0 },
            tail_visited: tail_visited,
        }
    }

    fn post_move(&mut self) {
        // calculate the position of head and tail, then try to follow
        if self.head.is_touching(&self.tail) {
            // println!("{:?} and {:?} is touched", self.head, self.tail);
            return;
        }

        let new_tail = self.tail.keep_up(&self.head);
        self.tail_visited.insert(new_tail.clone());
        self.tail = new_tail;

        println!("tail moved to {:?}", &self.tail);
    }

    fn move_left(&mut self, times: usize) {
        for _i in 0..times {
            self.head.x -= 1;
            self.post_move();
        }
    }

    fn move_right(&mut self, times: usize) {
        for _i in 0..times {
            self.head.x += 1;
            self.post_move();
        }
    }

    fn move_up(&mut self, times: usize) {
        for _i in 0..times {
            self.head.y += 1;
            self.post_move();
        }
    }

    fn move_down(&mut self, times: usize) {
        for _i in 0..times {
            self.head.y -= 1;
            self.post_move();
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rope = Rope::init();

    for line in input.lines() {
        let mut iter = line.split(' ');
        let command = iter.next().unwrap();
        let steps: usize = iter.next().unwrap().parse().unwrap();

        println!("{:?} {:?}", command, steps);

        match command {
            "L" => rope.move_left(steps),
            "R" => rope.move_right(steps),
            "U" => rope.move_up(steps),
            "D" => rope.move_down(steps),
            _ => println!("Unknown command: {:?}", command),
        }
    }

    Some(rope.tail_visited.len() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
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
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
