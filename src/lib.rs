#[macro_use] extern crate lalrpop_util;

use regex::Regex;

// lalrpop_mod!(pub axon); // synthesized by LALRPOP

#[cfg(test)]
mod tests;
mod ast;
mod lex;

pub fn desugar(src: &str) -> String {
    let trap_re = Regex::new(r"->\s*([a-z][a-zA-Z0-9_]*)").unwrap();
    let src = trap_re.replace_all(src, ".trap(\"$1\")");

    let get_re = Regex::new(r###"([a-zA-Z0-9_\)\]\}"][[:space:]&&[^\n]]*)\[(.+)\]"###).unwrap();
    let src = get_re.replace_all(&src, "$1.get($2)");

    src.into()
}

fn highlight_keyword(src: &str, keyword: &str) -> String {
    let regex_str = format!("\\b({})\\b", keyword);
    let re = Regex::new(&regex_str).unwrap();
    let replacement_str = format!("___{}___", keyword.to_uppercase());
    re.replace_all(src, &replacement_str).into()
}

pub fn add_semicolons(src: &str) -> String {
    // ->
    unimplemented!()
}

#[cfg(test)]
mod desugar_tests {
    use super::desugar;

    #[test]
    fn desugar_trap_works() {
        let src = "var->test";
        let ds = desugar(src);
        assert_eq!(ds, "var.trap(\"test\")");
    }

    #[test]
    fn desugar_trap_with_whitespace_works() {
        let src = "var\n->\t\n test";
        let ds = desugar(src);
        assert_eq!(ds, "var\n.trap(\"test\")");
    }

    #[test]
    fn desugar_get_works() {
        assert_eq!(desugar("var[test]"), "var.get(test)");
        assert_eq!(desugar("var [0]"), "var .get(0)");
        assert_eq!(desugar("var\t [test123_]"), "var\t .get(test123_)");
    }

    #[test]
    fn desugar_get_does_not_run_if_there_are_newlines() {
        assert_eq!(desugar("var\n[test]"), "var\n[test]");
        assert_eq!(desugar("var \n\n [0]"), "var \n\n [0]");
        assert_eq!(desugar("var\t\n[test123_]"), "var\t\n[test123_]");
    }

    #[test]
    fn desugar_get_works_if_square_brackets_are_in_index_brackets() {
        assert_eq!(desugar("var[[0, 1, 2][0]]"), "var.get([0, 1, 2][0])");
        assert_eq!(desugar("var[\"]\"]"), "var.get(\"]\")");
    }
}