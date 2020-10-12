#[derive(Debug)]
pub struct Def {
    id: Id,
    expr: Expr,
}

impl Def {
    pub fn new(id: Id, expr: Expr) -> Self {
        Self {
            id,
            expr,
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Def(Box<Def>),
    Do(Exprs),
    Id(Id),
    Lambda(Lambda),
    Lit(Lit),
}

#[derive(Debug)]
pub struct Exprs {
    pub exprs: Vec<Expr>,
}

impl Exprs {
    pub fn new(exprs: Vec<Expr>) -> Self {
        Self { exprs }
    }
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
    pub args: Vec<Param>,
    pub body: Box<Expr>,
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
    pub param: Id,
    pub default_arg: Option<Expr>,
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
