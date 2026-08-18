use std::str::FromStr;

pub fn main() {
    let input = include_str!("../../day06_part1.txt");
    let result = part_one(input);
    println!("Part 1: {}", result.unwrap());
}

struct Datastream(String);

impl Datastream {
    const MARKER_LEN: usize = 4;

    fn start_of_packet(&self) -> Option<usize> {
        let is_unique = |window: &[char]| {
            window
                .iter()
                .enumerate()
                .all(|(i, c)| !window[i + 1..].contains(c))
        };

        self.0
            .chars()
            .collect::<Vec<_>>()
            .windows(Self::MARKER_LEN)
            .position(is_unique)
            .map(|start_idx| start_idx + Self::MARKER_LEN)
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

    datastream.start_of_packet()
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
}
