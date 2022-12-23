use alloc::vec::Vec;

use crate::{
    err::Err,
    modules, runtime,
    types::{self, Addr},
};

pub trait Store {
    fn new() -> Self;

    // Modules
    fn instantiate(
        &mut self,
        module: &modules::Module,
        externvals: Vec<runtime::ExternalVal>,
    ) -> Result<&runtime::ModuleInstance, Err>;

    // Functions
    fn func_alloc(&mut self, functype: types::Function, hostfunc: modules::HostFunc) -> Addr;
    fn func_type(&self, addr: Addr) -> types::Function;
    fn invoke(&mut self, addr: Addr, values: Vec<runtime::Val>) -> Result<Vec<runtime::Val>, Err>;

    //Tables
    fn table_alloc(&mut self, tabletype: types::Table) -> Addr;
    fn table_type(&self, addr: Addr) -> types::Table;
    fn table_read(&self, addr: Addr, index: usize) -> Result<runtime::Ref, Err>;
    fn table_write(&mut self, addr: Addr, index: usize, value: runtime::Ref) -> Result<(), Err>;
    fn table_size(&self, addr: Addr) -> usize;
    fn table_grow(&mut self, addr: Addr, n: u32, init: runtime::Ref) -> Result<(), Err>;

    // Memories
    fn mem_alloc(&mut self, memtyp: types::Mem) -> Addr;
    fn mem_type(&self, addr: Addr) -> types::Mem;
    fn mem_read(&self, addr: Addr, index: u32) -> Result<u8, Err>;
    fn mem_write(&mut self, addr: Addr, index: u32, value: u8) -> Result<(), Err>;
    fn mem_size(&self, addr: Addr) -> u32;
    fn mem_grow(&mut self, addr: Addr, n: u32, init: runtime::Ref) -> Result<(), Err>;

    // Globals
    fn global_alloc(&mut self, globtype: types::Global) -> Addr;
    fn global_type(&self, addr: Addr) -> types::Global;
    fn global_read(&self, addr: Addr) -> Result<runtime::Val, Err>;
    fn global_write(&mut self, addr: Addr, value: runtime::Val) -> Result<(), Err>;
}

pub trait Module: Sized {
    fn decode(bytes_: &Vec<u8>) -> Result<Self, Err>;
    fn parse(source: &str) -> Result<Self, Err>;
    fn validate(&self) -> Result<(), Err>;
}

pub trait ModuleInstance {
    fn export(&self, name: &str) -> Result<runtime::ExternalVal, Err>;
}
