/*
    Designed to be a per cycle MOS 6502 CPU emulator. Each cycle bus states
    are correct. Wasted reads, writes are included. Pipelining is functional.
    Terminalogy/names are from MCS6500 Family Programming Manual.

    It does not emulate half cycles.
*/

mod microcode_engine;
mod mnemonic;

use microcode_engine::*;

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


    mc_engine: MicrocodeEngine,
}

impl Mos6502 {
    
    // 
    pub fn new() -> Self {
        // TODO(matt): verify what we want pc to start as.
        Mos6502 {
            a: 0, x: 0, y: 0, s: 0, pc: 0x00ff, p: 0x22, ir: 0, adl: 0, adh: 0, 
            mc_engine: MicrocodeEngine { cycles: 0, mc_cycles: 0, instruction_loaded: false, instruction: InstructionDef::from(0x00), sub_step: 0 },
        }
    }

    // steps the CPU one cycle
    pub fn step(&mut self, bus: &mut Bus) {
        if self.mc_engine.cycles <= 8 {
            // TODO(matt): add start-up instructions
            // ref <https://www.c64-wiki.com/index.php/Reset_(Process)>
        } else {
            // normal operation
            let mut cycle_complete = false;
            // load instruction once
            if !self.mc_engine.instruction_loaded { 
                self.mc_engine.instruction = InstructionDef::from(self.ir);
                self.mc_engine.sub_step = 0;
            }
            // loop of micro_ops until next cycle op
            while !cycle_complete {
                let ref micro_op = self.mc_engine.instruction.ops[self.mc_engine.sub_step];
                match micro_op {
                    // TODO(matt): impletment micro_ops
                    _ => panic!("Unimplemented microcode {:?}", micro_op),
                }
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
