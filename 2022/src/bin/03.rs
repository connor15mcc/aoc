use std::str::FromStr;

pub fn main() {
    let input = include_str!("../../day03_part1.txt");
    let result = part_one(input);
    println!("Part 1: {}", result.unwrap());
}

#[derive(Debug)]
struct Rucksack {
    compartment_priority_masks: (u64, u64),
}

impl Rucksack {
    fn shared_priority(self) -> u32 {
        let shared_mask = self.compartment_priority_masks.0 & self.compartment_priority_masks.1;
        shared_mask.trailing_zeros() + 1
    }
}

impl FromStr for Rucksack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_at(s.len() / 2);

        let to_mask = |compartment: &str| {
            compartment.chars().fold(0_u64, |mask, c| {
                let bit = Item(c).priority() - 1;
                mask | (1_u64 << bit)
            })
        };

        Ok(Self {
            compartment_priority_masks: (to_mask(left), to_mask(right)),
        })
    }
}

struct Item(char);

impl Item {
    fn priority(self) -> u32 {
        match self.0 {
            c @ 'a'..='z' => c as u32 - 'a' as u32 + 1,
            c @ 'A'..='Z' => c as u32 - 'A' as u32 + 27,
            c => panic!("item must be an ASCII letter, got {}", c),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim();
            let rucksack = Rucksack::from_str(line).expect("valid rucksacks on each line");
            rucksack.shared_priority()
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "#;
        let result = part_one(input);
        assert_eq!(result, Some(157));
    }
}
