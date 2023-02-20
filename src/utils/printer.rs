use crate::backend::anf::*;
use crate::frontend::ast::*;
use itertools::Itertools;
use std::cell::Cell;
use std::fmt::{self, Debug, Display};

pub struct INDT;
pub struct DEDT;
pub struct NWLN;

thread_local! {
    static INDT_LEVEL: Cell<usize> = Cell::new(0);
}

impl Display for INDT {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        INDT_LEVEL.with(|c| {
            let x = c.get();
            c.set(x + 1);
        });
        Ok(())
    }
}

impl Display for DEDT {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        INDT_LEVEL.with(|c| {
            let x = c.get();
            c.set(x - 1);
        });
        Ok(())
    }
}

impl Display for NWLN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        INDT_LEVEL.with(|c| write!(f, "\n{:width$}", "", width = c.get() * 2))
    }
}

impl Debug for INDT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self)
    }
}

impl Debug for DEDT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self)
    }
}

impl Debug for NWLN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self)
    }
}

impl Display for LitVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LitVal::Int(x) => write!(f, "{x}"),
            LitVal::Real(x) => write!(f, "{x}"),
            LitVal::Bool(x) => write!(f, "{x}"),
            LitVal::Char(x) => write!(f, "{x}"),
            LitVal::Unit => write!(f, "()"),
        }
    }
}

impl Display for LitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LitType::Int => write!(f, "Int"),
            LitType::Real => write!(f, "Real"),
            LitType::Bool => write!(f, "Bool"),
            LitType::Char => write!(f, "Char"),
            LitType::Unit => write!(f, "()"),
        }
    }
}

impl Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Builtin::IAdd => write!(f, "iadd"),
            Builtin::ISub => write!(f, "isub"),
            Builtin::IMul => write!(f, "imul"),
            Builtin::IDiv => write!(f, "idiv"),
            Builtin::IRem => write!(f, "irem"),
            Builtin::INeg => write!(f, "ineg"),
            Builtin::RAdd => write!(f, "radd"),
            Builtin::RSub => write!(f, "rsub"),
            Builtin::RMul => write!(f, "rmul"),
            Builtin::RDiv => write!(f, "rdiv"),
            Builtin::BAnd => write!(f, "band"),
            Builtin::BOr => write!(f, "bor"),
            Builtin::BNot => write!(f, "bnot"),
            Builtin::ICmpEq => write!(f, "icmpeq"),
            Builtin::ICmpNe => write!(f, "icmpne"),
            Builtin::ICmpGr => write!(f, "icmpgr"),
            Builtin::ICmpGe => write!(f, "icmpge"),
            Builtin::ICmpLs => write!(f, "icmpls"),
            Builtin::ICmpLe => write!(f, "icmple"),
            Builtin::RCmpEq => write!(f, "rcmpeq"),
            Builtin::RCmpNe => write!(f, "rcmpne"),
            Builtin::RCmpGr => write!(f, "rcmpgr"),
            Builtin::RCmpGe => write!(f, "rcmpge"),
            Builtin::RCmpLs => write!(f, "rcmpls"),
            Builtin::RCmpLe => write!(f, "rcmple"),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Lit { lit, .. } => {
                write!(f, "{lit}")
            }
            Expr::Var { var, .. } => {
                write!(f, "{var}")
            }
            Expr::Prim { prim, args, .. } => {
                let args = args.iter().format(&", ");
                write!(f, "@{prim}({args})")
            }
            Expr::Fun { pars, body, .. } => {
                let pars = pars.iter().format(&", ");
                write!(f, "fn ({pars}) {{{INDT}{NWLN}{body}{DEDT}{NWLN}}}")
            }
            Expr::App { func, args, .. } => {
                let args = args.iter().format(&", ");
                write!(f, "{func}({args})")
            }
            Expr::ExtCall { func, args, .. } => {
                let args = args.iter().format(&", ");
                write!(f, "#{func}({args})")
            }
            Expr::Cons { cons, args, .. } => {
                let args = args.iter().format(&", ");
                write!(f, "{cons}({args})")
            }
            Expr::Case { expr, rules, .. } => {
                assert!(!rules.is_empty());
                write!(f, "case {expr} of")?;
                for rule in rules {
                    write!(f, "{NWLN}| {rule}")?;
                }
                write!(f, "{NWLN}end")
            }
            Expr::Ifte {
                cond, trbr, flbr, ..
            } => {
                write!(f, "if {cond}{NWLN}then {trbr}{NWLN}else {flbr}")
            }
            Expr::Begin { block, .. } => {
                write!(f, "begin{INDT}{NWLN}")?;
                write!(f, "{block}")?;
                write!(f, "{DEDT}{NWLN}end")
            }
            Expr::Letrec { decls, block, .. } => {
                write!(f, "letrec{INDT}")?;
                for decl in decls {
                    write!(f, "{NWLN}{decl}")?;
                }
                write!(f, "{DEDT}{NWLN}in{INDT}{NWLN}")?;
                write!(f, "{block}")?;
                write!(f, "{DEDT}{NWLN}end")
            }
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Block { stmts, retn, .. } = self;
        let mut first = true;
        for stmt in stmts {
            if first {
                first = false;
                write!(f, "{stmt}")?;
            } else {
                write!(f, "{NWLN}{stmt}")?;
            }
        }
        if let Some(retn) = retn {
            if first {
                write!(f, "{retn}")?;
            } else {
                write!(f, "{NWLN}{retn}")?;
            }
        }
        Ok(())
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Bind {
                bind, typ, expr, ..
            } => {
                if let Some(typ) = typ {
                    write!(f, "let {bind}: {typ} = {expr};")
                } else {
                    write!(f, "let {bind} = {expr};")
                }
            }
            Stmt::Do { expr, .. } => {
                write!(f, "{expr};")
            }
        }
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pattern::Var { var, .. } => {
                write!(f, "{var}")
            }
            Pattern::Lit { lit, .. } => {
                write!(f, "{lit}")
            }
            Pattern::Cons { cons, pars, .. } => {
                if pars.is_empty() {
                    write!(f, "{cons}")
                } else {
                    let pars = pars.iter().format(&", ");
                    write!(f, "{cons}({pars})")
                }
            }
            Pattern::Wild { .. } => {
                write!(f, "_")
            }
        }
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Rule { patn, body, .. } = self;
        write!(f, "{patn} => {body}")
    }
}

