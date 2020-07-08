use crate::types::data_type::DataType;

mod kind;
mod address_space;
mod nat;
mod nat2nat;
#[macro_use]
mod nat2data;
#[macro_use]
mod data_type;
mod r#type;

fn print_dep_fun_identifier(t: r#type::Type) -> () {
    match t {
        r#type::Type::DepFunType(x, t) => print!("{}\n", x.name()),
        _ => {}
    }
}

#[test]
fn test_print_dep_fun_identifier() {
    use r#type::Type::*;
    // let t = Type::Data(DataType::ScalarType(data_type::ScalarType::Int));
    let t = DepFunType(
        Box::new(DataType::Identifier(String::from("x"))),
        Box::new(Data(DataType::ScalarType(data_type::ScalarType::Int)))
    );

    let  n = nat::Nat::Identifier(String::from("n"));
    let dt = data_type::DataType::ArrayType(
        n,
        Box::new(DataType::ScalarType(data_type::ScalarType::Int))
    );

    let  n2 = nat::Nat::Identifier(String::from("n"));

    let n2d = n2d!( ("n") => dt!(n2; DataType::ScalarType(data_type::ScalarType::Int)));

    print_dep_fun_identifier(t);
}