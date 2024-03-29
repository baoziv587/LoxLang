//! A module describing Lox-specific Result and Error types

use std::error;
use std::fmt;
use std::io;
use std::result;

/// A Lox-Specific Result Type
pub type Result<T> = result::Result<T, Error>;

/// A Lox-Specific Error
#[derive(Debug)]
pub enum Error {
    /// Returned if the CLI command is used incorrectly
    Usage,
    /// Returned if there is an error reading from a file or stdin
    IO(io::Error),
    /// Returned if the scanner encounters an error
    Lexical(u64, u64, String, String),
    /// Returned if the parser encounters an error
    Parse(u64, u64, String, String),
    /// Returned if there is an error at runtime
    Runtime(u64, String, String),
    /// Sentinel error for break statements
    Break(u64),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Usage => write!(f, "Usage: rlox [script]"),
            Error::IO(ref e) => e.fmt(f),
            Error::Lexical(ref line, ref pos, ref msg, ref whence) => write!(
                f,
                "Lexical Error [line {} {}] {}: {:?}",
                line, pos, msg, whence
            ),
            Error::Parse(ref line, ref pos, ref msg, ref near) => write!(
                f,
                "Parse Error [line {} {}] {}: near {}",
                line, pos, msg, &near
            ),
            Error::Runtime(ref line, ref msg, ref near) => {
                write!(f, "Runtime Error [line {}] {}: near {}", line, msg, &near)
            }
            Error::Break(ref line) => write!(
                f,
                "Runtime Error [line {}] unexpected break statement",
                line
            ),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Usage => "usage error",
            Error::IO(ref e) => e.description(),
            Error::Lexical(_, _, _, _) => "lexical error",
            Error::Parse(_, _, _, _) => "parse error",
            Error::Runtime(_, _, _) => "runtime error",
            Error::Break(_) => "break error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IO(ref e) => e.cause(),
            _ => None,
        }
    }
}