impl Display for Varient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Varient { cons, pars, .. } = self;
        if pars.is_empty() {
            write!(f, "{cons}")
        } else {
            let pars = pars.iter().format(&", ");
            write!(f, "{cons}[{pars}]")
        }
    }
}

impl Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Decl::Func {
                name,
                gens,
                pars,
                res,
                body,
                ..
            } => {
                let gens = if gens.is_empty() {
                    "".to_string()
                } else {
                    format!("[{}]", gens.iter().format(&", "))
                };
                let pars = pars
                    .iter()
                    .map(|(par, typ)| format!("{par}: {typ}"))
                    .format(&", ");
                let res = if matches!(
                    res,
                    Type::Lit {
                        lit: LitType::Unit,
                        ..
                    }
                ) {
                    "".to_string()
                } else {
                    format!(": {res}")
                };
                write!(f, "fun {name}{gens}({pars}){res} = {body}")
            }
            Decl::Data {
                name, pars, vars, ..
            } => {
                if pars.is_empty() {
                    write!(f, "data {name} =")?;
                } else {
                    let pars = pars.iter().format(&", ");
                    write!(f, "data {name}[{pars}] =")?;
                }
                assert!(!vars.is_empty());
                for var in vars {
                    write!(f, "{NWLN}| {var}")?;
                }
                write!(f, "{NWLN}end")
            }
            Decl::Type {
                name, pars, typ, ..
            } => {
                if pars.is_empty() {
                    write!(f, "type {name} = {typ};")
                } else {
                    let pars = pars.iter().format(&", ");
                    write!(f, "type {name}[{pars}] = {typ};")
                }
            }
            Decl::Extern {
                name,
                gens: pars,
                typ,
                ..
            } => {
                let pars = if pars.is_empty() {
                    "".to_string()
                } else {
                    format!("[{}]", pars.iter().format(&", "))
                };
                write!(f, "extern {name}{pars}: {typ};")
            }
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Lit { lit, .. } => {
                write!(f, "{lit}")
            }
            Type::Var { var, .. } => {
                write!(f, "{var}")
            }
            Type::Fun { pars, res, .. } => {
                let pars = pars.iter().format(&", ");
                write!(f, "fn({pars}) -> {res}")
            }
            Type::App { cons, args, .. } => {
                assert!(!args.is_empty());
                let args = args.iter().format(&", ");
                write!(f, "{cons}[{args}]")
            }
        }
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atom::Var(x) => write!(f, "{x}"),
            Atom::Int(x) => write!(f, "{x}"),
            Atom::Real(x) => write!(f, "{x}"),
            Atom::Bool(x) => write!(f, "{x}"),
            Atom::Char(x) => write!(f, "{x}"),
            Atom::Unit => write!(f, "()"),
        }
    }
}

