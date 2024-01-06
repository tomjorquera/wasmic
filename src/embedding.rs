use core::cell::RefCell;

use alloc::{string::ToString, vec::Vec};

use crate::{
    err::Err,
    modules::{self, HostFunc},
    runtime,
    types::{self, Addr},
};

pub trait Store {
    fn new() -> Self;

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

pub trait Instanciable<'a>: Sized {
    fn export(&self, name: &str) -> Result<runtime::ExternalVal, Err>;
    fn instanciate(
        store: &'a mut runtime::Store<'a>,
        module: &modules::Module,
        externvals: Vec<runtime::ExternalVal>,
    ) -> Result<&'a RefCell<Self>, Err>;
}

impl<'a> Instanciable<'a> for runtime::ModuleInstance {
    fn export(&self, name: &str) -> Result<runtime::ExternalVal, crate::err::Err> {
        for export in &self.exports {
            if export.name == name {
                return Result::Ok(export.value);
            }
        }
        return Result::Err(Err::ModuleInstanceExportNotFound(name.to_string()));
    }

    fn instanciate(
        store: &'a mut runtime::Store<'a>,
        module: &modules::Module,
        externvals: Vec<runtime::ExternalVal>,
    ) -> Result<&'a RefCell<runtime::ModuleInstance>, Err> {
        let mut instance = runtime::ModuleInstance {
            types: vec![],
            funct: vec![],
            tables: vec![],
            mems: vec![],
            globals: vec![],
            elems: vec![],
            datas: vec![],
            exports: vec![],
        };

        for externval in &externvals {
            match externval {
                &runtime::ExternalVal::Fun(addr) => {
                    if store.funcinstances.len() <= addr {
                        return Result::Err(Err::UndefinedFunction(addr));
                    }
                }
                &runtime::ExternalVal::Global(addr) => {
                    if store.globals.len() <= addr {
                        return Result::Err(Err::UndefinedGlobal(addr));
                    }
                }
                &runtime::ExternalVal::Mem(addr) => {
                    if store.mems.len() <= addr {
                        return Result::Err(Err::UndefinedMem(addr));
                    }
                }
                &runtime::ExternalVal::Table(addr) => {
                    if store.tables.len() <= addr {
                        return Result::Err(Err::UndefinedTable(addr));
                    }
                }
            }
        }

        instance.types = module.types.clone();

        store.modules.push(RefCell::new(instance));
        let instance_ref = &store.modules[store.modules.len() - 1];

        for func in &module.funcs {
            let func_inst = runtime::InternalFuncInstance {
                functype: module.types[func.functype].clone(),
                module: &instance_ref,
                code: func.clone(),
            };
            store
                .funcinstances
                .push(runtime::FuncInstance::Internal(func_inst));
            let addr = store.funcinstances.len() - 1;
            instance_ref.borrow_mut().funct.push(addr);
        }

        for table in &module.tables {
            let table_inst = RefCell::new(runtime::Table {
                tabletype: table.tabletype,
                elem: vec![],
            });
            store.tables.push(table_inst);
            instance_ref
                .borrow_mut()
                .tables
                .push(store.tables.len() - 1);
        }

        for mem in &module.mems {
            let mem_inst = RefCell::new(runtime::Mem {
                memtype: mem.memtype,
                data: vec![],
            });
            store.mems.push(mem_inst);
            instance_ref.borrow_mut().mems.push(store.mems.len() - 1);
        }

        for global in &module.globals {
            let glob_inst = RefCell::new(runtime::Global {
                globaltype: global.globaltype,
                value: runtime::Val::Num(runtime::Num::I32(0)), // TODO execute global.init
            });
            store.globals.push(glob_inst);
            instance_ref
                .borrow_mut()
                .globals
                .push(store.globals.len() - 1);
        }

        for elem in &module.elems {
            let elem_inst = RefCell::new(runtime::Elem {
                elemtype: elem.elemtype,
                elem: vec![], // TODO copy elements from module according to mode
            });
            store.elems.push(elem_inst);
            instance_ref.borrow_mut().elems.push(store.elems.len() - 1);
        }

