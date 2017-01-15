/*
    Designed to be a per cycle MOS 6502 CPU emulator. Each cycle bus states
    are correct. Wasted reads, writes are included. Pipelining is functional.
    Terminalogy/names are from MCS6500 Family Programming Manual.

    It does not emulate half cycles.
*/

use std::collections::HashMap;

mod mnemonic;

// Defines where the bits of the processor status register are.
const C_BIT: u8 = 0b0000_0001;
const Z_BIT: u8 = 0b0000_0010;
const I_BIT: u8 = 0b0000_0100;
const D_BIT: u8 = 0b0000_1000;
const B_BIT: u8 = 0b0001_0000;
const V_BIT: u8 = 0b0100_0000;
const N_BIT: u8 = 0b1000_0000;

// Memory bus interface.
pub trait Bus {
    fn read_8(&self, addr: u16) -> u8;
    fn write_8(&mut self, addr: u16, value: u8);
}

pub struct Mos6502 {
    // accumulator
    pub a: u8,
    // x-index
    pub x: u8,
    // y-index
    pub y: u8,
    // stack pointer
    pub s: u8,
    // program counter
    pub pc: u16,
    // processor status flags
    // nv-bdizc
    // TODO(matt): add better description
    pub p: u8,

    // internal variables

    // instruction register
    ir: u8,

    // address bus low and high
    adl: u8,
    adh: u8,

    // counter for cycles executed
    cycles: usize,
    // microcode cycles executed
    mc_cycles: usize,

    sub_step: usize,

    instructions: HashMap<u8, Vec<MicroCode>>,
}

impl Mos6502 {
    
    // 
    pub fn new() -> Self {
        let mut cpu = Mos6502 {
            a: 0, x: 0, y: 0, s: 0, pc: 0x00ff, p: 0x22, ir: 0, adl: 0, adh: 0, 
            cycles: 0, mc_cycles: 0, instructions: HashMap::new(), sub_step: 0,
        };

        use self::MicroCode::*;
        use self::What::*;
        // addressing modes
        const IMPLIED: [MicroCode; 4] = [Fetch(Opcode), NextCycle, Fetch(DiscardOpcode), NextCycle];
        const IMMEDIATE: [MicroCode; 4] = [Fetch(Opcode), NextCycle, Fetch(Imm), NextCycle];
        const ABSOLUTE: [MicroCode; 8] = [Fetch(Opcode), NextCycle, Fetch(Adl), NextCycle, Fetch(Adh), NextCycle, Read(Adh_Adl), NextCycle];
        const ZERO_PAGE: [MicroCode; 6] = [Fetch(Opcode), NextCycle, Fetch(Adl), NextCycle, Read(Adl), NextCycle];
        const RELATIVE: [MicroCode; 4] = [Fetch(Opcode), NextCycle, Fetch(Offset), NextCycle]; // TODO(matt): finish
        const ABSOLUTE_X: [MicroCode; 8] = [Fetch(Opcode), NextCycle, Fetch(Adl), NextCycle, Fetch(Adh), NextCycle, Read(Adh_Adl), NextCycle]; // TODO(matt): finish
        /*const ABSOLUTE_Y: [MicroCode; 2] = [];
        const ZERO_PAGE_X: [MicroCode; 2] = [];
        const ZERO_PAGE_Y: [MicroCode; 2] = [];
        const INDIRECT: [MicroCode; 2] = [];
        const INDEXED_INDIRECT: [MicroCode; 2] = [];
        const INDIRECT_INDEXED: [MicroCode; 2] = [];
        */

        // instructions
        const CLC: [MicroCode; 2] = [Clear(1), NextCycle];

        // this can be a macro
        macro_rules! add_instruction {
            ($op:tt, $addr:tt, $inst:tt) => (
                cpu.instructions.insert($op, concatenate_arrays::<MicroCode>(&$addr, &$inst));
            )
        }

        add_instruction!(0x18, IMMEDIATE, CLC);

        //cpu.instructions.insert(0x18, concatenate_arrays::<MicroCode>(&IMMEDIATE, &CLC));

        cpu
    }

    // steps the CPU one cycle
    pub fn step(&mut self, bus: &mut Bus) {
        if self.cycles <= 8 {
            // TODO(matt): add start-up instructions
            // ref <https://www.c64-wiki.com/index.php/Reset_(Process)>
        } else {
            // normal operation
            let mut cycle_complete = false;
            // loop of micro_ops until next cycle op
            if let Some(inst) = self.instructions.get(&self.ir) {
                while !cycle_complete {
                    let ref micro_op = inst[self.sub_step];
                    match micro_op {
                        // TODO(matt): impletment micro_ops
                        _ => panic!("Unimplemented microcode {:?}", micro_op),
                    }
                }
            } else {
                panic!("Opcode 0x{:02x} - Instruction not implemented!");
            }
        }
    }

    // interrupt request
    pub fn irq() {
        unimplemented!();
    }

    // fires an non-maskable interrupt
    pub fn nmi() {
        unimplemented!();
    }
}

#[derive(Debug, Clone)]
enum What {
    Opcode,
    DiscardOpcode,
    Adl,
    Adh,
    Adh_Adl,
    Imm,
    Offset,
}

#[derive(Debug, Clone)]
enum MicroCode {
    Fetch(What),
    Read(What),
    NextCycle,

    Clear(u8),
    Set(u8),

    // TODO(matt): add other microcode instructions...
}

fn concatenate_arrays<T: Clone>(x: &[T], y: &[T]) -> Vec<T> {
    let mut concat: Vec<T> = vec![x[0].clone(); x.len()];
 
    concat.clone_from_slice(x);
    concat.extend_from_slice(y);
 
    concat
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
