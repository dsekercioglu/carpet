use crate::carpet_assembler::assembler::CI;
use std::fs::OpenOptions;
use std::io::Read;


use std::error::Error;

const LOAD: &str = "load";
const LOADI: &str = "loadi";
const LOADF: &str = "loadf";
const PRINT: &str = "print";
const INC: &str = "inc";
const DEC: &str = "dec";
const ADD: &str = "add";
const SUB: &str = "sub";
const MUL: &str = "mul";
const DIV: &str = "div";
const MOD: &str = "mod";
const FADD: &str = "fadd";
const FSUB: &str = "fsub";
const FMUL: &str = "fmul";
const FDIV: &str = "fdiv";
const HLT: &str = "hlt";
const JMP: &str = "jmp";
const JMPB: &str = "jmpb";
const JMPF: &str = "jmpf";
const EQ: &str = "eq";
const NE: &str = "ne";
const GT: &str = "gt";
const LT: &str = "lt";
const GTQ: &str = "gtq";
const LTQ: &str = "ltq";
const FEQ: &str = "feq";
const FNE: &str = "fne";
const FGT: &str = "fgt";
const FLT: &str = "flt";
const FGTQ: &str = "fgtq";
const FLTQ: &str = "fltq";
const PUSH: &str = "push";
const SPUSH: &str = "spush";
const POP: &str = "pop";
const SPOP: &str = "spop";
const SREAD: &str = "sread";
const SWRITE: &str = "swrite";
const MOV: &str = "mov";
const JEQ: &str = "jeq";
const JNE: &str = "jne";
const MALLOC: &str = "malloc";
const FREE: &str = "free";
const FTOI: &str = "ftoi";
const ITOF: &str = "itof";
const I32: &str = "i32";
const F32: &str = "f32";


pub struct Parser {}

/*
    GTQ(Register, Register, Register),
    LTQ(Register, Register, Register),
    FEQ(Register, Register, Register),
    FNE(Register, Register, Register),
    FGT(Register, Register, Register),
    FLT(Register, Register, Register),
    FGTQ(Register, Register, Register),
    FLTQ(Register, Register, Register),
    PUSH(Register),
    POP(Register),
    SREAD(Register, Register),
    SWRITE(Register, Register),
    MOV(Register, Register),
    JEQ(Register, Register),
 */
impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_ci_asm(&self, path: &str) -> Result<Vec<CI>, Box<dyn Error>> {
        let mut byte_code = String::new();
        OpenOptions::new()
            .read(true)
            .open(path)
            .unwrap()
            .read_to_string(&mut byte_code)
            .unwrap();
        let mut ci_code = vec![];
        for line in byte_code.lines() {
            let split = line.split_ascii_whitespace().collect::<Vec<_>>();
            let val = match split[0] {
                LOAD => {
                    CI::LOAD(split[1].parse::<u8>()?, split[2].parse::<u32>()?)
                }
                LOADI => {
                    let val = split[2].parse::<i32>()?;
                    let val_u32 = unsafe { std::mem::transmute::<i32, u32>(val) };
                    CI::LOAD(split[1].parse::<u8>()?, val_u32)
                }
                LOADF => {
                    let val = split[2].parse::<f32>()?;
                    let val_u32 = unsafe { std::mem::transmute::<f32, u32>(val) };
                    CI::LOAD(split[1].parse::<u8>()?, val_u32)
                }
                PRINT => {
                    CI::PRINT(split[1].parse::<u8>()?)
                }
                INC => {
                    CI::INC(split[1].parse::<u8>()?)
                }
                DEC => {
                    CI::DEC(split[1].parse::<u8>()?)
                }
                ADD => {
                    CI::ADD(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                SUB => {
                    CI::SUB(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                MUL => {
                    CI::MUL(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                DIV => {
                    CI::DIV(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                MOD => {
                    CI::MOD(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                FADD => {
                    CI::FADD(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                FSUB => {
                    CI::FSUB(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                FMUL => {
                    CI::FMUL(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                FDIV => {
                    CI::FDIV(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                HLT => {
                    CI::HLT
                }
                JMP => {
                    CI::JMP(split[1].parse::<u8>()?)
                }
                JMPB => {
                    CI::JMPB(split[1].parse::<u8>()?)
                }
                JMPF => {
                    CI::JMPF(split[1].parse::<u8>()?)
                }
                EQ => {
                    CI::EQ(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                NE => {
                    CI::NE(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                GT => {
                    CI::GT(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                LT => {
                    CI::LT(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                GTQ => {
                    CI::GTQ(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                LTQ => {
                    CI::LTQ(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                FEQ => {
                    CI::FEQ(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                FNE => {
                    CI::FNE(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                FGT => {
                    CI::FGT(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                FLT => {
                    CI::FLT(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                FGTQ => {
                    CI::FGTQ(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                FLTQ => {
                    CI::FLTQ(split[1].parse::<u8>()?, split[2].parse::<u8>()?, split[3].parse::<u8>()?)
                }
                PUSH => {
                    CI::PUSH(split[1].parse::<u8>()?)
                }
                SPUSH => {
                    CI::SPUSH(split[1].parse::<u8>()?)
                }
                POP => {
                    CI::POP(split[1].parse::<u8>()?)
                }
                SPOP => {
                    CI::SPOP(split[1].parse::<u8>()?)
                }
                SREAD => {
                    CI::SREAD(split[1].parse::<u8>()?, split[2].parse::<u8>()?)
                }
                SWRITE => {
                    CI::SWRITE(split[1].parse::<u8>()?, split[2].parse::<u8>()?)
                }
                MOV => {
                    CI::MOV(split[1].parse::<u8>()?, split[2].parse::<u8>()?)
                }
                JEQ => {
                    CI::JEQ(split[1].parse::<u8>()?, split[2].parse::<u8>()?)
                }
                JNE => {
                    CI::JNE(split[1].parse::<u8>()?, split[2].parse::<u8>()?)
                }
                MALLOC => {
                    CI::MALLOC(split[1].parse::<u8>()?, split[2].parse::<u8>()?)
                }
                FREE => {
                    CI::FREE(split[1].parse::<u8>()?)
                }
                FTOI => {
                    CI::FTOI(split[1].parse::<u8>()?, split[2].parse::<u8>()?)
                }
                ITOF => {
                    CI::ITOF(split[1].parse::<u8>()?, split[2].parse::<u8>()?)
                }
                I32 => {
                    CI::I32(split[1].parse::<u8>()?, split[2].parse::<u8>()?)
                }
                F32 => {
                    CI::F32(split[1].parse::<u8>()?, split[2].parse::<u8>()?)
                }
                _ => {
                    panic!("illegal CVM instruction")
                }
            };
            ci_code.push(val);
        }
        Ok(ci_code)
    }
}