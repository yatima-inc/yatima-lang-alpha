#![feature(proc_macro_hygiene, decl_macro)]

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
#[cfg(test)]
extern crate rand;

#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
extern crate rocket;

#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
extern crate log;

#[macro_use]
extern crate hashexpr;

pub mod anon_term;
pub mod core;
pub mod decode_error;
pub mod definition;
pub mod hashspace;
pub mod meta_term;
pub mod package;
pub mod parse;
#[cfg(not(target_arch = "wasm32"))]
pub mod repl;
pub mod term;
pub mod unembed_error;
#[cfg(target_arch = "wasm32")]
pub mod wasm_binds;
