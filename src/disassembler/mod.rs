//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

mod opcode_handlers;

use core::cmp::Ordering;

use crate::MgMipsVersion;

use super::instruction::*;
use super::instruction::mnemonics::*;
use super::operands::*;
use super::utils::string::*;
use super::error::*;

#[derive(Debug, Copy, Clone)]
pub struct MgDisassembler{
    pub version: MgMipsVersion,
}

struct FieldInfos{
    mask: u32,                    //The mask of bits this field takes
    op_type: Option<MgOperandType>,         //Defines the type of this operand, if there's no type, the field 
                                    //reprsented by this struct should be skipped
    coprocessor: Option<MgCoprocessor>,     //Defines the coprocessor of the register if op_type a register
    fixed: bool,                    //Means that the field is supposed to be 0x00
    operand_order: usize,           //Order of operand in the instruction string
}

impl FieldInfos{
    fn reg_field(operand_order: usize, coprocessor: MgCoprocessor, op_type: MgOperandType) -> FieldInfos{
        FieldInfos{
            mask: 0b11111, op_type: Some(op_type),
            coprocessor: Some(coprocessor), fixed: false,
            operand_order
        }
    }
    fn default_reg_field(operand_order: usize, coprocessor: MgCoprocessor) -> FieldInfos{
        FieldInfos{
            mask: 0b11111, op_type: Some(MgOperandType::Reg),
            coprocessor: Some(coprocessor), fixed: false,
            operand_order
        }
    }
    fn default_imm_field(operand_order: usize) -> FieldInfos{
        FieldInfos{
            mask: 0b1111111111111111, op_type: Some(MgOperandType::Imm),
            coprocessor: Some(MgCoprocessor::Cpu), fixed: false,
            operand_order
        }
    }
    fn imm_field(order: usize, mask: u32) -> FieldInfos{
        FieldInfos{
            mask: mask, op_type: Some(MgOperandType::Imm),
            coprocessor: None, fixed: false,
            operand_order: order
        }
    }
    fn fixed_field(operand_order: usize, mask: u32) -> FieldInfos{
        FieldInfos{
            mask: mask, op_type: None,
            coprocessor: None, fixed: true,
            operand_order
        }
    }
    fn default_fixed_field() -> FieldInfos{
        FieldInfos{
            mask: 0b11111, op_type: None,
            coprocessor: None, fixed: true,
            operand_order: 4
        }
    }

}

