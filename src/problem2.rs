/// --- Day 2: Dive! ---
///
/// Now, you need to figure out how to pilot this thing.
///
/// It seems like the submarine can take a series of commands like forward 1,
/// down 2, or up 3:
///
/// forward X increases the horizontal position by X units.
/// down X increases the depth by X units.
/// up X decreases the depth by X units.
///
/// Note that since you're on a submarine, down and up affect your depth, and
/// so they have the opposite result of what you might expect.
///
/// The submarine seems to already have a planned course (your puzzle
/// input). You should probably figure out where it's going. For example:
///
/// forward 5
/// down 5
/// forward 8
/// up 3
/// down 8
/// forward 2
///
/// Your horizontal position and depth both start at 0. The steps above would
/// then modify them as follows:
///
/// forward 5 adds 5 to your horizontal position, a total of 5.
/// down 5 adds 5 to your depth, resulting in a value of 5.
/// forward 8 adds 8 to your horizontal position, a total of 13.
/// up 3 decreases your depth by 3, resulting in a value of 2.
/// down 8 adds 8 to your depth, resulting in a value of 10.
/// forward 2 adds 2 to your horizontal position, a total of 15.
///
/// After following these instructions, you would have a horizontal position of
/// 15 and a depth of 10. (Multiplying these together produces 150.)
///
/// Calculate the horizontal position and depth you would have after following
/// the planned course. What do you get if you multiply your final horizontal
/// position by your final depth?
///
/// --- Part Two ---
///
/// Based on your calculations, the planned course doesn't seem to make any
/// sense. You find the submarine manual and discover that the process is
/// actually slightly more complicated.
///
/// In addition to horizontal position and depth, you'll also need to track a
/// third value, aim, which also starts at 0. The commands also mean something
/// entirely different than you first thought:
///
/// down X increases your aim by X units.
/// up X decreases your aim by X units.
/// forward X does two things:
/// It increases your horizontal position by X units.
/// It increases your depth by your aim multiplied by X.
///
/// Again note that since you're on a submarine, down and up do the opposite of
/// what you might expect: "down" means aiming in the positive direction.
///
/// Now, the above example does something different:
///
/// forward 5 adds 5 to your horizontal position, a total of 5. Because your aim is 0, your depth does not change.
/// down 5 adds 5 to your aim, resulting in a value of 5.
/// forward 8 adds 8 to your horizontal position, a total of 13. Because your aim is 5, your depth increases by 8*5=40.
/// up 3 decreases your aim by 3, resulting in a value of 2.
/// down 8 adds 8 to your aim, resulting in a value of 10.
/// forward 2 adds 2 to your horizontal position, a total of 15. Because your aim is 10, your depth increases by 2*10=20 to a total of 60.

/// After following these new instructions, you would have a horizontal
/// position of 15 and a depth of 60. (Multiplying these produces 900.)

/// Using this new interpretation of the commands, calculate the horizontal
/// position and depth you would have after following the planned course. What
/// do you get if you multiply your final horizontal position by your final
/// depth?
use structopt::StructOpt;
use thiserror::Error;

use crate::utils::{parse_lines_from_path, ProblemResult};

#[derive(StructOpt, Debug)]
pub struct P2 {
    #[structopt(
        short = "i",
        long = "input",
        default_value = "inputs/problem2/input.txt"
    )]
    input: String,
}

impl P2 {
    pub fn run(&self) -> ProblemResult {
        let commands: Vec<Command> = parse_lines_from_path(&self.input)?;

        let (p1, p2) = run_problem(commands)?;
        println!("Part 1: {}", p1);
        println!("Part 2: {}", p2);

        Ok(())
    }
}

fn run_problem(cmds: Vec<Command>) -> ProblemResult<(u64, u64)> {
    let mut sub1 = Submarine::new();
    sub1.apply_all_part1(cmds.iter().cloned());
    let part1 = sub1.depth * sub1.horizontal_pos;

    let mut sub2 = Submarine::new();
    sub2.apply_all(cmds.iter().cloned());
    let part2 = sub2.depth * sub2.horizontal_pos;

    Ok((part1, part2))
}

#[derive(Clone, Copy, Debug)]
enum Command {
    Forward(u64),
    Up(u64),
    Down(u64),
}

#[derive(Error, Debug)]
pub enum CommandParseError {
    #[error("Expected two words in command: got {0}")]
    WrongWordCount(usize),

    #[error("Failed to parse command magnitude")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("Invalid command verb: {0}")]
    InvalidVerb(String),
}

impl std::str::FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() != 2 {
            return Err(CommandParseError::WrongWordCount(parts.len()));
        }

        let verb = parts[0];
        let mag = u64::from_str(parts[1])?;

        let parsed = match verb {
            "forward" => Command::Forward(mag),
            "up" => Command::Up(mag),
            "down" => Command::Down(mag),
            _ => return Err(CommandParseError::InvalidVerb(verb.to_owned())),
        };

        Ok(parsed)
    }
}

struct Submarine {
    pub depth: u64,
    pub horizontal_pos: u64,
    pub aim: u64,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            depth: 0,
            horizontal_pos: 0,
            aim: 0,
        }
    }

    fn apply_all(&mut self, cmds: impl Iterator<Item = Command>) {
        for cmd in cmds {
            self.apply(cmd)
        }
    }

    fn apply(&mut self, cmd: Command) {
        match cmd {
            Command::Forward(x) => {
                self.horizontal_pos += x;
                self.depth += x * self.aim;
            }
            Command::Down(x) => {
                self.aim += x;
            }
            Command::Up(x) => {
                self.aim -= x;
            }
        }
    }

    fn apply_all_part1(&mut self, cmds: impl Iterator<Item = Command>) {
        for cmd in cmds {
            match cmd {
                Command::Forward(x) => {
                    self.horizontal_pos += x;
                }
                Command::Up(x) => {
                    self.depth -= x;
                }
                Command::Down(x) => {
                    self.depth += x;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{run_problem, Command, ProblemResult};
    use crate::utils::parse_lines;

    #[test]
    fn test_example() -> ProblemResult<()> {
        let s = br#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;
        let commands: Vec<Command> = parse_lines(&s[..])?;
        let (p1, p2) = run_problem(commands)?;
        assert_eq!(p1, 150);
        assert_eq!(p2, 900);

        Ok(())
    }
}
