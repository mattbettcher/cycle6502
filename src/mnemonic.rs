/*
    Defines the Mnemonic names for the 6502. Provides translation from opcode and string.
*/

use std::str::FromStr;

#[derive(Debug)]
enum Mnemonic {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    // TODO(matt): add undocumented mnemonics
}

impl From<u8> for Mnemonic {
    // matches any addressing mode version of an opcode to its Mnemonic.
    // Note: see addressing.rs for getting an opcodes addressing mode.
    fn from(op: u8) -> Self {
        use self::Mnemonic::*;

        match op {
            0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => ADC,

            // TODO(matt): add all the other opcodes -> mnemonics
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct ParseMnemonicError { }

impl FromStr for Mnemonic {
    type Err = ParseMnemonicError;
    fn from_str(s: &str) -> Result<Self, ParseMnemonicError> {
        use self::Mnemonic::*;

        match s.to_uppercase().as_str() {
            "ADC" => Ok(ADC),
            "AND" => Ok(AND),
            "ASL" => Ok(ASL),
            "BCC" => Ok(BCC),
            "BCS" => Ok(BCS),
            "BEQ" => Ok(BEQ),
            "BIT" => Ok(BIT),
            "BMI" => Ok(BMI),
            "BNE" => Ok(BNE),
            "BPL" => Ok(BPL),
            "BRK" => Ok(BRK),
            "BVC" => Ok(BVC),
            "BVS" => Ok(BVS),
            "CLC" => Ok(CLC),
            "CLD" => Ok(CLD),
            "CLI" => Ok(CLI),
            "CLV" => Ok(CLV),
            "CMP" => Ok(CMP),
            "CPX" => Ok(CPX),
            "CPY" => Ok(CPY),
            "DEC" => Ok(DEC),
            "DEX" => Ok(DEX),
            "DEY" => Ok(DEY),
            "EOR" => Ok(EOR),
            "INC" => Ok(INC),
            "INX" => Ok(INX),
            "INY" => Ok(INY),
            "JMP" => Ok(JMP),
            "JSR" => Ok(JSR),
            "LDA" => Ok(LDA),
            "LDX" => Ok(LDX),
            "LDY" => Ok(LDY),
            "LSR" => Ok(LSR),
            "NOP" => Ok(NOP),
            "ORA" => Ok(ORA),
            "PHA" => Ok(PHA),
            "PHP" => Ok(PHP),
            "PLA" => Ok(PLA),
            "PLP" => Ok(PLP),
            "ROL" => Ok(ROL),
            "ROR" => Ok(ROR),
            "RTI" => Ok(RTI),
            "RTS" => Ok(RTS),
            "SBC" => Ok(SBC),
            "SEC" => Ok(SEC),
            "SED" => Ok(SED),
            "SEI" => Ok(SEI),
            "STA" => Ok(STA),
            "STX" => Ok(STX),
            "STY" => Ok(STY),
            "TAX" => Ok(TAX),
            "TAY" => Ok(TAY),
            "TSX" => Ok(TSX),
            "TXA" => Ok(TXA),
            "TXS" => Ok(TXS),
            "TYA" => Ok(TYA),
            // TODO(matt): add undocumented mnemonics
            _ => Err(ParseMnemonicError{}),
        }
    }
}