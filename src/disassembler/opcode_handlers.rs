//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use crate::instruction::*;
use crate::instruction::mnemonics::*;
use crate::operands::*;
use crate::disassembler::*;
use crate::MgMips32;
use registers::*;
use FieldInfos;

//TODO: Je dois mettre les bonnes exceptions
//TODO: Dans le Release1 mfmc0 avait une autre exception, je dois rajouter les versions pour ça
impl MgDisassembler{
    pub (super) fn no_instructions(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        Err(MgError::throw_error(MgErrorCode::NoInstruction, prototype.opcode, prototype.address, prototype.machine_code))
    }
    pub (super) fn special_opcode_map(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        static SPECIAL_MAP: [fn(disassembler: &MgDisassembler, &mut MgInstructionPrototype) -> Result<(), MgError>; 64] = [
        MgDisassembler::sll,  MgDisassembler::movci,  MgDisassembler::srl_sra,  MgDisassembler::srl_sra,  MgDisassembler::sllv,  MgDisassembler::no_instructions,  MgDisassembler::srlv_srav,  MgDisassembler::srlv_srav,
        MgDisassembler::jr,  MgDisassembler::jalr,  MgDisassembler::movn_movz,  MgDisassembler::movn_movz,  MgDisassembler::syscall_break,  MgDisassembler::syscall_break,  MgDisassembler::no_instructions,  MgDisassembler::sync,
        MgDisassembler::mfhi_mflo,  MgDisassembler::mthi_mtlo,  MgDisassembler::mfhi_mflo,  MgDisassembler::mthi_mtlo,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
        MgDisassembler::mult_multu_div_divu,  MgDisassembler::mult_multu_div_divu,  MgDisassembler::mult_multu_div_divu,  MgDisassembler::mult_multu_div_divu,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
        MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,
        MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::slt_sltu,  MgDisassembler::slt_sltu,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
        MgDisassembler::tge_tgeu_tlt_tltu,  MgDisassembler::tge_tgeu_tlt_tltu,  MgDisassembler::tge_tgeu_tlt_tltu,  MgDisassembler::tge_tgeu_tlt_tltu,  MgDisassembler::teq_tne,  MgDisassembler::seleqz_selnez,  MgDisassembler::teq_tne,  MgDisassembler::seleqz_selnez,
        MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions ];

        SPECIAL_MAP[(prototype.machine_code & 0b111111) as usize](self, prototype)
    }
    pub (super) fn regimm_opcode_map(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let imm_order: usize;
        let rs: Option<FieldInfos>;
        static MENMONICS: [[Option<MgMnemonic>; 8]; 4] =
        [   [Some(MgMnemonic::MgMneBltz),  Some(MgMnemonic::MgMneBgez),  Some(MgMnemonic::MgMneBltzl),  Some(MgMnemonic::MgMneBgezl),  None, None, None, None],
            [Some(MgMnemonic::MgMneTgei),  Some(MgMnemonic::MgMneTgeiu),  Some(MgMnemonic::MgMneTlti),  Some(MgMnemonic::MgMneTltiu),  Some(MgMnemonic::MgMneTeqi), None, Some(MgMnemonic::MgMneTnei), None],
            [Some(MgMnemonic::MgMneBltzal),  Some(MgMnemonic::MgMneBgezal),  Some(MgMnemonic::MgMneBltzall),  Some(MgMnemonic::MgMneBgezall), None, None, None, None],
            [None, None, None, None, None, None, None, Some(MgMnemonic::MgMneSynci)]];

        if let MgMipsVersion::M32(MgMips32::MgR6) = self.version {
            if prototype.machine_code >> 19 & 0b11 == 1{
                return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            }
        };
        prototype.mnemonic = MENMONICS[(prototype.machine_code >> 19 & 0b11) as usize][(prototype.machine_code >> 16 & 0b111) as usize];
        prototype.category = Some(match prototype.machine_code >> 19 & 3{
            3 => MgInstructionCategory::MemoryControl,
            1 => {
                prototype.is_conditional = true;
                MgInstructionCategory::Trap
            },
            _ => {
                prototype.is_relative = true;
                prototype.is_conditional = true;
                MgInstructionCategory::BranchJump
            },
        });

        if (prototype.machine_code >> 16 & 0b111111) == 0x11
        && (prototype.machine_code >> 21 & 0b11111) == 0{
            prototype.mnemonic = Some(MgMnemonic::MgMneBal);
            rs = None;
            imm_order = 0;
            prototype.is_conditional = false;
        }
        else if (prototype.machine_code >> 16 & 0b111111) == 0x1f{
            imm_order = 0;
            rs = Some(FieldInfos::default_reg_field(1, MgCoprocessor::Cpu));
        }
        else{
            imm_order = 1;
            rs = Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu));
        }

        return MgDisassembler::imm_format(self, prototype, rs, None, Some(FieldInfos::default_imm_field(imm_order)))
    }
    pub (super) fn special2_opcode_map(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let MgMipsVersion::M32(MgMips32::MgR6) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };
        static SPECIAL2_MAP: [fn(disassembler: &MgDisassembler, &mut MgInstructionPrototype) -> Result<(), MgError>; 64] = 
            [   MgDisassembler::madd_maddu,  MgDisassembler::madd_maddu,  MgDisassembler::mul,  MgDisassembler::no_instructions,  MgDisassembler::msub_msubu,  MgDisassembler::msub_msubu,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::clz_clo,  MgDisassembler::clz_clo,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::sdbbp ];
        SPECIAL2_MAP[(prototype.machine_code & 0b111111) as usize](self, prototype)
    }
    pub (super) fn special3_opcode_map(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        static SPECIAL3_MAP: [fn(disassembler: &MgDisassembler, &mut MgInstructionPrototype) -> Result<(), MgError>; 64] = 
            [   MgDisassembler::ext,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::ins,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::bshfl,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::cache_pref,  MgDisassembler::sc_ll,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::cache_pref,  MgDisassembler::sc_ll,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::rdhwr,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions ];
        
        SPECIAL3_MAP[(prototype.machine_code & 0b111111) as usize](self, prototype)
    }
    pub (super) fn cop0_opcode_map(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        static COP0_MAP: [fn(disassembler: &MgDisassembler, &mut MgInstructionPrototype) -> Result<(), MgError>; 32] =
            [   MgDisassembler::mov_cp0,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::mov_cp0,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::gpr_shadowset,  MgDisassembler::mfmc0,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::gpr_shadowset,  MgDisassembler::no_instructions,
                MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,
                MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0];
        // unimplemented!("[-]Opcode map isn't implemented yet!");
        // prototype.coprocessor = MgCoprocessor::Cp0;
        COP0_MAP[(prototype.machine_code >> 21 & 0b11111) as usize](self, prototype)
    }
    pub (super) fn cop1_opcode_map(&self, _instruction: &mut MgInstructionPrototype) -> Result<(), MgError>{
        static _COP1_MAP: [fn(disassembler: &MgDisassembler, &mut MgInstructionPrototype) -> Result<(), MgError>; 64] =
        [   MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions ];
        unimplemented!("[-]Cop1 opcode map isn't implemented yet!");

        // COP1_MAP[(prototype.machine_code >> 26) as usize](prototype)
    }
    pub (super) fn cop2_opcode_map(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        static COP2_MAP: [fn(disassembler: &MgDisassembler, &mut MgInstructionPrototype) -> Result<(), MgError>; 64] = 
        [   MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::load_store_cp2,  MgDisassembler::load_store_cp2,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::load_store_cp2,  MgDisassembler::load_store_cp2,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions ];
        return COP2_MAP[(prototype.machine_code >> 21 & 0b11111) as usize](self, prototype)
    }
    pub (super) fn cop1x_opcode_map(&self,prototype : &mut MgInstructionPrototype) -> Result<(), MgError>{
        let MgMipsVersion::M32(MgMips32::MgR6) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };
        static _COP1X_MAP: [fn(disassembler: &MgDisassembler, &mut MgInstructionPrototype) -> Result<(), MgError>; 64] = 
        [   MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions ];
        unimplemented!("[-]Opcode map isn't implemented yet!");

        // prototype.coprocessor = MgCoprocessor::Cp1x;
        // _COP1X_MAP[(prototype.machine_code >> 26) as usize](prototype)
    }
    pub (super) fn pcrel_opcode_map(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let MgMipsVersion::M32(MgMips32::MgR6) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };
        static PCREL_MAP: [fn(disassembler: &MgDisassembler, &mut MgInstructionPrototype) -> Result<(), MgError>; 32] =[   
            MgDisassembler::addiupc,  MgDisassembler::addiupc,  MgDisassembler::lwpc,  MgDisassembler::lwpc,  MgDisassembler::lwupc,  MgDisassembler::lwupc,  MgDisassembler::ldpc,  MgDisassembler::no_instructions,
            MgDisassembler::addiupc,  MgDisassembler::addiupc,  MgDisassembler::lwpc,  MgDisassembler::lwpc,  MgDisassembler::lwupc,  MgDisassembler::lwupc,  MgDisassembler::ldpc,  MgDisassembler::no_instructions,
            MgDisassembler::addiupc,  MgDisassembler::addiupc,  MgDisassembler::lwpc,  MgDisassembler::lwpc,  MgDisassembler::lwupc,  MgDisassembler::lwupc,  MgDisassembler::ldpc,  MgDisassembler::auipc,
            MgDisassembler::addiupc,  MgDisassembler::addiupc,  MgDisassembler::lwpc,  MgDisassembler::lwpc,  MgDisassembler::lwupc,  MgDisassembler::lwupc,  MgDisassembler::ldpc,  MgDisassembler::aluipc
        ];

        prototype.is_relative = true;
        prototype.format = Some(MgInstructionFormat::Imm);
        let imm = FieldInfos::imm_field(1, 0b1111111111111111);
        let rs = Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu));
        if let Err(e) = PCREL_MAP[(prototype.machine_code >> 16 & 0b11111) as usize](self, prototype){
            return Err(e)
        }
        self.imm_format(prototype, rs, None, Some(imm))
    }

    //Opcode handlers

    //Default opcode field handlers
    pub(super) fn j(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = Some(MgMnemonic::MgMneJ);
        MgDisassembler::jump_format(self, prototype)
    }
    pub(super) fn jal(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = Some(MgMnemonic::MgMneJal);
        MgDisassembler::jump_format(self, prototype)
    }
    pub(super) fn beq(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    

        prototype.is_relative = true;
        prototype.category = Some(MgInstructionCategory::BranchJump);
        prototype.mnemonic = Some(MgMnemonic::MgMneBeq);
        prototype.is_conditional = true;
        
        return MgDisassembler::imm_format(self, prototype, Some(rs), Some(rt), Some(FieldInfos::default_imm_field(2)));
    }
    pub(super) fn bne(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    

        prototype.is_relative = true;
        prototype.category = Some(MgInstructionCategory::BranchJump);
        prototype.mnemonic = Some(MgMnemonic::MgMneBne);
        prototype.is_conditional = true;
        
        return MgDisassembler::imm_format(self, prototype, Some(rs), Some(rt), Some(FieldInfos::default_imm_field(2)));
    }
    pub(super) fn pop30(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: Option<FieldInfos>;
        let rt: Option<FieldInfos>;    
        let imm: Option<FieldInfos>;    

        prototype.is_relative = true;
        prototype.is_conditional = true;
        prototype.category = Some(MgInstructionCategory::BranchJump);

        (rt, rs, imm) = if let MgMipsVersion::M32(MgMips32::MgPreR6) = self.version{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        }else if let MgMipsVersion::M32(MgMips32::MgR6) = self.version{
            if prototype.machine_code >> 0x15 & 0b11111 < prototype.machine_code >> 0x10 & 0b11111 && 
            prototype.machine_code >> 0x10 & 0b11111 != 0 && prototype.machine_code >> 0x15 & 0b11111 != 0{
                prototype.mnemonic = Some(MgMnemonic::MgMneBnec);
                (Some(FieldInfos::default_reg_field(1, MgCoprocessor::Cpu)),     //rt
                Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(2)))   //rs
            }else if prototype.machine_code >> 0x15 & 0b11111 == 0x00 && prototype.machine_code >> 0x10 & 0b11111 != 0x00{
                prototype.mnemonic = Some(MgMnemonic::MgMneBnezalc);
                (Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), None, Some(FieldInfos::default_imm_field(1)))
            }else {
                prototype.mnemonic = Some(MgMnemonic::MgMneBnvc);      
                (Some(FieldInfos::default_reg_field(1, MgCoprocessor::Cpu)),     //rt
                Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(2)))   //rs
            }
        }else {
            unimplemented!();
        };
        
        return MgDisassembler::imm_format(self, prototype, rs, rt, imm);
    }
    pub(super) fn pop10(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: Option<FieldInfos>;
        let rt: Option<FieldInfos>;    
        let imm: Option<FieldInfos>;    

        prototype.is_relative = true;
        prototype.is_conditional = true;
        prototype.category = Some(MgInstructionCategory::BranchJump);

        (rt, rs, imm) = if let MgMipsVersion::M32(MgMips32::MgPreR6) = self.version{
            return self.addi_addiu(prototype);
        }else if let MgMipsVersion::M32(MgMips32::MgR6) = self.version{
            if prototype.machine_code >> 0x15 & 0b11111 < prototype.machine_code >> 0x10 & 0b11111 && 
            prototype.machine_code >> 0x10 & 0b11111 != 0 && prototype.machine_code >> 0x15 & 0b11111 != 0{
                prototype.mnemonic = Some(MgMnemonic::MgMneBeqc);
                (Some(FieldInfos::default_reg_field(1, MgCoprocessor::Cpu)),     //rt
                Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(2)))   //rs
            }else if prototype.machine_code >> 0x15 & 0b11111 == 0x00 && prototype.machine_code >> 0x10 & 0b11111 != 0x00{
                prototype.mnemonic = Some(MgMnemonic::MgMneBeqzalc);
                (Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), None, Some(FieldInfos::default_imm_field(1)))
            }else {
                prototype.mnemonic = Some(MgMnemonic::MgMneBovc);      
                (Some(FieldInfos::default_reg_field(1, MgCoprocessor::Cpu)),     //rt
                Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(2)))   //rs
            }
        }else {
            unimplemented!();
        };
        
        return MgDisassembler::imm_format(self, prototype, rs, rt, imm);
    }
    pub(super) fn blez_pop06(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: Option<FieldInfos>;
        let rt: Option<FieldInfos>;    
        let imm: Option<FieldInfos>;    

        prototype.is_relative = true;
        prototype.is_conditional = true;
        prototype.category = Some(MgInstructionCategory::BranchJump);

        (rt, rs, imm) = if let MgMipsVersion::M32(MgMips32::MgPreR6) = self.version{
            if prototype.machine_code >> 0x10 & 0b11111 != 0{
                return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            }
            prototype.mnemonic = Some(MgMnemonic::MgMneBlez);      
            (None, Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(1)))
        }else if let MgMipsVersion::M32(MgMips32::MgR6) = self.version{
            if prototype.machine_code >> 0x15 & 0b11111 == prototype.machine_code >> 0x10 & 0b11111 && prototype.machine_code >> 0x10 & 0b11111 != 0{
                prototype.mnemonic = Some(MgMnemonic::MgMneBgezalc);
                (Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)),     //rt
                None, Some(FieldInfos::default_imm_field(1)))   //rs
            }else if prototype.machine_code >> 0x15 & 0b11111 != prototype.machine_code >> 0x10 & 0b11111 && 
            prototype.machine_code >> 0x10 & 0b11111 != 0 && prototype.machine_code >> 0x15 & 0b11111 != 0{
                prototype.mnemonic = Some(MgMnemonic::MgMneBgeuc);
                (Some(FieldInfos::default_reg_field(1, MgCoprocessor::Cpu)),     //rt
                Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(2)))   //rs
            }else if prototype.machine_code >> 0x15 & 0b11111 == 0x00 && prototype.machine_code >> 0x10 & 0b11111 != 0x00{
                prototype.mnemonic = Some(MgMnemonic::MgMneBlezalc);
                (Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), None, Some(FieldInfos::default_imm_field(1)))
            }else {
                prototype.mnemonic = Some(MgMnemonic::MgMneBlez);      
                (None,     //rt
                Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(1)))   //rs
            }
        }else {
            unimplemented!();
        };
        
        return MgDisassembler::imm_format(self, prototype, rs, rt, imm);
    }
    pub(super) fn bgtz_pop07(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: Option<FieldInfos>;
        let rt: Option<FieldInfos>;    
        let imm: Option<FieldInfos>;    

        prototype.is_relative = true;
        prototype.is_conditional = true;
        prototype.category = Some(MgInstructionCategory::BranchJump);

        (rt, rs, imm) = if let MgMipsVersion::M32(MgMips32::MgPreR6) = self.version{
            if prototype.machine_code >> 0x10 & 0b11111 != 0{
                return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            }
            prototype.mnemonic = Some(MgMnemonic::MgMneBgtz);      
            (None, Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(1)))
        }else if let MgMipsVersion::M32(MgMips32::MgR6) = self.version{
            if prototype.machine_code >> 0x15 & 0b11111 == prototype.machine_code >> 0x10 & 0b11111 && prototype.machine_code >> 0x10 & 0b11111 != 0{
                prototype.mnemonic = Some(MgMnemonic::MgMneBgtzalc);
                (Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)),     //rt
                None, Some(FieldInfos::default_imm_field(1)))   //rs
            }else if prototype.machine_code >> 0x15 & 0b11111 != prototype.machine_code >> 0x10 & 0b11111 && 
            prototype.machine_code >> 0x10 & 0b11111 != 0 && prototype.machine_code >> 0x15 & 0b11111 != 0{
                prototype.mnemonic = Some(MgMnemonic::MgMneBltuc);
                (Some(FieldInfos::default_reg_field(1, MgCoprocessor::Cpu)),     //rt
                Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(2)))   //rs
            }else if prototype.machine_code >> 0x15 & 0b11111 == 0x00 && prototype.machine_code >> 0x10 & 0b11111 != 0x00{
                prototype.mnemonic = Some(MgMnemonic::MgMneBltzalc);
                (Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), None, Some(FieldInfos::default_imm_field(1)))
            }else {
                prototype.mnemonic = Some(MgMnemonic::MgMneBgtz);      
                (None,     //rt
                Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(1)))   //rs
            }
        }else {
            unimplemented!();
        };
        
        return MgDisassembler::imm_format(self, prototype, rs, rt, imm);
    }
    pub(super) fn addi_addiu(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        prototype.mnemonic = if prototype.machine_code >> 26 & 1 == 1{
            Some(MgMnemonic::MgMneAddiu)
        }else {
            Some(MgMnemonic::MgMneAddi)
        };
        prototype.category = Some(MgInstructionCategory::Arithmetic);
        return MgDisassembler::imm_format(self, prototype, Some(rs), Some(rt), Some(sa));
    }
    pub(super) fn slti_sltiu(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        prototype.category = Some(MgInstructionCategory::Arithmetic);
        prototype.mnemonic = if prototype.machine_code >> 26 & 1 == 1{
            Some(MgMnemonic::MgMneSltiu)
        }else{
            Some(MgMnemonic::MgMneSlti)
        };

        return MgDisassembler::imm_format(self, prototype, Some(rs), Some(rt), Some(sa));
    }
    pub(super) fn andi(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        prototype.mnemonic = Some(MgMnemonic::MgMneAndi);
        prototype.category = Some(MgInstructionCategory::Logical);

        return MgDisassembler::imm_format(self, prototype, Some(rs), Some(rt), Some(sa));
    }
    pub(super) fn ori(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        prototype.mnemonic = Some(MgMnemonic::MgMneOri);
        prototype.category = Some(MgInstructionCategory::Logical);
        
        return MgDisassembler::imm_format(self, prototype, Some(rs), Some(rt), Some(sa));
    }
    pub(super) fn xori(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        prototype.mnemonic = Some(MgMnemonic::MgMneXori);
        prototype.category = Some(MgInstructionCategory::Logical);
        
        return MgDisassembler::imm_format(self, prototype, Some(rs), Some(rt), Some(sa));
    }
    pub(super) fn lui(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(1);

        prototype.mnemonic = Some(MgMnemonic::MgMneLui);
        prototype.category = Some(MgInstructionCategory::Logical);

        return MgDisassembler::imm_format(self, prototype, Some(FieldInfos::default_fixed_field()), Some(rt), Some(sa));
    }
    pub(super) fn beql(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let MgMipsVersion::M32(MgMips32::MgR6) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };
        let rs: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let imm: FieldInfos = FieldInfos::default_imm_field(2);

        prototype.is_relative = true;
        prototype.category = Some(MgInstructionCategory::BranchJump);
        prototype.mnemonic = Some(MgMnemonic::MgMneBeql);
        prototype.is_conditional = true;
        
        return MgDisassembler::imm_format(self, prototype, Some(rs), Some(rt), Some(imm));
    }
    pub(super) fn bnel(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let MgMipsVersion::M32(MgMips32::MgR6) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };
        let rs: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let imm: FieldInfos = FieldInfos::default_imm_field(2);

        prototype.is_relative = true;
        prototype.category = Some(MgInstructionCategory::BranchJump);
        prototype.mnemonic = Some(MgMnemonic::MgMneBnel);
        prototype.is_conditional = true;
        
        return MgDisassembler::imm_format(self, prototype, Some(rs), Some(rt), Some(imm));
    }
    pub(super) fn blezl_pop26(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: Option<FieldInfos>;
        let rt: Option<FieldInfos>;    
        let imm: Option<FieldInfos>;    

        prototype.is_relative = true;
        prototype.is_conditional = true;
        prototype.category = Some(MgInstructionCategory::BranchJump);

        (rt, rs, imm) = if let MgMipsVersion::M32(MgMips32::MgPreR6) = self.version{
            if prototype.machine_code >> 0x10 & 0b11111 != 0{
                return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            }
            prototype.mnemonic = Some(MgMnemonic::MgMneBlezl);      
            (None, Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(1)))
        }else if let MgMipsVersion::M32(MgMips32::MgR6) = self.version{
            if prototype.machine_code >> 0x10 & 0b11111 == 0{
                return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            }
            if prototype.machine_code >> 0x15 & 0b11111 == prototype.machine_code >> 0x10 & 0b11111 && prototype.machine_code >> 0x10 & 0b11111 != 0{
                prototype.mnemonic = Some(MgMnemonic::MgMneBgezc);
                (Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)),     //rt
                None, Some(FieldInfos::default_imm_field(1)))   //rs
            }else if prototype.machine_code >> 0x15 & 0b11111 != prototype.machine_code >> 0x10 & 0b11111 && 
            prototype.machine_code >> 0x10 & 0b11111 != 0 && prototype.machine_code >> 0x15 & 0b11111 != 0{
                prototype.mnemonic = Some(MgMnemonic::MgMneBgec);
                (Some(FieldInfos::default_reg_field(1, MgCoprocessor::Cpu)),     //rt
                Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(2)))   //rs
            }else{
                prototype.mnemonic = Some(MgMnemonic::MgMneBlezc);
                (Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), None, Some(FieldInfos::default_imm_field(1)))
            }
        }else {
            unimplemented!();
        };
        
        return MgDisassembler::imm_format(self, prototype, rs, rt, imm);
    }
    pub(super) fn bgtzl_pop27(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: Option<FieldInfos>;
        let rt: Option<FieldInfos>;    
        let imm: Option<FieldInfos>;    

        prototype.is_relative = true;
        prototype.is_conditional = true;
        prototype.category = Some(MgInstructionCategory::BranchJump);

        (rt, rs, imm) = if let MgMipsVersion::M32(MgMips32::MgPreR6) = self.version{
            if prototype.machine_code >> 0x10 & 0b11111 != 0{
                return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            }
            prototype.mnemonic = Some(MgMnemonic::MgMneBgtzl);
            (None, Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(1)))
        }else if let MgMipsVersion::M32(MgMips32::MgR6) = self.version{
            if prototype.machine_code >> 0x10 & 0b11111 == 0{
                return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            }
            if prototype.machine_code >> 0x15 & 0b11111 == prototype.machine_code >> 0x10 & 0b11111 && prototype.machine_code >> 0x10 & 0b11111 != 0{
                prototype.mnemonic = Some(MgMnemonic::MgMneBltzc);
                (Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)),     //rt
                None, Some(FieldInfos::default_imm_field(1)))   //rs
            }else if prototype.machine_code >> 0x15 & 0b11111 != prototype.machine_code >> 0x10 & 0b11111 && 
            prototype.machine_code >> 0x10 & 0b11111 != 0 && prototype.machine_code >> 0x15 & 0b11111 != 0{
                prototype.mnemonic = Some(MgMnemonic::MgMneBltc);
                (Some(FieldInfos::default_reg_field(1, MgCoprocessor::Cpu)),     //rt
                Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(2)))   //rs
            }else{
                prototype.mnemonic = Some(MgMnemonic::MgMneBgtzc);
                (Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), None, Some(FieldInfos::default_imm_field(1)))
            }
        }else {
            unimplemented!();
        };
        
        return MgDisassembler::imm_format(self, prototype, rs, rt, imm);
    }
    pub(super) fn jalx(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let MgMipsVersion::M32(MgMips32::MgR6) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };
        prototype.mnemonic = Some(MgMnemonic::MgMneJalx);
        MgDisassembler::jump_format(self, prototype)
    }
    pub (super) fn lwr_swr_lwl_swl(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let mnemonics_id: [[Option<MgMnemonic>; 2]; 2]=[[Some(MgMnemonic::MgMneLwl), Some(MgMnemonic::MgMneSwl)], [Some(MgMnemonic::MgMneLwr), Some(MgMnemonic::MgMneSwr)]];
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let rt = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);

        match self.version{
            MgMipsVersion::M32(MgMips32::MgPreR6) => (),
            MgMipsVersion::M32(MgMips32::MgR6) => return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code)),
            _ => unimplemented!(),
        }
                
        (prototype.category,prototype.mnemonic) = if prototype.opcode >> 3 & 1 == 0{
            (Some(MgInstructionCategory::Load), mnemonics_id[(prototype.opcode >> 2 & 1) as usize][0])
        }else{
            (Some(MgInstructionCategory::Store), mnemonics_id[(prototype.opcode >> 2 & 1) as usize][1])
        };

        return MgDisassembler::imm_format(self, prototype, Some(base), Some(rt), Some(FieldInfos::default_imm_field(1)))
    }
    pub(super) fn pop76(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let imm: FieldInfos;
        let rs: FieldInfos;
        let rt: Option<FieldInfos>;
        if let MgMipsVersion::M32(MgMips32::MgPreR6) = self.version{
            return self.load_store_cp2(prototype)
        }
        prototype.category = Some(MgInstructionCategory::BranchJump);
        (rs, rt, imm) = if prototype.machine_code >> 21 & 0b11111 != 0{
            prototype.is_relative = true;
            prototype.is_conditional = true;
            prototype.mnemonic = Some(MgMnemonic::MgMneBnezc);
            (FieldInfos::default_reg_field(0, MgCoprocessor::Cpu), None, FieldInfos::imm_field(1, 0b111111111111111111111))
        }else{
            prototype.mnemonic = Some(MgMnemonic::MgMneJialc);
            (FieldInfos::default_fixed_field(), Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), FieldInfos::default_imm_field(1))
        };
 
        return MgDisassembler::imm_format(self, prototype, Some(rs), rt, Some(imm))
    }
    pub(super) fn pop66(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let imm: FieldInfos;
        let rs: FieldInfos;
        let rt: Option<FieldInfos>;
        if let MgMipsVersion::M32(MgMips32::MgPreR6) = self.version{
            return self.load_store_cp2(prototype)
        }
        prototype.category = Some(MgInstructionCategory::BranchJump);
        (rs, rt, imm) = if prototype.machine_code >> 21 & 0b11111 != 0{
            prototype.is_relative = true;
            prototype.is_conditional = true;
            prototype.mnemonic = Some(MgMnemonic::MgMneBeqzc);
            (FieldInfos::default_reg_field(0, MgCoprocessor::Cpu), None, FieldInfos::imm_field(1, 0b111111111111111111111))
        }else{
            prototype.mnemonic = Some(MgMnemonic::MgMneJic);
            (FieldInfos::default_fixed_field(), Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), FieldInfos::default_imm_field(1))
        };
 
        return MgDisassembler::imm_format(self, prototype, Some(rs), rt, Some(imm))
    }
    pub(super) fn jic_jialc(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        if let MgMipsVersion::M32(MgMips32::MgPreR6) = self.version{
            return self.load_store_cp2(prototype)
        };

        prototype.mnemonic = if prototype.opcode >> 3 & 1 == 1{
            Some(MgMnemonic::MgMneJialc)
        }else{
            Some(MgMnemonic::MgMneJic)
        };
        prototype.category = Some(MgInstructionCategory::BranchJump);
        return MgDisassembler::imm_format(self, prototype, Some(FieldInfos::default_fixed_field()), Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(1)))
    }
    pub(super) fn bc_balc(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        if let MgMipsVersion::M32(MgMips32::MgPreR6) = self.version{
            return self.load_store_cp2(prototype)
        };

        prototype.mnemonic = if prototype.opcode >> 3 & 1 == 1{
             Some(MgMnemonic::MgMneBalc)
        }else{
            Some(MgMnemonic::MgMneBc)
        };

        //Some attributes about the instruction
        prototype.format = Some(MgInstructionFormat::Jump);
        prototype.operand_num = 1 ;
        prototype.is_conditional = true;
        prototype.category = Some(MgInstructionCategory::BranchJump);
        prototype.operand[0] = Some(MgOpImmediate::new_imm_opreand((prototype.machine_code & 0x3FFFFFF) as u64));

        Ok(())
    }
    pub(super) fn load_store_cp2(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let mnemonic: [[Option<MgMnemonic>; 2]; 2] = [[Some(MgMnemonic::MgMneLwc2), Some(MgMnemonic::MgMneSwc2)], [Some(MgMnemonic::MgMneLdc2), Some(MgMnemonic::MgMneSdc2)]];
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let rt = FieldInfos::default_reg_field(0, MgCoprocessor::Cp2);
        let imm: Option<FieldInfos>;

         match self.version{
            MgMipsVersion::M32(MgMips32::MgR6)=> {
                (prototype.mnemonic, prototype.category) = (mnemonic[(prototype.machine_code >> 23 & 1) as usize][(prototype.machine_code >> 21 & 1) as usize], Some(if prototype.machine_code >> 21 & 1 == 0{
                    MgInstructionCategory::Load
                }else{
                    MgInstructionCategory::Store
                }));
                imm = Some(FieldInfos::imm_field(1, 0b11111111111));
            },
            MgMipsVersion::M32(MgMips32::MgPreR6) => {
                if prototype.opcode == 0b010010{
                    return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
                }
                prototype.mnemonic = mnemonic[(prototype.opcode >> 2 & 1) as usize][(prototype.opcode >> 3 & 1) as usize];
                prototype.category = if prototype.opcode >> 3 & 1 == 0{
                    Some(MgInstructionCategory::Load)
                }else{
                    Some(MgInstructionCategory::Store)
                };
                imm = Some(FieldInfos::default_imm_field(1));
            }
            _ => unimplemented!(),
        };

        return MgDisassembler::imm_format(self, prototype, Some(base), Some(rt), imm)
    }
    pub (super) fn sc_ll(&self, prototype : &mut MgInstructionPrototype) -> Result<(), MgError>{
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let rt = FieldInfos::default_reg_field(0, MgCoprocessor::Cp2);
        let imm: Option<FieldInfos>;

        prototype.is_conditional = true;
        (prototype.mnemonic, prototype.category) = match self.version{
            MgMipsVersion::M32(MgMips32::MgPreR6) => {
                if 0b011111 == prototype.opcode{
                    return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
                }else {
                    imm = Some(FieldInfos::default_imm_field(1));
                    if prototype.opcode >> 3 & 1 == 1{
                        (Some(MgMnemonic::MgMneSc), Some(MgInstructionCategory::Store))
                    }else {
                        (Some(MgMnemonic::MgMneLl), Some(MgInstructionCategory::Load))
                    }
                }
            },
            MgMipsVersion::M32(MgMips32::MgR6) =>{
                if 0b011111 != prototype.opcode{
                    return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
                }else{
                    if prototype.machine_code >> 6 == 1{
                        return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
                    }
                    imm = None;
                    prototype.operand[1] = Some(MgOpImmediate::new_imm_opreand((prototype.machine_code >> 7 & 0b111111111) as u64));
                    prototype.operand_num += 1;
                    if prototype.machine_code >> 4 & 1 != 1{
                        (Some(MgMnemonic::MgMneSc), Some(MgInstructionCategory::Store))
                    }else {
                        (Some(MgMnemonic::MgMneLl), Some(MgInstructionCategory::Load))
                    }
                }
            },
            _ => unimplemented!(),
        };

        return MgDisassembler::imm_format(self, prototype, Some(base), Some(rt), imm)
    }
    pub(super) fn cpu_loadstore(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let rt: FieldInfos;
        let mnemonics: [[Option<MgMnemonic>; 7]; 4] = [
            [Some(MgMnemonic::MgMneLb), Some(MgMnemonic::MgMneLh), None, Some(MgMnemonic::MgMneLw), Some(MgMnemonic::MgMneLbu), Some(MgMnemonic::MgMneLhu), None],
            [Some(MgMnemonic::MgMneSb), Some(MgMnemonic::MgMneSh), None, Some(MgMnemonic::MgMneSw), None, None, None],
            [None, Some(MgMnemonic::MgMneLwc1), None, None, None, Some(MgMnemonic::MgMneLdc1), None],
            [None, Some(MgMnemonic::MgMneSwc1), None, None, None, Some(MgMnemonic::MgMneSdc1), None]
        ];

        prototype.mnemonic = mnemonics[(prototype.machine_code >> 29 & 3) as usize][(prototype.machine_code >> 26 & 7) as usize];

        if (prototype.machine_code >> 29 & 3) == 6 
        || (prototype.machine_code >> 29 & 3) == 7
        && (prototype.machine_code >> 27 & 3) == 0{
            rt = FieldInfos::default_reg_field(0, MgCoprocessor::Cp1);
        }
        else {
            rt = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);
        }

        prototype.category = Some(match prototype.machine_code & 1{
            0 => MgInstructionCategory::Load,
            1 => MgInstructionCategory::Store,
            _ => return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
        });

        return MgDisassembler::imm_format(self, prototype, Some(base), Some(rt), Some(FieldInfos::default_imm_field(1)))
    }
    pub(super) fn cache_pref(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let op: FieldInfos = FieldInfos::imm_field(0, 0b11111);

        return if let MgMipsVersion::M32(MgMips32::MgPreR6) = self.version{
            if prototype.opcode == 0b011111{
                Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            } else{
                (prototype.mnemonic, prototype.category) = if prototype.opcode == 0b110011{
                    (Some(MgMnemonic::MgMnePref), Some(MgInstructionCategory::MemoryControl))
                }else if prototype.opcode == 0b101111{
                    (Some(MgMnemonic::MgMneCache), Some(MgInstructionCategory::Priviledge))
                }else{
                    return Err(MgError::throw_error(MgErrorCode::DevError, prototype.opcode, prototype.address, prototype.machine_code))
                };
                MgDisassembler::imm_format(self, prototype, Some(base), Some(op), Some(FieldInfos::default_imm_field(1)))
            }
        } else if let MgMipsVersion::M32(MgMips32::MgR6) = self.version{
            if prototype.opcode != 0b011111{
                Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            } else  if prototype.machine_code >> 6 & 1 == 1{
                Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
            } else{
                (prototype.mnemonic, prototype.category) = if prototype.machine_code >> 4 & 1 == 1{
                    (Some(MgMnemonic::MgMnePref), Some(MgInstructionCategory::MemoryControl))
                } else if prototype.machine_code >> 4 & 1 == 0{
                    (Some(MgMnemonic::MgMneCache), Some(MgInstructionCategory::Priviledge))
                }else{
                    return Err(MgError::throw_error(MgErrorCode::DevError, prototype.opcode, prototype.address, prototype.machine_code))
                };
                prototype.operand_num = 1;
                prototype.operand[1] = Some(MgOpImmediate::new_imm_opreand((prototype.machine_code >> 7 & 0b111111111) as u64));
                MgDisassembler::imm_format(self, prototype, Some(base), Some(op), None)
            }
        }
        else {
            unimplemented!();
        }
    }

    //Special
    pub(super) fn sll(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_fixed_field();
        let rt: FieldInfos;
        let rd: FieldInfos;
        let sa: FieldInfos;

        if prototype.machine_code >> 11 & 0b111111111111111 == 0{
            prototype.mnemonic = match prototype.machine_code >> 6 & 0b11111{
                0 => Some(MgMnemonic::MgMneNop),
                1 => Some(MgMnemonic::MgMneSsnop),
                3 => Some(MgMnemonic::MgMneEhb),
                5 => Some(MgMnemonic::MgMnePause),
                _ => return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
            };
            rt = FieldInfos::default_fixed_field();
            rd = FieldInfos::default_fixed_field();
            sa = FieldInfos::default_fixed_field();

            prototype.category = Some(MgInstructionCategory::Control);
        }
        else{
            prototype.mnemonic = Some(MgMnemonic::MgMneSll);
            prototype.category = Some(MgInstructionCategory::Shift);

            rt = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
            rd = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
            sa = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Imm);
        }
        
        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), Some(sa))
    }
    pub(super) fn movci(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        //Reserved Instruction, Coprocessor Unusable
        if (prototype.machine_code >> 6 & 0b11111) != 0
        ||(prototype.machine_code >> 17 & 1) != 0{
            return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
        }
        let mnemonics = [Some(MgMnemonic::MgMneMovf), Some(MgMnemonic::MgMneMovt)];
        let registers: [&str; 8] = [ MG_REG_FCC0, MG_REG_FCC1, MG_REG_FCC2, MG_REG_FCC3, MG_REG_FCC4, MG_REG_FCC5, MG_REG_FCC6, MG_REG_FCC7,];
        
        prototype.format = Some(MgInstructionFormat::CoditionCodeFpu);
        prototype.category = Some(MgInstructionCategory::Move);
        prototype.mnemonic = mnemonics[(prototype.machine_code >> 16 & 1) as usize];

        prototype.operand_num = 3;
        prototype.operand[0] = Some(MgOpRegister::new_reg_opreand((prototype.machine_code >> 11 & 0b11111) as u8, MgCoprocessor::Cpu));
        prototype.operand[1] = Some(MgOpRegister::new_reg_opreand((prototype.machine_code >> 21 & 0b11111) as u8, MgCoprocessor::Cpu));
        prototype.operand[2] = Some(MgOpRegister::new_reg_operand_str(registers[(prototype.machine_code >> 18 & 0b111) as usize], MgCoprocessor::Cp1));

        Ok(())
    }
    pub(super) fn srl_sra(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::fixed_field(4, 0b1111);
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let sa: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Imm);

        prototype.mnemonic = match prototype.machine_code & 1{
            1 => Some(MgMnemonic::MgMneSra),
            0 => {
                match prototype.machine_code >> 6 & 1 {
                    1 => Some(MgMnemonic::MgMneRotr),
                    0 => Some(MgMnemonic::MgMneSrl),
                    _ => None
                }
            },
            _ => None
        };

        prototype.category = Some(MgInstructionCategory::Shift);
        return MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), Some(sa))
    }
    pub(super) fn sllv(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = Some(MgMnemonic::MgMneSllv);
        prototype.category = Some(MgInstructionCategory::Shift);

        let sa: FieldInfos = FieldInfos::default_fixed_field();
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);

        return MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), Some(sa))
    }
    pub(super) fn srlv_srav(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let sa: FieldInfos = FieldInfos::fixed_field(4, 0b1111);
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Imm);

        prototype.mnemonic = match prototype.machine_code & 1{
            1 => Some(MgMnemonic::MgMneSrav),
            0 => {
                match prototype.machine_code >> 6 & 1 {
                    1 => Some(MgMnemonic::MgMneRotrv),
                    0 => Some(MgMnemonic::MgMneSrlv),
                    _ => None
                }
            },
            _ => None
        };

        prototype.category = Some(MgInstructionCategory::Shift);
        return MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), Some(sa))
    }
    pub(super) fn jr(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::fixed_field(4, 0b1111111111);
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);

        prototype.category = Some(MgInstructionCategory::BranchJump);

        if (prototype.machine_code >> 6 & 0b10000) != 0{
            prototype.mnemonic = Some(MgMnemonic::MgMneJrhb);
        }
        else{
            prototype.mnemonic = Some(MgMnemonic::MgMneJr);
        }

        MgDisassembler::reg_format(self, prototype, Some(rs), None, Some(rd), None)
    }
    pub(super) fn jalr(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::default_fixed_field();
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        
        prototype.category = Some(MgInstructionCategory::BranchJump);

        if (prototype.machine_code >> 6 & 0b10000) != 0{
            prototype.mnemonic = Some(MgMnemonic::MgMneJalrhb)
        }
        else{
            prototype.mnemonic = Some(MgMnemonic::MgMneJalr)
        }

        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), None)
    }
    pub(super) fn movn_movz(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);

        prototype.category = Some(MgInstructionCategory::Move);
        prototype.is_conditional = true;

        if prototype.machine_code & 0b111111 == 0b001010{
            prototype.mnemonic = Some(MgMnemonic::MgMneMovz);
        }
        else{
            prototype.mnemonic = Some(MgMnemonic::MgMneMovn);
        }
        return MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn syscall_break(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = match prototype.machine_code & 1{
            1 => Some(MgMnemonic::MgMneBreak),
            0 => Some(MgMnemonic::MgMneSyscall),
            _ => None
        };
        prototype.category = Some(MgInstructionCategory::Trap);
        prototype.format = Some(MgInstructionFormat::Other);
        prototype.operand[0] = Some(MgOpImmediate::new_imm_opreand(((prototype.machine_code >> 6) & 0xFFFFF) as u64));

        Ok(())
    }
    pub(super) fn sync(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::fixed_field(4, 0b111111111111111);
        let sa: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Imm);

        //Setting the attributes
        prototype.mnemonic = Some(MgMnemonic::MgMneSync);
        prototype.category = Some(MgInstructionCategory::MemoryControl);
        MgDisassembler::reg_format(self, prototype, None, None, Some(rd), Some(sa))
    }
    pub(super) fn mfhi_mflo(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        if let MgMipsVersion::M32(MgMips32::MgR6) = self.version{
            return if prototype.machine_code & 2 != 0{
                Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            } else{
                self.clz_clo(prototype)
            }
        }
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let mnemonics = [Some(MgMnemonic::MgMneMfhi), Some(MgMnemonic::MgMneMflo)];

        prototype.mnemonic = mnemonics[(prototype.machine_code >> 1 & 1) as usize];
        prototype.category = Some(MgInstructionCategory::Move);

        MgDisassembler::reg_format(self, prototype, None, Some(FieldInfos::fixed_field(4, 0b1111111111)), Some(rd), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn mthi_mtlo(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        if let MgMipsVersion::M32(MgMips32::MgR6) = self.version{
            return if prototype.machine_code & 2 != 0{
                Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            } else{
                self.clz_clo(prototype)
            }
        }
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let mnemonics = [Some(MgMnemonic::MgMneMthi), Some(MgMnemonic::MgMneMtlo)];
        
        prototype.mnemonic = mnemonics[(prototype.machine_code >> 1 & 1) as usize];
        prototype.category = Some(MgInstructionCategory::Move);

        MgDisassembler::reg_format(self, prototype, Some(rs), None, None, Some(FieldInfos::fixed_field(4, 0b111111111111111)))
    }
    pub(super) fn mult_multu_div_divu(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let mnemonics = [[Some(MgMnemonic::MgMneMult), Some(MgMnemonic::MgMneMultu)], [Some(MgMnemonic::MgMneDiv), Some(MgMnemonic::MgMneDivu)]];

        prototype.category = Some(MgInstructionCategory::Arithmetic);
        prototype.mnemonic = mnemonics[(prototype.machine_code >> 1 & 1) as usize][(prototype.machine_code & 1) as usize];

        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), None, Some(FieldInfos::fixed_field(4, 0b1111111111)))
    }
    pub(super) fn add_addu_sub_subu_and_or_xor_nor(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let mnemonics = [[[Some(MgMnemonic::MgMneAdd), Some(MgMnemonic::MgMneAddu)], [Some(MgMnemonic::MgMneSub), Some(MgMnemonic::MgMneSubu)]], [[Some(MgMnemonic::MgMneAnd), Some(MgMnemonic::MgMneOr)], [Some(MgMnemonic::MgMneXor), Some(MgMnemonic::MgMneNor)]]];

        prototype.mnemonic = mnemonics[(prototype.machine_code >> 2 & 1) as usize][(prototype.machine_code >> 1 & 1) as usize][(prototype.machine_code & 1) as usize];
        if (prototype.machine_code >> 2 & 1) == 1{
            prototype.category = Some(MgInstructionCategory::Logical);
        }
        else{
            prototype.category = Some(MgInstructionCategory::Arithmetic);
            if (prototype.machine_code & 1) == 0{
            }
        }

        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn slt_sltu(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let mnemonics = [Some(MgMnemonic::MgMneSlt), Some(MgMnemonic::MgMneSltu)];

        prototype.category = Some(MgInstructionCategory::Arithmetic);
        prototype.is_conditional = true;
        prototype.mnemonic = mnemonics[(prototype.machine_code & 1) as usize];

        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn tge_tgeu_tlt_tltu(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let mnemonics = [[Some(MgMnemonic::MgMneTge), Some(MgMnemonic::MgMneTgeu)], [Some(MgMnemonic::MgMneTlt), Some(MgMnemonic::MgMneTltu)]];
        
        prototype.mnemonic = mnemonics[(prototype.machine_code >> 1 & 1) as usize][(prototype.machine_code & 1) as usize];
        prototype.category = Some(MgInstructionCategory::Trap);

        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), None, Some(FieldInfos::imm_field(2, 0b1111111111)))
    }
    pub(super) fn seleqz_selnez(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);

        match self.version{
            MgMipsVersion::M32(MgMips32::MgPreR6) => return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code)),
            MgMipsVersion::M32(MgMips32::MgR6) => (),
            _ => unimplemented!(),
        }

        prototype.is_conditional = true;

        prototype.category = Some(MgInstructionCategory::Move);
        prototype.mnemonic = if prototype.machine_code >> 1 & 1 == 1{
            Some(MgMnemonic::MgMneSelnez)
        }else{
            Some(MgMnemonic::MgMneSeleqz)
        };
        
        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd) , Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn teq_tne(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        
        prototype.category = Some(MgInstructionCategory::Trap);
        prototype.mnemonic = if prototype.machine_code >> 1 & 1 == 0{
            Some(MgMnemonic::MgMneTeq)
        }else{
            Some(MgMnemonic::MgMneTne)
        };

        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), None, Some(FieldInfos::imm_field(2, 0b1111111111)))
    }

    //Special2
    pub(super) fn madd_maddu(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);

        prototype.category = Some(MgInstructionCategory::Arithmetic);
        prototype.mnemonic = match prototype.machine_code & 1{
            0 => Some(MgMnemonic::MgMneMadd),
            1 => Some(MgMnemonic::MgMneMaddu),
            _ => None
        };

        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), None, Some(FieldInfos::fixed_field(4, 0b1111111111)))
    }
    pub(super) fn mul(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);

        prototype.category = Some(MgInstructionCategory::Arithmetic);
        prototype.mnemonic = Some(MgMnemonic::MgMneMul);

        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn msub_msubu(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);

        prototype.category = Some(MgInstructionCategory::Arithmetic);
        prototype.mnemonic = match prototype.machine_code & 1{
            0 => Some(MgMnemonic::MgMneMsub),
            1 => Some(MgMnemonic::MgMneMsubu),
            _ => None
        };

        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), None, Some(FieldInfos::fixed_field(4, 0b1111111111)))
    }
    pub(super) fn clz_clo(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);

        prototype.category = Some(MgInstructionCategory::Arithmetic);
        prototype.mnemonic = match prototype.machine_code & 1{
            0 => Some(MgMnemonic::MgMneClz),
            1 => Some(MgMnemonic::MgMneClo),
            _ => None
        };

        return if let MgMipsVersion::M32(MgMips32::MgR6) = self.version{
            if prototype.machine_code >> 6 & 1 == 0{
                Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
            }else{
                MgDisassembler::reg_format(self, prototype, Some(rs), Some(FieldInfos::default_fixed_field()), Some(rd), None)
            }
        }else{
            MgDisassembler::reg_format(self, prototype, Some(rs), None, Some(rd), Some(FieldInfos::default_fixed_field()))
        }
    }
    pub(super) fn sdbbp(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = Some(MgMnemonic::MgMneSdbbp);
        prototype.category = Some(MgInstructionCategory::Trap);
        prototype.format = Some(MgInstructionFormat::Other);
        prototype.operand[0] = Some(MgOpImmediate::new_imm_opreand(((prototype.machine_code >> 6) & 0xFFFFF) as u64));

        Ok(())
    }

    //Special3 They need some testing
    pub(super) fn ext(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = Some(MgMnemonic::MgMneExt);
        prototype.category = Some(MgInstructionCategory::InsertExtract);

        prototype.operand_num = 4;
        prototype.operand[0] = Some(MgOpRegister::new_reg_opreand((prototype.machine_code >> 16 & 0b11111) as u8, MgCoprocessor::Cpu));
        prototype.operand[1] = Some(MgOpRegister::new_reg_opreand((prototype.machine_code >> 21 & 0b11111) as u8, MgCoprocessor::Cpu));
        prototype.operand[2] = Some(MgOpImmediate::new_imm_opreand((prototype.machine_code >> 6 & 0b11111) as u64));
        prototype.operand[3] = Some(MgOpImmediate::new_imm_opreand((prototype.machine_code >> 11 & 0b11111) as u64));
        Ok(())
    }
    pub(super) fn ins(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = Some(MgMnemonic::MgMneIns);
        prototype.category = Some(MgInstructionCategory::InsertExtract);

        prototype.operand_num = 4;
        prototype.operand[0] = Some(MgOpRegister::new_reg_opreand((prototype.machine_code >> 16 & 0b11111) as u8, MgCoprocessor::Cpu));
        prototype.operand[1] = Some(MgOpRegister::new_reg_opreand((prototype.machine_code >> 21 & 0b11111) as u8, MgCoprocessor::Cpu));
        prototype.operand[2] = Some(MgOpImmediate::new_imm_opreand((prototype.machine_code >> 6 & 0b11111) as u64));
        prototype.operand[3] = Some(MgOpImmediate::new_imm_opreand((prototype.machine_code >> 11 & 0b11111) as u64));
        Ok(())
    }
    pub(super) fn bshfl(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);

        (prototype.category, prototype.mnemonic) = match prototype.machine_code >> 6 & 0b11111{
            0b00010 => (Some(MgInstructionCategory::InsertExtract), Some(MgMnemonic::MgMneWsbh)),
            0b10000 => (Some(MgInstructionCategory::Arithmetic),Some(MgMnemonic::MgMneSeb)),
            0b11000 => (Some(MgInstructionCategory::Arithmetic),Some(MgMnemonic::MgMneSeh)),
            _ => return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
        };
        
        MgDisassembler::reg_format(self, prototype, Some(FieldInfos::default_fixed_field()), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn rdhwr(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        
        prototype.category = Some(MgInstructionCategory::Move);
        prototype.mnemonic = Some(MgMnemonic::MgMneRdhwr);

        MgDisassembler::reg_format(self, prototype, Some(FieldInfos::default_fixed_field()), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }

    //CP0
    pub(super) fn mov_cp0(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let mnemonics = [Some(MgMnemonic::MgMneMfc0), Some(MgMnemonic::MgMneMtc0)];
        if (prototype.machine_code >> 3 & 0b11111111) != 0{
            return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
        }

        prototype.category = Some(MgInstructionCategory::Priviledge);
        prototype.format = Some(MgInstructionFormat::Other);
        prototype.mnemonic = mnemonics[(prototype.machine_code >> 23 & 1) as usize];
        prototype.operand_num = 3;

        prototype.operand[0] = Some(MgOpRegister::new_reg_opreand((prototype.machine_code >> 16 & 0b11111) as u8, MgCoprocessor::Cpu));
        prototype.operand[1] = Some(MgOpRegister::new_reg_opreand((prototype.machine_code >> 11 & 0b11111) as u8, MgCoprocessor::Cpu));
        prototype.operand[2] = Some(MgOpImmediate::new_imm_opreand((prototype.machine_code & 7) as u64));
        Ok(())
    }
    pub(super) fn gpr_shadowset(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let mnemonics = [Some(MgMnemonic::MgMneRdpgpr), Some(MgMnemonic::MgMneWrpgpr)];

        prototype.category = Some(MgInstructionCategory::Priviledge);
        prototype.mnemonic = mnemonics[(prototype.machine_code >> 23 & 1) as usize];
        MgDisassembler::cpx_cpu_transfer_format(self, prototype, FieldInfos::default_reg_field(1, MgCoprocessor::Cpu), FieldInfos::default_reg_field(0, MgCoprocessor::Cpu))
    }
    pub(super) fn mfmc0(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let mnemonics = [Some(MgMnemonic::MgMneDi), Some(MgMnemonic::MgMneEi)];

        if prototype.machine_code & 0b11111 != 0 ||
        (prototype.machine_code >> 6 & 0b11111) != 0 || 
        (prototype.machine_code >> 11 & 0b01100) != 0b01100 {
            return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
        }
        
        prototype.category = Some(MgInstructionCategory::Priviledge);
        prototype.format = Some(MgInstructionFormat::Mfmc0);
        prototype.mnemonic = mnemonics[(prototype.machine_code >> 5 & 1) as usize];
        prototype.operand_num = 1;
        prototype.operand[0] = Some(MgOpRegister::new_reg_opreand((prototype.machine_code >> 16 & 0b11111) as u8, MgCoprocessor::Cpu));

        Ok(())
    }
    pub(super) fn c0(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let mnemonics: [[Option<MgMnemonic>; 8]; 8] = [
            [None,  Some(MgMnemonic::MgMneTlbr),  Some(MgMnemonic::MgMneTlbwi),  None,  None,  None,  Some(MgMnemonic::MgMneTlbwr),  None],
            [Some(MgMnemonic::MgMneTlbp),  None,  None,  None,  None,  None,  None,  None],
            [None,  None,  None,  None,  None,  None,  None,  None],
            [Some(MgMnemonic::MgMneEret),  None,  None,  None,  None,  None,  None,  Some(MgMnemonic::MgMneDeret)], 
            [Some(MgMnemonic::MgMneWait),  None,  None,  None,  None,  None,  None,  None],
            [None,  None,  None,  None,  None,  None,  None,  None],
            [None,  None,  None,  None,  None,  None,  None,  None],
            [None,  None,  None,  None,  None,  None,  None,  None]
        ];
        if (prototype.machine_code >> 6 & 0b1111111111111111111) != 0 ||
        (prototype.machine_code >> 25 & 1) != 1{
            return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
        }

        prototype.category = Some(MgInstructionCategory::Priviledge);
        prototype.format = Some(MgInstructionFormat::Other);
        prototype.mnemonic = mnemonics[(prototype.machine_code >> 3 & 0b111) as usize][(prototype.machine_code & 0b111) as usize];
        Ok(())
    }

    //pcrel
    pub (super) fn addiupc(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = Some(MgMnemonic::MgMneAddiupc);
        prototype.category = Some(MgInstructionCategory::Arithmetic);
        Ok(())
    }
    pub (super) fn lwpc(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = Some(MgMnemonic::MgMneLwpc);
        prototype.category = Some(MgInstructionCategory::Load);
        Ok(())
    }
    pub (super) fn lwupc(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = Some(MgMnemonic::MgMneLwupc);
        prototype.category = Some(MgInstructionCategory::Load);
        Ok(())
    }
    pub (super) fn aluipc(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = Some(MgMnemonic::MgMneAluipc);
        prototype.category = Some(MgInstructionCategory::Logical);
        Ok(())
    }
    pub (super) fn ldpc(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{      //The manual didn't have an entry 
        prototype.mnemonic = Some(MgMnemonic::MgMneLdpc);                                                   //for that instruction but it was mentionned in the Table A.13
        prototype.category = Some(MgInstructionCategory::Load);
        Ok(())
    }
    pub (super) fn auipc(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = Some(MgMnemonic::MgMneAuipc);
        prototype.category = Some(MgInstructionCategory::Logical);
        Ok(())
    }
}