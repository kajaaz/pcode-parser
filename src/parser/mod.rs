/*
    Credits and Acknowledgments
    
    This Rust program was developed with the assistance and contributions of the following individuals and organizations:
    
    - @cchr-ledger : For providing the template for this parser, which was crucial for the successful completion of this project.
    - Michael Chesser and Rubens Brandao : For their extensive knowledge in binaries parsing and Rust language, which greatly enhanced the functionality of this program.

    Their expertise and support have been invaluable.
*/

#![allow(dead_code)]

pub mod parser_impl;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Opcode {
    BoolAnd,
    BoolNegate,
    BoolOr,
    BoolXor,
    Branch,
    BrandInd,
    CBranch,
    Call,
    CallInd,
    CallOther,
    CPoolRef,
    Copy,
    Float2Float,
    FloatAbs,
    FloatAdd,
    FloatCeil,
    FloatDiv,
    FloatEqual,
    FloatFloor,
    FloatLess,
    FloatLessEqual,
    FloatMult,
    FloatNaN,
    FloatNeg,
    FloatNotEqual,
    FloatRound,
    FloatSqrt,
    FloatSub,
    Int2Comp,
    Int2Float,
    IntAdd,
    IntAnd,
    IntCarry,
    IntDiv,
    IntEqual,
    IntLeft,
    IntLess,
    IntLessEqual,
    IntMult,
    IntNegate,
    IntNotEqual,
    IntOr,
    IntRem,
    IntRight,
    IntSdiv,
    IntSExt,
    IntSCarry,
    IntSBorrow,
    IntSLess,
    IntSLessEqual,
    IntSRem,
    IntSub,
    IntSright,
    IntXor,
    IntZExt,
    Load,
    New,
    Piece,
    Return,
    Store,
    Subpiece,
    Trunc,
    UserDefined,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Size {
    Byte,
    Half,
    Word,
    Quad,
}

impl Size {
    pub fn to_bitvector_size(self) -> u32 {
        match self {
            Size::Byte => 8,
            Size::Half => 16,
            Size::Word => 32,
            Size::Quad => 64,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Varnode {
    pub var: Var,
    pub size: Size,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Var {
    Const(String),
    Unique(Addr),
    Register(Addr),
    Memory(Addr),
}

#[derive(Clone, Debug)]
pub struct Inst {
    pub opcode: Opcode,
    pub output: Option<Varnode>,
    pub inputs: Vec<Varnode>,
}

pub type Addr = u64;

#[derive(Debug)]
pub struct CodeListing(BTreeMap<Addr, Vec<Inst>>);

impl CodeListing {
    pub fn new() -> Self {
        CodeListing(BTreeMap::new())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Value {
    pub data: i32,
}

impl Value {
    pub fn from_quad(input: u64) -> Self {
        // Assuming the input is i32,
        Self { data: input as i32 }
    }

    pub fn from_word(input: u32) -> Self {
        Self { data: input as i32 }
    }

    pub fn from_half(input: u16) -> Self {
        Self { data: input as i32 }
    }

    pub fn from_byte(input: u8) -> Self {
        Self { data: input as i32 }
    }
}
