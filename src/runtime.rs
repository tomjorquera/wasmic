use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    embedding,
    err::Err,
    instr::Instr,
    modules::{Func, HostFunc},
    types::{self, Addr},
};

#[derive(Clone, Copy)]
pub enum Num {
    I32(u32),
    I64(u64),
    F32(f32),
    F64(f64),
}

#[derive(Clone, Copy)]
pub enum Ref {
    Null(types::Ref),
    Func(Addr),
    Extern(Addr),
}

#[derive(Clone, Copy)]
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

pub struct Store {
    pub funcinstances: Vec<FuncInstance>,
    pub funcdefs: Vec<Func>,
    pub funchosts: Vec<HostFunc>,
    pub tables: Vec<Table>,
    pub mems: Vec<Mem>,
    pub globals: Vec<Global>,
    pub elems: Vec<Elem>,
    pub datas: Vec<Data>,
    pub instances: Vec<ModuleInstance>,
}

pub struct ModuleInstance {
    pub types: Vec<types::Function>,
    pub funct: Vec<usize>,
    pub tables: Vec<usize>,
    pub mems: Vec<usize>,
    pub globals: Vec<usize>,
    pub elems: Vec<usize>,
    pub datas: Vec<usize>,
    pub exports: Vec<Export>,
}

pub enum FuncInstance {
    Internal(InternalFuncInstance),
    Host(HostFuncInstance),
}

pub struct InternalFuncInstance {
    pub functype: types::Function,
    pub module: Addr,
    pub code: Addr,
}
pub struct HostFuncInstance {
    pub functype: types::Function,
    pub code: Addr,
}

pub struct Table {
    pub tabletype: types::Table,
    pub elem: Vec<Ref>,
}
pub struct Mem {
    pub memtype: types::Mem,
    pub data: Vec<u8>,
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
    pub data: Vec<u8>,
}
pub struct Export {
    pub name: String,
    pub value: ExternalVal,
}
pub struct Import {
    pub module: String,
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

impl embedding::ModuleInstance for ModuleInstance {
    fn export(&self, name: &str) -> Result<self::ExternalVal, crate::err::Err> {
        for export in &self.exports {
            if export.name == name {
                return Result::Ok(export.value);
            }
        }
        return Result::Err(Err::ModuleInstanceExportNotFound(name.to_string()));
    }
}

impl embedding::Store for Store {
    fn new() -> Self {
        Store {
            funcinstances: vec![],
            funcdefs: vec![],
            funchosts: vec![],
            tables: vec![],
            mems: vec![],
            globals: vec![],
            elems: vec![],
            datas: vec![],
            instances: vec![],
        }
    }

    fn instantiate(
        &mut self,
        module: &crate::modules::Module,
        externvals: Vec<self::ExternalVal>,
    ) -> Result<&self::ModuleInstance, Err> {
        let mut instance = ModuleInstance {
            types: vec![],
            funct: vec![],
            tables: vec![],
            mems: vec![],
            globals: vec![],
            elems: vec![],
            datas: vec![],
            exports: vec![],
        };

        instance.types = module.types.clone();
        self.instances.push(instance);
        let inst_addr = self.instances.len() - 1;
        let instance_ref = self.instances.last_mut().unwrap();

        for func in &module.funcs {
            self.funcdefs.push(func.clone());
            let func_inst = InternalFuncInstance {
                functype: module.types[func.functype].clone(),
                module: inst_addr,
                code: self.funcdefs.len() - 1,
            };
            self.funcinstances.push(FuncInstance::Internal(func_inst));
            let addr = self.funcinstances.len() - 1;
            instance_ref.funct.push(addr);
        }

        for table in &module.tables {
            let table_inst = Table {
                tabletype: table.tabletype,
                elem: vec![],
            };
            self.tables.push(table_inst);
            instance_ref.tables.push(self.tables.len() - 1);
        }

        for mem in &module.mems {
            let mem_inst = Mem {
                memtype: mem.memtype,
                data: vec![],
            };
            self.mems.push(mem_inst);
            instance_ref.mems.push(self.mems.len() - 1);
        }

        for global in &module.globals {
            let glob_inst = Global {
                globaltype: global.globaltype,
                value: Val::Num(Num::I32(0)), // TODO execute global.init
            };
            self.globals.push(glob_inst);
            instance_ref.globals.push(self.globals.len() - 1);
        }

        for elem in &module.elems {
            let elem_inst = Elem {
                elemtype: elem.elemtype,
                elem: vec![], // TODO copy elements from module according to mode
            };
            self.elems.push(elem_inst);
            instance_ref.elems.push(self.elems.len() - 1);
        }

        for data in &module.datas {
            let data_inst = Data {
                data: data.init.clone(),
            };
            self.datas.push(data_inst);
            instance_ref.datas.push(self.datas.len() - 1);
        }

        return Ok(self.instances.last_mut().unwrap());
    }