impl MgDisassembler{
    pub fn new_disassembler(version: MgMipsVersion) -> MgDisassembler{
        MgDisassembler{
            version
        }
    }
    //Opcode handlers map
    pub fn disassemble(&self, memory: u32, address: u64) -> Result<MgInstruction, MgError>{
        //Une map qui réunit tous les handlers des opcodes, il y a d'autre map dans cette map
        const OPCODE_MAP: [fn (disass: &MgDisassembler, instruction: &mut MgInstructionContext) -> Result<(), MgError>; 64] = [
            MgDisassembler::special_opcode_map, MgDisassembler::regimm_opcode_map, MgDisassembler::j, MgDisassembler::jal, MgDisassembler::beq, MgDisassembler::bne,  MgDisassembler::blez,  MgDisassembler::bgtz,
            MgDisassembler::bovc_bnvc,  MgDisassembler::addi_addiu,  MgDisassembler::slti_sltiu,  MgDisassembler::slti_sltiu,  MgDisassembler::andi,  MgDisassembler::ori,  MgDisassembler::xori,  MgDisassembler::lui,
            MgDisassembler::cop0_opcode_map,  MgDisassembler::cop1_opcode_map,  MgDisassembler::cop2_opcode_map,  MgDisassembler::cop1x_opcode_map,  MgDisassembler::beql,  MgDisassembler::bnel,  MgDisassembler::blezl,  MgDisassembler::bgtzl,
            MgDisassembler::bovc_bnvc,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::special2_opcode_map,  MgDisassembler::jalx,  MgDisassembler::no_instructions,  MgDisassembler::special3_opcode_map,
            MgDisassembler::cpu_loadstore,  MgDisassembler::cpu_loadstore,  MgDisassembler::lwr_swr_lwl_swl,  MgDisassembler::cpu_loadstore,  MgDisassembler::cpu_loadstore,  MgDisassembler::cpu_loadstore,  MgDisassembler::lwr_swr_lwl_swl,  MgDisassembler::no_instructions,
            MgDisassembler::cpu_loadstore,  MgDisassembler::cpu_loadstore,  MgDisassembler::lwr_swr_lwl_swl,  MgDisassembler::cpu_loadstore,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::lwr_swr_lwl_swl,  MgDisassembler::cache_pref,
            MgDisassembler::sc_ll,  MgDisassembler::cpu_loadstore,  MgDisassembler::bc_balc,  MgDisassembler::cache_pref,  MgDisassembler::no_instructions, MgDisassembler::cpu_loadstore, MgDisassembler::jic_jialc,  MgDisassembler::no_instructions,
            MgDisassembler::sc_ll,  MgDisassembler::cpu_loadstore,  MgDisassembler::bc_balc,  MgDisassembler::pcrel_opcode_map,  MgDisassembler::no_instructions,  MgDisassembler::cpu_loadstore,  MgDisassembler::jic_jialc,  MgDisassembler::no_instructions];

        let mut instruction: MgInstructionContext = MgInstructionContext{
            category: None,
            format: None,
            operand_num: 0,
            is_conditional: false,
            opcode: (memory >> 26) as u8,
            coprocessor: match memory >> 26{
                0x10 => Some(MgCoprocessor::Cp0),
                0x11 => Some(MgCoprocessor::Cp1),
                0x12 => Some(MgCoprocessor::Cp2),
                0x13 => Some(MgCoprocessor::Cp1x),
                _ => Some(MgCoprocessor::Cpu),
            },
            machine_code: memory,
            operand: [None; 4],
            is_relative: false,
            is_region: false,
            string: MgString::new_lmstring(),
            version: Some(self.version),
            mnemonic: None,
            mnemonic_id: None,
            address,
        };
        
        return match OPCODE_MAP[(memory >> 26) as usize](self, &mut instruction) {
            Err(e) => Err(e),
            Ok(()) => {
                match MgInstruction::new_instruction(instruction){
                    Ok(i) => Ok(i),
                    Err(e) => return Err(e),
                }
            },
        }
    }
    fn reg_format(&self, instruction: &mut MgInstructionContext, rs: Option<FieldInfos>, rt: Option<FieldInfos>, rd: Option<FieldInfos>, sa: Option<FieldInfos>) -> Result<(), MgError>{
        let mut hex_num: MgString = MgString::new_lmstring();
        let comma: &str = ", ";

        instruction.format = Some(MgInstructionFormat::Reg);

        //Rs field
        if let Some(field) = rs{
            let field_mask_result = instruction.machine_code >> 21 & field.mask;
            if field.fixed == false{
                if let Some(op_type) = field.op_type {
                    instruction.operand[field.operand_order] = match op_type{
                        MgOperandType::Imm =>{
                            instruction.operand_num += 1;
                            Some(MgOpImmediate::new_imm_opreand(field_mask_result as u64))
                        },
                        MgOperandType::Reg => {
                            instruction.operand_num += 1;
                            let Some(cop) = field.coprocessor else{
                                return Err(MgError::throw_error(MgErrorCode::DevError, instruction.opcode, instruction.address, instruction.machine_code))
                            };
                            Some(MgOpRegister::new_reg_opreand(field_mask_result as u8, cop))
                        },
                    }
                }
            }
            else if field_mask_result != 0{
                return Err(MgError::throw_error(MgErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
            }
        }
        //Rt field
        if let Some(field) = rt{
            let field_mask_result = instruction.machine_code >> 16 & field.mask;
            if field.fixed == false{
                if let Some(op_type) = field.op_type {
                    instruction.operand[field.operand_order] = match op_type{
                        MgOperandType::Imm =>{
                            instruction.operand_num += 1;
                            Some(MgOpImmediate::new_imm_opreand(field_mask_result as u64))
                        },
                        MgOperandType::Reg => {
                            instruction.operand_num += 1;
                            let Some(cop) = field.coprocessor else{
                                return Err(MgError::throw_error(MgErrorCode::DevError, instruction.opcode, instruction.address, instruction.machine_code))
                            };
                            Some(MgOpRegister::new_reg_opreand(field_mask_result as u8, cop))
                        },        
                    }
                }
            }
            else if field_mask_result != 0{
                return Err(MgError::throw_error(MgErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
            }
        }
        //Rd field
        if let Some(field) = rd{
            let field_mask_result = instruction.machine_code >> 11 & field.mask;
            if field.fixed == false{
                if let Some(op_type) = field.op_type {
                    instruction.operand[field.operand_order] = match op_type{
                        MgOperandType::Imm =>{
                            instruction.operand_num += 1;
                            Some(MgOpImmediate::new_imm_opreand(field_mask_result as u64))
                        },
                        MgOperandType::Reg => {
                            instruction.operand_num += 1;
                            let Some(cop) = field.coprocessor else{
                                return Err(MgError::throw_error(MgErrorCode::DevError, instruction.opcode, instruction.address, instruction.machine_code))
                            };
                            Some(MgOpRegister::new_reg_opreand(field_mask_result as u8, cop))
                        },        
                    }
                }
            }
            else if field_mask_result != 0{
                return Err(MgError::throw_error(MgErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
            }
        }
        //Sa field
        if let Some(field) = sa{
            let field_mask_result = instruction.machine_code >> 6 & field.mask;
            if field.fixed == false{
                if let Some(op_type) = field.op_type {
                    instruction.operand[field.operand_order] = match op_type{
                        MgOperandType::Imm =>{
                            instruction.operand_num += 1;
                            Some(MgOpImmediate::new_imm_opreand(field_mask_result as u64))
                        },
                        MgOperandType::Reg => {
                            instruction.operand_num += 1;
                            let Some(cop) = field.coprocessor else{
                                return Err(MgError::throw_error(MgErrorCode::DevError, instruction.opcode, instruction.address, instruction.machine_code))
                            };
                            Some(MgOpRegister::new_reg_opreand(field_mask_result as u8, cop))
                        },        
                    }
                }
            }
            else if field_mask_result != 0{
                return Err(MgError::throw_error(MgErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
            }
        }

        let Some(mne) = instruction.mnemonic else{
            return Err(MgError::throw_error(MgErrorCode::DevError, instruction.opcode, instruction.address, instruction.machine_code))
        };
        instruction.string.append_str(mne);
        instruction.string.append_char(' ');
        for i in 0..instruction.operand_num{
            if let Some(MgOperand::MgOpRegister(reg)) = instruction.operand[i]{
                instruction.string.append_str(reg.get_register());
            }
            else if let Some(MgOperand::MgOpImmediate(imm)) = instruction.operand[i]{
                hex_num.num_to_str(imm.get_value());
                instruction.string.append_string(&hex_num);
            }

            if instruction.operand_num - 1 > i{
                instruction.string.append_str(&comma);
            }
        }
        Ok(())
    }
    fn basic_str_format(instruction: &mut MgInstructionContext) -> Result<(), MgError>{
        let mut hex_num: MgString = MgString::new_lmstring();
        let comma: &str = ", ";

        let Some(mne) = instruction.mnemonic else{
            return Err(MgError::throw_error(MgErrorCode::DevError, instruction.opcode, instruction.address, instruction.machine_code))
        };
        instruction.string.append_str(mne);
        instruction.string.append_char(' ');
        for i in 0..instruction.operand_num{
            if let Some(MgOperand::MgOpRegister(reg)) = instruction.operand[i]{
                instruction.string.append_str(reg.get_register());
            }
            else if let Some(MgOperand::MgOpImmediate(imm)) = instruction.operand[i]{
                hex_num.num_to_str(imm.get_value());
                instruction.string.append_string(&hex_num);
            }

            if instruction.operand_num - 1 > i{
                instruction.string.append_str(&comma);
            }
        }
        Ok(())
    }
    fn cpx_cpu_transfer_format(&self, instruction: &mut MgInstructionContext, rt: FieldInfos, rd: FieldInfos) -> Result<(), MgError>{
        if (instruction.machine_code & 0b11111111111) != 0{
            return Err(MgError::throw_error(MgErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
        }

        instruction.format = Some(MgInstructionFormat::CpxCpuTransfer);

        let (Some(rd_cop), Some(rt_cop)) = (rd.coprocessor, rt.coprocessor) else{
            return Err(MgError::throw_error(MgErrorCode::DevError, instruction.opcode, instruction.address, instruction.machine_code))
        };
        instruction.operand_num = 2;
        instruction.operand[rd.operand_order] = Some(MgOpRegister::new_reg_opreand((instruction.machine_code >> 11 & rd.mask) as u8, rd_cop));
        instruction.operand[rt.operand_order] = Some(MgOpRegister::new_reg_opreand((instruction.machine_code >> 16 & rt.mask) as u8, rt_cop));

        MgDisassembler::basic_str_format(instruction)
    }
    fn imm_format(&self, instruction: &mut MgInstructionContext, rs: Option<FieldInfos>, rt: Option<FieldInfos>, imm: Option<FieldInfos>) -> Result<(), MgError>{

        //Some attributes about the instruction and setting the operands
        instruction.format = Some(MgInstructionFormat::Imm);
        //Rs field
        if let Some(field) = rs{
            let field_mask_result = instruction.machine_code >> 21 & field.mask;
            if field.fixed == false{
                if let Some(op_type) = field.op_type {
                    instruction.operand[field.operand_order] = match op_type{
                        MgOperandType::Imm =>{
                            instruction.operand_num += 1;
                            Some(MgOpImmediate::new_imm_opreand(field_mask_result as u64))
                        },
                        MgOperandType::Reg => {
                            instruction.operand_num += 1;
                            let Some(cop) = field.coprocessor else{
                                return Err(MgError::throw_error(MgErrorCode::DevError, instruction.opcode, instruction.address, instruction.machine_code))
                            };
                            Some(MgOpRegister::new_reg_opreand(field_mask_result as u8, cop))
                        },        
                    }
                }
            }
            else if field_mask_result != 0{
                return Err(MgError::throw_error(MgErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
            }
        }
        //Rt field
        if let Some(field) = rt{
            let field_mask_result = instruction.machine_code >> 16 & field.mask;
            if field.fixed == false{
                if let Some(op_type) = field.op_type {
                    instruction.operand[field.operand_order] = match op_type{
                        MgOperandType::Imm =>{
                            instruction.operand_num += 1;
                            Some(MgOpImmediate::new_imm_opreand(field_mask_result as u64))
                        },
                        MgOperandType::Reg => {
                            instruction.operand_num += 1;
                            let Some(cop) = field.coprocessor else{
                                return Err(MgError::throw_error(MgErrorCode::DevError, instruction.opcode, instruction.address, instruction.machine_code))
                            };
                            Some(MgOpRegister::new_reg_opreand(field_mask_result as u8, cop))
                        },        
                    }
                }
            }
            else if field_mask_result != 0{
                return Err(MgError::throw_error(MgErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
            }
        }
        //Imm field
        if let Some(imm) = imm{
            instruction.operand[imm.operand_order] = Some(MgOpImmediate::new_imm_opreand((instruction.machine_code & 0b1111111111111111) as u64));
            instruction.operand_num += 1;
        }
        let (Some(mne), Some(category)) = (instruction.mnemonic, instruction.category) else{
            return Err(MgError::throw_error(MgErrorCode::DevError, instruction.opcode, instruction.address, instruction.machine_code))
        };

        return if category == MgInstructionCategory::Load || category == MgInstructionCategory::Store
        || category == MgInstructionCategory::MemoryControl || mne.cmp(MG_MNE_CACHE)  == Ordering::Equal{
            MgDisassembler::imm_loadstore_str_format(instruction)
        }
        else {
            MgDisassembler::imm_default_str_format(instruction)
        }
    }
    fn imm_default_str_format(instruction: &mut MgInstructionContext) -> Result<(), MgError>{
        let mut hex_num: MgString = MgString::new_lmstring();
        let comma: &str = ", ";

        let Some(mne) = instruction.mnemonic else{
            return Err(MgError::throw_error(MgErrorCode::DevError, instruction.opcode, instruction.address, instruction.machine_code))
        };
        instruction.string.append_str(mne);
        instruction.string.append_char(' ');
        for i in 0..instruction.operand_num{
            if let Some(MgOperand::MgOpRegister(reg)) = instruction.operand[i]{
                instruction.string.append_str(reg.get_register());
            }
            else if let Some(MgOperand::MgOpImmediate(imm)) = instruction.operand[i]{
                hex_num.num_to_str(imm.get_value());
                instruction.string.append_string(&hex_num);
            }

            if instruction.operand_num - 1 > i{
                instruction.string.append_str(&comma);
            }
        }
        Ok(())
    }
    fn imm_loadstore_str_format(instruction: &mut MgInstructionContext) -> Result<(), MgError>{
        let mut hex_num: MgString = MgString::new_lmstring();
        let comma: &str = ", ";

        let Some(mne) = instruction.mnemonic else{
            return Err(MgError::throw_error(MgErrorCode::DevError, instruction.opcode, instruction.address, instruction.machine_code))
        };
        instruction.string.append_str(mne);
        instruction.string.append_char(' ');
        for i in 0..instruction.operand_num - 1{
            if let Some(MgOperand::MgOpRegister(reg)) = instruction.operand[i]{
                instruction.string.append_str(reg.get_register());
            }
            else if let Some(MgOperand::MgOpImmediate(imm)) = instruction.operand[i]{
                hex_num.num_to_str(imm.get_value());
                instruction.string.append_string(&hex_num);
            }
            if instruction.operand_num - 2 > i{
                instruction.string.append_str(&comma);
            }
        }
        instruction.string.append_char('(');
        if let Some(MgOperand::MgOpRegister(reg)) = instruction.operand[instruction.operand_num - 1]{
            instruction.string.append_str(reg.get_register());
        }
        instruction.string.append_char(')');
        Ok(())
    }
    fn jump_format(&self, instruction: &mut MgInstructionContext) -> Result<(), MgError>{
        let mut hex_num: MgString = MgString::new_lmstring();

        //Some attributes about the instruction
        instruction.format = Some(MgInstructionFormat::Jump);
        instruction.operand_num = 1 ;
        instruction.is_region = true;
        instruction.category = Some(MgInstructionCategory::BranchJump);
        instruction.operand[0] = Some(MgOpImmediate::new_imm_opreand((instruction.machine_code & 0x3FFFFFF) as u64));

        //Formatting the string
        //If the branch/jump is relative, the string will show it's destination address instead of the offset
        let (Some(MgOperand::MgOpImmediate(imm)), Some(mne)) = (instruction.operand[0], instruction.mnemonic) else{
            return Err(MgError::throw_error(MgErrorCode::DevError, instruction.opcode, instruction.address, instruction.machine_code))
        };
        instruction.string.append_str(mne);
        hex_num.num_to_str(imm.get_value() * 0x4 + instruction.address);
        instruction.string.append_char(' ');
        instruction.string.append_string(&hex_num);

        Ok(())
    }
}