use crate::ast::*;

lalrpop_mod!(pub axon); // synthesized by LALRPOP

macro_rules! p {
    ($i:ident, $lit:literal) => {
        axon::$i::new().parse($lit).unwrap()
    }
}

#[test]
fn literals_work() {
    assert_eq!(p!(LitParser, "null"), Lit::Null);
    assert_eq!(p!(LitParser, "true"), Lit::Bool(true));
    assert_eq!(p!(LitParser, "false"), Lit::Bool(false));
}

#[test]
fn lambdas_work() {
    p!(LambdaParser, "() => null");
    p!(LambdaParser, "someVar => null");
    p!(LambdaParser, "(someVar) => null");
    p!(LambdaParser, "(oneVar, twoVar) => null");
    p!(LambdaParser, "(oneVar: null, twoVar: null) => null");
}