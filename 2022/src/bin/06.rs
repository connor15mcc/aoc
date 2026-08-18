use std::str::FromStr;

pub fn main() {
    let input = include_str!("../../day06_part1.txt");
    let result = part_one(input);
    println!("Part 1: {}", result.unwrap());

    let input = include_str!("../../day06_part2.txt");
    let result = part_two(input);
    println!("Part 2: {}", result.unwrap());
}

struct Datastream(String);

struct MarkerLen(usize);

impl Datastream {
    const START_OF_PACKET: MarkerLen = MarkerLen(4);
    const START_OF_MESSAGE: MarkerLen = MarkerLen(14);

    fn start_pos(&self, MarkerLen(len): MarkerLen) -> Option<usize> {
        let is_unique = |window: &[char]| {
            window
                .iter()
                .enumerate()
                .all(|(i, c)| !window[i + 1..].contains(c))
        };

        self.0
            .chars()
            .collect::<Vec<_>>()
            .windows(len)
            .position(is_unique)
            .map(|start_idx| start_idx + len)
    }
}

impl FromStr for Datastream {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Datastream(s.to_string()))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.trim().lines();
    let datastream = Datastream::from_str(lines.next().expect("one line of a datastream"))
        .expect("valid datastream");
    assert_eq!(lines.next(), None, "datastream must be exactly one line");

    datastream.start_pos(Datastream::START_OF_PACKET)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.trim().lines();
    let datastream = Datastream::from_str(lines.next().expect("one line of a datastream"))
        .expect("valid datastream");
    assert_eq!(lines.next(), None, "datastream must be exactly one line");

    datastream.start_pos(Datastream::START_OF_MESSAGE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let cases = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (input, expected) in cases {
            assert_eq!(part_one(input), Some(expected), "input: {input}");
        }
    }

    #[test]
    fn test_part_two() {
        let cases = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (input, expected) in cases {
            assert_eq!(part_two(input), Some(expected), "input: {input}");
        }
    }
}
