pub type ProblemResult<T = ()> = anyhow::Result<T>;

pub trait Problem: structopt::StructOpt {
    fn run(&self) -> ProblemResult;
}
