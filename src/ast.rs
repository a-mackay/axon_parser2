#[derive(Debug)]
pub enum Expr {
    Id(Id),
    Lambda(Lambda),
    Lit(Lit),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Id(String);

impl Id {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Debug)]
pub struct Lambda {
    args: Vec<Param>,
    body: Box<Expr>,
}

impl Lambda {
    pub fn new(args: Vec<Param>, body: Expr) -> Self {
        Self {
            args: args,
            body: Box::new(body),
        }
    }

    pub fn new_no_args(body: Expr) -> Self {
        Self::new(vec![], body)
    }
}

#[derive(Debug, PartialEq)]
pub enum Lit {
    Bool(bool),
    Null,
    Num(f64),
}

#[derive(Debug)]
pub struct Param {
    param: Id,
    default_arg: Option<Expr>,
}

impl Param {
    pub fn new(param: Id) -> Self {
        Self {
            param,
            default_arg: None,
        }
    }

    pub fn new_with_default_arg(param: Id, default_arg: Expr) -> Self {
        Self {
            param,
            default_arg: Some(default_arg),
        }
    }
}
