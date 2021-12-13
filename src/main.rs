mod problem1;
mod problem2;
mod problem3;
mod problem4;
mod problem5;
mod utils;

use crate::utils::ProblemResult;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
enum Opt {
    /// Run problem 1.
    P1(problem1::P1),
    /// Run problem 2.
    P2(problem2::P2),
    /// Run problem 3.
    P3(problem3::P3),
    /// Run problem 4.
    P4(problem4::P4),
    /// Run problem 5.
    P5(problem5::P5),
}

impl Opt {
    fn run(&self) -> ProblemResult {
        match self {
            Opt::P1(p) => p.run(),
            Opt::P2(p) => p.run(),
            Opt::P3(p) => p.run(),
            Opt::P4(p) => p.run(),
            Opt::P5(p) => p.run(),
        }
    }
}

fn main() -> ProblemResult {
    let opt = Opt::from_args();
    opt.run()
}
