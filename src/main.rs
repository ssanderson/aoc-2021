mod problem;
mod problem1;

use crate::problem::{Problem, ProblemResult};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
enum Opt {
    /// Run problem 1.
    P1(problem1::P1),
}

impl Opt {
    fn run(&self) -> ProblemResult {
        match self {
            Opt::P1(p) => p.run(),
        }
    }
}

fn main() -> ProblemResult {
    let opt = Opt::from_args();
    opt.run()
}
