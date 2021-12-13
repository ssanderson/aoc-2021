/// --- Day 4: Giant Squid ---
///
/// You're already almost 1.5km (almost a mile) below the surface of the ocean,
/// already so deep that you can't see any sunlight. What you can see, however,
/// is a giant squid that has attached itself to the outside of your submarine.
///
/// Maybe it wants to play bingo?
///
/// Bingo is played on a set of boards each consisting of a 5x5 grid of
/// numbers. Numbers are chosen at random, and the chosen number is marked on
/// all boards on which it appears. (Numbers may not appear on all boards.) If
/// all numbers in any row or any column of a board are marked, that board
/// wins. (Diagonals don't count.)
///
/// The submarine has a bingo subsystem to help passengers (currently, you and
/// the giant squid) pass the time. It automatically generates a random order
/// in which to draw numbers and a random set of boards (your puzzle
/// input). For example:
///
/// 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
///
/// 22 13 17 11  0
///  8  2 23  4 24
/// 21  9 14 16  7
///  6 10  3 18  5
///  1 12 20 15 19
///
///  3 15  0  2 22
///  9 18 13 17  5
/// 19  8  7 25 23
/// 20 11 10 24  4
/// 14 21 16 12  6
///
/// 14 21 17 24  4
/// 10 16 15  9 19
/// 18  8 23 26 20
/// 22 11 13  6  5
///  2  0 12  3  7
///
/// After the first five numbers are drawn (7, 4, 9, 5, and 11), there are no
/// winners, but the boards are marked as follows (shown here adjacent to each
/// other to save space):
///
/// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
///  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
/// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
///  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
///  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
///
/// After the next six numbers are drawn (17, 23, 2, 0, 14, and 21), there are
/// still no winners:
///
/// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
///  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
/// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
///  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
///  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
///
/// Finally, 24 is drawn:
///
/// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
///  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
/// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
///  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
///  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
///
/// At this point, the third board wins because it has at least one complete
/// row or column of marked numbers (in this case, the entire top row is
/// marked: 14 21 17 24 4).
///
/// The score of the winning board can now be calculated. Start by finding the
/// sum of all unmarked numbers on that board; in this case, the sum is
/// 188. Then, multiply that sum by the number that was just called when the
/// board won, 24, to get the final score, 188 * 24 = 4512.
///
/// To guarantee victory against the giant squid, figure out which board will
/// win first. What will your final score be if you choose that board?
use std::str::FromStr;

use structopt::StructOpt;
use thiserror::Error;

use crate::utils::ProblemResult;

#[derive(StructOpt, Debug)]
pub struct P4 {
    #[structopt(
        short = "i",
        long = "input",
        default_value = "inputs/problem4/input.txt"
    )]
    input: String,
}

