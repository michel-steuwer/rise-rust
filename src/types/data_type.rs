use super::nat::Nat;
use super::nat2data::Nat2Data;
use super::kind::*;

#[derive(Debug)]
pub enum DataType {
    Identifier(String),
    ScalarType(ScalarType),
    NatType,
    VectorType(Nat, Box<DataType>),
    IndexType(Nat),
    PairType(Box<DataType>, Box<DataType>),
    Apply(Box<Nat2Data>, Nat),
    ArrayType(Nat, Box<DataType>),
    DepArrayType(Nat, Box<Nat2Data>)
}

#[derive(Debug)]
pub enum ScalarType {
    Int,
    Float
}

impl Identifier for DataType {
    fn kind(&self) -> Kind {
        return Kind::Data;
    }

    fn name(&self) -> &str {
        return match self {
            DataType::Identifier(s) => s,
            _ => unreachable!()
        }
    }
}

#[macro_export]
macro_rules! dt {
    ( $n:expr ; $dt:expr ) => {{
        use crate::types::*;
        data_type::DataType::ArrayType($n, Box::new($dt))
    }};
}