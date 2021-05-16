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
  let parse_fn = tag("function");
  let parse_return = tag("return");
  let paren_open = tag("(");
  let paren_close = tag(")");
  let brace_open = tag("{");
  let brace_close = tag("}");

  // let (theInput, (_, _, functionName, _, _, _, id1,
  //      idRest, _, _)) = tuple(
  //   (parseFunction, space1, parseIdent, space0, parenOpen, space0, opt(parseIdent),
  //    many0(tuple((tag(","), space0, parseIdent, space0))), space0, parenClose)
  // )(input)?;
  let (the_input, (/* 'function' */ _, _, functionName, _, /* '(' */ _, _, maybeParamIdent, _, /* ')' */ _, _,
  /* '{' */ _, _, /* 'return' */ _, _, returnExpr, _, /* ';' */ _, _, /* '}' */ _)) = tuple(
    (parse_fn, space1, parse_ident, space0, paren_open, space0, opt(parse_ident), space0, paren_close, space0,
     brace_open, space0, parse_return, space0, parse_expression, space0, tag(";"), space0, brace_close)
  )(input)?;

  // println!("id1 = {:?}", id1);
  // println!("idRest {:?}", idRest);
  println!("~~~");
  println!("theInput: {:?}", the_input);
  println!("functionName = {:?}", functionName);
  println!("maybeParamIdent: {:?}", maybeParamIdent);
  println!("parsedExpr: {:?}", returnExpr);

  let mut parameters = vec![];
  maybeParamIdent.map(|ident| parameters.push(ident));

  let the_function = Function {
    name: functionName,
    parameters: parameters,
    declared_vars: vec![],
    body: Statement::Empty,
    return_expr: returnExpr,
  };
  Ok((the_input, the_function))

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


fn parse_ident<'a>(input: &'a str) -> PResult<'a, Ident> {
  println!(" ::parseIdent, input = '{}'", input);
  let re = regex::Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
  let parser = nom::regexp::str::re_find::<Error<&'a str>>(re);
  let result = parser(input);
  result.map(|(rem, parsed)| (rem, Ident::from(parsed)))
  // failure(Error::new(input, ErrorKind::RegexpMatch))
}

fn parse_int<'a>(input: &'a str) -> PResult<'a, i64> {
  let re = regex::Regex::new(r"^-?(0|[1-9][0-9]*)").unwrap();
  let parser = nom::regexp::str::re_find::<Error<&'a str>>(re);
  let result = parser(input);
  result.map(|(rem, parsed)| (rem, parsed.parse().unwrap()))
}

fn parse_input<'a>(input: &'a str) -> PResult<'a, ()> {
  let re = regex::Regex::new(r"^input").unwrap();
  let parser = nom::regexp::str::re_find::<Error<&'a str>>(re);
  let result = parser(input);
  result.map(|(rem, _parsed)| (rem, ()))
}

fn parseStatement(input: &str) -> PResult<Statement> {
  failure(Error::new(input, ErrorKind::Not))
}

fn parse_expression<'a>(input: &'a str) -> PResult<'a, Expression> {
  println!(" ::parseExpression, input = '{}'", input);
  // FIXME: only parses ints
  if let Ok((rem, parsed)) = parse_int(input) {
    return Ok((rem, Expression::Int(parsed)))
  }
  // int parse failed, try input
  let input_result = tag::<&'a str, &'a str, nom::error::Error<&'a str>>("input")(input);
  if let Ok((rem, _)) = input_result {
    if rem.len() == 0  {
      return Ok((rem, Expression::Input));
    }
  }
  let ident_result = parse_ident(input);
  ident_result.map(|(rem, ident)| (rem, Expression::Ident(ident)))
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_basic_ident() {
    let mut parse_result = parse_ident("x");
    assert_eq!(parse_result.is_ok(), true);
    assert_eq!(parse_result.unwrap().1.0, "x");

    parse_result = parse_ident("Hello");
    assert_eq!(parse_result.is_ok(), true);
    assert_eq!(parse_result.unwrap().1.0, "Hello");

    parse_result = parse_ident("_");
    assert_eq!(parse_result.is_ok(), true);
    assert_eq!(parse_result.unwrap().1.0, "_");

    parse_result = parse_ident("_woof");
    assert_eq!(parse_result.is_ok(), true);
    assert_eq!(parse_result.unwrap().1.0, "_woof");

    parse_result = parse_ident("9abc");
    println!("result from '9abc': {:?}", parse_result);
    assert_eq!(parse_result.is_err(), true);
  }

  #[test]
  fn test_parse_expression_ident() {
    let parse_result = parse_expression("abc");
    assert_eq!(parse_result.is_ok(), true);
    let mut is_ident = false;
    if let Expression::Ident(ident) = parse_result.unwrap().1 {
      is_ident = true;
      assert_eq!(ident.0, "abc");
    }
    assert_eq!(is_ident, true);
  }

  #[test]
  fn test_parse_expression_input() {
    let mut parse_result = parse_expression("input");
    assert_eq!(parse_result.is_ok(), true);
    let mut is_input = false;
    if let Expression::Input = parse_result.unwrap().1 {
      is_input = true;
    }
    assert_eq!(is_input, true);

    parse_result = parse_expression("inputt");
    assert_eq!(parse_result.is_ok(), true);
    let mut is_ident = false;
    if let Expression::Ident(_) = parse_result.unwrap().1 {
      is_ident = true;
    }
    assert_eq!(is_ident, true);

    parse_result = parse_expression("inputs");
    assert_eq!(parse_result.is_ok(), true);
    let mut is_ident = false;
    if let Expression::Ident(_) = parse_result.unwrap().1 {
      is_ident = true;
    }
    assert_eq!(is_ident, true);
  }
}