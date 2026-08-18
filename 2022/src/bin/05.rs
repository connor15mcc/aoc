use std::{ops::Sub, str::FromStr};

pub fn main() {
    let input = include_str!("../../day05_part1.txt");
    let result = part_one(input);
    println!("Part 1: {}", result.unwrap());
}

#[derive(Debug)]
struct Drawing(Vec<Vec<Crate>>);

impl Drawing {
    fn rearrange(&mut self, rearrangement: Rearrangement) {
        let Rearrangement { quantity, from, to } = rearrangement;

        for _ in 0..quantity {
            let c = self.0[from]
                .pop()
                .expect("stack must be non-empty to remove a crate");
            self.0[to].push(c);
        }
    }

    fn tops(&self) -> String {
        let mut tops = String::new();
        for stack in &self.0 {
            tops.push(stack.last().expect("all columns have (>=)1 crate on top").0);
        }
        tops
    }
}

impl FromStr for Drawing {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let crate_rows: Vec<Vec<Option<Crate>>> = s
            .lines()
            // parse only the crate lines
            .take_while(|line| line.contains('['))
            .map(|line| {
                line.as_bytes()
                    // each crate is 3 char wide (`[x]`), plus a gap
                    .chunks(4)
                    .map(|column| {
                        column
                            .get(1)
                            .copied()
                            // is a `Crate` iff the middle char is non-whitespace
                            .filter(|b| !b.is_ascii_whitespace())
                            .map(|b| Crate(b as char))
                    })
                    .collect()
            })
            .collect();

        let stacks = {
            let column_count = crate_rows.first().map_or(0, Vec::len);
            let mut stacks = vec![Vec::new(); column_count];

            for row in crate_rows.into_iter().rev() {
                for (stack, c) in stacks.iter_mut().zip(row) {
                    if let Some(c) = c {
                        stack.push(c);
                    }
                }
            }

            stacks.retain(|stack| !stack.is_empty());
            stacks
        };

        Ok(Self(stacks))
    }
}

#[derive(Debug, Clone)]
struct Crate(char);

#[derive(Debug)]
struct Rearrangement {
    quantity: usize,
    from: usize,
    to: usize,
}

impl FromStr for Rearrangement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.trim().split_whitespace().collect();

        match parts.as_slice() {
            ["move", quantity, "from", from, "to", to] => Ok(Self {
                quantity: quantity
                    .parse::<usize>()
                    .map_err(|_| format!("invalid quantity: {quantity}"))?,
                from: from
                    .parse::<usize>()
                    .map_err(|_| format!("invalid source: {from}"))?
                    .sub(1),
                to: to
                    .parse::<usize>()
                    .map_err(|_| format!("invalid destination: {to}"))?
                    .sub(1),
            }),
            _ => Err(format!("invalid rearrangement: {s:?}")),
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut lines = input.lines();
    let mut lines = lines.by_ref().skip_while(|line| line.trim().is_empty());

    let drawing = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    let mut drawing = Drawing::from_str(&drawing).expect("proper crate drawing");

    let rearrangements: Vec<_> = lines
        .take_while(|line| !line.trim().is_empty())
        .map(|line| Rearrangement::from_str(line.trim()).expect("crate rearrangement"))
        .collect();

    for rearrangement in rearrangements {
        drawing.rearrange(rearrangement);
    }

    Some(drawing.tops())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"
                [D]    
            [N] [C]    
            [Z] [M] [P]
             1   2   3 

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
        "#;
        let result = part_one(input);
        assert_eq!(result, Some("CMZ".to_string()));
    }
}
