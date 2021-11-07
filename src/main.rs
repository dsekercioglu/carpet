use crate::carpet::cvm::CVM;
use crate::carpet_assembler::assembler::{CarpetAssembler};
use crate::parser::parse::Parser;
use std::time::Instant;

mod carpet;
mod carpet_assembler;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parser = Parser::new();
    let mut cvm = CVM::new();
    for program_path in &args[1..] {
        let code = parser.parse_ci_asm(&program_path).unwrap();
        let carpet_assembler = CarpetAssembler::new();
        let program = carpet_assembler.generate_byte_code(
            code
        );
        cvm.new_program(
            program
        );

        let time = Instant::now();
        cvm.run();
        println!();
        println!("program ran in {:?}", time.elapsed());
    }
}