impl Display for UnOpPrim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnOpPrim::Move => write!(f, "move"),
            UnOpPrim::INeg => write!(f, "ineg"),
        }
    }
}

impl Display for BinOpPrim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinOpPrim::IAdd => write!(f, "iadd"),
            BinOpPrim::ISub => write!(f, "isub"),
            BinOpPrim::IMul => write!(f, "imul"),
            BinOpPrim::ICmpEq => write!(f, "icmpeq"),
            BinOpPrim::ICmpNe => write!(f, "icmpne"),
            BinOpPrim::ICmpGr => write!(f, "icmpgr"),
            BinOpPrim::ICmpGe => write!(f, "icmpge"),
            BinOpPrim::ICmpLs => write!(f, "icmpls"),
            BinOpPrim::ICmpLe => write!(f, "icmple"),
        }
    }
}

impl Display for MExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MExpr::LetIn { decls, cont } => {
                write!(f, "letrec{INDT}")?;
                for decl in decls {
                    write!(f, "{NWLN}{decl}")?;
                }
                write!(f, "{DEDT}{NWLN}in{INDT}{NWLN}{cont}{DEDT}{NWLN}end")
            }
            MExpr::UnOp {
                bind,
                prim,
                arg1,
                cont,
            } => {
                write!(f, "let {bind} = {prim}({arg1});{NWLN}{cont}")
            }
            MExpr::BinOp {
                bind,
                prim,
                arg1,
                arg2,
                cont,
            } => {
                write!(f, "let {bind} = {prim}({arg1},{arg2});{NWLN}{cont}")
            }
            MExpr::Call {
                bind,
                func,
                args,
                cont,
            } => {
                let args = args.iter().format(&", ");
                write!(f, "let {bind} = {func}({args});{NWLN}{cont}")
            }
            MExpr::ExtCall {
                bind,
                func,
                args,
                cont,
            } => {
                let args = args.iter().format(&", ");
                write!(f, "let {bind} = {func}({args});{NWLN}{cont}")
            }
            MExpr::Retn { arg1 } => {
                write!(f, "return {arg1}")
            }
            MExpr::Alloc { bind, size, cont } => {
                write!(f, "let {bind} = alloc[{size}];{NWLN}{cont}")
            }
            MExpr::Load {
                bind,
                arg1,
                index,
                cont,
            } => {
                write!(f, "let {bind} = load {arg1}[{index}];{NWLN}{cont}")
            }
            MExpr::Store {
                arg1,
                index,
                arg2,
                cont,
            } => {
                write!(f, "store {arg1}[{index}] := {arg2};{NWLN}{cont}")
            }
            MExpr::Offset {
                bind,
                arg1,
                index,
                cont,
            } => {
                write!(f, "let {bind} = offset {arg1}[{index}];{NWLN}{cont}")
            }
            MExpr::Ifte {
                bind,
                arg1,
                brch1,
                brch2,
                cont,
            } => {
                write!(f, "let {bind} = if({arg1}) then")?;
                write!(f, "{INDT}{NWLN}{brch1}{DEDT}{NWLN}else")?;
                write!(f, "{INDT}{NWLN}{brch2}{DEDT}{NWLN};{NWLN}{cont}")
            }
            MExpr::Switch {
                bind,
                arg1,
                brchs,
                dflt,
                cont,
            } => {
                write!(f, "let {bind} = switch({arg1}) {{{INDT}")?;
                for (i, brch) in brchs.iter() {
                    write!(f, "{NWLN}case {i}:{INDT}{NWLN}{brch}{DEDT}")?;
                }
                if let Some(dflt) = dflt {
                    write!(f, "{NWLN}default:{INDT}{NWLN}{dflt}{DEDT}")?;
                }
                write!(f, "{DEDT}{NWLN}}}{NWLN}{cont}")
            }
        }
    }
}

impl Display for MDecl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let MDecl { func, pars, body } = self;
        let pars = pars.iter().format(&", ");
        write!(f, "fun {func}({pars}) = {INDT}{NWLN}{body}{DEDT}")
    }
}

#[test]
pub fn printer_ident_test() {
    let string1 = format!(
        "\n\
        hello{INDT}{NWLN}\
        world{INDT}{NWLN}\
        hello{INDT}{NWLN}\
        world{DEDT}{NWLN}\
        hello{DEDT}{NWLN}\
        world{DEDT}{NWLN}\
        hello world!\n\
    "
    );

    let string2 = r#"
hello
  world
    hello
      world
    hello
  world
hello world!
"#;

    assert_eq!(string1, string2)
}
