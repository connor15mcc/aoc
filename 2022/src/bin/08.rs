use std::{ops::Index, str::FromStr};

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Height>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Height(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Grid {
    fn new(width: usize, height: usize, cells: Vec<Height>) -> Result<Self, String> {
        if width == 0 || height == 0 {
            return Err("grid is empty".to_string());
        }

        if width.checked_mul(height).is_none() {
            return Err("grids must be square".to_string());
        }

        if cells.len() != width * height {
            return Err("grid has the wrong number of cells".to_string());
        }

        Ok(Self {
            width,
            height,
            cells,
        })
    }

    fn positions(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| Position { x, y }))
    }

    fn is_visible(&self, position: Position) -> bool {
        Direction::ALL
            .into_iter()
            .any(|direction| self.visible_from(position, direction))
    }

    fn scenic_score(&self, position: Position) -> usize {
        Direction::ALL
            .into_iter()
            .map(|direction| self.viewing_distance(position, direction))
            .product()
    }

    fn visible_from(&self, position: Position, direction: Direction) -> bool {
        let height = self[position];

        std::iter::successors(self.step(position, direction), |&position| {
            self.step(position, direction)
        })
        .map(|position| self[position])
        .all(|other| other < height)
    }

    fn viewing_distance(&self, position: Position, direction: Direction) -> usize {
        let height = self[position];

        std::iter::successors(self.step(position, direction), |position| {
            self.step(*position, direction)
        })
        .map(|position| self[position])
        .scan(false, |blocked, other| {
            if *blocked {
                return None;
            }

            *blocked = other >= height;
            Some(other)
        })
        .count()
    }

    fn step(&self, position: Position, direction: Direction) -> Option<Position> {
        let (dx, dy) = direction.delta();

        let x = position.x.checked_add_signed(dx)?;
        let y = position.y.checked_add_signed(dy)?;

        if x < self.width && y < self.height {
            Some(Position { x, y })
        } else {
            None
        }
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return Err("no lines".to_string());
        }

        let width = lines[0].len();

        if width == 0 {
            return Err("grid has no width".to_string());
        }

        let height = lines.len();

        let mut cells = Vec::with_capacity(width * height);

        for line in &lines {
            let line = line.trim();
            if line.len() != width {
                return Err("grid has unequal width+height".to_string());
            }

            for byte in line.bytes() {
                let digit = byte.checked_sub(b'0').ok_or("invalid tree height")?;

                cells.push(Height(digit));
            }
        }

        Grid::new(width, height, cells)
    }
}

impl Index<Position> for Grid {
    type Output = Height;

    fn index(&self, position: Position) -> &Self::Output {
        let index = position.y * self.width + position.x;
        &self.cells[index]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ];

    fn delta(self) -> (isize, isize) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }
}

pub fn main() {
    let input = include_str!("../../day08_part1.txt");
    let result = part_one(input);
    println!("Part 1: {}", result.unwrap());

    let input = include_str!("../../day08_part2.txt");
    let result = part_two(input);
    println!("Part 2: {}", result.unwrap());
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input.trim()).expect("valid grid");
    let visible = grid
        .positions()
        .filter(|&position| grid.is_visible(position))
        .count();
    Some(visible as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input.trim()).expect("valid grid");
    let best_score = grid
        .positions()
        .map(|position| grid.scenic_score(position))
        .max();
    Some(best_score.expect("there must be a best treehouse location") as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"
            30373
            25512
            65332
            33549
            35390
        "#;
        let result = part_one(input);
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = r#"
            30373
            25512
            65332
            33549
            35390
        "#;
        let result = part_two(input);
        assert_eq!(result, Some(8));
    }
}
