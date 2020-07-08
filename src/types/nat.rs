use super::nat2nat::Nat2Nat;
use super::kind::{Kind, Identifier};

#[derive(Debug)]
pub enum Nat {
    Identifier(String),
    Literal(i32),
    UnaryOp(String, Box<Nat>),
    BinaryOp(Box<Nat>, String, Box<Nat>),
    Apply(Box<Nat2Nat>, Box<Nat>)
}

impl Identifier for Nat {
    fn kind(&self) -> Kind {
        return Kind::Nat;
    }

    fn name(&self) -> &str {
        return match self {
            Nat::Identifier(s) => s,
            _ => unreachable!()
        }
    }
}

use Nat::*;

pub fn ident(name: &str) -> Nat {
    Identifier(String::from(name))
}

pub fn lit(i: i32) -> Nat {
    Literal(i)
}

pub fn u_op(name: &str, n: Nat) -> Nat {
    UnaryOp(String::from(name), Box::new(n))
}

pub fn bin_op(n1: Nat, name: &str, n2: Nat) -> Nat {
    BinaryOp(Box::new(n1), String::from(name), Box::new(n2))
}

pub fn apply(n2n: Nat2Nat, n: Nat) -> Nat {
    Apply(Box::new(n2n), Box::new(n))
}
