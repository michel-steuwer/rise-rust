#[macro_use]
mod expr;

#[test]
fn test_fun() {
    use expr::*;

    let f = fun( |x| x );

    dbg!(f);
}