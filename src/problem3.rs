/// --- Day 3: Binary Diagnostic ---
///
/// The submarine has been making some odd creaking noises, so you ask it to
/// produce a diagnostic report just in case.
///
/// The diagnostic report (your puzzle input) consists of a list of binary
/// numbers which, when decoded properly, can tell you many useful things about
/// the conditions of the submarine. The first parameter to check is the power
/// consumption.
///
/// You need to use the binary numbers in the diagnostic report to generate two
/// new binary numbers (called the gamma rate and the epsilon rate). The power
/// consumption can then be found by multiplying the gamma rate by the epsilon
/// rate.
///
/// Each bit in the gamma rate can be determined by finding the most common bit
/// in the corresponding position of all numbers in the diagnostic report. For
/// example, given the following diagnostic report:
///
/// 00100
/// 11110
/// 10110
/// 10111
/// 10101
/// 01111
/// 00111
/// 11100
/// 10000
/// 11001
/// 00010
/// 01010
///
/// Considering only the first bit of each number, there are five 0 bits and
/// seven 1 bits. Since the most common bit is 1, the first bit of the gamma
/// rate is 1.
///
/// The most common second bit of the numbers in the diagnostic report is 0, so
/// the second bit of the gamma rate is 0.
///
/// The most common value of the third, fourth, and fifth bits are 1, 1, and 0,
/// respectively, and so the final three bits of the gamma rate are 110.
///
/// So, the gamma rate is the binary number 10110, or 22 in decimal.
///
/// The epsilon rate is calculated in a similar way; rather than use the most
/// common bit, the least common bit from each position is used. So, the
/// epsilon rate is 01001, or 9 in decimal. Multiplying the gamma rate (22) by
/// the epsilon rate (9) produces the power consumption, 198.
///
/// Use the binary numbers in your diagnostic report to calculate the gamma
/// rate and epsilon rate, then multiply them together. What is the power
/// consumption of the submarine? (Be sure to represent your answer in decimal,
/// not binary.)
///
/// --- Part Two ---
///
/// Next, you should verify the life support rating, which can be determined by
/// multiplying the oxygen generator rating by the CO2 scrubber rating.
///
/// Both the oxygen generator rating and the CO2 scrubber rating are values
/// that can be found in your diagnostic report - finding them is the tricky
/// part. Both values are located using a similar process that involves
/// filtering out values until only one remains. Before searching for either
/// rating value, start with the full list of binary numbers from your
/// diagnostic report and consider just the first bit of those numbers. Then:
///
/// Keep only numbers selected by the bit criteria for the type of rating value
/// for which you are searching. Discard numbers which do not match the bit
/// criteria.  If you only have one number left, stop; this is the rating value
/// for which you are searching.  Otherwise, repeat the process, considering
/// the next bit to the right.  The bit criteria depends on which type of
/// rating value you want to find:
///
/// To find oxygen generator rating, determine the most common value (0 or 1)
/// in the current bit position, and keep only numbers with that bit in that
/// position. If 0 and 1 are equally common, keep values with a 1 in the
/// position being considered.
///
/// To find CO2 scrubber rating, determine the least common value (0 or 1) in
/// the current bit position, and keep only numbers with that bit in that
/// position. If 0 and 1 are equally common, keep values with a 0 in the
/// position being considered.
///
/// For example, to determine the oxygen generator rating value using the same
/// example diagnostic report from above:
///
/// Start with all 12 numbers and consider only the first bit of each
/// number. There are more 1 bits (7) than 0 bits (5), so keep only the 7
/// numbers with a 1 in the first position: 11110, 10110, 10111, 10101, 11100,
/// 10000, and 11001.  Then, consider the second bit of the 7 remaining
/// numbers: there are more 0 bits (4) than 1 bits (3), so keep only the 4
/// numbers with a 0 in the second position: 10110, 10111, 10101, and 10000.
/// In the third position, three of the four numbers have a 1, so keep those
/// three: 10110, 10111, and 10101.  In the fourth position, two of the three
/// numbers have a 1, so keep those two: 10110 and 10111.  In the fifth
/// position, there are an equal number of 0 bits and 1 bits (one each). So, to
/// find the oxygen generator rating, keep the number with a 1 in that
/// position: 10111.  As there is only one number left, stop; the oxygen
/// generator rating is 10111, or 23 in decimal.  Then, to determine the CO2
/// scrubber rating value from the same example above:
///
/// Start again with all 12 numbers and consider only the first bit of each
/// number. There are fewer 0 bits (5) than 1 bits (7), so keep only the 5
/// numbers with a 0 in the first position: 00100, 01111, 00111, 00010, and
/// 01010.  Then, consider the second bit of the 5 remaining numbers: there are
/// fewer 1 bits (2) than 0 bits (3), so keep only the 2 numbers with a 1 in
/// the second position: 01111 and 01010.  In the third position, there are an
/// equal number of 0 bits and 1 bits (one each). So, to find the CO2 scrubber
/// rating, keep the number with a 0 in that position: 01010.  As there is only
/// one number left, stop; the CO2 scrubber rating is 01010, or 10 in decimal.
/// Finally, to find the life support rating, multiply the oxygen generator
/// rating (23) by the CO2 scrubber rating (10) to get 230.
///
/// Use the binary numbers in your diagnostic report to calculate the oxygen
/// generator rating and CO2 scrubber rating, then multiply them together. What
/// is the life support rating of the submarine? (Be sure to represent your
/// answer in decimal, not binary.)
use std::convert::TryInto;
use structopt::StructOpt;
use thiserror::Error;

