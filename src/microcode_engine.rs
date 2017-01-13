/*
    Defines the microcode architecture used to implement the 6502.
*/

#[derive(Debug)]
pub enum MicroOperation {
    Fetch,
    FetchNext,
    NextCycle,

    Clear(u8),
    Set(u8),

    // TODO(matt): add other microcode instructions...
}

#[derive(Debug)]
pub struct MicrocodeEngine {
    // counter for cycles executed
    pub cycles: usize,
    // microcode cycles executed
    pub mc_cycles: usize,

    pub instruction_loaded: bool,
    pub instruction: InstructionDef,
    pub sub_step: usize,
}

#[derive(Debug)]
pub struct InstructionDef { pub ops: Vec<MicroOperation> }

impl From<u8> for InstructionDef {
    fn from(op:u8) -> Self {
        unimplemented!()
    }
}