use alloc::vec::Vec;
use core::ops::{BitAnd, BitOr, BitXor, Not};

use crate::{
    instr::Instr,
    numeric::SupportedInteger,
    runtime::{ModuleInstance, Num, Ref, Store, Val},
};

trait Stack {
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

struct Thread {
    frame: Frame,
    program: Vec<Instr>,
}

pub struct Trap {} // TODO

trait VM {
    fn run(&mut self, thread: Thread);
}

impl VM for Store {
    fn run(&mut self, mut thread: Thread) {
        let mut stack: Vec<StackEntry> = vec![];
        for op in thread.program {
            match op {
                // Numeric
                Instr::I32Const(val) => stack.push_into(val),
                Instr::I32Clz => stack.unop(&u32::clz),
                Instr::I32Ctz => stack.unop(&u32::ctz),
                Instr::I32PopCnt => stack.unop(&u32::popcnt),
                Instr::I32Eqz => stack.testop(&u32::eqz),
                Instr::I32Eq => stack.relop(&u32::eq_),
                Instr::I32Ne => stack.relop(&u32::ne_),
                Instr::I32LtU => stack.relop(&u32::ltu),
                Instr::I32LtS => stack.relop(&u32::lts),
                Instr::I32GtU => stack.relop(&u32::gtu),
                Instr::I32GtS => stack.relop(&u32::gts),
                Instr::I32LeU => stack.relop(&u32::leu),
                Instr::I32LeS => stack.relop(&u32::les),
                Instr::I32GeU => stack.relop(&u32::geu),
                Instr::I32GeS => stack.relop(&u32::ges),
                Instr::I32Add => stack.binop(&u32::wrapping_add),
                Instr::I32Sub => stack.binop(&u32::wrapping_sub),
                Instr::I32Mul => stack.binop(&u32::wrapping_mul),
                Instr::I32DivU => stack.binop(&u32::wrapping_div),
                Instr::I32DivS => stack.binop(&u32::div_s),
                Instr::I32RemU => stack.binop(&u32::wrapping_rem),
                Instr::I32RemS => stack.binop(&u32::rem_s),
                Instr::I32Not => stack.unop(&u32::not),
                Instr::I32And => stack.binop(&u32::bitand),
                Instr::I32Or => stack.binop(&u32::bitor),
                Instr::I32Xor => stack.binop(&u32::bitxor),
                Instr::I32Shl => stack.binop(&u32::shl),
                Instr::I32ShrU => stack.binop(&u32::shr_u),
                Instr::I32ShrS => stack.binop(&u32::shr_s),
                Instr::I32Rotl => stack.binop(&u32::rotl),
                Instr::I32Rotr => stack.binop(&u32::rotr),

                Instr::I64Const(val) => stack.push_into(val),
                Instr::I64Clz => stack.unop(&u64::clz),
                Instr::I64Ctz => stack.unop(&u64::ctz),
                Instr::I64PopCnt => stack.unop(&u64::popcnt),
                Instr::I64Eqz => stack.testop(&u64::eqz),
                Instr::I64Eq => stack.relop(&u64::eq_),
                Instr::I64Ne => stack.relop(&u64::ne_),
                Instr::I64LtU => stack.relop(&u64::ltu),
                Instr::I64LtS => stack.relop(&u64::lts),
                Instr::I64GtU => stack.relop(&u64::gtu),
                Instr::I64GtS => stack.relop(&u64::gts),
                Instr::I64LeU => stack.relop(&u64::leu),
                Instr::I64LeS => stack.relop(&u64::les),
                Instr::I64GeU => stack.relop(&u64::geu),
                Instr::I64GeS => stack.relop(&u64::ges),
                Instr::I64Add => stack.binop(&u64::wrapping_add),
                Instr::I64Sub => stack.binop(&u64::wrapping_sub),
                Instr::I64Mul => stack.binop(&u64::wrapping_mul),
                Instr::I64DivU => stack.binop(&u64::wrapping_div),
                Instr::I64DivS => stack.binop(&u64::div_s),
                Instr::I64RemU => stack.binop(&u64::wrapping_rem),
                Instr::I64RemS => stack.binop(&u64::rem_s),
                Instr::I64Not => stack.unop(&u64::not),
                Instr::I64And => stack.binop(&u64::bitand),
                Instr::I64Or => stack.binop(&u64::bitor),
                Instr::I64Xor => stack.binop(&u64::bitxor),
                Instr::I64Shl => stack.binop(&u64::shl),
                Instr::I64ShrU => stack.binop(&u64::shr_u),
                Instr::I64ShrS => stack.binop(&u64::shr_s),
                Instr::I64Rotl => stack.binop(&u64::rotl),
                Instr::I64Rotr => stack.binop(&u64::rotr),

                Instr::F32Const(val) => stack.push_into(val),
                Instr::F64Const(val) => stack.push_into(val),
                // Ref
                Instr::RefNull(reftype) => {
                    stack.push(StackEntry::Value(Val::Ref(Ref::Null(reftype))))
                }
                Instr::RefFunc(func_idx) => {
                    // TODO validate index
                    let func_addr = thread.frame.module.funct[func_idx];
                    stack.push(StackEntry::Value(Val::Ref(Ref::Func(func_addr))))
                }
                // Var
                Instr::LocalGet(local_idx) => {
                    // TODO validate index
                    stack.push(StackEntry::Value(thread.frame.locals[local_idx]));
                }
                Instr::LocalSet(local_idx) => {
                    // TODO validate top of stack is value
                    match stack.pop().unwrap() {
                        StackEntry::Value(val) => {
                            // TODO validate index
                            thread.frame.locals[local_idx] = val;
                        }
                        _ => unreachable!(),
                    }
                }
                Instr::LocalTee(local_idx) => {
                    // TODO validate top of stack is value
                    match stack.peek().unwrap() {
                        StackEntry::Value(val) => {
                            thread.frame.locals[local_idx] = val;
                        }
                        _ => unreachable!(),
                    }
                }
                Instr::GlobalGet(global_idx) => {
                    // TODO validate index
                    let glob_addr = thread.frame.module.globals[global_idx];
                    stack.push(StackEntry::Value(self.globals[glob_addr].value));
                }
                Instr::GlobalSet(global_idx) => {
                    // TODO validate top of stack is value
                    match stack.pop().unwrap() {
                        StackEntry::Value(val) => {
                            // TODO validate index
                            let glob_addr = thread.frame.module.globals[global_idx];
                            self.globals[glob_addr].value = val;
                        }
                        _ => unreachable!(),
                    }
                }
                Instr::Nop => {}
                Instr::Unreachable => {
                    panic!("Reached unreachable")
                }
            }
        }
    }
}
