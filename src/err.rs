use alloc::string::String;

use crate::types::{self, Addr};

#[derive(Debug)]
pub enum Err {
    ModuleDecode,
    ModuleParse,
    ModuleInstanceExportNotFound(String),
    OutOfBoundTableAccess,
    TrapUnreachable,
    InvariantViolatedAllResultsAreValues,
    AssertFailedEnoughVauesToReturn,
    AssertFailedFrameOnTopOfStack,
    AssertFailedFuncInstanceExists,
    AssertFailedEnoughStackValuesForFunctionCall,
    UndefinedFunction(Addr),
    UndefinedGlobal(Addr),
    UndefinedMem(Addr),
    UndefinedTable(Addr),
    IntegerOverflow,
    InvalidLimit(types::Limits),
}