        for data in &module.datas {
            let data_inst = RefCell::new(runtime::Data {
                data: data.init.clone(),
            });
            store.datas.push(data_inst);
            instance_ref.borrow_mut().datas.push(store.datas.len() - 1);
        }

        // TODO Export Value Typing validation (4.5.2)
        for export in &module.exports {
            match export.desc {
                modules::ExportDesc::Func(idx) => {
                    instance_ref.borrow_mut().exports.push(runtime::Export {
                        name: export.name.clone(),
                        value: runtime::ExternalVal::Fun(instance_ref.borrow().funct[idx]),
                    });
                }
                modules::ExportDesc::Table(idx) => {
                    instance_ref.borrow_mut().exports.push(runtime::Export {
                        name: export.name.clone(),
                        value: runtime::ExternalVal::Table(instance_ref.borrow().tables[idx]),
                    });
                }
                modules::ExportDesc::Mem(idx) => {
                    instance_ref.borrow_mut().exports.push(runtime::Export {
                        name: export.name.clone(),
                        value: runtime::ExternalVal::Mem(instance_ref.borrow().mems[idx]),
                    });
                }
                modules::ExportDesc::Global(idx) => {
                    instance_ref.borrow_mut().exports.push(runtime::Export {
                        name: export.name.clone(),
                        value: runtime::ExternalVal::Global(instance_ref.borrow().globals[idx]),
                    });
                }
            }
        }

        return Ok(&store.modules[store.modules.len() - 1]);
    }
}

impl<'a> Store for runtime::Store<'a> {
    fn new() -> Self {
        runtime::Store {
            modules: vec![],
            funcinstances: vec![],
            tables: vec![],
            mems: vec![],
            globals: vec![],
            elems: vec![],
            datas: vec![],
        }
    }

    fn func_alloc(&mut self, functype: types::Function, hostfunc: HostFunc) -> Addr {
        let func_inst = runtime::HostFuncInstance { functype };
        self.funcinstances
            .push(runtime::FuncInstance::Host(func_inst));
        return self.funcinstances.len() - 1;
    }

    fn func_type(&self, addr: Addr) -> types::Function {
        match &self.funcinstances[addr] {
            runtime::FuncInstance::Internal(f) => f.functype.clone(),
            runtime::FuncInstance::Host(f) => f.functype.clone(),
        }
    }

    fn invoke(&mut self, addr: Addr, values: Vec<runtime::Val>) -> Result<Vec<runtime::Val>, Err> {
        todo!()
    }

    fn table_alloc(&mut self, tabletype: types::Table) -> Addr {
        let table_inst = RefCell::new(runtime::Table {
            tabletype,
            elem: vec![],
        });
        self.tables.push(table_inst);
        return self.tables.len() - 1;
    }

    fn table_type(&self, addr: Addr) -> types::Table {
        self.tables[addr].borrow().tabletype
    }

    fn table_read(&self, addr: Addr, index: usize) -> Result<runtime::Ref, Err> {
        self.tables[addr]
            .borrow()
            .elem
            .get(index)
            .map(|val| *val)
            .ok_or(Err::OutOfBoundTableAccess)
    }

    fn table_write(&mut self, addr: Addr, index: usize, value: runtime::Ref) -> Result<(), Err> {
        if index >= self.tables[addr].borrow().elem.len() {
            return Result::Err(Err::OutOfBoundTableAccess);
        }
        self.tables[addr].borrow_mut().elem[index] = value;
        Result::Ok(())
    }

    fn table_size(&self, addr: Addr) -> usize {
        self.tables[addr].borrow().elem.len()
    }

    fn table_grow(&mut self, addr: Addr, n: u32, init: runtime::Ref) -> Result<(), Err> {
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

    fn mem_grow(&mut self, addr: Addr, n: u32, init: runtime::Ref) -> Result<(), Err> {
        todo!()
    }

    fn global_alloc(&mut self, globtype: types::Global) -> Addr {
        todo!()
    }

    fn global_type(&self, addr: Addr) -> types::Global {
        todo!()
    }

    fn global_read(&self, addr: Addr) -> Result<runtime::Val, Err> {
        todo!()
    }

    fn global_write(&mut self, addr: Addr, value: runtime::Val) -> Result<(), Err> {
        todo!()
    }
}
