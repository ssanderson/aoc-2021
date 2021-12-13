/// --- Day 5: Hydrothermal Venture ---
///
/// You come across a field of hydrothermal vents on the ocean floor! These
/// vents constantly produce large, opaque clouds, so it would be best to avoid
/// them if possible.
///
/// They tend to form in lines; the submarine helpfully produces a list of
/// nearby lines of vents (your puzzle input) for you to review. For example:
///
/// 0,9 -> 5,9
/// 8,0 -> 0,8
/// 9,4 -> 3,4
/// 2,2 -> 2,1
/// 7,0 -> 7,4
/// 6,4 -> 2,0
/// 0,9 -> 2,9
/// 3,4 -> 1,4
/// 0,0 -> 8,8
/// 5,5 -> 8,2
///
/// Each line of vents is given as a line segment in the format x1,y1 -> x2,y2
/// where x1,y1 are the coordinates of one end the line segment and x2,y2 are
/// the coordinates of the other end. These line segments include the points at
/// both ends. In other words:
///
/// An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
/// An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
///
/// For now, only consider horizontal and vertical lines: lines where either x1
/// = x2 or y1 = y2.
///
/// So, the horizontal and vertical lines from the above list would produce the
/// following diagram:
///
/// .......1..
/// ..1....1..
/// ..1....1..
/// .......1..
/// .112111211
/// ..........
/// ..........
/// ..........
/// ..........
/// 222111....
///
/// In this diagram, the top left corner is 0,0 and the bottom right corner is
/// 9,9. Each position is shown as the number of lines which cover that point
/// or . if no line covers that point. The top-left pair of 1s, for example,
/// comes from 2,2 -> 2,1; the very bottom row is formed by the overlapping
/// lines 0,9 -> 5,9 and 0,9 -> 2,9.
///
/// To avoid the most dangerous areas, you need to determine the number of
/// points where at least two lines overlap. In the above example, this is
/// anywhere in the diagram with a 2 or larger - a total of 5 points.
///
/// Consider only horizontal and vertical lines. At how many points do at least
/// two lines overlap?
///
/// --- Part Two ---
///
/// Unfortunately, considering only horizontal and vertical lines doesn't give
/// you the full picture; you need to also consider diagonal lines.
///
/// Because of the limits of the hydrothermal vent mapping system, the lines in
/// your list will only ever be horizontal, vertical, or a diagonal line at
/// exactly 45 degrees. In other words:
///
/// An entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
/// An entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.
///
/// Considering all lines from the above example would now produce the
/// following diagram:
///
/// 1.1....11.
/// .111...2..
/// ..2.1.111.
/// ...1.2.2..
/// .112313211
/// ...1.2....
/// ..1...1...
/// .1.....1..
/// 1.......1.
/// 222111....
///
/// You still need to determine the number of points where at least two lines
/// overlap. In the above example, this is still anywhere in the diagram with a
/// 2 or larger - now a total of 12 points.
///
/// Consider all of the lines. At how many points do at least two lines
/// overlap?
use std::str::FromStr;

use structopt::StructOpt;
use thiserror::Error;

use crate::utils::{parse_lines_from_path, ProblemResult};

#[derive(StructOpt, Debug)]
pub struct P5 {
    #[structopt(
        short = "i",
        long = "input",
        default_value = "inputs/problem5/input.txt"
    )]
    input: String,
}

impl P5 {
    pub fn run(&self) -> ProblemResult {
        let lines: Vec<Line> = parse_lines_from_path(&self.input)?;
        let (p1, p2) = run_problem(&lines)?;

        println!("Part 1: {}", p1);
        println!("Part 2: {}", p2);

        Ok(())
    }
}

fn run_problem(lines: &Vec<Line>) -> ProblemResult<(usize, usize)> {
    let rect_lines: Vec<_> = lines
        .iter()
        .filter(|line| line.is_horizontal_or_vertical())
        .cloned()
        .collect();
    let p1 = count_overlaps(&rect_lines);
    let p2 = count_overlaps(lines);

    Ok((p1, p2))
}

fn count_overlaps(lines: &Vec<Line>) -> usize {
    let mut locs = std::collections::HashMap::<(i16, i16), u64>::new();
    for line in lines {
        for point in line.iter_points() {
            let entry = locs.entry(point).or_insert(0);
            *entry += 1;
        }
    }
    locs.iter().filter(|&(_, &v)| v > 1).count()
}

#[derive(Debug, Clone, Copy)]
struct Line {
    x1: i16,
    y1: i16,
    x2: i16,
    y2: i16,
}

impl Line {
    fn is_horizontal_or_vertical(&self) -> bool {
        (self.x1 == self.x2) || (self.y1 == self.y2)
    }

    fn iter_points(&self) -> impl Iterator<Item = (i16, i16)> {
        let dx: i16 = match self.x1.cmp(&self.x2) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };

        let dy: i16 = match self.y1.cmp(&self.y2) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };

        let count = std::cmp::max((self.x1 - self.x2).abs(), (self.y1 - self.y2).abs()) + 1;

        let x1 = self.x1;
        let y1 = self.y1;

        (0..count).map(move |dist| {
            (x1 + dist * dx, y1 + dist * dy)
        })
    }
}

#[derive(Error, Debug)]
pub enum LineParseError {
    #[error("Invalid line format: {0}")]
    InvalidFormat(String),

    #[error("Failed to parse int pair: {0}")]
    InvalidIntPair(String),

    #[error("Failed to parse coordinate")]
    InvalidInt(#[from] std::num::ParseIntError),
}

impl FromStr for Line {
    type Err = LineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (coord1, coord2) =
            split_exactly_once(s, " -> ").ok_or(LineParseError::InvalidFormat(s.to_owned()))?;

        let (x1, y1) = parse_pair::<i16>(coord1)?;
        let (x2, y2) = parse_pair::<i16>(coord2)?;

        Ok(Line { x1, y1, x2, y2 })
    }
}

fn parse_pair<T: FromStr>(s: &str) -> Result<(T, T), LineParseError>
where
    LineParseError: std::convert::From<<T as std::str::FromStr>::Err>,
{
    let (a, b) = split_exactly_once(s, ",").ok_or(LineParseError::InvalidIntPair(s.to_owned()))?;
    let a = a.parse::<T>()?;
    let b = b.parse::<T>()?;

    Ok((a, b))
}

fn split_exactly_once<'a>(s: &'a str, pat: &str) -> Option<(&'a str, &'a str)> {
    let mut parts = s.split(pat);
    match (parts.next(), parts.next()) {
        (Some(p1), Some(p2)) => Some((p1, p2)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{run_problem, Line, ProblemResult};
    use crate::utils::parse_lines;

    #[test]
    fn test_example() -> ProblemResult<()> {
        let s = br#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;
        let lines: Vec<Line> = parse_lines(&s[..])?;
        let (p1, p2) = run_problem(&lines)?;

        assert_eq!(p1, 5);
        assert_eq!(p2, 12);

        Ok(())
    }
}
