use crate::carpet::instructions::Opcode;

use crate::carpet::cvm_heap::CVMHeap;
use std::mem::transmute;


pub const REGISTER_COUNT: usize = 32;
pub const STACK_SIZE: usize = 256;

#[derive(Debug)]
pub struct CVM {
    registers: [u32; REGISTER_COUNT],
    counter: usize,
    program: Vec<u8>,

    stack: [u32; STACK_SIZE],
    stack_pointer: usize,

    heap: CVMHeap,
}


impl CVM {
    pub fn new() -> Self {
        Self {
            registers: [0u32; REGISTER_COUNT],
            counter: 0,
            program: vec![],
            stack: [0u32; STACK_SIZE],
            stack_pointer: 0,
            heap: CVMHeap::new(),
        }
    }

    pub fn read_next_raw(&mut self) -> u32 {
        let index = self.next_8_bits() as usize;
        self.registers[index]
    }

    pub fn read_next_i32(&mut self) -> i32 {
        let index = self.next_8_bits() as usize;
        unsafe { transmute::<u32, i32>(self.registers[index]) }
    }

    pub fn read_next_f32(&mut self) -> f32 {
        let index = self.next_8_bits() as usize;
        unsafe { transmute::<u32, f32>(self.registers[index]) }
    }

    pub fn write_next_raw(&mut self, value: u32) {
        let index = self.next_8_bits() as usize;
        self.registers[index] = value;
    }

    pub fn write_next_i32(&mut self, value: i32) {
        let index = self.next_8_bits() as usize;
        self.registers[index] = unsafe { transmute::<i32, u32>(value) };
    }

    pub fn write_next_f32(&mut self, value: f32) {
        let index = self.next_8_bits() as usize;
        self.registers[index] = unsafe { transmute::<f32, u32>(value) };
    }

    pub fn new_program(&mut self, program: Vec<u8>) {
        self.registers = [0u32; REGISTER_COUNT];
        self.program = program;
        self.counter = 0;
    }

    pub fn run(&mut self) {
        let mut running = true;
        while running {
            running = self.execute_instruction();
        }
    }

