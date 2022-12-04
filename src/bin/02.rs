/**
--- Day 2: Rock Paper Scissors ---
The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z
This strategy guide predicts and recommends the following:

In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?
*/

/**
 * rules:
{ A => rock, B => paper, C => Scissors }
{ X => rock, Y => paper, Z => Scissors }
{ Rock => 1, Paper => 2, Scissors => 3 }
{ los => 0, draw => 3, won => 6 }
 */
#[derive(PartialEq, Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Debug)]
enum Result {
    Win,
    Lose,
    Equal,
}

impl Result {
    fn reversed(&self) -> Result {
        if self == &Result::Win {
            Result::Lose
        } else if self == &Result::Lose {
            Result::Win
        } else {
            Result::Equal
        }
    }
}

struct Rule {
    opt: Shape,
    wins: Shape,
    lose: Shape,
}

impl Rule {
    fn from_xyz(c: char) -> Rule {
        if c == 'X' {
            Rule::rock()
        } else if c == 'Y' {
            Rule::paper()
        } else {
            Rule::scissors()
        }
    }

    fn from_abc(c: char) -> Rule {
        if c == 'A' {
            Rule::rock()
        } else if c == 'B' {
            Rule::paper()
        } else {
            Rule::scissors()
        }
    }

    fn rock() -> Rule {
        Rule {
            opt: Shape::Rock,
            wins: Shape::Scissors,
            lose: Shape::Paper,
        }
    }

    fn paper() -> Rule {
        Rule {
            opt: Shape::Paper,
            wins: Shape::Rock,
            lose: Shape::Scissors,
        }
    }

    fn scissors() -> Rule {
        Rule {
            opt: Shape::Scissors,
            wins: Shape::Paper,
            lose: Shape::Rock,
        }
    }

    fn get_result(&self, opponent: &Rule) -> Result {
        if self.opt == opponent.opt {
            Result::Equal
        } else if self.wins == opponent.opt {
            Result::Win
        } else {
            Result::Lose
        }
    }

    fn to_get_result(&self, result: &Result) -> Shape {
        match result {
            Result::Equal => self.opt,
            Result::Win => self.lose,
            _ => self.wins,
        }
    }
}

fn base_score(o: Shape) -> u32 {
    if o == Shape::Rock {
        1
    } else if o == Shape::Paper {
        2
    } else {
        3
    }
}

fn match_score(r: Result) -> u32 {
    match r {
        Result::Win => 6,
        Result::Equal => 3,
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score = 0;

    for s in input.split('\n') {
        let opponent = Rule::from_abc(s.chars().nth(0).unwrap()); // X, Y, Z
        let me = Rule::from_xyz(s.chars().nth(2).unwrap()); // A, B, C
        score += match_score(me.get_result(&opponent));
        score += base_score(me.opt);
    }

    Some(score)
}

/*
--- Part Two ---
The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?
*/

fn result_from_xyz(c: char) -> Result {
    match c {
        'X' => Result::Lose,
        'Y' => Result::Equal,
        _ => Result::Win,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score = 0;

    for s in input.split('\n') {
        let opponent = Rule::from_abc(s.chars().nth(0).unwrap());
        let result = result_from_xyz(s.chars().nth(2).unwrap());
        let my_shape = opponent.to_get_result(&result);

        println!(
            "{:?} vs {:?} => {:?}",
            &my_shape,
            &opponent.opt,
            &result
        );

        score += match_score(result);
        score += base_score(my_shape);
    }

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(11906));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(11186));
    }
}
