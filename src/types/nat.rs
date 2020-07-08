use super::nat2nat::Nat2Nat;
use super::kind::*;

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
