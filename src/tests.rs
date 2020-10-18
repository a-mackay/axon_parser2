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

fn termbase_is_lit(termbase: TermBase, lit: Lit) -> bool {
    match termbase {
        TermBase::Lit(l) => l == lit,
        _ => false,
    }
}

fn termbase_is_var(termbase: TermBase, var: Id) -> bool {
    match termbase {
        TermBase::Var(i) => i == var,
        _ => false,
    }
}

fn termbase_fncall_args(termbase: TermBase) -> Vec<Arg> {
    match termbase {
        TermBase::FnCall(fnc) => fnc.args.args,
        _ => panic!("Expected FnCall"),
    }
}

#[test]
fn termbases_work() {
    let tb = p!(TermBaseParser, "null");
    assert!(termbase_is_lit(tb, Lit::Null));
    let tb = p!(TermBaseParser, "true");
    assert!(termbase_is_lit(tb, Lit::Bool(true)));
    let tb = p!(TermBaseParser, "varName");
    assert!(termbase_is_var(tb, Id::new("varName")));
    let tb = p!(TermBaseParser, "varName()");
    assert_eq!(termbase_fncall_args(tb).len(), 0);

    let tb = p!(TermBaseParser, "varName(a)");
    assert_eq!(termbase_fncall_args(tb).len(), 1);

    let tb = p!(TermBaseParser, "varName(a, b)");
    assert_eq!(termbase_fncall_args(tb).len(), 2);
}