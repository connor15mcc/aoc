pub fn main() {
    let input1 = include_str!("../../day02_part1.txt");
    let result = part_one(input1);
    println!("Part one: {}", result.unwrap());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Win,
    Draw,
    Lose,
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
}
