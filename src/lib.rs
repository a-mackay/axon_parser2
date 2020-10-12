#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub axon); // synthesized by LALRPOP

#[test]
fn calculator1() {
    assert!(axon::TermParser::new().parse("22").is_ok());
    assert!(axon::TermParser::new().parse("(22)").is_ok());
    assert!(axon::TermParser::new().parse("((((22))))").is_ok());
    assert!(axon::TermParser::new().parse("((22)").is_err());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