    fn execute_instruction(&mut self) -> bool {
        if self.counter >= self.program.len() {
            return false;
        }
        let instruction = self.decode_opcode();

        match instruction {
            Opcode::HLT => {
                println!("Program terminated successfully");
                return false;
            }
            Opcode::LOAD => {
                let register = self.next_8_bits();
                let value = self.next_32_bits();
                self.registers[register as usize] = value;
                self.next_16_bits();
            }
            Opcode::PRINT => {
                let print_value = self.read_next_raw();
                print!("{}", unsafe { transmute::<u32, char>(print_value) });
                self.next_16_bits();
            }
            Opcode::INC => {
                let rw_register = self.next_8_bits() as usize;
                let val = unsafe { transmute::<i32, u32>(transmute::<u32, i32>(self.registers[rw_register]) + 1) };
                self.registers[rw_register as usize] = val;
                self.next_16_bits();
            }
            Opcode::DEC => {
                let rw_register = self.next_8_bits() as usize;
                let val = unsafe { transmute::<i32, u32>(transmute::<u32, i32>(self.registers[rw_register]) - 1) };
                self.registers[rw_register as usize] = val;
                self.next_16_bits();
            }
            Opcode::ADD => {
                let register_0 = self.read_next_i32();
                let register_1 = self.read_next_i32();
                self.write_next_i32(register_0 + register_1);
            }
            Opcode::SUB => {
                let register_0 = self.read_next_i32();
                let register_1 = self.read_next_i32();
                self.write_next_i32(register_0 - register_1);
            }
            Opcode::MUL => {
                let register_0 = self.read_next_i32();
                let register_1 = self.read_next_i32();
                self.write_next_i32(register_0 * register_1);
            }
            Opcode::DIV => {
                let register_0 = self.read_next_i32();
                let register_1 = self.read_next_i32();
                self.write_next_i32(register_0 / register_1);
            }
            Opcode::MOD => {
                let register_0 = self.read_next_i32();
                let register_1 = self.read_next_i32();
                self.write_next_i32(register_0 % register_1);
            }
            Opcode::FADD => {
                let register_0 = self.read_next_f32();
                let register_1 = self.read_next_f32();
                self.write_next_f32(register_0 + register_1);
            }
            Opcode::FSUB => {
                let register_0 = self.read_next_f32();
                let register_1 = self.read_next_f32();
                self.write_next_f32(register_0 - register_1);
            }
            Opcode::FMUL => {
                let register_0 = self.read_next_f32();
                let register_1 = self.read_next_f32();
                self.write_next_f32(register_0 * register_1);
            }
            Opcode::FDIV => {
                let register_0 = self.read_next_f32();
                let register_1 = self.read_next_f32();
                self.write_next_f32(register_0 / register_1);
            }
            Opcode::JMP => {
                let jump = self.read_next_i32();
                self.next_16_bits();
                self.counter = jump as usize;
            }
            Opcode::JMPF => {
                let mut counter = self.counter;
                let jump = self.read_next_i32();
                counter += jump as usize;
                self.next_16_bits();
                self.next_8_bits();
                self.counter = counter;
            }
            Opcode::JMPB => {
                let mut counter = self.counter;
                let jump = self.read_next_i32();
                counter -= jump as usize;
                self.next_16_bits();
                self.next_8_bits();
                self.counter = counter;
            }
            Opcode::EQ => {
                let register_0 = self.read_next_i32();
                let register_1 = self.read_next_i32();
                self.write_next_raw(if register_0 == register_1 { 1 } else { 0 });
            }
            Opcode::NE => {
                let register_0 = self.read_next_i32();
                let register_1 = self.read_next_i32();
                self.write_next_raw(if register_0 != register_1 { 1 } else { 0 });
            }
            Opcode::GT => {
                let register_0 = self.read_next_i32();
                let register_1 = self.read_next_i32();
                self.write_next_raw(if register_0 > register_1 { 1 } else { 0 });
            }
            Opcode::LT => {
                let register_0 = self.read_next_i32();
                let register_1 = self.read_next_i32();
                self.write_next_raw(if register_0 < register_1 { 1 } else { 0 });
            }
            Opcode::GTQ => {
                let register_0 = self.read_next_i32();
                let register_1 = self.read_next_i32();
                self.write_next_raw(if register_0 >= register_1 { 1 } else { 0 });
            }
            Opcode::LTQ => {
                let register_0 = self.read_next_i32();
                let register_1 = self.read_next_i32();
                self.write_next_raw(if register_0 <= register_1 { 1 } else { 0 });
            }
            Opcode::FEQ => {
                let register_0 = self.read_next_f32();
                let register_1 = self.read_next_f32();
                self.write_next_raw(if register_0 == register_1 { 1 } else { 0 });
            }
            Opcode::FNE => {
                let register_0 = self.read_next_f32();
                let register_1 = self.read_next_f32();
                self.write_next_raw(if register_0 != register_1 { 1 } else { 0 });
            }
            Opcode::FGT => {
                let register_0 = self.read_next_f32();
                let register_1 = self.read_next_f32();
                self.write_next_raw(if register_0 > register_1 { 1 } else { 0 });
            }
            Opcode::FLT => {
                let register_0 = self.read_next_f32();
                let register_1 = self.read_next_f32();
                self.write_next_raw(if register_0 < register_1 { 1 } else { 0 });
            }
            Opcode::FGTQ => {
                let register_0 = self.read_next_f32();
                let register_1 = self.read_next_f32();
                self.write_next_raw(if register_0 >= register_1 { 1 } else { 0 });
            }
            Opcode::FLTQ => {
                let register_0 = self.read_next_f32();
                let register_1 = self.read_next_f32();
                self.write_next_raw(if register_0 <= register_1 { 1 } else { 0 });
            }
            Opcode::JEQ => {
                let register_0 = self.read_next_raw();
                let register_1 = self.read_next_raw();
                self.next_8_bits();
                if register_0 != 0 {
                    self.counter = register_1 as usize;
                }
            }
            Opcode::JNE => {
                let register_0 = self.read_next_raw();
                let register_1 = self.read_next_raw();
                self.next_8_bits();
                if register_0 == 0 {
                    self.counter = register_1 as usize;
                }
            }
            Opcode::PUSH => {
                let value = self.read_next_raw();
                if self.stack_pointer >= self.stack.len() {
                    println!("CVM stack overflow");
                    return false;
                }
                self.stack[self.stack_pointer] = value;
                self.stack_pointer += 1;
                self.next_16_bits();
            }
            Opcode::SPUSH => {
                let amount = self.read_next_raw() as usize;
                self.stack_pointer += amount;
                if self.stack_pointer > self.stack.len() {
                    println!("CVM stack overflow");
                    return false;
                }
                self.next_16_bits();
            }
            Opcode::POP => {
                self.stack_pointer -= 1;
                if let Some(&value) = self.stack.get(self.stack_pointer) {
                    self.write_next_raw(value);
                    self.next_8_bits();
                } else {
                    println!("CVM stack underflow");
                    return false;
                }
            }
            Opcode::SPOP => {
                let amount = self.read_next_raw() as usize;
                if amount > self.stack_pointer {
                    println!("CVM stack underflow");
                    return false;
                }
                self.stack_pointer -= amount;
                self.next_16_bits();
            }
            Opcode::READ => {
                let index = self.read_next_raw() as usize;
                let value = if index >= STACK_SIZE {
                    *self.heap.value_at(index - STACK_SIZE)
                } else {
                    self.stack[index]
                };
                self.write_next_raw(value);
                self.next_8_bits();
            }
            Opcode::WRITE => {
                let read_register = self.next_8_bits() as usize;
                let index = self.read_next_raw() as usize;
                if index >= STACK_SIZE {
                    *self.heap.value_at(index - STACK_SIZE) = self.registers[read_register]
                } else {
                    self.stack[index] = self.registers[read_register]
                };
                self.next_8_bits();
            }
            Opcode::MOV => {
                let value = self.read_next_raw();
                self.write_next_raw(value);
                self.next_8_bits();
            }
            Opcode::MALLOC => {
                let read_register = self.read_next_i32() as usize;
                let pointer = self.heap.alloc(read_register) as u32;
                self.write_next_raw(pointer);
                self.next_8_bits();
            }
            Opcode::FREE => {
                let read_register = self.read_next_i32() as usize;
                self.heap.free(read_register);
                self.next_16_bits();
            }
            Opcode::FTOI => {
                let read_register = self.read_next_raw();
                let new_val = unsafe { transmute::<u32, f32>(read_register) } as i32;
                self.write_next_i32(new_val);
                self.next_8_bits();
            }
            Opcode::ITOF => {
                let read_register = self.read_next_raw();
                let new_val = unsafe { transmute::<u32, i32>(read_register) } as f32;
                self.write_next_f32(new_val);
                self.next_8_bits();
            }
            Opcode::I32 => {
                let read_register = self.read_next_raw();
                let value = read_register as i32;
                self.write_next_i32(value);
                self.next_8_bits();
            }
            Opcode::F32 => {
                let read_register = self.read_next_raw();
                let value = read_register as f32;
                self.write_next_f32(value);
                self.next_8_bits();
            }
        }
        true
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.counter];
        self.counter += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let out = Self::u16_le(&self.program[self.counter..self.counter + 2]);
        self.counter += 2;
        out
    }

    fn next_32_bits(&mut self) -> u32 {
        let out = Self::u32_le(&self.program[self.counter..self.counter + 4]);
        self.counter += 4;
        out
    }

    fn u16_le(array: &[u8]) -> u16 {
        ((array[0] as u16) << 0) +
            ((array[1] as u16) << 8)
    }

    fn u32_le(array: &[u8]) -> u32 {
        ((array[0] as u32) << 0) +
            ((array[1] as u32) << 8) +
            ((array[2] as u32) << 16) +
            ((array[3] as u32) << 24)
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = unsafe { transmute::<u8, Opcode>(self.program[self.counter]) };
        self.counter += 1;
        return opcode;
    }
}