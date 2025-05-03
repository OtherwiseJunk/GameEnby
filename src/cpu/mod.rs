pub mod flags_register;
pub mod instruction;
pub mod registers;

use self::instruction::{ArithmeticTarget, Instruction};
use self::registers::Registers;
use crate::memory_bus::MemoryBus;

struct CPU {
    registers: Registers,
    program_counter: u16,
    stack_pointer: u16,
    memory_bus: MemoryBus,
}

impl CPU {
    fn step(&mut self) {
        let mut instruction_byte = self.memory_bus.read_byte(self.program_counter);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.memory_bus.read_byte(self.program_counter + 1);
        }
        let next_pc: u16 = if let Some(instruction) =
            Instruction::from_byte(instruction_byte, false)
        {
            self.execute(instruction);
        } else {
            let description = format!("{:#04x}", if prefixed { "CB" } else { instruction_byte });
            panic!("Unknown instruction found for: {}", description);
        };

        self.program_counter = next_pc;
    }
    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => match target {
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                    self.program_counter.wrapping_add(1)
                }
                _ => self.program_counter,
            },
            _ => self.program_counter,
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        // set flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        new_value
    }
}
