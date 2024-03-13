use crate::lexer::{Keyword, Operator, Token, Type};
use std::any::Any;
pub struct Atom {
    pub atype: AtomType,
    body: Option<Box<dyn Any>>,
}
pub enum AtomType {
    Program,
    BoxFunction,
    Assignment,
    Expression,
    Return,
    Variable,
    Arg,
    FunctionCall,
    VariableDec,
    VariableDef,
    RefAtom,
    Nil,
}

pub struct RefAtom {
    atom: *const Atom,
}

impl RefAtom {
    pub fn new(atom: *const Atom) -> Self {
        Self { atom }
    }
}

pub struct Program {
    name: String,
    crate_type: bool,
    start: *const Atom,
}

impl Program {
    pub fn new(name: String, crate_type: bool, start: *const Atom) -> Self {
        Program {
            name,
            crate_type,
            start,
        }
    }
}

pub struct Function {
    id: String,
    arguments: Vec<Atom>,
    return_type: Type,
    statements: Vec<Atom>,
}

impl Function {
    pub fn new(id: String, arguments: Vec<Atom>, return_type: Type, statements: Vec<Atom>) -> Self {
        Function {
            id,
            arguments,
            return_type,
            statements,
        }
    }
}

pub struct Return {
    expr: Atom,
}

impl Return {
    pub fn new(expr: Atom) -> Self {
        Self { expr }
    }
}

pub struct Statement {
    stype: AtomType,
    body: *const Atom,
}

impl Statement {
    pub fn new(stype: AtomType, body: *const Atom) -> Self {
        Self { stype, body }
    }
}

pub struct Assignment {
    lhs: *const Atom,
    rhs: Atom,
}

impl Assignment {
    pub fn new(lhs: *const Atom, rhs: Atom) -> Self {
        Self { lhs, rhs }
    }
}

pub struct Const<T> {
    ctype: Type,
    value: T,
}

impl<T> Const<T> {
    pub fn new(ctype: Type, value: T) -> Self {
        Self { ctype, value }
    }
}

pub struct BinaryOp {
    operator: Operator,
    lhs: Atom,
    rhs: Atom,
}

impl BinaryOp {
    pub fn new(operator: Operator, lhs: Atom, rhs: Atom) -> Self {
        Self { operator, lhs, rhs }
    }
}

pub struct Variable {
    id: String,
    vtype: Type,
}

impl Variable {
    pub fn new(id: String, vtype: Type) -> Self {
        Self { id, vtype }
    }
}

pub struct Arg {
    id: String,
    vtype: Type,
}

impl Arg {
    pub fn new(id: String, vtype: Type) -> Self {
        Self { id, vtype }
    }
}

pub struct Expression {
    etype: AtomType,
    body: *const Atom,
}

impl Expression {
    pub fn new(etype: AtomType, body: *const Atom) -> Self {
        Self { etype, body }
    }
}

pub struct VariableDec {
    id: String,
    vtype: Type,
}

impl VariableDec {
    pub fn new(id: String, vtype: Type) -> Self {
        Self { id, vtype }
    }
}

pub struct VariableDef {
    dec: *const Atom,
    assign: *const Atom,
}

impl VariableDef {
    pub fn new(dec: *const Atom, assign: *const Atom) -> Self {
        Self { dec, assign }
    }
}

pub struct FunctionCall {
    id: *const Atom,
    input: Vec<Atom>,
}

impl FunctionCall {
    pub fn new(id: *const Atom, input: Vec<Atom>) -> Self {
        Self { id, input }
    }
}

impl Atom {
    pub fn new(atype: AtomType, body: Option<Box<dyn Any>>) -> Self {
        Atom { atype, body }
    }
}
