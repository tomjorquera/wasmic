#![crate_type = "lib"]
#![no_std]

#[macro_use]
extern crate alloc;

//extern crate wasmic_macro;

mod embedding;
mod err;
mod instr;
mod modules;
mod numeric;
mod runtime;
mod types;
mod validation;
mod vm;
