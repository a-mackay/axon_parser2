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
    Do(Lines),
    Id(Id),
    Lambda(Lambda),
    Lit(Lit),
}

#[derive(Debug)]
pub struct Lines {
    pub lines: Vec<ExprOrStmt>,
}

impl Lines {
    pub fn new(lines: Vec<ExprOrStmt>) -> Self {
        Self { lines }
    }

    // pub fn new_from_expr(expr: Expr) -> Self {
    //     Self {
    //         lines: vec![ExprOrStmt::Expr(expr)],
    //     }
    // }

    // pub fn new_from_stmt(stmt: Stmt) -> Self {
    //     Self {
    //         lines: vec![ExprOrStmt::Stmt(stmt)],
    //     }
    // }

    pub fn new_from_eos(eos: ExprOrStmt) -> Self {
        Self {
            lines: vec![eos],
        }
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
    pub body: Box<Lines>,
}

impl Lambda {
    pub fn new(args: Vec<Param>, body: Lines) -> Self {
        Self {
            args: args,
            body: Box::new(body),
        }
    }

    pub fn new_no_args(body: Lines) -> Self {
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

#[derive(Debug)]
pub enum Stmt {
    Def(Def),
    Return(Expr),
    Throw(Expr),
}

#[derive(Debug)]
pub enum ExprOrStmt {
    Expr(Expr),
    Stmt(Stmt),
}