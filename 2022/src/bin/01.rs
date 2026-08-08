pub fn main() {
    let input = include_str!("../../day01_part1.txt");
    let result = part_one(input);
    println!("{}", result.unwrap())
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
}
