//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

pub mod mnemonics;
use mnemonics::{MgMnemonic, MG_MNEMONICS};
use super::MgMipsVersion;
use super::operands::*;
use super::error::*;
use super::utils::string::MgString;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MgInstructionFormat{
    Imm, Reg, Jump, Other,
    CoditionCodeFpu, CpxCpuTransfer,
    Mfmc0, 
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MgInstructionCategory{
    BranchJump, Load,
    Store, Move, Priviledge,
    Logical, Arithmetic, Control,
    Trap, MemoryControl, _Ejtag,
    InsertExtract, Shift,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MgCoprocessor{
    Cpu, Cp0, Cp1, Cp2, Cp1x
}

#[derive(Debug)]
pub (crate) struct MgInstructionContext{
    pub address: u64,
    pub mnemonic: Option<&'static str>,
    pub opcode: u8,
    pub machine_code: u32,
    pub string: MgString,
    pub category: Option<MgInstructionCategory>,
    pub format: Option<MgInstructionFormat>,
    pub coprocessor: Option<MgCoprocessor>,
    pub version: Option<MgMipsVersion>,
    pub is_conditional: bool,
    pub is_relative: bool,
    pub is_region: bool,
    pub operand_num: usize,
    pub operand: [Option<MgOperand>; 4],    //L'ordre des opérandes suit celui du format en chaîne de caractères 
}

//TODO: Version
#[derive(Debug)]
pub struct MgInstruction{
    address: u64,
    mnemonic: &'static str,
    operand: [Option<MgOperand>; 4],    //L'ordre des opérandes suit celui du format en chaîne de caractères 
    machine_code: u32,
    operand_num: usize,
    string: MgString,
    category: MgInstructionCategory,
    format: MgInstructionFormat,
    coprocessor: MgCoprocessor,
    is_conditional: bool,
    is_relative: bool,
    opcode: u8,
    version: MgMipsVersion,
    is_region: bool,
}

pub fn get_mnemonic(mnemonic: MgMnemonic) -> &'static str{
    MG_MNEMONICS[mnemonic as usize]
}

impl MgInstruction{
    pub (crate) fn new_instruction(context: MgInstructionContext) -> Result<MgInstruction, MgError>{
        let (Some(category), Some(format)) = (context.category, context.format) else{
            return Err(MgError::throw_error(MgErrorCode::DevError, context.opcode, context.address, context.machine_code))
        };
        let Some(version) = context.version else{
            return Err(MgError::throw_error(MgErrorCode::DevError, context.opcode, context.address, context.machine_code))
        };
        let (Some(coprocessor), Some(mnemonic)) = (context.coprocessor, context.mnemonic) else{
            return Err(MgError::throw_error(MgErrorCode::DevError, context.opcode, context.address, context.machine_code))
        };
        Ok(MgInstruction{
            address: context.address,
            opcode: context.opcode,
            machine_code: context.machine_code,
            mnemonic,
            string: context.string,
            category,
            format,
            coprocessor,
            is_conditional: context.is_conditional,
            is_relative: context.is_relative,
            is_region: context.is_region,
            operand_num: context.operand_num,
            version,
            operand: context.operand
        })
    }
    pub fn is_region(&self) -> bool{
        self.is_region
    }
    pub fn is_relative(&self) -> bool{
        self.is_relative
    }
    pub fn is_conditional(&self) -> bool{
        self.is_conditional
    }
    pub fn get_operand_num(&self) -> usize{
        self.operand_num
    }
    pub fn get_coprocessor(&self) -> MgCoprocessor{
        self.coprocessor
    }
    pub fn get_address(&self) -> u64{
        self.address
    }
    pub fn get_category(&self) -> MgInstructionCategory{
        self.category
    }
    pub fn get_format(&self) -> MgInstructionFormat{
        self.format
    }
    pub fn get_mnemonic(&self) -> &str{
        self.mnemonic
    }
    pub fn get_string(&self) -> &[char]{
        self.string.data()
    }
    pub fn get_machine_code(&self) -> u32{
        self.machine_code
    }
    pub fn get_opcode(&self) -> u8{
        self.opcode
    }
    pub fn get_operand(&self, index: usize) -> Option<MgOperand>{
        self.operand[index]
    }
}