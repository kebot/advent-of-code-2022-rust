/*
The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.

The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.

They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:

    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.

Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

[D]
[N] [C]
[Z] [M] [P]
 1   2   3
In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:

        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3
Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:

        [Z]
        [N]
[M]     [D]
[C]     [P]
 1   2   3
Finally, one crate is moved from stack 1 to stack 2:

        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3
The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.

After the rearrangement procedure completes, what crate ends up on top of each stack?
*/

struct Stacks {
    val: Vec<Vec<char>>,
}

impl Stacks {
    fn new() -> Self {
        Stacks { val: Vec::new() }
    }

    fn add_line(&mut self, line: &str) {
        // 3char<space>3char<space>
        if line.contains('[') {
            let mut chars = line.chars();
            let mut i: usize = 0;

            loop {
                if let (Some(_a), Some(b), Some(_c)) = (chars.next(), chars.next(), chars.next()) {
                    // println!("push stack {} -> {}", i + 1, b);

                    while self.val.len() < (i + 1) {
                        self.val.push(Vec::new())
                    }

                    if b != ' ' {
                        self.val[i].push(b);
                    }

                    let space = chars.next();
                    if space.is_none() {
                        break;
                    }
                    i = i + 1
                } else {
                    break;
                }
            }
        }
    }

    fn reverse_all(&mut self) {
        // https://stackoverflow.com/questions/39622783/how-can-i-do-a-mutable-borrow-in-a-for-loop
        // by default self.val is immutable
        for stack in self.val.iter_mut() {
            stack.reverse();
        }
    }

    fn command(&mut self, line: &str) {
        let commands: Vec<&str> = line.split(' ').collect::<Vec<&str>>();

        let amount: usize = commands[1].parse().unwrap();
        let from: usize = commands[3].parse().unwrap();
        let to: usize = commands[5].parse().unwrap();

        println!("move {} elements from {} to {}", amount, from, to);

        for _i in 0..amount {
            let a = self.val[from - 1].pop();

            if a.is_some() {
                self.val[to - 1].push(a.unwrap());
            }
        }
    }

    fn command2(&mut self, line: &str) {
        let commands: Vec<&str> = line.split(' ').collect::<Vec<&str>>();

        let amount: usize = commands[1].parse().unwrap();
        let from: usize = commands[3].parse().unwrap();
        let to: usize = commands[5].parse().unwrap();

        // println!("move {} elements from {} to {}", amount, from, to);

        let count = self.val[from - 1].len();

        let a = count - amount;

        let moving_part: Vec<char> = self.val[from - 1]
            .drain(a..)
            .collect();

        for part in moving_part {
            self.val[to - 1].push(part);
        }

        println!("move {} elements from {} to {}", amount, from, to);

        // for _i in 0..amount {
        //     let a = self.val[from - 1].pop();

        //     if a.is_some() {
        //         self.val[to - 1].push(a.unwrap());
        //     }
        // }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut input_mode_build_stack: bool = true;
    let mut stacks = Stacks::new();

    for line in input.split('\n') {
        if input_mode_build_stack {
            stacks.add_line(line);
        } else {
            stacks.command(line);
            println!("{:?}", stacks.val);
        }

        if line.is_empty() {
            stacks.reverse_all();
            input_mode_build_stack = false;
        }
    }
    let chars: String = stacks
        .val
        .iter()
        .map(|stack| stack[stack.len() - 1])
        .collect();
    Some(chars)
}

/*
--- Part Two ---
As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.

Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.

The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.

Again considering the example above, the crates begin in the same configuration:

    [D]
[N] [C]
[Z] [M] [P]
 1   2   3
Moving a single crate from stack 2 to stack 1 behaves the same as before:

[D]
[N] [C]
[Z] [M] [P]
 1   2   3
However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:

        [D]
        [N]
    [C] [Z]
    [M] [P]
 1   2   3
Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:

        [D]
        [N]
[C]     [Z]
[M]     [P]
 1   2   3
Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:

        [D]
        [N]
        [Z]
[M] [C] [P]
 1   2   3
In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.

Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies. After the rearrangement procedure completes, what crate ends up on top of each stack?
*/

pub fn part_two(input: &str) -> Option<String> {
    let mut input_mode_build_stack: bool = true;
    let mut stacks = Stacks::new();

    for line in input.split('\n') {
        if input_mode_build_stack {
            stacks.add_line(line);
        } else {
            stacks.command2(line);
            println!("{:?}", &stacks.val);
        }

        if line.is_empty() {
            stacks.reverse_all();
            input_mode_build_stack = false;
        }
    }
    let chars: String = stacks
        .val
        .iter()
        .map(|stack| stack[stack.len() - 1])
        .collect();
    Some(chars)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