use crate::utils::{parse_lines_from_path, ProblemResult};

#[derive(StructOpt, Debug)]
pub struct P3 {
    #[structopt(
        short = "i",
        long = "input",
        default_value = "inputs/problem3/input.txt"
    )]
    input: String,
}

impl P3 {
    pub fn run(&self) -> ProblemResult {
        let nums: Vec<BinaryInt<12>> = parse_lines_from_path(&self.input)?;

        let (p1, p2) = run_problem(nums)?;

        println!("Part 1: {}", p1);
        println!("Part 2: {}", p2);

        Ok(())
    }
}

fn run_problem<const N: usize>(nums: Vec<BinaryInt<N>>) -> ProblemResult<(u64, u64)> {
    let (gamma, epsilon) = compute_rates(&nums);
    let gamma: u64 = gamma.into();
    let epsilon: u64 = epsilon.into();

    let oxygen_rating: u64 = compute_rating(&nums, Rating::Oxygen)?.into();
    let co2_rating: u64 = compute_rating(&nums, Rating::CO2)?.into();

    Ok((gamma * epsilon, oxygen_rating * co2_rating))
}

#[derive(Debug, Clone, Copy)]
struct BinaryInt<const N: usize>([bool; N]);

impl<const N: usize> BinaryInt<N> {
    fn at(&self, i: usize) -> bool {
        self.0[i]
    }

    fn invert(&self) -> BinaryInt<N> {
        let mut bits = [false; N];
        for (i, bit) in self.0.iter().enumerate() {
            bits[i] = !bit;
        }
        BinaryInt(bits)
    }
}

impl<const N: usize> std::str::FromStr for BinaryInt<N> {
    type Err = IntParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed_bits: Vec<bool> = s
            .chars()
            .map(|c| match c {
                '0' => Ok(false),
                '1' => Ok(true),
                _ => Err(IntParseError::InvalidBit(c)),
            })
            .collect::<Result<_, _>>()?;

        if parsed_bits.len() != N {
            return Err(IntParseError::WrongBitCount {
                expected: N,
                actual: parsed_bits.len(),
            });
        }

        let arr: [bool; N] = parsed_bits.try_into().unwrap();
        Ok(BinaryInt(arr))
    }
}

impl<const N: usize> From<BinaryInt<N>> for u64 {
    fn from(i: BinaryInt<N>) -> u64 {
        let mut out: u64 = 0;

        for (i, &bit) in i.0.iter().rev().enumerate() {
            out |= (bit as u64) << i;
        }

        out
    }
}

#[derive(Error, Debug)]
pub enum IntParseError {
    #[error("Invalid bit: {0:?}")]
    InvalidBit(char),
    #[error("Wrong number of bits. Expected {expected}. Got {actual}.")]
    WrongBitCount { expected: usize, actual: usize },
}

fn compute_rates<const N: usize>(nums: &Vec<BinaryInt<N>>) -> (BinaryInt<N>, BinaryInt<N>) {
    let gamma = {
        let mut bits = [false; N];
        for i in 0..N {
            bits[i] = most_common_bit(nums.iter().map(|num| num.at(i)));
        }
        BinaryInt(bits)
    };
    let epsilon = gamma.invert();

    (gamma, epsilon)
}

#[derive(PartialEq)]
enum Rating {
    Oxygen,
    CO2,
}

#[derive(Error, Debug)]
pub enum InvalidInput {
    #[error("Filter did not produce a unique value")]
    NonUnique,
}

fn compute_rating<const N: usize>(nums: &Vec<BinaryInt<N>>, rating: Rating) -> ProblemResult<u64> {
    let mut nums: Vec<BinaryInt<N>> = nums.clone();
    for i in 0..N {
        let most_common = most_common_bit(nums.iter().map(|n| n.at(i)));
        let filter_bit = if rating == Rating::Oxygen {
            most_common
        } else {
            !most_common
        };
        nums.retain(|b| b.at(i) == filter_bit);

        if nums.len() == 1 {
            return Ok(nums[0].into());
        } else if nums.len() == 0 {
            return Err(InvalidInput::NonUnique)?;
        }
    }

    return Err(InvalidInput::NonUnique)?;
}

struct BitCounts {
    trues: usize,
    falses: usize,
}

impl BitCounts {
    fn new() -> BitCounts {
        BitCounts {
            trues: 0,
            falses: 0,
        }
    }
}

fn most_common_bit(it: impl Iterator<Item = bool>) -> bool {
    let counts = count_values(it);
    if counts.trues >= counts.falses {
        true
    } else {
        false
    }
}

fn count_values(it: impl Iterator<Item = bool>) -> BitCounts {
    let mut counts = BitCounts::new();
    for elem in it {
        if elem {
            counts.trues += 1;
        } else {
            counts.falses += 1;
        }
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::{run_problem, BinaryInt, ProblemResult};
    use crate::utils::parse_lines;

    #[test]
    fn test_example() -> ProblemResult<()> {
        let s = br#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;
        let nums: Vec<BinaryInt<5>> = parse_lines(&s[..])?;
        let (p1, p2) = run_problem(nums)?;
        assert_eq!(p1, 198);
        assert_eq!(p2, 230);
        Ok(())
    }
}
