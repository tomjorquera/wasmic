use alloc::vec::Vec;

use crate::types;

pub struct Context {
    pub types: Vec<types::Function>,
    pub funcs: Vec<types::Function>,
    pub tables: Vec<types::Table>,
    pub mems: Vec<types::Mem>,
    pub globals: Vec<types::Global>,
    pub elems: Vec<types::Ref>,
    pub data: Vec<()>,
    pub locals: Vec<types::Value>,
    pub labels: Vec<types::Result>,
    pub ret: Option<types::Result>,
    pub references: Vec<types::Index>,
}

pub trait Validable {
    fn is_valid(&self, context: &Context, k: Option<u32>) -> bool;
}
