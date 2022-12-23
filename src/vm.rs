use alloc::vec::Vec;

use crate::{
    instr::Instr,
    runtime::{ModuleInstance, Num, Ref, Store, Val},
};

struct VM {
    store: Store,
    thread: Thread,
}

pub trait Stack {
    fn push(&mut self, entry: StackEntry);
    fn pop(&mut self) -> Option<StackEntry>;
    fn push_into<T: Into<StackEntry>>(&mut self, val: T);
    fn pop_from<T: From<StackEntry>>(&mut self) -> T;
    fn peek(&self) -> Option<StackEntry>;
    fn unop<T: From<StackEntry> + Into<StackEntry>>(&mut self, f: &dyn Fn(T) -> T);
    fn binop<T: From<StackEntry> + Into<StackEntry>>(&mut self, f: &dyn Fn(T, T) -> T);
    fn testop<T: From<bool> + From<StackEntry> + Into<StackEntry>>(
        &mut self,
        f: &dyn Fn(T) -> bool,
    );
    fn relop<T: From<bool> + From<StackEntry> + Into<StackEntry>>(
        &mut self,
        f: &dyn Fn(T, T) -> bool,
    );
}

impl Stack for Vec<StackEntry> {
    fn push(&mut self, entry: StackEntry) {
        self.push(entry)
    }

    fn push_into<T: Into<StackEntry>>(&mut self, val: T) {
        self.push(val.into());
    }

    fn pop_from<T: From<StackEntry>>(&mut self) -> T {
        // TODO validate top of stack
        match self.pop().unwrap() {
            entry => T::from(entry),
            _ => panic!("not a value"),
        }
    }

    fn pop(&mut self) -> Option<StackEntry> {
        todo!()
    }

    fn peek(&self) -> Option<StackEntry> {
        todo!()
    }

    fn unop<T: From<StackEntry> + Into<StackEntry>>(&mut self, f: &dyn Fn(T) -> T) {
        // TODO validate top of stack
        let val = self.pop_from();
        let res = f(val);
        self.push_into(res);
    }

    fn binop<T: From<StackEntry> + Into<StackEntry>>(&mut self, f: &dyn Fn(T, T) -> T) {
        // TODO validate top of stack
        let val1 = self.pop_from();
        let val2 = self.pop_from();
        let res = f(val1, val2);
        self.push_into(res);
    }
    fn testop<T: From<bool> + From<StackEntry> + Into<StackEntry>>(
        &mut self,
        f: &dyn Fn(T) -> bool,
    ) {
        // TODO validate top of stack
        let val = self.pop_from();
        let res = f(val);
        if res {
            self.push_into(1u32);
        } else {
            self.push_into(0u32);
        }
    }
    fn relop<T: From<bool> + From<StackEntry> + Into<StackEntry>>(
        &mut self,
        f: &dyn Fn(T, T) -> bool,
    ) {
        // TODO validate top of stack
        let val1 = self.pop_from();
        let val2 = self.pop_from();
        if f(val1, val2) {
            self.push_into(1u32);
        } else {
            self.push_into(0u32);
        }
    }
}

pub enum StackEntry {
    Value(Val),
    Label(Label),
    Activation(i64, Frame),
}

impl From<StackEntry> for u32 {
    fn from(entry: StackEntry) -> Self {
        // TODO validate?
        match entry {
            StackEntry::Value(Val::Num(Num::I32(val))) => val,
            _ => panic!("not a u32 value"),
        }
    }
}

impl Into<StackEntry> for u32 {
    fn into(self) -> StackEntry {
        StackEntry::Value(Val::Num(Num::I32(self)))
    }
}

impl From<StackEntry> for u64 {
    fn from(entry: StackEntry) -> Self {
        // TODO validate?
        match entry {
            StackEntry::Value(Val::Num(Num::I64(val))) => val,
            _ => panic!("not a u64 value"),
        }
    }
}

impl Into<StackEntry> for u64 {
    fn into(self) -> StackEntry {
        StackEntry::Value(Val::Num(Num::I64(self)))
    }
}

impl Into<StackEntry> for f32 {
    fn into(self) -> StackEntry {
        StackEntry::Value(Val::Num(Num::F32(self)))
    }
}

impl Into<StackEntry> for f64 {
    fn into(self) -> StackEntry {
        StackEntry::Value(Val::Num(Num::F64(self)))
    }
}

pub struct Label {
    pub arity: i64,
    pub instr: Vec<Instr>,
}

pub struct Frame {
    pub locals: Vec<Val>,
    pub module: ModuleInstance,
}

pub struct Thread {
    frame: Frame,
    instr: Vec<Instr>,
}

pub struct Trap {} // TODO
