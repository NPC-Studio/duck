use crate::parse::Identifier;

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Ty {
    Null,
    Any,
    Undefined,
    Noone,
    Bool,
    Real,
    Str,
    Var(Var),
    Array(Box<Ty>),
    Record(Record),
    Func(Func),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Func {
    Def(Def),
    Call(Call),
}
impl Func {
    pub fn parameters(&self) -> &[Ty] {
        match self {
            Func::Def(inner) => &inner.parameters,
            Func::Call(inner) => &inner.parameters,
        }
    }
    pub fn parameters_mut(&mut self) -> &mut [Ty] {
        match self {
            Func::Def(inner) => &mut inner.parameters,
            Func::Call(inner) => &mut inner.parameters,
        }
    }
    pub fn return_type(&self) -> &Ty {
        match self {
            Func::Def(inner) => &inner.return_type,
            Func::Call(inner) => &inner.return_type,
        }
    }
    pub fn return_type_mut(&mut self) -> &mut Ty {
        match self {
            Func::Def(inner) => &mut inner.return_type,
            Func::Call(inner) => &mut inner.return_type,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Def {
    pub binding: Option<Binding>,
    pub parameters: Vec<Ty>,
    pub return_type: Box<Ty>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    pub parameters: Vec<Ty>,
    pub return_type: Box<Ty>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Binding {
    Method {
        local_scope: Var,
        self_scope: Var,
    },
    Constructor {
        local_scope: Var,
        self_scope: Var,
        inheritance: Option<Identifier>,
    },
}