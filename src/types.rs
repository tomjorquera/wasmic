use alloc::vec::Vec;

use crate::validation::{Context, Validable};

pub type Index = usize;
pub type Addr = usize;

#[derive(Clone, Copy)]
pub enum Number {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Clone, Copy)]
pub enum Ref {
    Func,
    Extern,
}

#[derive(Clone, Copy)]
pub enum Value {
    Num(Number),
    Vec,
    Ref(Ref),
}

#[derive(Clone)]
pub struct Function {
    pub input: Vec<Value>,
    pub output: Vec<Value>,
}

#[derive(Clone, Copy)]
pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

#[derive(Clone, Copy)]
pub struct Mem {
    pub limits: Limits,
}

#[derive(Clone, Copy)]
pub struct Table {
    pub limits: Limits,
    pub reftype: Ref,
}

#[derive(Clone, Copy)]
pub enum Mut {
    Const,
    Var,
}

#[derive(Clone, Copy)]
pub struct Global {
    pub mutable: Mut,
    pub val: Value,
}

#[derive(Clone)]
pub enum Extern {
    Func(Function),
    Table(Table),
    Mem(Mem),
    Global(Global),
}

// Validation

impl Validable for Number {
    fn is_valid(&self, _: &Context, _: Option<u32>) -> bool {
        true
    }
}

impl Validable for Ref {
    fn is_valid(&self, _: &Context, _: Option<u32>) -> bool {
        true
    }
}

impl Validable for Value {
    fn is_valid(&self, _: &Context, _: Option<u32>) -> bool {
        true
    }
}

impl Validable for Result {
    fn is_valid(&self, _: &Context, _: Option<u32>) -> bool {
        true
    }
}

impl Validable for Function {
    fn is_valid(&self, _: &Context, _: Option<u32>) -> bool {
        true
    }
}

impl Validable for Limits {
    fn is_valid(&self, context: &Context, k: Option<u32>) -> bool {
        match (self.max, k) {
            (None, None) => true,
            (Some(max), None) => self.min <= max,
            (None, Some(limit)) => self.min <= limit,
            (Some(max), Some(limit)) => self.min <= max && self.min <= limit && max <= limit,
        }
    }
}

impl Validable for Mem {
    fn is_valid(&self, context: &Context, _: Option<u32>) -> bool {
        self.limits.is_valid(context, Some(2_u32.pow(16)))
    }
}

impl Validable for Table {
    fn is_valid(&self, context: &Context, _: Option<u32>) -> bool {
        self.limits.is_valid(context, Some(u32::MAX))
    }
}

impl Validable for Mut {
    fn is_valid(&self, _: &Context, _: Option<u32>) -> bool {
        true
    }
}

impl Validable for Global {
    fn is_valid(&self, _: &Context, _: Option<u32>) -> bool {
        true
    }
}

impl Validable for Extern {
    fn is_valid(&self, context: &Context, k: Option<u32>) -> bool {
        match self {
            Extern::Func(fun) => fun.is_valid(context, k),
            Extern::Table(table) => table.is_valid(context, k),
            Extern::Mem(mem) => mem.is_valid(context, k),
            Extern::Global(glob) => glob.is_valid(context, k),
        }
    }
}
