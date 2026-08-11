use std::str::FromStr;

pub fn main() {
    let input = include_str!("../../day03_part1.txt");
    let result = part_one(input);
    println!("Part 1: {}", result.unwrap());

    let input = include_str!("../../day03_part2.txt");
    let result = part_two(input);
    println!("Part 2: {}", result.unwrap());
}

#[derive(Debug)]
struct Rucksack {
    items: u64,
    compartment_priority_masks: (u64, u64),
}

impl Rucksack {
    fn shared_priority(self) -> u32 {
        let shared_mask = self.compartment_priority_masks.0 & self.compartment_priority_masks.1;
        shared_mask.trailing_zeros() + 1
    }

    fn badge(group: &[Self]) -> Result<Item, String> {
        let [first, second, third] = group else {
            return Err(format!(
                "for safety, Elves must be in groups of 3 (not {})",
                group.len()
            ));
        };

        let badge = (first.items) & (second.items) & (third.items);
        let count = badge.count_ones();
        if count != 1 {
            return Err(format!(
                "Elven groups must share exactly one badge, found {}",
                count
            ));
        }

        Item::from_priority(badge.trailing_zeros() + 1)
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
        let left = to_mask(left);
        let right = to_mask(right);

        Ok(Self {
            items: left | right,
            compartment_priority_masks: (left, right),
        })
    }
}

struct Item(char);

impl Item {
    fn from_priority(priority: u32) -> Result<Self, String> {
        match priority {
            1..=26 => Ok(Self(char::from_u32('a' as u32 + priority - 1).unwrap())),
            27..=52 => Ok(Self(char::from_u32('A' as u32 + priority - 27).unwrap())),
            _ => Err(format!("invalid item priority: {priority}")),
        }
    }

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

pub fn part_two(input: &str) -> Option<u32> {
    let rucksacks: Vec<_> = input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim();
            Rucksack::from_str(line).expect("valid rucksacks on each line")
        })
        .collect();

    let sum = rucksacks
        .chunks_exact(3)
        .map(|group| {
            let badge = Rucksack::badge(group).expect("groups share a badge");
            badge.priority()
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

    #[test]
    fn test_part_two() {
        let input = r#"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "#;
        let result = part_two(input);
        assert_eq!(result, Some(70));
    }
}
