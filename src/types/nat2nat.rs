use super::nat::Nat;
use super::kind::*;

#[derive(Debug)]
pub enum Nat2Nat {
    Identifier(String),
    Lambda(Nat, Nat)
}

impl Identifier for Nat2Nat {
    fn kind(&self) -> Kind {
        return Kind::Nat2Nat;
    }

    fn name(&self) -> &str {
        return match self {
            Nat2Nat::Identifier(s) => s,
            _ => unreachable!()
        }
    }
}
