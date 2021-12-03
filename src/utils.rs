use std::fmt::Debug;
use std::fs::File;

use std::io::{BufRead, BufReader, Error as IOError, Read};
use std::str::FromStr;

use thiserror::Error;

pub type ProblemResult<T = ()> = anyhow::Result<T>;

#[derive(Error, Debug)]
pub enum ParseLinesError<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Debug + 'static,
{
    #[error("Failed to open file: {path}")]
    FileOpen { path: String, source: IOError },

    #[error("Failed reading line {lineno}")]
    LineRead { source: IOError, lineno: usize },

    #[error("Failed parsing line {lineno}: {line:?}")]
    Parse {
        lineno: usize,
        line: String,
        source: <T as FromStr>::Err,
    },
}

/// Read lines from a file and parse into a sequence of values.
pub fn parse_lines_from_path<T>(path: &str) -> Result<Vec<T>, ParseLinesError<T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    parse_lines(
        File::open(path).map_err(|source| ParseLinesError::FileOpen {
            path: path.to_owned(),
            source,
        })?,
    )
}

/// Split a readable sequence into lines and parse into a sequence of values.
pub fn parse_lines<T>(readable: impl Read) -> Result<Vec<T>, ParseLinesError<T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    let lines = BufReader::new(readable).lines();
    let mut vals: Vec<T> = Vec::new();

    for (lineno, maybe_line) in lines.enumerate() {
        let line = maybe_line.map_err(|e| ParseLinesError::LineRead { source: e, lineno })?;
        let val = T::from_str(&line).map_err(|e| ParseLinesError::Parse {
            source: e,
            lineno,
            line,
        })?;
        vals.push(val);
    }

    Ok(vals)
}
