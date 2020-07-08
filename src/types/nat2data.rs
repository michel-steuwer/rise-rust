use super::nat::Nat;
use super::data_type::DataType;
use super::kind::*;

#[derive(Debug)]
pub enum Nat2Data {
    Identifier(String),
    Lambda(Nat, DataType)
}

impl Identifier for Nat2Data {
    fn kind(&self) -> Kind {
        return Kind::Nat2Data;
    }

    fn name(&self) -> &str {
        return match self {
            Nat2Data::Identifier(s) => s,
            _ => unreachable!()
        }
    }
}

#[macro_export]
macro_rules! n2d {
    ( ($x:expr) => $body:expr ) => {{
        use crate::types::*;
        let n = nat::Nat::Identifier(String::from($x));
        nat2data::Nat2Data::Lambda(n, $body)
    }};
}
