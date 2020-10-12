use crate::ast::*;

lalrpop_mod!(pub axon); // synthesized by LALRPOP

macro_rules! p {
    ($i:ident, $lit:literal) => {
        axon::$i::new().parse($lit).unwrap()
    }
}

#[test]
fn defs_work() {
    p!(DefParser, "x: null");
    p!(DefParser, "x:null");
    p!(DefParser, "x:\nnull");
    p!(DefParser, "x:\ndo null end");
}

#[test]
fn do_blocks_work() {
    assert_eq!(p!(DoParser, "do null end").exprs.len(), 1);
    assert_eq!(p!(DoParser, "do null; null end").exprs.len(), 2);
    assert_eq!(p!(DoParser, "do\nnull\nnull\nend").exprs.len(), 2);
    assert_eq!(p!(DoParser, "do\nnull\nnull;end").exprs.len(), 2);
    assert_eq!(p!(DoParser, "do null; endFunc end").exprs.len(), 2);
}

#[test]
fn exprs_work() {
    assert_eq!(p!(ExprsParser, "null").exprs.len(), 1);
    assert_eq!(p!(ExprsParser, "null; null").exprs.len(), 2);
    assert_eq!(p!(ExprsParser, "null\nnull").exprs.len(), 2);
    assert_eq!(p!(ExprsParser, "null\nnull;").exprs.len(), 2);
}

#[test]
fn literals_work() {
    assert_eq!(p!(LitParser, "null"), Lit::Null);
    assert_eq!(p!(LitParser, "true"), Lit::Bool(true));
    assert_eq!(p!(LitParser, "false"), Lit::Bool(false));
}

#[test]
fn lambdas_work() {
    assert_eq!(p!(LambdaParser, "() => null").args.len(), 0);
    assert_eq!(p!(LambdaParser, "someVar => null").args.len(), 1);
    assert_eq!(p!(LambdaParser, "(someVar) => null").args.len(), 1);
    assert_eq!(p!(LambdaParser, "(oneVar, twoVar) => null").args.len(), 2);

    let lambda = p!(LambdaParser, "(oneVar: null, twoVar: null) => null");
    let args = lambda.args;
    assert_eq!(args[0].param, Id::new("oneVar"));
    assert_eq!(args[1].param, Id::new("twoVar"));
}
