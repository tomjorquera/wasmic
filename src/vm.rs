use alloc::vec::Vec;
use core::{
    cell::RefCell,
    ops::{BitAnd, BitOr, BitXor, Not},
};

use crate::{
    err,
    instr::Instr,
    numeric::SupportedInteger,
    runtime::{
        Frame, FrameState, FuncInstance, HostFuncInstance, InternalFuncInstance, Label, Ref,
        StackEntry, Store, Val,
    },
};

trait Stack<'a> {
    fn push(&mut self, entry: StackEntry<'a>);
    fn pop(&mut self) -> Option<StackEntry<'a>>;
    fn push_into<T: Into<StackEntry<'a>>>(&mut self, val: T);
    fn pop_from<T: From<StackEntry<'a>>>(&mut self) -> T;
    fn peek(&self) -> Option<StackEntry<'a>>;
    fn unop<T: From<StackEntry<'a>> + Into<StackEntry<'a>>>(&mut self, f: &dyn Fn(T) -> T);
    fn binop<T: From<StackEntry<'a>> + Into<StackEntry<'a>>>(&mut self, f: &dyn Fn(T, T) -> T);
    fn testop<T: From<bool> + From<StackEntry<'a>> + Into<StackEntry<'a>>>(
        &mut self,
        f: &dyn Fn(T) -> bool,
    );
    fn relop<T: From<bool> + From<StackEntry<'a>> + Into<StackEntry<'a>>>(
        &mut self,
        f: &dyn Fn(T, T) -> bool,
    );
}

