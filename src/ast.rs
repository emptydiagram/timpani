
#[derive(Debug)]
pub struct Ident(pub String);

impl From<&'_ str> for Ident {
  fn from (s: &str) -> Ident {
    return Ident(s.to_string());
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOperator {
  Add,
  Sub,
  Mul,
  Div,
  GreaterThan,
  LessThan,
  Equals,
}

#[derive(Debug)]
pub enum Expression {
  Int(i64),
  Ident(Ident),
  Group(Box<Expression>),
  Input,
  BinOp(BinaryOperator, Box<Expression>, Box<Expression>),
  FuncCall(Ident, Vec<Expression>)
}

#[derive(Debug)]
pub enum Statement {
  Assign(Ident, Expression),
  Output(Expression),
  ComposeTwo(Box<Statement>, Box<Statement>),
  Compose(Vec<Statement>),
  Empty,
  If {
    cond: Expression,
    then: Option<Box<Statement>>,
    else_: Option<Box<Statement>>
  },
  While {
    cond: Expression,
    then: Option<Box<Statement>>,
  },
}

#[derive(Debug)]
pub struct Function {
  pub name: Ident,
  pub parameters: Vec<Ident>,
  pub declared_vars: Vec<Ident>,
  pub body: Statement,
  pub return_expr: Expression,
}

#[derive(Debug)]
pub struct Program {
  pub functions: Vec<Function>
}