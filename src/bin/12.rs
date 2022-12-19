// use rayon::prelude::*;
use std::{
    cmp::Ordering::Equal,
    collections::{HashMap, VecDeque},
};

type Pos = (usize, usize);
type Unvisited = HashMap<Pos, Node>;
type Trails = VecDeque<Pos>;

#[derive(Clone, Debug)]
struct Node {
    val: usize,
    pos: (usize, usize),
    dist: f32,
}

fn parse_input(input: &str) -> (Unvisited, Pos, Pos, Trails) {
    let mut unvisited = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut trails = VecDeque::new();

    for (i, line) in input.lines().enumerate() {
        for (j, mut char) in line.chars().enumerate() {
            let (pos, dist) = ((i, j), f32::INFINITY);

            if char == 'S' {
                (start, char) = (pos, 'a');
                // char = 'a'
            } else if char == 'E' {
                (end, char) = (pos, 'z');
            }

            if char == 'a' {
                trails.push_back(pos);
            }

            let val = (char as u8 - 'a' as u8) as usize;
            unvisited.insert(pos, Node { pos, dist, val });
        }
    }

    (unvisited, start, end, trails)
}

/// An implementation of Dijkstra's shortest path algorithm, how fun!
/// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Algorithm
/// 1. Mark all nodes as unvisited and create the unvisited set.
/// 2. Assign a tentative distance to each node. Zero for starting node and infinity
///    for all others.
/// 3. Calculate a tentative distance for all unvisited neighbours. Set the smaller
///    value of newly calc'ed and already set distance.
/// 4. Mark current node as visited, remove from unvisited set.
/// 5. Base case. If end node has been visited or if the smalllest distance in the
///    unvisited set is infinity, then the algorithm is finished.
/// 6. Else, select unvisited node with smallest distance and loop back to 3.
fn dijkstra(unvisited: &mut Unvisited, start: Pos, end: Pos) -> Option<f32> {
    fn update_neighbor(unvisited: &mut Unvisited, cur: &Node, pos: Pos) {
        if let Some(neigh) = unvisited.get_mut(&(pos)) {
            // Steepness check.
            if neigh.val <= cur.val + 1 {
                neigh.dist = neigh.dist.min(cur.dist + 1.);
            }
        }
    }

    let mut res = None;
    let mut next_pos = start;

    loop {
        let cur = unvisited.remove(&next_pos).unwrap();

        let pos = cur.pos;

        update_neighbor(unvisited, &cur, (pos.0, pos.1 + 1));
        update_neighbor(unvisited, &cur, (pos.0 + 1, pos.1));
        if pos.1 > 0 {
            update_neighbor(unvisited, &cur, (pos.0, pos.1 - 1));
        }
        if pos.0 > 0 {
            update_neighbor(unvisited, &cur, (pos.0 - 1, pos.1));
        }

        if pos == end {
            res = Some(cur.dist);
            break;
        }

        if unvisited
            .iter()
            .find(|(_, n)| n.dist != f32::INFINITY)
            .is_none()
        {
            break;
        }

        // Compare f32's which don't implement `Cmp`.
        // https://www.reddit.com/r/rust/comments/29kia3/comment/cilrzik/?utm_source=share&utm_medium=web2x&context=3
        if let Some(min) = unvisited
            .iter()
            .min_by(|a, b| a.1.dist.partial_cmp(&b.1.dist).unwrap_or(Equal))
            .map(|(_, n)| n.pos)
        {
            next_pos = min;
        }
    }

    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut unvisited, start, end, _trails) = parse_input(input);
    unvisited.get_mut(&start).unwrap().dist = 0.;

    match dijkstra(&mut unvisited, start, end) {
        Some(v) => Some(v as u32),
        _ => None
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

// pub fn part_two() -> Option<u32> {
//     let (unvisited, _start, end, trails) = parse_input();
//     trails
//         .into_par_iter()
//         .map(|start| {
//             let mut unvisited = unvisited.clone();
//             unvisited.get_mut(&start).unwrap().dist = 0.;
//             dijkstra(&mut unvisited, start, end)
//         })
//         .map(|o| o.unwrap_or(f32::INFINITY))
//         .min_by(|a, b| a.partial_cmp(&b).unwrap_or(Equal))
// }

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
