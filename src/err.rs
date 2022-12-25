use alloc::string::String;

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
}
