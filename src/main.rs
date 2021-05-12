use ast::{Expression, Function, Ident, Program, Statement };

mod ast;

fn main() {
    println!("Hello, world!");

    let main1 = Function {
        name: Ident(String::from("main")),
        parameters: vec![],
        declared_vars: vec![],
        body: Statement::Empty,
        return_expr: Expression::Int(5),
    };
    let prog1 = Program {
        functions: vec![main1]
    };
    println!("program =  {:?}", prog1);
}
