use crate::cpu::instructions::macros::implement_arithmetic_executor;
use crate::{
    cpu::state::{InstructionExecutor, InstructionState},
    memory::{LoadOps, MemoryProcessor, StoreOps},
    riscv::{Instruction, InstructionType, Register},
};
use nexus_common::cpu::{Processor, Registers};

pub struct SubInstruction {
    rd: (Register, u32),
    rs1: u32,
    rs2: u32,
}

implement_arithmetic_executor!(SubInstruction, |a: u32, b: u32| a.wrapping_sub(b));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::state::Cpu;
    use crate::riscv::{BuiltinOpcode, Instruction, Opcode, Register};

    #[test]
    fn test_sub_instruction() {
        let mut cpu = Cpu::default();

        // Set initial register values
        cpu.registers.write(Register::X1, 50);
        cpu.registers.write(Register::X2, 20);

        let bare_instruction = Instruction::new_ir(Opcode::from(BuiltinOpcode::SUB), 3, 1, 2);

        let mut instruction = SubInstruction::decode(&bare_instruction, &cpu.registers);

        // Execute the add instruction
        instruction.execute();
        let res = instruction.write_back(&mut cpu);

        // Check the result
        assert_eq!(res, Some(30));
        assert_eq!(cpu.registers.read(Register::X3), 30);
    }

    #[test]
    fn test_sub_underflow() {
        let mut cpu = Cpu::default();

        // Set initial register values
        cpu.registers.write(Register::X1, 0);
        cpu.registers.write(Register::X2, 1);

        let bare_instruction = Instruction::new_ir(Opcode::from(BuiltinOpcode::SUB), 3, 1, 2);

        let mut instruction = SubInstruction::decode(&bare_instruction, &cpu.registers);

        // Execute the sub instruction
        instruction.execute();
        let res = instruction.write_back(&mut cpu);

        // Check the result (should wrap around to u32::MAX)
        assert_eq!(res, Some(u32::MAX));
        assert_eq!(cpu.registers.read(Register::X3), u32::MAX);
    }
}
