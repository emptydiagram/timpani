use crate::Statement;
use thiserror::Error;

#[derive (Error, Debug)]
pub enum ParseError<'a> {
  #[error("Not yet implemented")]
  NotYetImplemented,

  #[error("Error parsing string")]
  ParseStringError {
    message: &'a str
  }
}

// function begin() {
//   return 5;
// }

use crate::Function;

struct ParseInfo<'a, T> {
  remaining: &'a str,
  parsed: T,
}

type PResult<T> = Result<T, ParseError<'static>>;

// Func ::= "function" Id "(" Id* ")" "{" Stmt* "}"
fn parseFunction<'a>(input: &'a str) -> PResult<ParseInfo<'a, Function>> {
  Err(ParseError::NotYetImplemented)
}

fn parseStr<'a>(searchStr: &str, input: &'a str) -> PResult<ParseInfo<'a, &'a str>> {
  Err(ParseError::NotYetImplemented)
}

fn parseStatement(input: &str) -> PResult<Statement> {
  Err(ParseError::NotYetImplemented)
}