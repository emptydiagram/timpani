use crate::ast::{
  Expression,
  Function,
  Ident,
  Statement,
};

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
  let parseFn = tag("function");
  let parseReturn = tag("return");
  let parenOpen = tag("(");
  let parenClose = tag(")");
  let braceOpen = tag("{");
  let braceClose = tag("}");

  // let (theInput, (_, _, functionName, _, _, _, id1,
  //      idRest, _, _)) = tuple(
  //   (parseFunction, space1, parseIdent, space0, parenOpen, space0, opt(parseIdent),
  //    many0(tuple((tag(","), space0, parseIdent, space0))), space0, parenClose)
  // )(input)?;
  let (theInput, (/* 'function' */ _, _, functionName, _, /* '(' */ _, _, /* ')' */ _, _,
  /* '{' */ _, _, /* 'return' */ _, _, returnExpr, _, /* ';' */ _, _, /* '}' */ _)) = tuple(
    (parseFn, space1, parseIdent, space0, parenOpen, space0, parenClose, space0,
     braceOpen, space0, parseReturn, space0, parseExpression, space0, tag(";"), space0, braceClose)
  )(input)?;

  println!("functionName = {:?}", functionName);
  // println!("id1 = {:?}", id1);
  // println!("idRest {:?}", idRest);
  println!("theInput: {:?}", theInput);
  println!("parsedExpr: {:?}", returnExpr);

  let theFunction = Function {
    name: functionName,
    parameters: vec![],
    declared_vars: vec![],
    body: Statement::Empty,
    return_expr: returnExpr,
  };
  Ok((theInput, theFunction))

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

fn parseIdent<'a>(input: &'a str) -> PResult<'a, Ident> {
  let re = regex::Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
  let parser = nom::regexp::str::re_find::<Error<&'a str>>(re);
  let result = parser(input);
  result.map(|(rem, parsed)| (rem, Ident::from(parsed)))
  // failure(Error::new(input, ErrorKind::RegexpMatch))
}

fn parseStatement(input: &str) -> PResult<Statement> {
  failure(Error::new(input, ErrorKind::Not))
}

fn parseExpression<'a>(input: &'a str) -> PResult<'a, Expression> {
  // FIXME: only parses ints
  let re = regex::Regex::new(r"-?(0|[1-9][0-9]*)").unwrap();
  let parser = nom::regexp::str::re_find::<Error<&'a str>>(re);
  let result = parser(input);
  result.map(|(rem, parsed)| (rem, Expression::Int(parsed.parse().unwrap())))
}