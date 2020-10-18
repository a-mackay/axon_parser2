#[derive(Debug)]
pub struct Arg(Expr);

impl Arg {
    pub fn new(expr: Expr) -> Self {
        Self(expr)
    }

}

#[derive(Debug)]
pub struct Args {
    args: Vec<Arg>,
}

impl Args {
    pub fn new(args: Vec<Arg>) -> Self {
        Self {
            args,
        }
    }

    pub fn empty() -> Self {
        Self {
            args: vec![],
        }
    }

    pub fn into_vec(self) -> Vec<Arg> {
        self.args
    }
}

#[derive(Debug)]
pub struct Assign {
    id: Id,
    expr: Expr,
}

impl Assign {
    pub fn new(id: Id, expr: Expr) -> Self {
        Self {
            id,
            expr,
        }
    }
}

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
pub struct FnCall {
    pub name: Id,
    pub args: Args,
}

impl FnCall {
    pub fn new(name: Id, args: Args) -> Self {
        Self {
            name,
            args,
        }
    }
}

#[derive(Debug)]
pub struct OneLiner {
    //
}

#[derive(Debug)]
pub struct Lines {
    pub lines: Vec<ExprOrStmt>,
}

impl Lines {
    pub fn new(lines: Vec<ExprOrStmt>) -> Self {
        Self { lines }
    }

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

#[derive(Debug)]
pub struct TermChain {
    pub term_base: TermBase,
    pub items: Vec<TermChainItem>,
}

impl TermChain {
    pub fn new(term_base: TermBase, items: Vec<TermChainItem>) -> Self {
        Self {
            term_base,
            items,
        }
    }
}

#[derive(Debug)]
pub enum TermChainItem {
    FnCall(FnCall),
    // Trap
    // Get
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
    Assign(Assign),
    Def(Def),
    Return(Expr),
    Throw(Expr),
}

#[derive(Debug)]
pub enum ExprOrStmt {
    Expr(Expr),
    Stmt(Stmt),
}

#[derive(Debug)]
pub enum TermBase {
    FnCall(FnCall),
    Var(Id),
    Lit(Lit),
}