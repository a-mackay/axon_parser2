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
    assert_eq!(p!(DoParser, "do null end").lines.len(), 1);
    assert_eq!(p!(DoParser, "do null; null end").lines.len(), 2);
    assert_eq!(p!(DoParser, "do\nnull\nnull\nend").lines.len(), 2);
    assert_eq!(p!(DoParser, "do\nnull\nnull;end").lines.len(), 2);
    assert_eq!(p!(DoParser, "do null; endFunc end").lines.len(), 2);
}

#[test]
fn exprs_work() {
    assert_eq!(p!(LinesParser, "null").lines.len(), 1);
    assert_eq!(p!(LinesParser, "null; null").lines.len(), 2);
    assert_eq!(p!(LinesParser, "null\nnull").lines.len(), 2);
    assert_eq!(p!(LinesParser, "null\nnull;").lines.len(), 2);
    assert_eq!(p!(LinesParser, "x: 1\nnull;").lines.len(), 2);
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
