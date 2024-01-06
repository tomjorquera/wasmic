use alloc::string::String;
use alloc::vec::Vec;

use crate::embedding;
use crate::instr;
use crate::types;
use crate::types::Index;
use crate::validation::Validable;

pub struct Module {
    pub types: Vec<types::Function>,
    pub funcs: Vec<Func>,
    pub tables: Vec<Table>,
    pub mems: Vec<Mem>,
    pub globals: Vec<Global>,
    pub elems: Vec<Element>,
    pub datas: Vec<Data>,
    pub start: Option<Index>,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
}

#[derive(Clone)]
pub struct Func {
    pub functype: Index,
    pub locals: Vec<types::Value>,
    pub body: instr::Expr,
}

#[derive(Clone, Copy)]
pub struct HostFunc {}

pub struct Table {
    pub tabletype: types::Table,
}

pub struct Mem {
    pub memtype: types::Mem,
}

pub struct Global {
    pub globaltype: types::Global,
    pub init: instr::Expr, // TODO Validation: constant expression
}

pub enum ElemMode {
    Passive,
    Active(Index, instr::Expr), // TODO Validation: constant expression
    Declarative,
}

pub struct Element {
    pub elemtype: types::Ref,
    pub init: Vec<instr::Expr>, // TODO Validation: constant expression
    pub mode: ElemMode,
}

pub enum DataMode {
    Passive,
    Active(Index, instr::Expr), // TODO Validation: constant expression
}

pub struct Data {
    pub init: Vec<types::Byte>,
    pub mode: DataMode,
}

pub enum ExportDesc {
    Func(Index),
    Table(Index),
    Mem(Index),
    Global(Index),
}

pub struct Export {
    pub name: String,
    pub desc: ExportDesc,
}

pub enum ImportDesc {
    Func(Index),
    Table(types::Table),
    Mem(types::Mem),
    Global(types::Global),
}

pub struct Import {
    module: String,
    name: String,
    desc: ImportDesc,
}

impl embedding::Module for Module {
    fn decode(bytes_: &Vec<u8>) -> Result<Self, crate::err::Err> {
        todo!()
    }

    fn parse(source: &str) -> Result<Self, crate::err::Err> {
        todo!()
    }

    fn validate(&self) -> Result<(), crate::err::Err> {
        todo!()
    }
}

// Validation

impl Validable for Module {
    fn is_valid(&self, context: &crate::validation::Context, k: Option<u32>) -> bool {
        todo!()
    }
}
