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
    pub start: Index,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
}

#[derive(Clone)]
pub struct Func {
    pub functype: Index,
    pub locals: Vec<types::Value>,
    pub body: Vec<instr::Instr>,
}

#[derive(Clone, Copy)]
pub struct HostFunc {} // TODO move it

pub struct Table {
    pub tabletype: types::Table,
}

pub struct Mem {
    pub memtype: types::Mem,
}

pub struct Global {
    pub globaltype: types::Global,
    pub init: Vec<instr::Instr>, // TODO constant expression
}

pub enum ElemMode {
    Passive,
    Active(Index, Vec<instr::Instr>), // TODO constant expression
    Declarative,
}

pub struct Element {
    pub elemtype: types::Ref,
    pub init: Vec<instr::Instr>, // TODO constant expression
    pub mode: ElemMode,
}

pub enum DataMode {
    Passive,
    Active(Index, Vec<instr::Instr>), // TODO constant expression
}

pub struct Data {
    pub init: Vec<u8>,
    pub mode: DataMode,
}

pub enum ExportImportDesc {
    Func(Index),
    Table(Index),
    Mem(Index),
    Global(Index),
}

pub struct Export {
    name: String,
    desc: ExportImportDesc,
}

pub struct Import {
    module: String,
    name: String,
    desc: ExportImportDesc,
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
