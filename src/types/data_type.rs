use super::nat::Nat;
use super::nat2data::Nat2Data;

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

use super::kind::{Kind, Identifier};

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

use crate::types::data_type::DataType::*;

pub fn ident(name: &str) -> DataType {
    Identifier(String::from(name))
}

pub fn nat_type() -> DataType {
    NatType
}

pub fn vector_type(n: Nat, dt: DataType) -> DataType {
    VectorType(n, Box::new(dt))
}

pub fn index_type(n: Nat) -> DataType {
    IndexType(n)
}

pub fn pair_type(fst: DataType, snd: DataType) -> DataType {
    PairType(Box::new(fst), Box::new(snd))
}

pub fn apply(n2d: Nat2Data, n: Nat) -> DataType {
    DataType::Apply(Box::new(n2d), n)
}

pub fn array_type(n: Nat, dt: DataType) -> DataType {
    ArrayType(n, Box::new(dt))
}

pub fn dep_array_type(n: Nat, n2d: Nat2Data) -> DataType {
    DepArrayType(n, Box::new(n2d))
}

#[derive(Debug)]
pub enum ScalarType {
    Int,
    Float
}

pub fn int() -> DataType {
    ScalarType( ScalarType::Int )
}

pub fn float() -> DataType {
    ScalarType( ScalarType::Float )
}