impl P4 {
    pub fn run(&self) -> ProblemResult {
        let input = std::fs::read_to_string(&self.input)?;
        let (p1, p2) = run_problem(&input)?;

        println!("Part 1: {}", p1);
        println!("Part 2: {}", p2);

        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
struct BoardCell {
    num: u8,
    drawn: bool,
}

#[derive(Clone, Debug)]
struct Board {
    /// Indexed as row, col.
    state: [[BoardCell; 5]; 5],
}

struct Bingo {
    turn: usize,
    draw: u8,
}

#[derive(Error, Debug)]
#[error("No Bingo")]
pub struct NoBingo {}

impl Board {
    fn new(nums: [[u8; 5]; 5]) -> Board {
        let init_cell = BoardCell {
            num: 0,
            drawn: false,
        };
        let mut state = [[init_cell; 5]; 5];

        for row in 0..5 {
            for col in 0..5 {
                state[row][col] = BoardCell {
                    num: nums[row][col],
                    drawn: false,
                };
            }
        }
        Board { state }
    }

    fn iter_cells(&self) -> impl Iterator<Item = &BoardCell> {
        self.state.iter().map(|row| row.iter()).flatten()
    }

    fn simulate(mut self, draws: &Vec<u8>) -> Result<(Board, Bingo), NoBingo> {
        for (turn, &draw) in draws.iter().enumerate() {
            if self.apply_draw(draw) {
                return Ok((self, Bingo { turn, draw }));
            }
        }

        Err(NoBingo {})
    }

    fn score(&self, bingo: &Bingo) -> u64 {
        let unmarked_cell_total: u64 = self
            .iter_cells()
            .filter_map(|cell| {
                if cell.drawn {
                    None
                } else {
                    Some(cell.num as u64)
                }
            })
            .sum();
        unmarked_cell_total * (bingo.draw as u64)
    }

    fn apply_draw(&mut self, num: u8) -> bool {
        let mut found_loc: Option<(usize, usize)> = None;

        'outer: for row in 0..5 {
            for col in 0..5 {
                let cell = &mut self.state[row][col];
                if cell.num == num {
                    cell.drawn = true;
                    found_loc = Some((row, col));
                    break 'outer;
                }
            }
        }

        match found_loc {
            Some((row, col)) => {
                let row_bingo = (0..5).map(|i| self.state[row][i].drawn).all(|x| x);
                let col_bingo = (0..5).map(|i| self.state[i][col].drawn).all(|x| x);
                row_bingo || col_bingo
            }
            None => false,
        }
    }
}

fn run_problem(input: &str) -> ProblemResult<(u64, u64)> {
    let parts: Vec<String> = input.split("\n\n").map(|s| s.to_owned()).collect();
    let first_line: &str = parts.first().ok_or(ParseProblemError::EmptyInput)?;

    let draws: Vec<u8> = first_line
        .split(",")
        .map(|n| n.parse())
        .collect::<Result<_, _>>()
        .map_err(|e| ParseProblemError::DrawNum(e))?;

    let boards: Vec<Board> = parts[1..]
        .iter()
        .enumerate()
        .map(|(i, s)| {
            Board::from_str(s).map_err(|e| ParseProblemError::BoardParse { n: i, source: e })
        })
        .collect::<Result<_, _>>()?;

    let boards = simulate_all(boards, &draws)?;
    let (winner, winner_bingo) = &boards[0];
    let (loser, loser_bingo) = &boards[boards.len() - 1];

    Ok((winner.score(&winner_bingo), loser.score(&loser_bingo)))
}

fn simulate_all(boards: Vec<Board>, draws: &Vec<u8>) -> ProblemResult<Vec<(Board, Bingo)>> {
    let mut results: Vec<(Board, Bingo)> = boards
        .into_iter()
        .map(|board| board.simulate(draws))
        .collect::<Result<_, _>>()?;

    results.sort_by_key(|(_, bingo)| bingo.turn);

    Ok(results)
}

#[derive(Error, Debug)]
pub enum ParseProblemError {
    #[error("Empty input")]
    EmptyInput,

    #[error("Failed to parse number draw.")]
    DrawNum(std::num::ParseIntError),

    #[error("Failed to parse board {n}.")]
    BoardParse { n: usize, source: ParseBoardError },
}

#[derive(Error, Debug)]
pub enum ParseBoardError {
    #[error("Expected 5 rows, got {0}")]
    BadRowCount(usize),

    #[error("Expected row {row} to contain 5 entries, got {len}")]
    BadRowSize { row: usize, len: usize },

    #[error("Failed to parse number: line={row} row={col}")]
    ParseInt {
        row: usize,
        col: usize,
        source: std::num::ParseIntError,
    },
}

impl FromStr for Board {
    type Err = ParseBoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        if lines.len() != 5 {
            return Err(ParseBoardError::BadRowCount(lines.len()));
        }

        let mut nums: [[u8; 5]; 5] = [[0; 5]; 5];

        for (row, line) in lines.iter().enumerate() {
            let parts: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
            if parts.len() != 5 {
                return Err(ParseBoardError::BadRowSize {
                    row,
                    len: parts.len(),
                });
            }

            for (col, num) in parts.iter().enumerate() {
                nums[row][col] = match u8::from_str(num) {
                    Ok(n) => n,
                    Err(source) => return Err(ParseBoardError::ParseInt { row, col, source }),
                }
            }
        }
        Ok(Board::new(nums))
    }
}

#[cfg(test)]
mod tests {
    use super::{run_problem, ProblemResult};
    #[test]
    fn test_example() -> ProblemResult<()> {
        let s = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;
        let (p1, _p2) = run_problem(&s)?;

        assert_eq!(p1, 4512);

        Ok(())
    }
}
