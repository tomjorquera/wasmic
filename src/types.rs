use crate::validation::{Context, Subtypable, Validable};
use alloc::vec::Vec;

pub type Byte = u8;
pub type Int = usize; // TODO should be u32 here, but then causes issue with native vec index

pub type Index = Int;
pub type Addr = Int;

#[derive(Clone, Copy)]
pub enum Number {
    I32,
    I64,
    F32,
    F64,
}

// TODO Vector Types (sec 2.3.2)
#[derive(Clone, Copy)]
pub enum Vector {
    Unimplemented,
}

#[derive(Clone, Copy)]
pub enum Ref {
    Func,
    Extern,
}

#[derive(Clone, Copy)]
pub enum Value {
    Num(Number),
    Vec(Vector),
    Ref(Ref),
}

pub type Result = Vec<Value>;

#[derive(Clone)]
pub struct Function {
    pub input: Result,
    pub output: Result,
}

#[derive(Clone, Copy)]
pub struct Limits {
    pub min: Int,
    pub max: Option<Int>,
}

#[derive(Clone, Copy)]
pub struct Mem {
    pub limits: Limits, // Note limits are given in units of page size (sec 2.3.8)
}

#[derive(Clone, Copy)]
pub struct Table {
    pub limits: Limits,
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
    fn is_valid(&self, _: &Context, _: Option<Int>) -> bool {
        true
    }
}

impl Validable for Ref {
    fn is_valid(&self, _: &Context, _: Option<Int>) -> bool {
        true
    }
}

impl Validable for Value {
    fn is_valid(&self, _: &Context, _: Option<Int>) -> bool {
        true
    }
}

impl Validable for Function {
    fn is_valid(&self, _: &Context, _: Option<Int>) -> bool {
        true
    }
}

impl Validable for Limits {
    fn is_valid(&self, _: &Context, k: Option<Int>) -> bool {
        match (self.max, k) {
            (None, None) => true,
            (Some(max), None) => self.min <= max,
            (None, Some(limit)) => self.min <= limit,
            (Some(max), Some(limit)) => self.min <= max && self.min <= limit && max <= limit,
        }
    }
}

impl Subtypable for Limits {
    fn is_subtype(&self, other: &Limits) -> bool {
        match (self.max, other.max) {
            (_, None) => self.min >= other.min,
            (None, Some(_)) => false,
            (Some(max1), Some(max2)) => self.min >= other.min && max1 <= max2,
        }
    }
}

impl Validable for Mem {
    fn is_valid(&self, context: &Context, _: Option<Int>) -> bool {
        self.limits.is_valid(context, Some(2_u32.pow(16) as usize))
    }
}

impl Validable for Table {
    fn is_valid(&self, context: &Context, _: Option<Int>) -> bool {
        self.limits.is_valid(context, Some(u32::MAX as usize))
    }
}

impl Validable for Mut {
    fn is_valid(&self, _: &Context, _: Option<Int>) -> bool {
        true
    }
}

impl Validable for Global {
    fn is_valid(&self, _: &Context, _: Option<Int>) -> bool {
        true
    }
}

impl Validable for Extern {
    fn is_valid(&self, context: &Context, k: Option<Int>) -> bool {
        match self {
            Extern::Func(fun) => fun.is_valid(context, k),
            Extern::Table(table) => table.is_valid(context, k),
            Extern::Mem(mem) => mem.is_valid(context, k),
            Extern::Global(glob) => glob.is_valid(context, k),
        }
    }
}
