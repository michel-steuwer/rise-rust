use core::fmt::Debug;
use std::fmt::{Formatter, Error};

#[derive(Debug)]
pub enum Kind { Data, Nat, AddrSpace, Nat2Nat, Nat2Data }

pub trait Identifier {
    fn kind(&self) -> Kind;
    fn name(&self) -> &str;
}

impl Debug for dyn Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Identifier({})", self.name())
    }
}
