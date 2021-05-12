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
  FuncCall(Ident, Expression[])
}

#[derive(Debug)]
pub enum Statement {
  Assign(Ident, Expression),
  Output(Expression),
  Compose(Statement, Statement),
  Empty(),
  If {
    cond: Expression,
    then: Option<Statement>,
    else: Option<Statement>
  },
  While {
    cond: Expression,
    then: Option<Statement>,
  },
}

pub struct Function {
  pub name: Ident,
  pub parameters: Ident[],
  pub declared_vars: Ident[],
  pub body: Statement,
  pub return_expr: Expression
}

pub struct Program(pub functions: Vec<Function>);