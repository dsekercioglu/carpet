#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    LOAD = 0,
    //LOAD(8), Register(8), Number(32)
    PRINT,
    //PRINT(8), Register(8)
    INC,
    //INC(8), Register(8)
    DEC,
    //DEC(8), Register(8)
    ADD,
    //ADD(8), Register(8), Register(8), Register(8)
    SUB,
    //SUB(8), Register(8), Register(8), Register(8)
    MUL,
    //MUL(8), Register(8), Register(8), Register(8)
    DIV,
    //DIV(8), Register(8), Register(8), Register(8)
    MOD,
    //MOD(8), Register(8), Register(8), Register(8)
    FADD,
    //ADD(8), Register(8), Register(8), Register(8)
    FSUB,
    //SUB(8), Register(8), Register(8), Register(8)
    FMUL,
    //MUL(8), Register(8), Register(8), Register(8)
    FDIV,
    //DIV(8), Register(8), Register(8), Register(8)
    HLT,
    //HLT(8)
    JMP,
    //JMP(8), Register(8)
    JMPF,
    //JMPF(8), Register(8)
    JMPB,
    //JMPB(8), Register(8)
    EQ,
    //EQ(8), Register(8), Register(8), Register(8)
    NE,
    //NE(8), Register(8), Register(8), Register(8)
    GT,
    //GT(8), Register(8), Register(8), Register(8)
    LT,
    //LT(8), Register(8), Register(8), Register(8)
    GTQ,
    //GTQ(8), Register(8), Register(8), Register(8)
    LTQ,
    //LTQ(8), Register(8), Register(8), Register(8)
    FEQ,
    //EQ(8), Register(8), Register(8), Register(8)
    FNE,
    //NE(8), Register(8), Register(8), Register(8)
    FGT,
    //GT(8), Register(8), Register(8), Register(8)
    FLT,
    //LT(8), Register(8), Register(8), Register(8)
    FGTQ,
    //GTQ(8), Register(8), Register(8), Register(8)
    FLTQ,
    //LTQ(8), Register(8), Register(8), Register(8)
    JEQ,
    //JEQ(8), Register(8), Register(8)
    JNE,
    //JEQ(8), Register(8), Register(8)
    MOV,
    //Move(8), Register(8), Register(8)
    PUSH,
    //Push(8), Register(8)
    SPUSH,
    //
    POP,
    //Pop(8), Register(8)
    SPOP,
    //SPop(8), Register(8)
    READ,
    //SREAD(8), Register(8)
    WRITE,
    //SREAD(8), Register(8), Register(8)
    MALLOC,
    //ALLOC(8), Register(8), Register(8)
    FREE,
    //HWRITE(8), Register(8), Register(8), Register(8)
    ITOF,
    FTOI,
    I32,
    F32,
}