impl<'a> Stack<'a> for Vec<StackEntry<'a>> {
    fn push(&mut self, entry: StackEntry<'a>) {
        self.push(entry)
    }

    fn push_into<T: Into<StackEntry<'a>>>(&mut self, val: T) {
        self.push(val.into());
    }

    fn pop_from<T: From<StackEntry<'a>>>(&mut self) -> T {
        // TODO validate top of stack
        match self.pop().unwrap() {
            entry => T::from(entry),
            _ => panic!("not a value"),
        }
    }

    fn pop(&mut self) -> Option<StackEntry<'a>> {
        todo!()
    }

    fn peek(&self) -> Option<StackEntry<'a>> {
        todo!()
    }

    fn unop<T: From<StackEntry<'a>> + Into<StackEntry<'a>>>(&mut self, f: &dyn Fn(T) -> T) {
        // TODO validate top of stack
        let val = self.pop_from();
        let res = f(val);
        self.push_into(res);
    }

    fn binop<T: From<StackEntry<'a>> + Into<StackEntry<'a>>>(&mut self, f: &dyn Fn(T, T) -> T) {
        // TODO validate top of stack
        let val1 = self.pop_from();
        let val2 = self.pop_from();
        let res = f(val1, val2);
        self.push_into(res);
    }
    fn testop<T: From<bool> + From<StackEntry<'a>> + Into<StackEntry<'a>>>(
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
    fn relop<T: From<bool> + From<StackEntry<'a>> + Into<StackEntry<'a>>>(
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

struct Thread<'a> {
    frame: Frame<'a>,
    program: &'a Vec<Instr>,
}

trait InstrStack {
    fn incr_ip(&mut self);
    fn curr_op(&self) -> Instr;
    fn jump(&mut self, target: usize);
}

pub struct Trap {} // TODO

pub fn run(store: &Store, frame: &Frame, program: &Vec<Instr>) -> Result<Vec<Val>, err::Err> {
    let mut stack: Vec<StackEntry> = vec![];
    let mut ip = 0;
    while ip < program.len() && program.len() > 0 {
        let op = program[ip];
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
            Instr::RefNull(reftype) => stack.push(StackEntry::Value(Val::Ref(Ref::Null(reftype)))),
            Instr::RefFunc(func_idx) => {
                // TODO validate index
                let func_addr = frame.framestate.borrow_mut().module.borrow().funct[func_idx];
                stack.push(StackEntry::Value(Val::Ref(Ref::Func(func_addr))))
            }
            // Var
            Instr::LocalGet(local_idx) => {
                // TODO validate index
                stack.push(StackEntry::Value(
                    frame.framestate.borrow_mut().locals[local_idx],
                ));
            }
            Instr::LocalSet(local_idx) => {
                // TODO validate top of stack is value
                match stack.pop().unwrap() {
                    StackEntry::Value(val) => {
                        // TODO validate index
                        frame.framestate.borrow_mut().locals[local_idx] = val;
                    }
                    _ => unreachable!(),
                }
            }
            Instr::LocalTee(local_idx) => {
                // TODO validate top of stack is value
                match stack.peek().unwrap() {
                    StackEntry::Value(val) => {
                        frame.framestate.borrow_mut().locals[local_idx] = val;
                    }
                    _ => unreachable!(),
                }
            }
            Instr::GlobalGet(global_idx) => {
                // TODO validate index
                let glob_addr = frame.framestate.borrow_mut().module.borrow().globals[global_idx];
                stack.push(StackEntry::Value(
                    store.globals[glob_addr].borrow_mut().value,
                ));
            }
            Instr::GlobalSet(global_idx) => {
                // TODO validate top of stack is value
                match stack.pop().unwrap() {
                    StackEntry::Value(val) => {
                        // TODO validate index
                        let glob_addr =
                            frame.framestate.borrow().module.borrow().globals[global_idx];
                        store.globals[glob_addr].borrow_mut().value = val;
                    }
                    _ => unreachable!(),
                }
            }
            Instr::Nop => {
                // Do nothing
            }
            Instr::Unreachable => return Result::Err(err::Err::TrapUnreachable),
            Instr::Return => {
                if frame.arity > stack.len() {
                    return Result::Err(err::Err::AssertFailedEnoughVauesToReturn);
                }
                let mut values = vec![];
                for _ in 0..frame.arity {
                    match stack.pop().unwrap() {
                        StackEntry::Value(val) => values.push(val),
                        _ => return Result::Err(err::Err::InvariantViolatedAllResultsAreValues),
                    }
                }

                match stack.pop() {
                    Option::Some(StackEntry::Activation(_, _)) => {
                        return Result::Ok(values);
                    }
                    _ => {
                        return Result::Err(err::Err::AssertFailedFrameOnTopOfStack);
                    }
                }
            }

            Instr::Call(idx) => {
                if frame.framestate.borrow_mut().module.borrow().funct.len() < idx {
                    return Result::Err(err::Err::AssertFailedFuncInstanceExists);
                }
                let faddr = &frame.framestate.borrow_mut().module.borrow().funct[idx];
                let finstance = &store.funcinstances[*faddr];

                match finstance {
                    FuncInstance::Internal(InternalFuncInstance {
                        functype,
                        module,
                        code,
                    }) => {
                        if stack.len() < functype.input.len() {
                            return Result::Err(
                                err::Err::AssertFailedEnoughStackValuesForFunctionCall,
                            );
                        }
                        let inner_frame = Frame {
                            arity: functype.output.len(),
                            framestate: RefCell::new(FrameState {
                                locals: vec![], // TODO pop functype.input.len() values from stack
                                module,
                            }),
                        };
                        stack.push(StackEntry::Activation(
                            0, // TODO ???
                            &inner_frame,
                        ));
                        let label = Label {
                            arity: functype.output.len(),
                            instr: &code.body,
                        };
                        //run(store, label.instr, &inner_frame);
                        unimplemented!()
                    }
                    FuncInstance::Host(HostFuncInstance { functype }) => {
                        unimplemented!()
                    }
                }
            }
        }
        ip += 1;
    }

    let mut res = vec![];
    for entry in stack {
        match entry {
            StackEntry::Value(val) => {
                res.push(val);
            }
            _ => return Result::Err(err::Err::InvariantViolatedAllResultsAreValues),
        }
    }
    return Result::Ok(res);
}

#[cfg(test)]
mod tests {
    extern crate std;

    use crate::runtime::{self, ModuleInstance};

    use super::*;

    #[test]
    fn add_two() -> Result<(), err::Err> {
        let mut store = Store::new();
        store.modules.push(RefCell::new(ModuleInstance::new()));
        let frame = Frame::new(&store.modules[0]);

        let res = run(
            &store,
            &frame,
            &vec![Instr::I32Const(1), Instr::I32Const(1), Instr::I32Add],
        )?;

        assert_eq!(res.len(), 1);
        assert_eq!(res[0], runtime::Val::Num(runtime::Num::I32(2)));
        Ok(())
    }
}
