use std::collections::{HashSet,VecDeque};
use std::hash::Hash;

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut last_four: VecDeque<char> = VecDeque::new();

    for (i, ch) in input.chars().enumerate() {
        if last_four.len() >= 4 {
            last_four.pop_front();
        }

        last_four.push_back(ch);

        println!("{:?}", last_four);

        if last_four.len() == 4 && has_unique_elements(&last_four) {
            return Some(i as u32 + 1)
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut last_14: VecDeque<char> = VecDeque::new();

    for (i, ch) in input.chars().enumerate() {
        if last_14.len() >= 14 {
            last_14.pop_front();
        }

        last_14.push_back(ch);

        println!("{:?}", last_14);

        if last_14.len() == 14 && has_unique_elements(&last_14) {
            return Some(i as u32 + 1)
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
