#![feature(range_into_bounds)]
#![feature(range_bounds_is_empty)]
use std::{
    ops::{IntoBounds, RangeBounds, RangeInclusive},
    str::FromStr,
};

pub fn main() {
    let input = include_str!("../../day04_part1.txt");
    let result = part_one(input);
    println!("Part 1: {}", result.unwrap());

    let input = include_str!("../../day04_part2.txt");
    let result = part_two(input);
    println!("Part 2: {}", result.unwrap());
}

#[derive(Debug)]
struct SectionAssignment(RangeInclusive<u32>);

impl SectionAssignment {
    fn contains(&self, other: &Self) -> bool {
        self.0.contains(other.0.start()) && self.0.contains(other.0.end())
    }

    fn fully_overlaps(&self, other: &Self) -> bool {
        self.contains(other) || other.contains(self)
    }

    fn overlaps(self, other: Self) -> bool {
        !self.0.intersect(other.0).is_empty()
    }
}

impl FromStr for SectionAssignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (low, high) = s.split_once("-").expect("section IDs are hyphenated");
        let low: u32 = low.parse().expect("section ID is a number");
        let high: u32 = high.parse().expect("section ID is a number");
        return Ok(Self(low..=high));
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let assignment_pairs = input
        .trim()
        .lines()
        .filter(|line| {
            let line = line.trim();
            let (left, right) = line.split_once(",").expect("elven pairs split by `,`");
            let left = SectionAssignment::from_str(left).expect("each elf has an assignment");
            let right = SectionAssignment::from_str(right).expect("each elf has an assignment");

            left.fully_overlaps(&right)
        })
        .count();

    Some(assignment_pairs as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let assignment_pairs = input
        .trim()
        .lines()
        .filter(|line| {
            let line = line.trim();
            let (left, right) = line.split_once(",").expect("elven pairs split by `,`");
            let left = SectionAssignment::from_str(left).expect("each elf has an assignment");
            let right = SectionAssignment::from_str(right).expect("each elf has an assignment");

            left.overlaps(right)
        })
        .count();

    Some(assignment_pairs as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        "#;
        let result = part_one(input);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = r#"
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        "#;
        let result = part_two(input);
        assert_eq!(result, Some(4));
    }
}
