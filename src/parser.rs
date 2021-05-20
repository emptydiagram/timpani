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
pub fn parse_function<'a>(input: &'a str) -> PResult<'a, Function> {
  let parse_fn = tag("function");
  let parse_return = tag("return");
  let paren_open = tag("(");
  let paren_close = tag(")");
  let brace_open = tag("{");
  let brace_close = tag("}");

  // TODO: support:
  //  - arbitrary number of params
  //  - local variable declaration
  //  - an arbitrary function body
  //     (but there's still a single return at the end, as dictated by the grammar)
  let (the_input, (/* 'function' */ _, _, function_name, _, /* '(' */ _, _, maybe_param_indent, _, /* ')' */ _, _,
  /* '{' */ _, _, /* 'return' */ _, _, return_expr, _, /* ';' */ _, _, /* '}' */ _)) = tuple(
    (parse_fn, space1, parse_ident, space0, paren_open, space0, opt(parse_ident), space0, paren_close, space0,
     brace_open, space0, parse_return, space0, parse_expression, space0, tag(";"), space0, brace_close)
  )(input)?;

  let mut parameters = vec![];
  maybe_param_indent.map(|ident| parameters.push(ident));

  let the_function = Function {
    name: function_name,
    parameters: parameters,
    declared_vars: vec![],
    body: Statement::Empty,
    return_expr: return_expr,
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

const IDENT_CONT_CHAR_REGEXP: &'static str = "^[a-zA-Z0-9_]";
const IDENT_REGEXP: &'static str = "^[a-zA-Z_][a-zA-Z0-9_]*";


fn parse_ident<'a>(input: &'a str) -> PResult<'a, Ident> {
  let re = regex::Regex::new(IDENT_REGEXP).unwrap();
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

fn parse_parens<'a>(input: &'a str) -> PResult<'a, Box<Expression>> {
  if input.len() == 0 {
    return failure(Error::new(input, ErrorKind::RegexpMatch))
  }
  if (input.as_bytes()[0] as char) != '(' {
    return failure(Error::new(input, ErrorKind::RegexpMatch))
  }

  let parse_result = parse_expression(&input[1..input.len()-1]);
  let (rem, sub_expr) = match parse_result {
    Err(e) => return Err(e),
    Ok((rem, expr)) => (rem, Box::new(expr)),
  };


  if (input.as_bytes()[input.len() - 1] as char) != ')' {
    return failure(Error::new(input, ErrorKind::RegexpMatch))
  }

  Ok((rem, sub_expr))
}

fn parse_func_call<'a>(input: &'a str) -> PResult<'a, (Ident, Vec<Expression>)> {
  let paren_open = tag("(");
  let paren_close = tag(")");
  // TODO: implement calling with arbitrary number of arguments
  let (the_input, (function_name, _, /* '(' */ _, _, maybe_arg_expr, _, /* ')' */ _ ))
    = tuple(
      (parse_ident, space0, paren_open, space0, opt(parse_expression), space0, paren_close)
    )(input)?;

  let mut arguments = vec![];
  maybe_arg_expr.map(|arg| arguments.push(arg));

  Ok((the_input, (function_name, arguments)))
}

fn parseStatement(input: &str) -> PResult<Statement> {
  failure(Error::new(input, ErrorKind::Not))
}

fn parse_expression<'a>(input_text: &'a str) -> PResult<'a, Expression> {
  // println!(" ::parse_expression, input = '{}'", input_text);
  if let Ok((rem, parsed)) = parse_int(input_text) {
    return Ok((rem, Expression::Int(parsed)))
  }
  // int parse failed, try input
  let input_result = parse_input(input_text);
  if let Ok((rem, _)) = input_result {
    let re = regex::Regex::new(IDENT_CONT_CHAR_REGEXP).unwrap();
    let parser = nom::regexp::str::re_find::<Error<&'a str>>(re);
    if parser(rem).is_err() {
      return Ok((rem, Expression::Input));
    }
  }
  let func_call_result = parse_func_call(input_text);
  if let Ok((rem, (ident, expressions))) = func_call_result {
    return Ok((rem, Expression::FuncCall(ident, expressions)));
  }

  let parens_result = parse_parens(input_text);
  if let Ok((rem, boxed_expr)) = parens_result {
    return Ok((rem, Expression::Group(boxed_expr)));
  }

  let ident_result = parse_ident(input_text);
  ident_result.map(|(rem, ident)| (rem, Expression::Ident(ident)))

  // TODO: parse binary operations
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

  #[test]
  fn test_parse_expression_func_call() {
    let mut parse_result = parse_expression("id(456)");
    assert_eq!(parse_result.is_ok(), true);
    let mut is_func_call = false;
    if let Expression::FuncCall(ident, exprs) = parse_result.unwrap().1 {
      is_func_call = true;
      assert_eq!(ident.0, "id");
      assert_eq!(exprs.len(), 1);

      if let Expression::Int(i) = exprs[0] {
        assert_eq!(i, 456);
      } else {
        panic!("Expected an integer expression.")
      }
    }
    assert_eq!(is_func_call, true);
  }

  #[test]
  fn test_parse_expression_parens() {
    let mut parse_result = parse_expression("(123)");
    assert_eq!(parse_result.is_ok(), true);
    let mut is_parens = false;
    if let Expression::Group(expr1) = parse_result.unwrap().1 {
      is_parens = true;
      if let Expression::Int(i) = *expr1 {
        assert_eq!(i, 123);
      } else {
        panic!("Expected an integer expression.")
      }
    }
    assert_eq!(is_parens, true);

    parse_result = parse_expression("((input))");
    assert_eq!(parse_result.is_ok(), true);
    let mut is_parens = false;
    if let Expression::Group(expr1) = parse_result.unwrap().1 {
      is_parens = true;
      let mut is_nested_parens = false;
      if let Expression::Group(expr2) = *expr1 {
        is_nested_parens = true;
        if let Expression::Input = *expr2 {
        } else {
          panic!("Expected inner expression to be an input")
        }
      }
      assert_eq!(is_nested_parens, true);
    }
    assert_eq!(is_parens, true);

    parse_result = parse_expression("(((AbrahamLincoln)))");
    assert_eq!(parse_result.is_ok(), true);
    let mut is_parens = false;
    let mut is_nested_parens = false;
    let mut is_nested2_parens = false;
    if let Expression::Group(expr1) = parse_result.unwrap().1 {
      is_parens = true;
      if let Expression::Group(expr2) = *expr1 {
        is_nested_parens = true;
        if let Expression::Group(expr3) = *expr2 {
          is_nested2_parens = true;
          if let Expression::Ident(ident) = *expr3 {
            assert_eq!(ident.0, "AbrahamLincoln");
          } else {
            panic!("Expected inner expression to be an ident")
          }
        }
      }
    }
    assert_eq!(is_parens, true);
    assert_eq!(is_nested_parens, true);
    assert_eq!(is_nested2_parens, true);
  }
}