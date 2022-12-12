struct TreeMap {
    v: Vec<Vec<u8>>,
}

impl TreeMap {
    fn from_input(input: &str) -> Self {
        let mut map = TreeMap {
            v: vec![]
        };

        for line in input.lines() {
            let mut row: Vec<u8> = Vec::new();

            for char in line.chars() {
                let n = char.to_digit(10).unwrap().try_into().ok().unwrap();
                row.push(n);
            }

            map.v.push(row);
        }

        map
    }

    fn out_edge (&self, x: i32, y: i32) -> bool {
        let height = self.v.len() as i32;
        let width = self.v.first().unwrap().len() as i32;

        if x < 0 || y < 0 || x > (width - 1) || y > (height - 1) {
            true
        } else {
            false
        }
    }

    fn is_visible (&self, x: usize, y: usize) -> bool {
        let value1 = self.v[y][x];

        for [delta_x, delta_y] in [[0, 1], [0, -1], [1, 0], [-1, 0]] {
            let mut x1: i32 = x as i32;
            let mut y1: i32 = y as i32;

            loop {
                x1 += delta_x;
                y1 += delta_y;

                if self.out_edge(x1, y1) {
                    return true
                }

                let value2 = self.v[y1 as usize][x1 as usize];
                if value2 >= value1 {
                    break;
                }
            }
        }

        return false
    }

    fn calculate_score (&self, x: usize, y: usize) -> u32 {
        let mut output = 1;

        let value1 = self.v[y][x];

        for [delta_x, delta_y] in [[0, 1], [0, -1], [1, 0], [-1, 0]] {
            let mut x1: i32 = x as i32;
            let mut y1: i32 = y as i32;
            let mut sum = 0;

            loop {
                x1 += delta_x;
                y1 += delta_y;

                if self.out_edge(x1, y1) {
                    break
                }

                sum += 1;

                let value2 = self.v[y1 as usize][x1 as usize];
                if value2 >= value1 {
                    break;
                }
            }

            output *= sum;
        }

        return output
    }

}

pub fn part_one(input: &str) -> Option<u32> {
    let m = TreeMap::from_input(input);

    let mut visible_trees: u32 = 0;
    for (y, line) in m.v.iter().enumerate() {
        for (x, _line2) in line.iter().enumerate() {
            if m.is_visible(x, y) {
                visible_trees += 1;
            }
        }
    }

    Some(visible_trees)
}

pub fn part_two(input: &str) -> Option<u32> {
    let m = TreeMap::from_input(input);
    let mut hightest_score: u32 = 0;

    for (y, line) in m.v.iter().enumerate() {
        for (x, _line2) in line.iter().enumerate() {
            let score = m.calculate_score(x, y);
            if score > hightest_score {
                hightest_score = score
            }
        }
    }

    Some(hightest_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
