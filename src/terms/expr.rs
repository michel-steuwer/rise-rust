use crate::types::r#type::Type;
use crate::terms::expr::Term::Identifier;
use crate::utils::fresh_name;

#[derive(Debug)]
pub struct Expr {
    pub term: Term,
    pub r#type: Option<Type>
}

fn expr(term: Term) -> Expr {
    Expr { term, r#type: None }
}

#[derive(Debug)]
pub enum Term {
    Identifier(String),
    Lambda(Box<Expr>, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    Literal(String)
}

pub fn ident(name: &str) -> Expr {
    expr(Term::Identifier(String::from(name)))
}

pub fn fun<F>(f: F) -> Expr
    where F: Fn(Expr) -> Expr
{
    expr( Term::Lambda(
        Box::new(ident(unsafe { fresh_name("x") }.as_str())),
        Box::new((f)(ident(unsafe { fresh_name("x") }.as_str())))
    ))
}

pub fn app(f: Expr, arg: Expr) -> Expr {
    expr(Term::App(
        Box::new(f),
        Box::new(arg)
    ))
}

pub fn lit(literal: &str) -> Expr {
    expr(Term::Literal(String::from(literal)))
}
