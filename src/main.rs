use std::{
    env,
    fs::File,
    io::{Read, Write},
};

use crate::{code_emitter::CodeEmitter, parser::Contract};

#[cfg(test)]
mod test;

mod code_emitter;
mod parser;
pub mod ts;

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in args.iter().skip(1) {
        print!("Compiling {}...", arg);
        let mut input = File::open(arg).unwrap();
        let mut input_str = String::new();
        input.read_to_string(&mut input_str).unwrap();
        let contract = Contract::from_str(&input_str).unwrap();
        let output = CodeEmitter.emit(&contract).unwrap();
        let mut out = File::create(format!("{}.ts", contract.name)).unwrap();
        out.write_all(output.as_bytes()).unwrap();
        println!(" OK! see {}.ts", contract.name);
    }
    let mut out = File::create("AbstractContract.ts").unwrap();
    out.write_all(CodeEmitter.emit_contract_abstraction().as_bytes())
        .unwrap();
    println!("All done!");
}
