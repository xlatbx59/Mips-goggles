//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

pub mod registers;
use super::instruction::MgCoprocessor;
use super::operands::registers::*;
use super::instruction;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MgOperandType{
    Imm, Reg
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct MgOpRegister{
    register: &'static str,
    coprocessor: MgCoprocessor,
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct MgOpImmediate{
    value: u64,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum MgOperand{
    MgOpRegister(MgOpRegister), MgOpImmediate(MgOpImmediate)
}

impl MgOpImmediate{
    pub fn new_imm_opreand(value: u64) -> MgOperand{
        MgOperand::MgOpImmediate(MgOpImmediate{
            value,
        })
    }
    pub fn get_value(&self) -> u64{
        self.value
    }
}

impl MgOpRegister{
    fn get_reg_str(register: u8, coprocessor: instruction::MgCoprocessor) -> &'static str{
        static CPU_REGISTER_TABLE: [&str; 32] = [
            MG_REG_ZERO, MG_REG_AT, MG_REG_V0, MG_REG_V1, MG_REG_A0, MG_REG_A1, MG_REG_A2, MG_REG_A3,
            MG_REG_T0, MG_REG_T1, MG_REG_T2, MG_REG_T3, MG_REG_T4, MG_REG_T5, MG_REG_T6, MG_REG_T7,
            MG_REG_S0, MG_REG_S1, MG_REG_S2, MG_REG_S3, MG_REG_S4, MG_REG_S5, MG_REG_S6, MG_REG_S7,
            MG_REG_T8, MG_REG_T9, MG_REG_K0, MG_REG_K1, MG_REG_GP, MG_REG_SP, MG_REG_FP, MG_REG_RA,
        ];
        static FPU_REGISTER_TABLE: [&str; 32] = [
            MG_REG_F0, MG_REG_F1, MG_REG_F2, MG_REG_F3, MG_REG_F4, MG_REG_F5, MG_REG_F6, MG_REG_F7,
            MG_REG_F8, MG_REG_F9, MG_REG_F10, MG_REG_F11, MG_REG_F12, MG_REG_F13, MG_REG_F14, MG_REG_F15,
            MG_REG_F16, MG_REG_F17, MG_REG_F18, MG_REG_F19, MG_REG_F20, MG_REG_F21, MG_REG_F22, MG_REG_F23,
            MG_REG_F24, MG_REG_F25, MG_REG_F26, MG_REG_F27, MG_REG_F28, MG_REG_F29, MG_REG_F30, MG_REG_F31,
        ];
        static MG_DEFAULT_REG_TABLE: [&str; 32] = [
            MG_REG_0, MG_REG_1, MG_REG_2, MG_REG_3, MG_REG_4, MG_REG_5, MG_REG_6, MG_REG_7,
            MG_REG_8, MG_REG_9, MG_REG_10, MG_REG_11, MG_REG_12, MG_REG_13, MG_REG_14, MG_REG_15,
            MG_REG_16, MG_REG_17, MG_REG_18, MG_REG_19, MG_REG_20, MG_REG_21, MG_REG_22, MG_REG_23,
            MG_REG_24, MG_REG_25, MG_REG_26, MG_REG_27, MG_REG_28, MG_REG_29, MG_REG_30, MG_REG_31,
        ];
        
        return match coprocessor{
            instruction::MgCoprocessor::Cp1 => FPU_REGISTER_TABLE[register as usize],
            instruction::MgCoprocessor::Cpu => CPU_REGISTER_TABLE[register as usize],
            _ => MG_DEFAULT_REG_TABLE[register as usize],
        }
    }
    pub fn new_reg_opreand(register: u8, coprocessor: MgCoprocessor) -> MgOperand{
        MgOperand::MgOpRegister(MgOpRegister{
            coprocessor,
            register: MgOpRegister::get_reg_str(register, coprocessor),
        })
    }
    pub fn new_reg_operand_str(reg_str: &'static str, coprocessor: instruction::MgCoprocessor) -> MgOperand{
        MgOperand::MgOpRegister(MgOpRegister{
            coprocessor,
            register: reg_str,
        })
    }
    pub fn get_register(&self) -> &'static str{
        self.register
    }
    pub fn get_coprocessor(&self) -> MgCoprocessor{
        self.coprocessor
    }

}