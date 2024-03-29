use super::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum LitVal {
    Int(i64),
    Real(f64),
    Bool(bool),
    Char(char),
    Unit,
}

impl LitVal {
    pub fn get_lit_type(&self) -> LitType {
        match self {
            LitVal::Int(_) => LitType::Int,
            LitVal::Real(_) => LitType::Real,
            LitVal::Bool(_) => LitType::Bool,
            LitVal::Char(_) => LitType::Char,
            LitVal::Unit => LitType::Unit,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Builtin {
    IAdd,
    ISub,
    IMul,
    IDiv,
    IRem,
    INeg,
    RAdd,
    RSub,
    RMul,
    RDiv,
    BAnd,
    BOr,
    BNot,
}

impl Builtin {
    pub fn get_arity(&self) -> usize {
        match self {
            Builtin::IAdd => 2,
            Builtin::ISub => 2,
            Builtin::IMul => 2,
            Builtin::IDiv => 2,
            Builtin::INeg => 1,
            Builtin::IRem => 2,
            Builtin::RAdd => 2,
            Builtin::RSub => 2,
            Builtin::RMul => 2,
            Builtin::RDiv => 2,
            Builtin::BAnd => 2,
            Builtin::BOr => 2,
            Builtin::BNot => 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Lit {
        lit: LitVal,
        span: Span,
    },
    Var {
        var: Ident,
        span: Span,
    },
    Prim {
        prim: Builtin,
        args: Vec<Expr>,
        span: Span,
    },
    Fun {
        pars: Vec<Ident>,
        body: Box<Expr>,
        span: Span,
    },
    App {
        func: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },
    ExtCall {
        func: InternStr,
        args: Vec<Expr>,
        span: Span,
    },
    Cons {
        cons: Ident,
        args: Vec<Expr>,
        span: Span,
    },
    Let {
        bind: Ident,
        expr: Box<Expr>,
        cont: Box<Expr>,
        span: Span,
    },
    Case {
        expr: Box<Expr>,
        rules: Vec<Rule>,
        span: Span,
    },
    Blk {
        decls: Vec<Decl>,
        cont: Box<Expr>,
        span: Span,
    },
}

impl Spanned for Expr {
    fn span(&self) -> &Span {
        match self {
            Expr::Lit { span, .. } => span,
            Expr::Var { span, .. } => span,
            Expr::Prim { span, .. } => span,
            Expr::Fun { span, .. } => span,
            Expr::App { span, .. } => span,
            Expr::ExtCall { span, .. } => span,
            Expr::Cons { span, .. } => span,
            Expr::Let { span, .. } => span,
            Expr::Case { span, .. } => span,
            Expr::Blk { span, .. } => span,
        }
    }
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Expr::Lit { span, .. } => span,
            Expr::Var { span, .. } => span,
            Expr::Prim { span, .. } => span,
            Expr::Fun { span, .. } => span,
            Expr::App { span, .. } => span,
            Expr::ExtCall { span, .. } => span,
            Expr::Cons { span, .. } => span,
            Expr::Let { span, .. } => span,
            Expr::Case { span, .. } => span,
            Expr::Blk { span, .. } => span,
        }
    }
}

impl Expr {
    pub fn is_simple(&self) -> bool {
        match self {
            Expr::Lit { .. } => true,
            Expr::Var { .. } => true,
            Expr::Prim { .. } => true,
            Expr::Fun { .. } => true,
            Expr::App { .. } => true,
            Expr::ExtCall { .. } => true,
            Expr::Cons { .. } => true,
            Expr::Let { .. } => false,
            Expr::Case { .. } => false,
            Expr::Blk { .. } => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rule {
    pub patn: Pattern,
    pub body: Expr,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    Var {
        var: Ident,
        span: Span,
    },
    Lit {
        lit: LitVal,
        span: Span,
    },
    Cons {
        cons: Ident,
        pars: Vec<Pattern>,
        span: Span,
    },
    Wild {
        span: Span,
    },
}

impl Pattern {
    pub fn is_wild_or_var(&self) -> bool {
        match self {
            Pattern::Var { .. } | Pattern::Wild { .. } => true,
            _ => false,
        }
    }
}
impl Pattern {
    pub fn get_freevars(&self) -> Vec<Ident> {
        let mut stack = vec![self];
        let mut vec = Vec::new();

        while let Some(with) = stack.pop() {
            match with {
                Pattern::Var { var, .. } => {
                    if !vec.contains(var) {
                        vec.push(*var);
                    }
                }
                Pattern::Lit { .. } => {}
                Pattern::Cons { pars, .. } => {
                    stack.extend(pars.into_iter());
                }
                Pattern::Wild { .. } => {}
            }
        }
        vec
    }
}

impl Spanned for Pattern {
    fn span(&self) -> &Span {
        match self {
            Pattern::Var { span, .. } => span,
            Pattern::Lit { span, .. } => span,
            Pattern::Cons { span, .. } => span,
            Pattern::Wild { span } => span,
        }
    }
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Pattern::Var { span, .. } => span,
            Pattern::Lit { span, .. } => span,
            Pattern::Cons { span, .. } => span,
            Pattern::Wild { span } => span,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Decl {
    Func {
        name: Ident,
        pars: Vec<Ident>,
        body: Box<Expr>,
        span: Span,
    },
    Data {
        name: Ident,
        pars: Vec<Ident>,
        vars: Vec<Varient>,
        span: Span,
    },
    Type {
        name: Ident,
        pars: Vec<Ident>,
        typ: Type,
        span: Span,
    },
    Extern {
        name: InternStr,
        pars: Vec<Ident>,
        typ: Type,
        span: Span,
    },
}

impl Decl {
    pub fn get_name(&self) -> Ident {
        match self {
            Decl::Func { name, .. } => *name,
            Decl::Data { name, .. } => *name,
            Decl::Type { name, .. } => *name,
            Decl::Extern { name, .. } => Ident::from(*name),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Varient {
    pub cons: Ident,
    pub pars: Vec<Type>,
    pub span: Span,
}

impl Spanned for Decl {
    fn span(&self) -> &Span {
        match self {
            Decl::Func { span, .. } => span,
            Decl::Data { span, .. } => span,
            Decl::Type { span, .. } => span,
            Decl::Extern { span, .. } => span,
        }
    }
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Decl::Func { span, .. } => span,
            Decl::Data { span, .. } => span,
            Decl::Type { span, .. } => span,
            Decl::Extern { span, .. } => span,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub enum LitType {
    Int,
    Real,
    Bool,
    Char,
    Unit,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Lit {
        lit: LitType,
        span: Span,
    },
    Var {
        var: Ident,
        span: Span,
    },
    Fun {
        pars: Vec<Type>,
        res: Box<Type>,
        span: Span,
    },
    App {
        cons: Ident,
        args: Vec<Type>,
        span: Span,
    },
}

impl Spanned for Type {
    fn span(&self) -> &Span {
        match self {
            Type::Lit { span, .. } => span,
            Type::Var { span, .. } => span,
            Type::Fun { span, .. } => span,
            Type::App { span, .. } => span,
        }
    }
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Type::Lit { span, .. } => span,
            Type::Var { span, .. } => span,
            Type::Fun { span, .. } => span,
            Type::App { span, .. } => span,
        }
    }
}
