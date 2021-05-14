use nom::combinator::opt;
use nom::multi::many0;
use nom::character::complete::{
  space0,
  space1,
};
use nom::sequence::tuple;
use nom::bytes::complete::{
  tag,
};
use nom::IResult;
use nom::error::{
  Error,
  ErrorKind
};
// use thiserror::Error;

use crate::ast::{
  Function,
  Statement,
};

// #[derive (Error, Debug)]
// pub enum ParseError<'a> {
//   #[error("Not yet implemented")]
//   NotYetImplemented,

//   #[error("Error parsing string")]
//   ParseStringError {
//     message: &'a str
//   }
// }


// To parse:
//   function begin() {
//     return 5;
//   }

struct ParseInfo<'a, T> {
  remaining: &'a str,
  parsed: T,
}

fn error<T>(error: Error<&str>) -> Result<T, nom::Err<Error<&str>>> {
  Err(nom::Err::Error(error))
}

fn failure<T>(failure: Error<&str>) -> Result<T, nom::Err<Error<&str>>> {
  Err(nom::Err::Failure(failure))
}

type PResult<'a, T> = IResult<&'a str, T>;

// Func ::= "function" Id "(" Id* ")" "{" Stmt* "}"
pub fn parseFunction<'a>(input: &'a str) -> PResult<'a, Function> {
  let function = tag("function");
  let parenOpen = tag("(");
  let parenClose = tag(")");
  let braceOpen = tag("{");
  // let braceClose = tag("}");

  let (theInput, (_, _, functionName, _, _, id1, _,
       idRest, _, _, _, _)) = tuple(
    (function, space1, parseIdent, space0, parenOpen, space0, opt(parseIdent),
     many0(tuple((tag(","), space0, parseIdent, space0))), parenClose, space0, braceOpen, space0)
  )(input)?;

  println!("functionName = {:?}", functionName);
  println!("id1 = {:?}", id1);
  println!("idRest {:?}", idRest);

  // let (input, fn_result) = tuple((parseStr("function", input), nom::space))(input)?;
  // nom::space<&'a str>
  // parseStr("(")
  // parse
  // match fn_result {
  //   Ok((remaining, output)) => {
  //     // TODO
  //   },
  //   Err(e) => {
  //     println!("got error doing parseStr: {:?}", e);
  //     panic!("oopsy woopsy")
  //   }
  // }
  failure(nom::error::Error::new(input, ErrorKind::RegexpMatch))
}

fn parseStr<'a>(searchStr: &str, input: &'a str) -> PResult<'a, &'a str> {
  let re = regex::Regex::new(searchStr).unwrap();
  let parser = nom::regexp::str::re_matches::<(&str, ErrorKind)>(re);
  let result = parser(input);
  println!("parseStr rem  = {:?}", result);
  match result {
    Ok((remaining, vecMatches)) => {
      if vecMatches.len() == 0 {
        return failure(nom::error::Error::new(remaining, ErrorKind::RegexpMatch));
      }
      println!("parse successful, remaining = {}", remaining);
      println!("vecMatches = {:?}", vecMatches);
      return Ok((remaining, vecMatches[0]))
    }
    Err(e) => {
      println!("eror parsing: {:?}", e)
    }
  }
  failure(Error::new(input, ErrorKind::RegexpMatch))
}

fn parseIdent<'a>(input: &'a str) -> PResult<'a, &'a str> {
  let re = regex::Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
  let parser = nom::regexp::str::re_find::<Error<&'a str>>(re);
  let result = parser(input);
  result
  // failure(Error::new(input, ErrorKind::RegexpMatch))
}

fn parseStatement(input: &str) -> PResult<Statement> {
  failure(Error::new(input, ErrorKind::Not))
}