#[derive(PartialEq, Eq, Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Function {
    pub return_type: Type,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub block: Block,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Type {
    pub name: String,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Statement {
    Return(Box<Expr>),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Expr {
    NumLiteral(String),
}
