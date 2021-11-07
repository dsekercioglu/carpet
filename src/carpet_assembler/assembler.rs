use crate::carpet::instructions::Opcode;

type Register = u8;

#[derive(Debug, Clone, Copy)]
pub enum CI {
    LOAD(Register, u32),
    PRINT(Register),
    INC(Register),
    DEC(Register),
    ADD(Register, Register, Register),
    SUB(Register, Register, Register),
    MUL(Register, Register, Register),
    DIV(Register, Register, Register),
    MOD(Register, Register, Register),
    FADD(Register, Register, Register),
    FSUB(Register, Register, Register),
    FMUL(Register, Register, Register),
    FDIV(Register, Register, Register),
    HLT,
    JMP(Register),
    JMPB(Register),
    JMPF(Register),
    EQ(Register, Register, Register),
    NE(Register, Register, Register),
    GT(Register, Register, Register),
    LT(Register, Register, Register),
    GTQ(Register, Register, Register),
    LTQ(Register, Register, Register),
    FEQ(Register, Register, Register),
    FNE(Register, Register, Register),
    FGT(Register, Register, Register),
    FLT(Register, Register, Register),
    FGTQ(Register, Register, Register),
    FLTQ(Register, Register, Register),
    PUSH(Register),
    SPUSH(Register),
    POP(Register),
    SPOP(Register),
    SREAD(Register, Register),
    SWRITE(Register, Register),
    MOV(Register, Register),
    JEQ(Register, Register),
    JNE(Register, Register),

    MALLOC(Register, Register),
    FREE(Register),
    FTOI(Register, Register),
    ITOF(Register, Register),
    I32(Register, Register),
    F32(Register, Register),
}

pub struct CarpetAssembler {}

impl CarpetAssembler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_byte_code(&self, instructions: Vec<CI>) -> Vec<u8> {
        let mut carpet_byte_code = Vec::with_capacity(instructions.len());
        for instruction in instructions {
            match instruction {
                CI::LOAD(register, number) => {
                    let mut bytes = [0u8; 4];
                    let mut current_byte = number;
                    for byte in &mut bytes {
                        *byte = current_byte as u8;
                        current_byte = current_byte >> 8;
                    }
                    carpet_byte_code.extend(
                        &[
                            Opcode::LOAD as u8,
                            register,
                            bytes[0],
                            bytes[1],
                            bytes[2],
                            bytes[3],
                            0,
                            0,
                        ]
                    );
                }
                CI::PRINT(register0) => {
                    carpet_byte_code.extend(&[Opcode::PRINT as u8, register0, 0, 0]);
                }
                CI::INC(register0) => {
                    carpet_byte_code.extend(&[Opcode::INC as u8, register0, 0, 0]);
                }
                CI::DEC(register0) => {
                    carpet_byte_code.extend(&[Opcode::DEC as u8, register0, 0, 0]);
                }
                CI::ADD(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::ADD as u8, operand0, operand1, out]);
                }
                CI::SUB(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::SUB as u8, operand0, operand1, out]);
                }
                CI::MUL(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::MUL as u8, operand0, operand1, out]);
                }
                CI::DIV(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::DIV as u8, operand0, operand1, out]);
                }
                CI::MOD(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::MOD as u8, operand0, operand1, out]);
                }
                CI::FADD(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::FADD as u8, operand0, operand1, out]);
                }
                CI::FSUB(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::FSUB as u8, operand0, operand1, out]);
                }
                CI::FMUL(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::FMUL as u8, operand0, operand1, out]);
                }
                CI::FDIV(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::FDIV as u8, operand0, operand1, out]);
                }
                CI::HLT => {
                    carpet_byte_code.extend(&[Opcode::HLT as u8, 0, 0, 0]);
                }
                CI::JMP(to) => {
                    carpet_byte_code.extend(&[Opcode::JMP as u8, to, 0, 0]);
                }
                CI::JMPB(amt) => {
                    carpet_byte_code.extend(&[Opcode::JMPB as u8, amt, 0, 0]);
                }
                CI::JMPF(amt) => {
                    carpet_byte_code.extend(&[Opcode::JMPF as u8, amt, 0, 0]);
                }
                CI::EQ(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::EQ as u8, operand0, operand1, out]);
                }
                CI::NE(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::NE as u8, operand0, operand1, out]);
                }
                CI::GT(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::GT as u8, operand0, operand1, out]);
                }
                CI::LT(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::LT as u8, operand0, operand1, out]);
                }
                CI::GTQ(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::GTQ as u8, operand0, operand1, out]);
                }
                CI::LTQ(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::LTQ as u8, operand0, operand1, out]);
                }
                CI::FEQ(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::FEQ as u8, operand0, operand1, out]);
                }
                CI::FNE(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::FNE as u8, operand0, operand1, out]);
                }
                CI::FGT(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::FGT as u8, operand0, operand1, out]);
                }
                CI::FLT(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::FLT as u8, operand0, operand1, out]);
                }
                CI::FGTQ(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::FGTQ as u8, operand0, operand1, out]);
                }
                CI::FLTQ(operand0, operand1, out) => {
                    carpet_byte_code.extend(&[Opcode::FLTQ as u8, operand0, operand1, out]);
                }
                CI::JEQ(check, jump) => {
                    carpet_byte_code.extend(&[Opcode::JEQ as u8, check, jump, 0]);
                }
                CI::JNE(check, jump) => {
                    carpet_byte_code.extend(&[Opcode::JNE as u8, check, jump, 0]);
                }
                CI::PUSH(register) => {
                    carpet_byte_code.extend(&[Opcode::PUSH as u8, register, 0, 0]);
                }
                CI::SPUSH(register) => {
                    carpet_byte_code.extend(&[Opcode::SPUSH as u8, register, 0, 0]);
                }
                CI::POP(register) => {
                    carpet_byte_code.extend(&[Opcode::POP as u8, register, 0, 0]);
                }
                CI::SPOP(register) => {
                    carpet_byte_code.extend(&[Opcode::SPOP as u8, register, 0, 0]);
                }
                CI::SREAD(register0, register1) => {
                    carpet_byte_code.extend(&[Opcode::READ as u8, register0, register1, 0]);
                }
                CI::SWRITE(register0, register1) => {
                    carpet_byte_code.extend(&[Opcode::WRITE as u8, register0, register1, 0]);
                }
                CI::MOV(register0, register1) => {
                    carpet_byte_code.extend(&[Opcode::MOV as u8, register0, register1, 0]);
                }
                CI::MALLOC(register0, register1) => {
                    carpet_byte_code.extend(&[Opcode::MALLOC as u8, register0, register1, 0]);
                }
                CI::FREE(register0) => {
                    carpet_byte_code.extend(&[Opcode::FREE as u8, register0, 0, 0]);
                }
                CI::FTOI(register0, register1) => {
                    carpet_byte_code.extend(&[Opcode::FTOI as u8, register0, register1, 0]);
                }
                CI::ITOF(register0, register1) => {
                    carpet_byte_code.extend(&[Opcode::ITOF as u8, register0, register1, 0]);
                }
                CI::I32(register0, register1) => {
                    carpet_byte_code.extend(&[Opcode::I32 as u8, register0, register1, 0]);
                }
                CI::F32(register0, register1) => {
                    carpet_byte_code.extend(&[Opcode::F32 as u8, register0, register1, 0]);
                }
            }
        }
        carpet_byte_code
    }
}