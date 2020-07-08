use crate::types::data_type::DataType;
use super::kind::Identifier;

#[derive(Debug)]
pub enum Type {
    Data(DataType),
    FunType(Box<Type>, Box<Type>),
    DepFunType(Box<dyn Identifier>, Box<Type>),
}

use Type::*;

pub fn dt(data_type: DataType) -> Type {
    Data( data_type )
}

pub fn fun_t(in_t: Type, out_t: Type) -> Type {
    FunType(Box::new(in_t), Box::new(out_t))
}

pub fn dep_fun_t<T: 'static + Identifier>(x: T, t: Type) -> Type {
    DepFunType(Box::new(x), Box::new(t))
}