/*
    Defines the microcode architecture used to implement the 6502.
*/

#[derive(Debug, Clone)]
pub enum MicroCode {
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

use std::collections::HashMap;

#[derive(Debug)]
pub struct InstructionDef { 
    pub ops: Vec<MicroCode>,
    
    pub instructions: Option<HashMap<u8, Vec<MicroCode>>>,
}

impl From<u8> for InstructionDef {
    
    fn from(op:u8) -> Self {
        use self::MicroCode::*;

        const IMM: [MicroCode; 2] = [Fetch, NextCycle];

        const CLC: [MicroCode; 2] = [Clear(1), NextCycle];

        let mut instructions = HashMap::new();

        // this can be a macro
        instructions.insert(0x18, concatenate_arrays::<MicroCode>(&IMM, &CLC));

        unimplemented!()
    }
}

fn concatenate_arrays<T: Clone>(x: &[T], y: &[T]) -> Vec<T> {
    let mut concat: Vec<T> = vec![x[0].clone(); x.len()];
 
    concat.clone_from_slice(x);
    concat.extend_from_slice(y);
 
    concat
}