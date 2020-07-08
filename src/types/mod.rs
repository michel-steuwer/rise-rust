pub mod kind;
pub mod address_space;
pub mod nat;
pub mod nat2nat;
#[macro_use]
pub mod nat2data;
#[macro_use]
pub mod data_type;
pub mod r#type;

fn print_dep_fun_identifier(t: r#type::Type) -> () {
    match t {
        r#type::Type::DepFunType(x,
                                 t) => print!("{}\n", x.name()),
        _ => {}
    }
}

use data_type::DataType;

#[test]
fn test_print_dep_fun_identifier() {
    use r#type::Type::*;
    use r#type::*;
    use data_type::*;

    let t = dep_fun_t(
        ident("x"),
        dt(int())
    );

    let dt = array_type(nat::ident("n"), int());

    dbg!(dt);

    print_dep_fun_identifier(t);
}