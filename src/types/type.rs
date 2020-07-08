use crate::types::data_type::DataType;
use super::kind::Identifier;

#[derive(Debug)]
pub enum Type {
    Data(DataType),
    FunType(Box<Type>, Box<Type>),
    DepFunType(Box<dyn Identifier>, Box<Type>),
}
