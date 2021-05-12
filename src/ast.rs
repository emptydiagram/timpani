
#[derive(Debug)]
pub struct Ident(pub String);

#[derive(Debug)]
pub enum BinaryOperator {
  Add,
  Sub,
  Mul,
  Div,
  GreaterThan,
  Equals,
}

#[derive(Debug)]
pub enum Expression {
  Int(i64),
  Ident(Ident),
  BinOp(BinaryOperator, Expression, Expression),
  Group(Expression),
  Input,
  FuncCall(Ident, Vec<Expression>)
}

#[derive(Debug)]
pub enum Statement {
  Assign(Ident, Expression),
  Output(Expression),
  ComposeTwo(Statement, Statement),
  Compose(Vec<Statement>),
  Empty,
  If {
    cond: Expression,
    then: Option<Statement>,
    else_: Option<Statement>
  },
  While {
    cond: Expression,
    then: Option<Statement>,
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