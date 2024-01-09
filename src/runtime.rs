use core::cell::RefCell;

use alloc::{string::String, vec::Vec};

use crate::{
    instr::Instr,
    modules::Func,
    types::{self, Addr},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Num {
    I32(u32),
    I64(u64),
    F32(f32),
    F64(f64),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Ref {
    Null(types::Ref),
    Func(Addr),
    Extern(Addr),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Val {
    Num(Num),
    Vec(u128),
    Ref(Ref),
}

impl Val {
    fn default_i32() -> Val {
        Val::Num(Num::I32(0))
    }
    fn default_i64() -> Val {
        Val::Num(Num::I64(0))
    }
    fn default_f32() -> Val {
        Val::Num(Num::F32(0.))
    }
    fn default_f64() -> Val {
        Val::Num(Num::F64(0.))
    }
    fn default_vec() -> Val {
        Val::Vec(0)
    }
    fn default_ref(reftype: types::Ref) -> Val {
        Val::Ref(Ref::Null(reftype))
    }
}

pub enum Res {
    Res(Vec<Val>),
    Trap,
}

pub struct Store<'a> {
    pub modules: Vec<RefCell<ModuleInstance>>, // To guarantee soundness, the Store need to own the instantiated modules
    pub funcinstances: Vec<FuncInstance<'a>>,
    pub tables: Vec<RefCell<Table>>,
    pub mems: Vec<RefCell<Mem>>,
    pub globals: Vec<RefCell<Global>>,
    pub elems: Vec<RefCell<Elem>>,
    pub datas: Vec<RefCell<Data>>,
}

impl<'a> Store<'a> {
    pub fn new() -> Store<'a> {
        Store {
            modules: vec![],
            funcinstances: vec![],
            tables: vec![],
            mems: vec![],
            globals: vec![],
            elems: vec![],
            datas: vec![],
        }
    }
}

pub struct ModuleInstance {
    pub types: Vec<types::Function>,
    pub funct: Vec<types::Addr>,
    pub tables: Vec<types::Addr>,
    pub mems: Vec<types::Addr>,
    pub globals: Vec<types::Addr>,
    pub elems: Vec<types::Addr>,
    pub datas: Vec<types::Addr>,
    pub exports: Vec<Export>,
}

impl ModuleInstance {
    pub fn new() -> ModuleInstance {
        ModuleInstance {
            types: vec![],
            funct: vec![],
            tables: vec![],
            mems: vec![],
            globals: vec![],
            elems: vec![],
            datas: vec![],
            exports: vec![],
        }
    }
}

pub enum FuncInstance<'a> {
    Internal(InternalFuncInstance<'a>),
    Host(HostFuncInstance),
}

pub struct InternalFuncInstance<'a> {
    pub functype: types::Function,
    pub module: &'a RefCell<ModuleInstance>,
    pub code: Func,
}
pub struct HostFuncInstance {
    pub functype: types::Function,
    //pub extrnfunc: fn(usize) -> usize, // TODO define Host function invocation mechanism
}

pub struct Table {
    pub tabletype: types::Table,
    pub elem: Vec<Ref>,
}
pub struct Mem {
    pub memtype: types::Mem,
    pub data: Vec<types::Byte>,
}
pub struct Global {
    pub globaltype: types::Global,
    pub value: Val,
}
pub struct Elem {
    pub elemtype: types::Ref,
    pub elem: Vec<Ref>,
}
pub struct Data {
    pub data: Vec<types::Byte>,
}
pub struct Export {
    pub name: String,
    pub value: ExternalVal,
}

#[derive(Clone, Copy)]
pub enum ExternalVal {
    Fun(Addr),
    Table(Addr),
    Mem(Addr),
    Global(Addr),
}

pub struct Label<'a> {
    pub arity: usize,
    pub instr: &'a Vec<Instr>,
}

pub struct FrameState<'a> {
    pub locals: Vec<Val>,
    pub module: &'a RefCell<ModuleInstance>,
}

pub struct Frame<'a> {
    pub arity: usize,
    pub framestate: RefCell<FrameState<'a>>,
}

impl<'a> Frame<'a> {
    pub fn new(module: &'a RefCell<ModuleInstance>) -> Frame<'a> {
        Frame {
            arity: 0,
            framestate: RefCell::new(FrameState {
                locals: vec![],
                module,
            }),
        }
    }
}

pub enum StackEntry<'a> {
    Value(Val),
    Label(Label<'a>),
    Activation(i64, &'a Frame<'a>),
}

impl<'a> From<StackEntry<'a>> for u32 {
    fn from(entry: StackEntry) -> Self {
        // TODO validate?
        match entry {
            StackEntry::Value(Val::Num(Num::I32(val))) => val,
            _ => panic!("not a u32 value"),
        }
    }
}

impl<'a> Into<StackEntry<'a>> for u32 {
    fn into(self) -> StackEntry<'a> {
        StackEntry::Value(Val::Num(Num::I32(self)))
    }
}

impl<'a> From<StackEntry<'a>> for u64 {
    fn from(entry: StackEntry) -> Self {
        // TODO validate?
        match entry {
            StackEntry::Value(Val::Num(Num::I64(val))) => val,
            _ => panic!("not a u64 value"),
        }
    }
}

impl<'a> Into<StackEntry<'a>> for u64 {
    fn into(self) -> StackEntry<'a> {
        StackEntry::Value(Val::Num(Num::I64(self)))
    }
}

impl<'a> Into<StackEntry<'a>> for f32 {
    fn into(self) -> StackEntry<'a> {
        StackEntry::Value(Val::Num(Num::F32(self)))
    }
}

impl<'a> Into<StackEntry<'a>> for f64 {
    fn into(self) -> StackEntry<'a> {
        StackEntry::Value(Val::Num(Num::F64(self)))
    }
}
