pub fn main() {
    let input = include_str!("../../day02_part1.txt");
    let result = part_one(input);
    println!("Part 1: {}", result.unwrap());

    let input = include_str!("../../day02_part2.txt");
    let result = part_two(input);
    println!("Part 2: {}", result.unwrap());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(self, other: Self) -> u32 {
        let our_selection = match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        let outcome = match self.against(other) {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        };

        our_selection + outcome
    }

    fn against(self, other: Self) -> Outcome {
        match (self, other) {
            (Move::Rock, Move::Scissors)
            | (Move::Paper, Move::Rock)
            | (Move::Scissors, Move::Paper) => Outcome::Win,

            (a, b) if a == b => Outcome::Draw,

            _ => Outcome::Lose,
        }
    }

    fn infer(outcome: Outcome, other: Self) -> Self {
        match (outcome, other) {
            (Outcome::Draw, other) => other,

            (Outcome::Win, Move::Rock) => Move::Paper,
            (Outcome::Win, Move::Paper) => Move::Scissors,
            (Outcome::Win, Move::Scissors) => Move::Rock,

            (Outcome::Lose, Move::Rock) => Move::Scissors,
            (Outcome::Lose, Move::Paper) => Move::Rock,
            (Outcome::Lose, Move::Scissors) => Move::Paper,
        }
    }
}

impl TryFrom<char> for Move {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let movement = match value {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            v => return Err(format!("unexpected move: {}", v)),
        };
        Ok(movement)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl TryFrom<char> for Outcome {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let outcome = match value {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            v => return Err(format!("unexpected outcome: {}", v)),
        };
        Ok(outcome)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_score = 0;
    for line in input.trim().lines() {
        let line = line.trim();
        let (theirs, ours) = line.split_once(' ').expect("rounds have exactly one space");

        let theirs = theirs.chars().next().expect("one char for theirs");
        let ours = ours.chars().next().expect("one char for ours");

        let theirs = Move::try_from(theirs).expect("they must make a valid move");
        let ours = Move::try_from(ours).expect("we must make a valid move");

        total_score += ours.score(theirs);
    }

    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_score = 0;
    for line in input.trim().lines() {
        let line = line.trim();
        let (theirs, outcome) = line.split_once(' ').expect("rounds have exactly one space");

        let theirs = theirs.chars().next().expect("one char for theirs");
        let outcome = outcome.chars().next().expect("one char for outcome");

        let theirs = Move::try_from(theirs).expect("they must make a valid move");
        let outcome = Outcome::try_from(outcome).expect("a provided valid outcome");
        let ours = Move::infer(outcome, theirs);

        total_score += ours.score(theirs);
    }

    Some(total_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"
            A Y
            B X
            C Z
        "#;
        let result = part_one(input);
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = r#"
            A Y
            B X
            C Z
        "#;
        let result = part_two(input);
        assert_eq!(result, Some(12));
    }
}