    fn func_alloc(&mut self, functype: types::Function, hostfunc: self::HostFunc) -> Addr {
        self.funchosts.push(hostfunc);
        let func_inst = HostFuncInstance {
            functype,
            code: self.funchosts.len() - 1,
        };
        self.funcinstances.push(FuncInstance::Host(func_inst));
        return self.funcinstances.len() - 1;
    }

    fn func_type(&self, addr: Addr) -> types::Function {
        match &self.funcinstances[addr] {
            FuncInstance::Internal(f) => f.functype.clone(),
            FuncInstance::Host(f) => f.functype.clone(),
        }
    }

    fn invoke(&mut self, addr: Addr, values: Vec<self::Val>) -> Result<Vec<self::Val>, Err> {
        todo!()
    }

    fn table_alloc(&mut self, tabletype: types::Table) -> Addr {
        let table_inst = Table {
            tabletype,
            elem: vec![],
        };
        self.tables.push(table_inst);
        return self.tables.len() - 1;
    }

    fn table_type(&self, addr: Addr) -> types::Table {
        self.tables[addr].tabletype
    }

    fn table_read(&self, addr: Addr, index: usize) -> Result<self::Ref, Err> {
        self.tables[addr]
            .elem
            .get(index)
            .map(|val| *val)
            .ok_or(Err::OutOfBoundTableAccess)
    }

    fn table_write(&mut self, addr: Addr, index: usize, value: self::Ref) -> Result<(), Err> {
        if index >= self.tables[addr].elem.len() {
            return Result::Err(Err::OutOfBoundTableAccess);
        }
        self.tables[addr].elem[index] = value;
        Result::Ok(())
    }

    fn table_size(&self, addr: Addr) -> usize {
        self.tables[addr].elem.len()
    }

    fn table_grow(&mut self, addr: Addr, n: u32, init: self::Ref) -> Result<(), Err> {
        todo!()
    }

    fn mem_alloc(&mut self, memtyp: types::Mem) -> Addr {
        todo!()
    }

    fn mem_type(&self, addr: Addr) -> types::Mem {
        todo!()
    }

    fn mem_read(&self, addr: Addr, index: u32) -> Result<u8, Err> {
        todo!()
    }

    fn mem_write(&mut self, addr: Addr, index: u32, value: u8) -> Result<(), Err> {
        todo!()
    }

    fn mem_size(&self, addr: Addr) -> u32 {
        todo!()
    }

    fn mem_grow(&mut self, addr: Addr, n: u32, init: self::Ref) -> Result<(), Err> {
        todo!()
    }

    fn global_alloc(&mut self, globtype: types::Global) -> Addr {
        todo!()
    }

    fn global_type(&self, addr: Addr) -> types::Global {
        todo!()
    }

    fn global_read(&self, addr: Addr) -> Result<self::Val, Err> {
        todo!()
    }

    fn global_write(&mut self, addr: Addr, value: self::Val) -> Result<(), Err> {
        todo!()
    }
}
