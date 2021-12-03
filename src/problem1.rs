// --- Day 1: Sonar Sweep ---

// You're minding your own business on a ship at sea when the overboard alarm
// goes off! You rush to see if you can help. Apparently, one of the Elves
// tripped and accidentally sent the sleigh keys flying into the ocean!

// Before you know it, you're inside a submarine the Elves keep ready for
// situations like this. It's covered in Christmas lights (because of course it
// is), and it even has an experimental antenna that should be able to track
// the keys if you can boost its signal strength high enough; there's a little
// meter that indicates the antenna's signal strength by displaying 0-50 stars.

// Your instincts tell you that in order to save Christmas, you'll need to get
// all fifty stars by December 25th.

// Collect stars by solving puzzles. Two puzzles will be made available on each
// day in the Advent calendar; the second puzzle is unlocked when you complete
// the first. Each puzzle grants one star. Good luck!

// As the submarine drops below the surface of the ocean, it automatically
// performs a sonar sweep of the nearby sea floor. On a small screen, the sonar
// sweep report (your puzzle input) appears: each line is a measurement of the
// sea floor depth as the sweep looks further and further away from the
// submarine.

// For example, suppose you had the following report:

// 199
// 200
// 208
// 210
// 200
// 207
// 240
// 269
// 260
// 263

// This report indicates that, scanning outward from the submarine, the sonar
// sweep found depths of 199, 200, 208, 210, and so on.

// The first order of business is to figure out how quickly the depth
// increases, just so you know what you're dealing with - you never know if the
// keys will get carried into deeper water by an ocean current or a fish or
// something.

// To do this, count the number of times a depth measurement increases from the
// previous measurement. (There is no measurement before the first
// measurement.) In the example above, the changes are as follows:

// 199 (N/A - no previous measurement)
// 200 (increased)
// 208 (increased)
// 210 (increased)
// 200 (decreased)
// 207 (increased)
// 240 (increased)
// 269 (increased)
// 260 (decreased)
// 263 (increased)
// In this example, there are 7 measurements that are larger than the previous measurement.

// How many measurements are larger than the previous measurement?
//
// --- Part Two ---

// Considering every single measurement isn't as useful as you expected:
// there's just too much noise in the data.

// Instead, consider sums of a three-measurement sliding window. Again
// considering the above example:

// 199  A
// 200  A B
// 208  A B C
// 210    B C D
// 200  E   C D
// 207  E F   D
// 240  E F G
// 269    F G H
// 260      G H
// 263        H

// Start by comparing the first and second three-measurement windows. The
// measurements in the first window are marked A (199, 200, 208); their sum is
// 199 + 200 + 208 = 607. The second window is marked B (200, 208, 210); its
// sum is 618. The sum of measurements in the second window is larger than the
// sum of the first, so this first comparison increased.

// Your goal now is to count the number of times the sum of measurements in
// this sliding window increases from the previous sum. So, compare A with B,
// then compare B with C, then C with D, and so on. Stop when there aren't
// enough measurements left to create a new three-measurement sum.

// In the above example, the sum of each three-measurement window is as follows:

// A: 607 (N/A - no previous sum)
// B: 618 (increased)
// C: 618 (no change)
// D: 617 (decreased)
// E: 647 (increased)
// F: 716 (increased)
// G: 769 (increased)
// H: 792 (increased)
// In this example, there are 5 sums that are larger than the previous sum.

// Consider sums of a three-measurement sliding window. How many sums are larger than the previous sum?

use anyhow::Context;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use structopt::StructOpt;

use crate::problem::{Problem, ProblemResult};

#[derive(StructOpt, Debug)]
pub struct P1 {
    #[structopt(
        short = "i",
        long = "input",
        default_value = "inputs/problem1/input.txt"
    )]
    input: String,
}

impl Problem for P1 {
    fn run(&self) -> ProblemResult {
        let depths = parse_ints(&self.input)?;

        let (incs, rolling_incs) = compute_increments(&depths)?;

        println!("Number of Increases: {}", incs);
        println!("Number of Rolling Window Increases: {}", rolling_incs);

        Ok(())
    }
}

fn compute_increments(ints: &Vec<u32>) -> ProblemResult<(usize, usize)> {
    let raw_increases = count_increases(ints.iter().cloned());

    let rolling_sums: Vec<u32> = ints.windows(3).map(|window| window.iter().sum()).collect();
    let rolling_sum_increases = count_increases(rolling_sums.into_iter());

    Ok((raw_increases, rolling_sum_increases))
}

fn count_increases(mut it: impl Iterator<Item = u32>) -> usize {
    let mut count = 0;
    let mut prev = match it.next() {
        Some(i) => i,
        None => return count,
    };

    for i in it {
        if i > prev {
            count += 1;
        }
        prev = i;
    }

    count
}

fn parse_ints(p: &str) -> ProblemResult<Vec<u32>> {
    let f = File::open(p).with_context(|| format!("Failed to open {}", p))?;

    let mut ints: Vec<u32> = Vec::new();
    for (i, maybe_line) in BufReader::new(f).lines().enumerate() {
        let line = maybe_line.context(format!("Failed reading line {} from {}", i, p))?;
        let int = u32::from_str(&line).context(format!("Failed to parse line {}: {}", i, line))?;
        ints.push(int);
    }

    Ok(ints)
}
