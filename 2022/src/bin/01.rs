pub fn main() {
    let input = include_str!("../../day01_part1.txt");
    let result = part_one(input);
    println!("{}", result.unwrap())
}

pub fn part_one(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"
        "#;
        let result = part_one(input);
        assert_eq!(result, None);
    }
}
