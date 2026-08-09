pub fn main() {
    let input1 = include_str!("../../day01_part1.txt");
    let result = part_one(input1);
    println!("Part one: {}", result.unwrap());

    let input2 = include_str!("../../day01_part2.txt");
    let result = part_two(input2);
    println!("Part two: {}", result.unwrap());
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut most_calories, mut current_cals) = (0, 0);
    for line in input.lines() {
        let line = line.trim();

        if line == "" {
            most_calories = most_calories.max(current_cals);
            current_cals = 0;
            continue;
        }

        let calories = line.parse::<u32>().expect("calories are ints");
        current_cals += calories;
    }

    Some(most_calories)
}

#[derive(Default)]
// one > two > three
struct CalorieTracker {
    one: u32,
    two: u32,
    three: u32,
}

impl CalorieTracker {
    fn track(&mut self, val: u32) {
        let Self { one, two, three } = self;

        // nothing to do
        if val <= *three {
            return;
        }

        if val > *one {
            // reorder all three
            *three = *two;
            *two = *one;
            *one = val;
        } else if val > *two {
            // reorder the latter two
            *three = *two;
            *two = val;
        } else if val > *three {
            // reorder the last
            *three = val;
        }
    }

    fn total(self) -> u32 {
        return self.one + self.two + self.three;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut tracker = CalorieTracker::default();
    let mut current_cals = 0;
    for line in input.lines() {
        let line = line.trim();

        if line == "" {
            tracker.track(current_cals);
            current_cals = 0;
            continue;
        }

        let calories = line.parse::<u32>().expect("calories are ints");
        current_cals += calories;
    }

    Some(tracker.total())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        "#;
        let result = part_one(input);
        assert_eq!(result, Some(24_000));
    }

    #[test]
    fn test_part_two() {
        let input = r#"
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        "#;
        let result = part_two(input);
        assert_eq!(result, Some(45_000));
    }
}
