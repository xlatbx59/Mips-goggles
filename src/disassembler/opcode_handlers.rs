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
    pub (super) fn no_instructions(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        Err(MgError::throw_error(MgErrorCode::NoInstruction, context.opcode, context.address, context.machine_code))
    }
    pub (super) fn special_opcode_map(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        static SPECIAL_MAP: [fn(disassembler: &MgDisassembler, &mut MgInstructionContext) -> Result<(), MgError>; 64] = [
        MgDisassembler::sll,  MgDisassembler::movci,  MgDisassembler::srl_sra,  MgDisassembler::srl_sra,  MgDisassembler::sllv,  MgDisassembler::no_instructions,  MgDisassembler::srlv_srav,  MgDisassembler::srlv_srav,
        MgDisassembler::jr,  MgDisassembler::jalr,  MgDisassembler::movn_movz,  MgDisassembler::movn_movz,  MgDisassembler::syscall_break,  MgDisassembler::syscall_break,  MgDisassembler::no_instructions,  MgDisassembler::sync,
        MgDisassembler::mfhi_mflo,  MgDisassembler::mthi_mtlo,  MgDisassembler::mfhi_mflo,  MgDisassembler::mthi_mtlo,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
        MgDisassembler::mult_multu_div_divu,  MgDisassembler::mult_multu_div_divu,  MgDisassembler::mult_multu_div_divu,  MgDisassembler::mult_multu_div_divu,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
        MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,
        MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::slt_sltu,  MgDisassembler::slt_sltu,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
        MgDisassembler::tge_tgeu_tlt_tltu,  MgDisassembler::tge_tgeu_tlt_tltu,  MgDisassembler::tge_tgeu_tlt_tltu,  MgDisassembler::tge_tgeu_tlt_tltu,  MgDisassembler::teq_tne,  MgDisassembler::seleqz_selnez,  MgDisassembler::teq_tne,  MgDisassembler::seleqz_selnez,
        MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions ];

        SPECIAL_MAP[(context.machine_code & 0b111111) as usize](self, context)
    }
    pub (super) fn regimm_opcode_map(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let imm_order: usize;
        let rs: Option<FieldInfos>;
        static MENMONICS: [[Option<&str>; 8]; 4] =
        [   [Some(MG_MNE_BLTZ),  Some(MG_MNE_BGEZ),  Some(MG_MNE_BLTZL),  Some(MG_MNE_BGEZL),  None,  None,  None,  None],
            [Some(MG_MNE_TGEI),  Some(MG_MNE_TGEIU),  Some(MG_MNE_TLTI),  Some(MG_MNE_TLTIU),  Some(MG_MNE_TEQI),  None,  Some(MG_MNE_TNEI),  None],
            [Some(MG_MNE_BLTZAL),  Some(MG_MNE_BGEZAL),  Some(MG_MNE_BLTZALL),  Some(MG_MNE_BGEZALL),  None,  None,  None,  None],
            [None,  None,  None,  None,  None,  None,  None,  Some(MG_MNE_SYNCI)] ];

        if let MgMipsVersion::M32(MgMips32::MgR6) = self.version {
            if context.machine_code >> 19 & 0b11 == 1{
                return Err(MgError::throw_error(MgErrorCode::VersionError, context.opcode, context.address, context.machine_code))
            }
        };
        context.mnemonic = MENMONICS[(context.machine_code >> 19 & 0b11) as usize][(context.machine_code >> 16 & 0b111) as usize];
        context.category = Some(match context.machine_code >> 19 & 3{
            3 => MgInstructionCategory::MemoryControl,
            1 => {
                context.is_conditional = true;
                MgInstructionCategory::Trap
            },
            _ => {
                context.is_relative = true;
                context.is_conditional = true;
                MgInstructionCategory::BranchJump
            },
        });

        if (context.machine_code >> 16 & 0b111111) == 0x11
        && (context.machine_code >> 21 & 0b11111) == 0{
            context.mnemonic = Some(MG_MNE_BAL);
            rs = None;
            imm_order = 0;
            context.is_conditional = false;
        }
        else if (context.machine_code >> 16 & 0b111111) == 0x1f{
            imm_order = 0;
            rs = Some(FieldInfos::default_reg_field(1, MgCoprocessor::Cpu));
        }
        else{
            imm_order = 1;
            rs = Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu));
        }

        return MgDisassembler::imm_format(self, context, rs, None, Some(FieldInfos::default_imm_field(imm_order)))
    }
    pub (super) fn special2_opcode_map(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let MgMipsVersion::M32(MgMips32::MgR6) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, context.opcode, context.address, context.machine_code))
        };
        static SPECIAL2_MAP: [fn(disassembler: &MgDisassembler, &mut MgInstructionContext) -> Result<(), MgError>; 64] = 
            [   MgDisassembler::madd_maddu,  MgDisassembler::madd_maddu,  MgDisassembler::mul,  MgDisassembler::no_instructions,  MgDisassembler::msub_msubu,  MgDisassembler::msub_msubu,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::clz_clo,  MgDisassembler::clz_clo,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::sdbbp ];
        SPECIAL2_MAP[(context.machine_code & 0b111111) as usize](self, context)
    }
    pub (super) fn special3_opcode_map(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        static SPECIAL3_MAP: [fn(disassembler: &MgDisassembler, &mut MgInstructionContext) -> Result<(), MgError>; 64] = 
            [   MgDisassembler::ext,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::ins,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::bshfl,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::cache_pref,  MgDisassembler::sc_ll,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::cache_pref,  MgDisassembler::sc_ll,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::rdhwr,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions ];
        
        SPECIAL3_MAP[(context.machine_code & 0b111111) as usize](self, context)
    }
    pub (super) fn cop0_opcode_map(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        static COP0_MAP: [fn(disassembler: &MgDisassembler, &mut MgInstructionContext) -> Result<(), MgError>; 32] =
            [   MgDisassembler::mov_cp0,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::mov_cp0,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::gpr_shadowset,  MgDisassembler::mfmc0,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::gpr_shadowset,  MgDisassembler::no_instructions,
                MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,
                MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0,  MgDisassembler::c0];
        // unimplemented!("[-]Opcode map isn't implemented yet!");
        // context.coprocessor = MgCoprocessor::Cp0;
        COP0_MAP[(context.machine_code >> 21 & 0b11111) as usize](self, context)
    }
    pub (super) fn cop1_opcode_map(&self, _instruction: &mut MgInstructionContext) -> Result<(), MgError>{
        static _COP1_MAP: [fn(disassembler: &MgDisassembler, &mut MgInstructionContext) -> Result<(), MgError>; 64] =
        [   MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions ];
        unimplemented!("[-]Cop1 opcode map isn't implemented yet!");

        // COP1_MAP[(context.machine_code >> 26) as usize](context)
    }
    pub (super) fn cop2_opcode_map(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        static COP2_MAP: [fn(disassembler: &MgDisassembler, &mut MgInstructionContext) -> Result<(), MgError>; 64] = 
        [   MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::load_store_cp2,  MgDisassembler::load_store_cp2,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::load_store_cp2,  MgDisassembler::load_store_cp2,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions ];
        return COP2_MAP[(context.machine_code >> 21 & 0b11111) as usize](self, context)
    }
    pub (super) fn cop1x_opcode_map(&self,context : &mut MgInstructionContext) -> Result<(), MgError>{
        let MgMipsVersion::M32(MgMips32::MgR6) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, context.opcode, context.address, context.machine_code))
        };
        static _COP1X_MAP: [fn(disassembler: &MgDisassembler, &mut MgInstructionContext) -> Result<(), MgError>; 64] = 
        [   MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
            MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions ];
        unimplemented!("[-]Opcode map isn't implemented yet!");

        // context.coprocessor = MgCoprocessor::Cp1x;
        // _COP1X_MAP[(context.machine_code >> 26) as usize](context)
    }
    pub (super) fn pcrel_opcode_map(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let MgMipsVersion::M32(MgMips32::MgR6) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, context.opcode, context.address, context.machine_code))
        };
        static PCREL_MAP: [fn(disassembler: &MgDisassembler, &mut MgInstructionContext) -> Result<(), MgError>; 32] =[   
            MgDisassembler::addiupc,  MgDisassembler::addiupc,  MgDisassembler::lwpc,  MgDisassembler::lwpc,  MgDisassembler::lwupc,  MgDisassembler::lwupc,  MgDisassembler::ldpc,  MgDisassembler::no_instructions,
            MgDisassembler::addiupc,  MgDisassembler::addiupc,  MgDisassembler::lwpc,  MgDisassembler::lwpc,  MgDisassembler::lwupc,  MgDisassembler::lwupc,  MgDisassembler::ldpc,  MgDisassembler::no_instructions,
            MgDisassembler::addiupc,  MgDisassembler::addiupc,  MgDisassembler::lwpc,  MgDisassembler::lwpc,  MgDisassembler::lwupc,  MgDisassembler::lwupc,  MgDisassembler::ldpc,  MgDisassembler::auipc,
            MgDisassembler::addiupc,  MgDisassembler::addiupc,  MgDisassembler::lwpc,  MgDisassembler::lwpc,  MgDisassembler::lwupc,  MgDisassembler::lwupc,  MgDisassembler::ldpc,  MgDisassembler::aluipc
        ];

        context.is_relative = true;
        context.format = Some(MgInstructionFormat::Imm);
        let imm = FieldInfos::imm_field(1, 0b1111111111111111);
        let rs = Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu));
        if let Err(e) = PCREL_MAP[(context.machine_code >> 16 & 0b11111) as usize](self, context){
            return Err(e)
        }
        self.imm_format(context, rs, None, Some(imm))
    }

    //Opcode handlers

    //Default opcode field handlers
    pub(super) fn j(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        context.mnemonic = Some(MG_MNE_J);
        MgDisassembler::jump_format(self, context)
    }
    pub(super) fn jal(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        context.mnemonic = Some(MG_MNE_JAL);
        MgDisassembler::jump_format(self, context)
    }
    pub(super) fn beq(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    

        context.is_relative = true;
        context.category = Some(MgInstructionCategory::BranchJump);
        context.mnemonic = Some(MG_MNE_BEQ);
        context.is_conditional = true;
        
        return MgDisassembler::imm_format(self, context, Some(rs), Some(rt), Some(FieldInfos::default_imm_field(2)));
    }
    pub(super) fn bne(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    

        context.is_relative = true;
        context.category = Some(MgInstructionCategory::BranchJump);
        context.mnemonic = Some(MG_MNE_BNE);
        context.is_conditional = true;
        
        return MgDisassembler::imm_format(self, context, Some(rs), Some(rt), Some(FieldInfos::default_imm_field(2)));
    }
    pub(super) fn blez(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        context.is_relative = true;
        context.mnemonic = Some(MG_MNE_BLEZ);
        context.is_conditional = true;
        context.category = Some(MgInstructionCategory::BranchJump);
        let rs: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        return MgDisassembler::imm_format(self, context, Some(rs), Some(FieldInfos::default_fixed_field()), Some(FieldInfos::default_imm_field(1)));
    }
    pub(super) fn bgtz(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        context.is_relative = true;
        context.mnemonic = Some(MG_MNE_BGTZ);
        context.category = Some(MgInstructionCategory::BranchJump);
        context.is_conditional = true;
        let rs: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        return MgDisassembler::imm_format(self, context, Some(rs), Some(FieldInfos::default_fixed_field()), Some(FieldInfos::default_imm_field(1)));
    }
    pub(super) fn addi_addiu(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        context.mnemonic = match context.machine_code >> 26 & 1 {
            1 => Some(MG_MNE_ADDIU),
            0 => {
                Some(MG_MNE_ADDI)
            }
            _ => None
        };
        context.category = Some(MgInstructionCategory::Arithmetic);
        return MgDisassembler::imm_format(self, context, Some(rs), Some(rt), Some(sa));
    }
    pub(super) fn slti_sltiu(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        context.category = Some(MgInstructionCategory::Arithmetic);
        (context.mnemonic_id, context.mnemonic) = match context.machine_code >> 26 & 1 {
            1 => (Some(MgMnemonic::MgMneSltiu), Some(MG_MNE_SLTIU)),
            0 => (Some(MgMnemonic::MgMneSlti), Some(MG_MNE_SLTI)),
            _ => (None, None)
        };

        return MgDisassembler::imm_format(self, context, Some(rs), Some(rt), Some(sa));
    }
    pub(super) fn andi(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        context.mnemonic = Some(MG_MNE_ANDI);
        context.category = Some(MgInstructionCategory::Logical);

        return MgDisassembler::imm_format(self, context, Some(rs), Some(rt), Some(sa));
    }
    pub(super) fn ori(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        context.mnemonic = Some(MG_MNE_ORI);
        context.category = Some(MgInstructionCategory::Logical);
        
        return MgDisassembler::imm_format(self, context, Some(rs), Some(rt), Some(sa));
    }
    pub(super) fn xori(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        context.mnemonic = Some(MG_MNE_XORI);
        context.category = Some(MgInstructionCategory::Logical);
        
        return MgDisassembler::imm_format(self, context, Some(rs), Some(rt), Some(sa));
    }
    pub(super) fn lui(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(1);

        context.mnemonic = Some(MG_MNE_LUI);
        context.category = Some(MgInstructionCategory::Logical);

        return MgDisassembler::imm_format(self, context, Some(FieldInfos::default_fixed_field()), Some(rt), Some(sa));
    }
    pub(super) fn beql(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let MgMipsVersion::M32(MgMips32::MgR6) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, context.opcode, context.address, context.machine_code))
        };
        let rs: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let imm: FieldInfos = FieldInfos::default_imm_field(2);

        context.is_relative = true;
        context.category = Some(MgInstructionCategory::BranchJump);
        context.mnemonic = Some(MG_MNE_BEQL);
        context.is_conditional = true;
        
        return MgDisassembler::imm_format(self, context, Some(rs), Some(rt), Some(imm));
    }
    pub(super) fn bnel(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let MgMipsVersion::M32(MgMips32::MgR6) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, context.opcode, context.address, context.machine_code))
        };
        let rs: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let imm: FieldInfos = FieldInfos::default_imm_field(2);

        context.is_relative = true;
        context.category = Some(MgInstructionCategory::BranchJump);
        context.mnemonic = Some(MG_MNE_BNEL);
        context.is_conditional = true;
        
        return MgDisassembler::imm_format(self, context, Some(rs), Some(rt), Some(imm));
    }
    pub(super) fn blezl(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let MgMipsVersion::M32(MgMips32::MgR6) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, context.opcode, context.address, context.machine_code))
        };
        context.is_relative = true;
        context.mnemonic = Some(MG_MNE_BLEZL);
        context.category = Some(MgInstructionCategory::BranchJump);
        context.is_conditional = true;
        let rs: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        return MgDisassembler::imm_format(self, context, Some(rs), Some(FieldInfos::default_fixed_field()), Some(FieldInfos::default_imm_field(1)));
    }
    pub(super) fn bgtzl(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let MgMipsVersion::M32(MgMips32::MgR6) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, context.opcode, context.address, context.machine_code))
        };
        context.is_relative = true;
        context.mnemonic = Some(MG_MNE_BGTZL);
        context.category = Some(MgInstructionCategory::BranchJump);
        context.is_conditional = true;
        let rs: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        return MgDisassembler::imm_format(self, context, Some(rs), Some(FieldInfos::default_fixed_field()), Some(FieldInfos::default_imm_field(1)));
    }
    pub(super) fn jalx(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let MgMipsVersion::M32(MgMips32::MgR6) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, context.opcode, context.address, context.machine_code))
        };
        context.mnemonic = Some(MG_MNE_JALX);
        MgDisassembler::jump_format(self, context)
    }
    pub (super) fn lwr_swr_lwl_swl(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let mnemonics: [[Option<&str>; 2]; 2]=[[Some(MG_MNE_LWL), Some(MG_MNE_SWL)], [Some(MG_MNE_LWR), Some(MG_MNE_SWR)]];
        let mnemonics_id: [[Option<MgMnemonic>; 2]; 2]=[[Some(MgMnemonic::MgMneLwl), Some(MgMnemonic::MgMneSwl)], [Some(MgMnemonic::MgMneLwr), Some(MgMnemonic::MgMneSwr)]];
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let rt = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);

        match self.version{
            MgMipsVersion::M32(MgMips32::MgPreR6) => (),
            MgMipsVersion::M32(MgMips32::MgR6) => return Err(MgError::throw_error(MgErrorCode::VersionError, context.opcode, context.address, context.machine_code)),
            _ => unimplemented!(),
        }
                
        (context.category,context.mnemonic,context.mnemonic_id) = match context.opcode >> 3 & 1{
            0 => (Some(MgInstructionCategory::Load), mnemonics[(context.opcode >> 2 & 1) as usize][0], mnemonics_id[(context.opcode >> 2 & 1) as usize][0]),
            1 => (Some(MgInstructionCategory::Store), mnemonics[(context.opcode >> 2 & 1) as usize][1], mnemonics_id[(context.opcode >> 2 & 1) as usize][1]),
            _ => (None, None, None)
        };

        return MgDisassembler::imm_format(self, context, Some(base), Some(rt), Some(FieldInfos::default_imm_field(1)))
    }
    pub(super) fn bovc_bnvc(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        if let MgMipsVersion::M32(MgMips32::MgPreR6) = self.version{
            return if context.opcode & 0b010000 == 0{
                self.addi_addiu(context)
            }else {
                self.no_instructions(context)
            }
        };

        context.is_conditional = true;
        context.is_relative = true;
        (context.mnemonic, context.mnemonic_id) = if context.opcode & 0b010000 == 0{
            (Some(MG_MNE_BOVC), Some(MgMnemonic::MgMneBovc))
        }else{
            (Some(MG_MNE_BNVC), Some(MgMnemonic::MgMneBnvc))
        };
        context.category = Some(MgInstructionCategory::BranchJump);
        return MgDisassembler::imm_format(self, context, Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_reg_field(1, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(2)))
    }
    pub(super) fn jic_jialc(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        if let MgMipsVersion::M32(MgMips32::MgPreR6) = self.version{
            return self.load_store_cp2(context)
        };

        (context.mnemonic, context.mnemonic_id) = if context.opcode >> 3 & 1 == 1{
            (Some(MG_MNE_JIALC), Some(MgMnemonic::MgMneJialc))
        }else{
            (Some(MG_MNE_JIC), Some(MgMnemonic::MgMneJic))
        };
        context.category = Some(MgInstructionCategory::BranchJump);
        return MgDisassembler::imm_format(self, context, Some(FieldInfos::default_fixed_field()), Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(1)))
    }
    pub(super) fn bc_balc(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        if let MgMipsVersion::M32(MgMips32::MgPreR6) = self.version{
            return self.load_store_cp2(context)
        };

        let mut hex_num: MgString = MgString::new_lmstring();
        (context.mnemonic, context.mnemonic_id) = if context.opcode >> 3 & 1 == 1{
            (Some(MG_MNE_BALC), Some(MgMnemonic::MgMneBalc))
        }else{
            (Some(MG_MNE_BC), Some(MgMnemonic::MgMneBc))
        };

        //Some attributes about the instruction
        context.format = Some(MgInstructionFormat::Jump);
        context.operand_num = 1 ;
        context.is_conditional = true;
        context.category = Some(MgInstructionCategory::BranchJump);
        context.operand[0] = Some(MgOpImmediate::new_imm_opreand((context.machine_code & 0x3FFFFFF) as u64));

        //Formatting the string
        //If the branch/jump is relative, the string will show it's destination address instead of the offset
        let (Some(MgOperand::MgOpImmediate(imm)), Some(mne)) = (context.operand[0], context.mnemonic) else{
            return Err(MgError::throw_error(MgErrorCode::DevError, context.opcode, context.address, context.machine_code))
        };
        context.string.append_str(mne);
        hex_num.num_to_str(imm.get_value() * 0x4 + context.address);
        context.string.append_char(' ');
        context.string.append_string(&hex_num);

        Ok(())
    }
    pub(super) fn load_store_cp2(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let mnemonics: [[Option<&str>; 2]; 2] = [[Some(MG_MNE_LWC2), Some(MG_MNE_SWC2)], [Some(MG_MNE_LDC2), Some(MG_MNE_SDC2)]];
        let mnemonic_id: [[Option<MgMnemonic>; 2]; 2] = [[Some(MgMnemonic::MgMneLwc2), Some(MgMnemonic::MgMneSwc2)], [Some(MgMnemonic::MgMneLdc2), Some(MgMnemonic::MgMneSdc2)]];
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let rt = FieldInfos::default_reg_field(0, MgCoprocessor::Cp2);
        let imm: Option<FieldInfos>;

         match self.version{
            MgMipsVersion::M32(MgMips32::MgR6)=> {
                (context.mnemonic, context.mnemonic_id, context.category) = (mnemonics[(context.machine_code >> 23 & 1) as usize][(context.machine_code >> 21 & 1) as usize], mnemonic_id[(context.machine_code >> 23 & 1) as usize][(context.machine_code >> 21 & 1) as usize], Some(if context.machine_code >> 21 & 1 == 0{
                    MgInstructionCategory::Load
                }else{
                    MgInstructionCategory::Store
                }));
                imm = Some(FieldInfos::imm_field(1, 0b11111111111));
            },
            MgMipsVersion::M32(MgMips32::MgPreR6) => {
                if context.opcode == 0b010010{
                    return Err(MgError::throw_error(MgErrorCode::VersionError, context.opcode, context.address, context.machine_code))
                }
                (context.mnemonic_id, context.mnemonic) = (mnemonic_id[(context.opcode >> 2 & 1) as usize][(context.opcode >> 3 & 1) as usize], mnemonics[(context.opcode >> 2 & 1) as usize][(context.opcode >> 3 & 1) as usize]);
                context.category = if context.opcode >> 3 & 1 == 0{
                    Some(MgInstructionCategory::Load)
                }else{
                    Some(MgInstructionCategory::Store)
                };
                imm = Some(FieldInfos::default_imm_field(1));
            }
            _ => unimplemented!(),
        };

        return MgDisassembler::imm_format(self, context, Some(base), Some(rt), imm)
    }
    pub (super) fn sc_ll(&self, context : &mut MgInstructionContext) -> Result<(), MgError>{
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let rt = FieldInfos::default_reg_field(0, MgCoprocessor::Cp2);
        let imm: Option<FieldInfos>;

        context.is_conditional = true;
        (context.mnemonic, context.mnemonic_id, context.category) = match self.version{
            MgMipsVersion::M32(MgMips32::MgPreR6) => {
                if 0b011111 == context.opcode{
                    return Err(MgError::throw_error(MgErrorCode::VersionError, context.opcode, context.address, context.machine_code))
                }else {
                    imm = Some(FieldInfos::default_imm_field(1));
                    if context.opcode >> 3 & 1 == 1{
                        (Some(MG_MNE_SC), Some(MgMnemonic::MgMneSc), Some(MgInstructionCategory::Store))
                    }else {
                        (Some(MG_MNE_LL), Some(MgMnemonic::MgMneLl), Some(MgInstructionCategory::Load))
                    }
                }
            },
            MgMipsVersion::M32(MgMips32::MgR6) =>{
                if 0b011111 != context.opcode{
                    return Err(MgError::throw_error(MgErrorCode::VersionError, context.opcode, context.address, context.machine_code))
                }else{
                    if context.machine_code >> 6 == 1{
                        return Err(MgError::throw_error(MgErrorCode::FieldBadValue, context.opcode, context.address, context.machine_code))
                    }
                    imm = None;
                    context.operand[1] = Some(MgOpImmediate::new_imm_opreand((context.machine_code >> 7 & 0b111111111) as u64));
                    context.operand_num += 1;
                    if context.machine_code >> 4 & 1 != 1{
                        (Some(MG_MNE_SC), Some(MgMnemonic::MgMneSc), Some(MgInstructionCategory::Store))
                    }else {
                        (Some(MG_MNE_LL), Some(MgMnemonic::MgMneLl), Some(MgInstructionCategory::Load))
                    }
                }
            },
            _ => unimplemented!(),
        };

        return MgDisassembler::imm_format(self, context, Some(base), Some(rt), imm)
    }
    pub(super) fn cpu_loadstore(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let rt: FieldInfos;
        let mnemonics: [[Option<&str>; 7]; 4] = [
            [Some(MG_MNE_LB), Some(MG_MNE_LH), None, Some(MG_MNE_LW), Some(MG_MNE_LBU), Some(MG_MNE_LHU), None],
            [Some(MG_MNE_SB), Some(MG_MNE_SH), None, Some(MG_MNE_SW), None, None, None],
            [None, Some(MG_MNE_LWC1), None, None, None, Some(MG_MNE_LDC1), None],
            [None, Some(MG_MNE_SWC1), None, None, None, Some(MG_MNE_SDC1), None]
        ];

        context.mnemonic = mnemonics[(context.machine_code >> 29 & 3) as usize][(context.machine_code >> 26 & 7) as usize];

        if (context.machine_code >> 29 & 3) == 6 
        || (context.machine_code >> 29 & 3) == 7
        && (context.machine_code >> 27 & 3) == 0{
            rt = FieldInfos::default_reg_field(0, MgCoprocessor::Cp1);
        }
        else {
            rt = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);
        }

        context.category = Some(match context.machine_code & 1{
            0 => MgInstructionCategory::Load,
            1 => MgInstructionCategory::Store,
            _ => return Err(MgError::throw_error(MgErrorCode::FieldBadValue, context.opcode, context.address, context.machine_code))
        });

        return MgDisassembler::imm_format(self, context, Some(base), Some(rt), Some(FieldInfos::default_imm_field(1)))
    }
    pub(super) fn cache_pref(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let op: FieldInfos = FieldInfos::imm_field(0, 0b11111);

        return if let MgMipsVersion::M32(MgMips32::MgPreR6) = self.version{
            if context.opcode == 0b011111{
                Err(MgError::throw_error(MgErrorCode::VersionError, context.opcode, context.address, context.machine_code))
            } else{
                (context.mnemonic, context.category) = if context.opcode == 0b110011{
                    (Some(MG_MNE_PREF), Some(MgInstructionCategory::MemoryControl))
                }else if context.opcode == 0b101111{
                    (Some(MG_MNE_CACHE), Some(MgInstructionCategory::Priviledge))
                }else{
                    return Err(MgError::throw_error(MgErrorCode::DevError, context.opcode, context.address, context.machine_code))
                };
                MgDisassembler::imm_format(self, context, Some(base), Some(op), Some(FieldInfos::default_imm_field(1)))
            }
        } else if let MgMipsVersion::M32(MgMips32::MgR6) = self.version{
            if context.opcode != 0b011111{
                Err(MgError::throw_error(MgErrorCode::VersionError, context.opcode, context.address, context.machine_code))
            } else  if context.machine_code >> 6 & 1 == 1{
                Err(MgError::throw_error(MgErrorCode::FieldBadValue, context.opcode, context.address, context.machine_code))
            } else{
                (context.mnemonic, context.category) = if context.machine_code >> 4 & 1 == 1{
                    (Some(MG_MNE_PREF), Some(MgInstructionCategory::MemoryControl))
                } else if context.machine_code >> 4 & 1 == 0{
                    (Some(MG_MNE_CACHE), Some(MgInstructionCategory::Priviledge))
                }else{
                    return Err(MgError::throw_error(MgErrorCode::DevError, context.opcode, context.address, context.machine_code))
                };
                context.operand_num = 1;
                context.operand[1] = Some(MgOpImmediate::new_imm_opreand((context.machine_code >> 7 & 0b111111111) as u64));
                MgDisassembler::imm_format(self, context, Some(base), Some(op), None)
            }
        }
        else {
            unimplemented!();
        }
    }

    //Special
    pub(super) fn sll(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_fixed_field();
        let rt: FieldInfos;
        let rd: FieldInfos;
        let sa: FieldInfos;

        if context.machine_code >> 11 & 0b111111111111111 == 0{
            context.mnemonic = match context.machine_code >> 6 & 0b11111{
                0 => Some(MG_MNE_NOP),
                1 => Some(MG_MNE_SSNOP),
                3 => Some(MG_MNE_EHB),
                5 => Some(MG_MNE_PAUSE),
                _ => return Err(MgError::throw_error(MgErrorCode::FieldBadValue, context.opcode, context.address, context.machine_code))
            };
            rt = FieldInfos::default_fixed_field();
            rd = FieldInfos::default_fixed_field();
            sa = FieldInfos::default_fixed_field();

            context.category = Some(MgInstructionCategory::Control);
        }
        else{
            context.mnemonic = Some(MG_MNE_SLL);
            context.category = Some(MgInstructionCategory::Shift);

            rt = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
            rd = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
            sa = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Imm);
        }
        
        MgDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd), Some(sa))
    }
    pub(super) fn movci(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        //Reserved Instruction, Coprocessor Unusable
        if (context.machine_code >> 6 & 0b11111) != 0
        ||(context.machine_code >> 17 & 1) != 0{
            return Err(MgError::throw_error(MgErrorCode::FieldBadValue, context.opcode, context.address, context.machine_code))
        }
        let mnemonics = [Some(MG_MNE_MOVF), Some(MG_MNE_MOVT)];
        let mut hex_num: MgString = MgString::new_lmstring();
        let comma: &str = ", ";
        let registers: [&str; 8] = [ MG_REG_FCC0, MG_REG_FCC1, MG_REG_FCC2, MG_REG_FCC3, MG_REG_FCC4, MG_REG_FCC5, MG_REG_FCC6, MG_REG_FCC7,];
        
        context.format = Some(MgInstructionFormat::CoditionCodeFpu);
        context.category = Some(MgInstructionCategory::Move);
        context.mnemonic = mnemonics[(context.machine_code >> 16 & 1) as usize];

        context.operand_num = 3;
        context.operand[0] = Some(MgOpRegister::new_reg_opreand((context.machine_code >> 11 & 0b11111) as u8, MgCoprocessor::Cpu));
        context.operand[1] = Some(MgOpRegister::new_reg_opreand((context.machine_code >> 21 & 0b11111) as u8, MgCoprocessor::Cpu));
        context.operand[2] = Some(MgOpRegister::new_reg_operand_str(registers[(context.machine_code >> 18 & 0b111) as usize], MgCoprocessor::Cp1));

        if let Some(mne) = context.mnemonic{
            context.string.append_str(mne);
        }
        context.string.append_char(' ');
        for i in 0..context.operand_num{
            if let Some(op) = context.operand[i]{
                match op{
                    MgOperand::MgOpRegister(reg) => _= context.string.append_str(reg.get_register()),
                    MgOperand::MgOpImmediate(imm) => {
                        hex_num.num_to_str(imm.get_value());
                        context.string.append_string(&hex_num);
                    },
                };
                if context.operand_num - 1 > i{
                    context.string.append_str(&comma);
                }
            }
        }
        Ok(())
    }
    pub(super) fn srl_sra(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::fixed_field(4, 0b1111);
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let sa: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Imm);

        context.mnemonic = match context.machine_code & 1{
            1 => Some(MG_MNE_SRA),
            0 => {
                match context.machine_code >> 6 & 1 {
                    1 => Some(MG_MNE_ROTR),
                    0 => Some(MG_MNE_SRL),
                    _ => None
                }
            },
            _ => None
        };

        context.category = Some(MgInstructionCategory::Shift);
        return MgDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd), Some(sa))
    }
    pub(super) fn sllv(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        context.mnemonic = Some(MG_MNE_SLLV);
        context.category = Some(MgInstructionCategory::Shift);

        let sa: FieldInfos = FieldInfos::default_fixed_field();
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);

        return MgDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd), Some(sa))
    }
    pub(super) fn srlv_srav(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let sa: FieldInfos = FieldInfos::fixed_field(4, 0b1111);
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Imm);

        context.mnemonic = match context.machine_code & 1{
            1 => Some(MG_MNE_SRAV),
            0 => {
                match context.machine_code >> 6 & 1 {
                    1 => Some(MG_MNE_ROTRV),
                    0 => Some(MG_MNE_SRLV),
                    _ => None
                }
            },
            _ => None
        };

        context.category = Some(MgInstructionCategory::Shift);
        return MgDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd), Some(sa))
    }
    pub(super) fn jr(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::fixed_field(4, 0b1111111111);
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);

        context.category = Some(MgInstructionCategory::BranchJump);

        if (context.machine_code >> 6 & 0b10000) != 0{
            context.mnemonic = Some(MG_MNE_JRHB);
        }
        else{
            context.mnemonic = Some(MG_MNE_JR);
        }

        MgDisassembler::reg_format(self, context, Some(rs), None, Some(rd), None)
    }
    pub(super) fn jalr(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::default_fixed_field();
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        
        context.category = Some(MgInstructionCategory::BranchJump);

        if (context.machine_code >> 6 & 0b10000) != 0{
            context.mnemonic = Some(MG_MNE_JALRHB)
        }
        else{
            context.mnemonic = Some(MG_MNE_JALR)
        }

        MgDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd), None)
    }
    pub(super) fn movn_movz(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);

        context.category = Some(MgInstructionCategory::Move);
        context.is_conditional = true;

        if context.machine_code & 0b111111 == 0b001010{
            context.mnemonic = Some(MG_MNE_MOVZ);
        }
        else{
            context.mnemonic = Some(MG_MNE_MOVN);
        }
        return MgDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn syscall_break(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let mut hex_num: MgString = MgString::new_lmstring();

        context.mnemonic = match context.machine_code & 1{
            1 => Some(MG_MNE_BREAK),
            0 => Some(MG_MNE_SYSCALL),
            _ => None
        };
        context.category = Some(MgInstructionCategory::Trap);
        context.format = Some(MgInstructionFormat::Other);
        context.operand[0] = Some(MgOpImmediate::new_imm_opreand(((context.machine_code >> 6) & 0xFFFFF) as u64));

        if let Some(MgOperand::MgOpImmediate(imm)) = context.operand[0]{
            hex_num.num_to_str(imm.get_value());
        };
        if let Some(mne) = context.mnemonic{
            context.string.append_str(mne);
            context.string.append_char(' ');
            context.string.append_string(&hex_num);
        }
        Ok(())
    }
    pub(super) fn sync(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::fixed_field(4, 0b111111111111111);
        let sa: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Imm);

        //Setting the attributes
        context.mnemonic = Some(MG_MNE_SYNC);
        context.category = Some(MgInstructionCategory::MemoryControl);
        MgDisassembler::reg_format(self, context, None, None, Some(rd), Some(sa))
    }
    pub(super) fn mfhi_mflo(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        if let MgMipsVersion::M32(MgMips32::MgR6) = self.version{
            return if context.machine_code & 2 != 0{
                Err(MgError::throw_error(MgErrorCode::VersionError, context.opcode, context.address, context.machine_code))
            } else{
                self.clz_clo(context)
            }
        }
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let mnemonics = [Some(MG_MNE_MFHI), Some(MG_MNE_MFLO)];

        context.mnemonic = mnemonics[(context.machine_code >> 1 & 1) as usize];
        context.category = Some(MgInstructionCategory::Move);

        MgDisassembler::reg_format(self, context, None, Some(FieldInfos::fixed_field(4, 0b1111111111)), Some(rd), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn mthi_mtlo(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        if let MgMipsVersion::M32(MgMips32::MgR6) = self.version{
            return if context.machine_code & 2 != 0{
                Err(MgError::throw_error(MgErrorCode::VersionError, context.opcode, context.address, context.machine_code))
            } else{
                self.clz_clo(context)
            }
        }
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let mnemonics = [Some(MG_MNE_MTHI), Some(MG_MNE_MTLO)];
        
        context.mnemonic = mnemonics[(context.machine_code >> 1 & 1) as usize];
        context.category = Some(MgInstructionCategory::Move);

        MgDisassembler::reg_format(self, context, Some(rs), None, None, Some(FieldInfos::fixed_field(4, 0b111111111111111)))
    }
    pub(super) fn mult_multu_div_divu(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let mnemonics = [[Some(MG_MNE_MULT), Some(MG_MNE_MULTU)], [Some(MG_MNE_DIV), Some(MG_MNE_DIVU)]];

        context.category = Some(MgInstructionCategory::Arithmetic);
        context.mnemonic = mnemonics[(context.machine_code >> 1 & 1) as usize][(context.machine_code & 1) as usize];

        MgDisassembler::reg_format(self, context, Some(rs), Some(rt), None, Some(FieldInfos::fixed_field(4, 0b1111111111)))
    }
    pub(super) fn add_addu_sub_subu_and_or_xor_nor(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let mnemonics = [[[Some(MG_MNE_ADD), Some(MG_MNE_ADDU)], [Some(MG_MNE_SUB), Some(MG_MNE_SUBU)]], [[Some(MG_MNE_AND), Some(MG_MNE_OR)], [Some(MG_MNE_XOR), Some(MG_MNE_NOR)]]];

        context.mnemonic = mnemonics[(context.machine_code >> 2 & 1) as usize][(context.machine_code >> 1 & 1) as usize][(context.machine_code & 1) as usize];
        if (context.machine_code >> 2 & 1) == 1{
            context.category = Some(MgInstructionCategory::Logical);
        }
        else{
            context.category = Some(MgInstructionCategory::Arithmetic);
            if (context.machine_code & 1) == 0{
            }
        }

        MgDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn slt_sltu(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let mnemonics = [Some(MG_MNE_SLT), Some(MG_MNE_SLTU)];

        context.category = Some(MgInstructionCategory::Arithmetic);
        context.is_conditional = true;
        context.mnemonic = mnemonics[(context.machine_code & 1) as usize];

        MgDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn tge_tgeu_tlt_tltu(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let mnemonics = [[Some(MG_MNE_TGE), Some(MG_MNE_TGEU)], [Some(MG_MNE_TLT), Some(MG_MNE_TLTU)]];
        
        context.mnemonic = mnemonics[(context.machine_code >> 1 & 1) as usize][(context.machine_code & 1) as usize];
        context.category = Some(MgInstructionCategory::Trap);

        MgDisassembler::reg_format(self, context, Some(rs), Some(rt), None, Some(FieldInfos::imm_field(2, 0b1111111111)))
    }
    pub(super) fn seleqz_selnez(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);

        match self.version{
            MgMipsVersion::M32(MgMips32::MgPreR6) => return Err(MgError::throw_error(MgErrorCode::VersionError, context.opcode, context.address, context.machine_code)),
            MgMipsVersion::M32(MgMips32::MgR6) => (),
            _ => unimplemented!(),
        }

        context.is_conditional = true;

        context.category = Some(MgInstructionCategory::Move);
        (context.mnemonic, context.mnemonic_id) = if context.machine_code >> 1 & 1 == 1{
            (Some(MG_MNE_SELNEZ), Some(MgMnemonic::MgMneSelnez))
        }else{
            (Some(MG_MNE_SELEQZ), Some(MgMnemonic::MgMneSeleqz))
        };
        
        MgDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd) , Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn teq_tne(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        
        context.category = Some(MgInstructionCategory::Trap);
        (context.mnemonic_id, context.mnemonic) = if context.machine_code >> 1 & 1 == 0{
            (Some(MgMnemonic::MgMneTeq), Some(MG_MNE_TEQ))
        }else{
            (Some(MgMnemonic::MgMneTne), Some(MG_MNE_TNE))
        };

        MgDisassembler::reg_format(self, context, Some(rs), Some(rt), None, Some(FieldInfos::imm_field(2, 0b1111111111)))
    }

    //Special2
    pub(super) fn madd_maddu(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);

        context.category = Some(MgInstructionCategory::Arithmetic);
        context.mnemonic = match context.machine_code & 1{
            0 => Some(MG_MNE_MADD),
            1 => Some(MG_MNE_MADDU),
            _ => None
        };

        MgDisassembler::reg_format(self, context, Some(rs), Some(rt), None, Some(FieldInfos::fixed_field(4, 0b1111111111)))
    }
    pub(super) fn mul(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);

        context.category = Some(MgInstructionCategory::Arithmetic);
        context.mnemonic = Some(MG_MNE_MUL);

        MgDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn msub_msubu(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);

        context.category = Some(MgInstructionCategory::Arithmetic);
        context.mnemonic = match context.machine_code & 1{
            0 => Some(MG_MNE_MSUB),
            1 => Some(MG_MNE_MSUBU),
            _ => None
        };

        MgDisassembler::reg_format(self, context, Some(rs), Some(rt), None, Some(FieldInfos::fixed_field(4, 0b1111111111)))
    }
    pub(super) fn clz_clo(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);

        context.category = Some(MgInstructionCategory::Arithmetic);
        context.mnemonic = match context.machine_code & 1{
            0 => Some(MG_MNE_CLZ),
            1 => Some(MG_MNE_CLO),
            _ => None
        };

        return if let MgMipsVersion::M32(MgMips32::MgR6) = self.version{
            if context.machine_code >> 6 & 1 == 0{
                Err(MgError::throw_error(MgErrorCode::FieldBadValue, context.opcode, context.address, context.machine_code))
            }else{
                MgDisassembler::reg_format(self, context, Some(rs), Some(FieldInfos::default_fixed_field()), Some(rd), None)
            }
        }else{
            MgDisassembler::reg_format(self, context, Some(rs), None, Some(rd), Some(FieldInfos::default_fixed_field()))
        }
    }
    pub(super) fn sdbbp(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let mut hex_num: MgString = MgString::new_lmstring();

        context.mnemonic = Some(MG_MNE_SDBBP);
        context.category = Some(MgInstructionCategory::Trap);
        context.format = Some(MgInstructionFormat::Other);
        context.operand[0] = Some(MgOpImmediate::new_imm_opreand(((context.machine_code >> 6) & 0xFFFFF) as u64));

        if let Some(MgOperand::MgOpImmediate(imm)) = context.operand[0]{
            hex_num.num_to_str(imm.get_value());
        };
        if let Some(mne) = context.mnemonic{
            context.string.append_str(mne);
            context.string.append_char(' ');
            context.string.append_string(&hex_num);
        }
        Ok(())
    }

    //Special3 They need some testing
    pub(super) fn ext(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let mut hex_num: MgString = MgString::new_lmstring();

        context.mnemonic = Some(MG_MNE_EXT);
        context.category = Some(MgInstructionCategory::InsertExtract);

        let success = MgDisassembler::reg_format(self, context, Some(rs), Some(rt), None, None);

        context.operand_num = 4;
        context.operand[2] = Some(MgOpImmediate::new_imm_opreand((context.machine_code >> 6 & 0b11111) as u64));
        context.operand[3] = Some(MgOpImmediate::new_imm_opreand((context.machine_code >> 11 & 0b11111) as u64));
        
        context.string.append_str(", ");
        if let Some(MgOperand::MgOpImmediate(imm)) = context.operand[2]{
            hex_num.num_to_str(imm.get_value());
            context.string.append_string(&hex_num);
        }
        context.string.append_str(", ");
        if let Some(MgOperand::MgOpImmediate(imm)) = context.operand[3]{
            hex_num.num_to_str(imm.get_value());
            context.string.append_string(&hex_num);
        }
        return success
    }
    pub(super) fn ins(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let mut hex_num: MgString = MgString::new_lmstring();

        context.mnemonic = Some(MG_MNE_INS);
        context.category = Some(MgInstructionCategory::InsertExtract);

        let success = MgDisassembler::reg_format(self, context, Some(rs), Some(rt), None, None);

        context.operand_num = 4;
        context.operand[2] = Some(MgOpImmediate::new_imm_opreand((context.machine_code >> 6 & 0b11111) as u64));
        context.operand[3] = Some(MgOpImmediate::new_imm_opreand((context.machine_code >> 11 & 0b11111) as u64));
        
        context.string.append_str(", ");
        if let Some(MgOperand::MgOpImmediate(imm)) = context.operand[3]{
            if let Some(MgOperand::MgOpImmediate(imm1)) = context.operand[2]{
                hex_num.num_to_str(imm.get_value() - imm1.get_value() + 1);
                context.string.append_string(&hex_num);
            }
        }
        context.string.append_str(", ");
        if let Some(MgOperand::MgOpImmediate(imm)) = context.operand[3]{
            hex_num.num_to_str(imm.get_value());
            context.string.append_string(&hex_num);
        }
        return success
    }
    pub(super) fn bshfl(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);

        context.mnemonic = match context.machine_code >> 6 & 0b11111{
            0b00010 => {
                context.category = Some(MgInstructionCategory::InsertExtract);
                Some(MG_MNE_WSBH)},
            0b10000 => {
                context.category = Some(MgInstructionCategory::Arithmetic);
                Some(MG_MNE_SEB)},
            0b11000 => {
                context.category = Some(MgInstructionCategory::Arithmetic);
                Some(MG_MNE_SEH)},
            _ => return Err(MgError::throw_error(MgErrorCode::FieldBadValue, context.opcode, context.address, context.machine_code))
        };
        
        MgDisassembler::reg_format(self, context, Some(FieldInfos::default_fixed_field()), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn rdhwr(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        
        context.category = Some(MgInstructionCategory::Move);
        context.mnemonic = Some(MG_MNE_RDHWR);

        MgDisassembler::reg_format(self, context, Some(FieldInfos::default_fixed_field()), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }

    //CP0
    pub(super) fn mov_cp0(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let mnemonics = [Some(MG_MNE_MFC0), Some(MG_MNE_MTC0)];
        if (context.machine_code >> 3 & 0b11111111) != 0{
            return Err(MgError::throw_error(MgErrorCode::FieldBadValue, context.opcode, context.address, context.machine_code))
        }

        context.category = Some(MgInstructionCategory::Priviledge);
        context.format = Some(MgInstructionFormat::Other);
        context.mnemonic = mnemonics[(context.machine_code >> 23 & 1) as usize];
        context.operand_num = 3;

        context.operand[0] = Some(MgOpRegister::new_reg_opreand((context.machine_code >> 16 & 0b11111) as u8, MgCoprocessor::Cpu));
        context.operand[1] = Some(MgOpRegister::new_reg_opreand((context.machine_code >> 11 & 0b11111) as u8, MgCoprocessor::Cpu));
        context.operand[2] = Some(MgOpImmediate::new_imm_opreand((context.machine_code & 7) as u64));

        MgDisassembler::basic_str_format(context)
    }
    pub(super) fn gpr_shadowset(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let mnemonics = [Some(MG_MNE_RDPGPR), Some(MG_MNE_WRPGPR)];

        context.category = Some(MgInstructionCategory::Priviledge);
        context.mnemonic = mnemonics[(context.machine_code >> 23 & 1) as usize];
        MgDisassembler::cpx_cpu_transfer_format(self, context, FieldInfos::default_reg_field(1, MgCoprocessor::Cpu), FieldInfos::default_reg_field(0, MgCoprocessor::Cpu))
    }
    pub(super) fn mfmc0(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let mnemonics = [Some(MG_MNE_DI), Some(MG_MNE_EI)];

        if context.machine_code & 0b11111 != 0 ||
        (context.machine_code >> 6 & 0b11111) != 0 || 
        (context.machine_code >> 11 & 0b01100) != 0b01100 {
            return Err(MgError::throw_error(MgErrorCode::FieldBadValue, context.opcode, context.address, context.machine_code))
        }
        
        context.category = Some(MgInstructionCategory::Priviledge);
        context.format = Some(MgInstructionFormat::Mfmc0);
        context.mnemonic = mnemonics[(context.machine_code >> 5 & 1) as usize];
        context.operand_num = 1;
        context.operand[0] = Some(MgOpRegister::new_reg_opreand((context.machine_code >> 16 & 0b11111) as u8, MgCoprocessor::Cpu));

        if let Some(mne) = context.mnemonic{
            context.string.append_str(mne);
            context.string.append_char(' ');
            if let Some(MgOperand::MgOpRegister(reg)) = context.operand[0]{
                context.string.append_str(reg.get_register());
            }
        }
        Ok(())
    }
    pub(super) fn c0(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        let mnemonics: [[Option<&str>; 8]; 8] = [
            [None,  Some(MG_MNE_TLBR),  Some(MG_MNE_TLBWI),  None,  None,  None,  Some(MG_MNE_TLBWR),  None],
            [Some(MG_MNE_TLBP),  None,  None,  None,  None,  None,  None,  None],
            [None,  None,  None,  None,  None,  None,  None,  None],
            [Some(MG_MNE_ERET),  None,  None,  None,  None,  None,  None,  Some(MG_MNE_DERET)], 
            [Some(MG_MNE_WAIT),  None,  None,  None,  None,  None,  None,  None],
            [None,  None,  None,  None,  None,  None,  None,  None],
            [None,  None,  None,  None,  None,  None,  None,  None],
            [None,  None,  None,  None,  None,  None,  None,  None]
        ];
        if (context.machine_code >> 6 & 0b1111111111111111111) != 0 ||
        (context.machine_code >> 25 & 1) != 1{
            return Err(MgError::throw_error(MgErrorCode::FieldBadValue, context.opcode, context.address, context.machine_code))
        }

        context.category = Some(MgInstructionCategory::Priviledge);
        context.format = Some(MgInstructionFormat::Other);
        context.mnemonic = mnemonics[(context.machine_code >> 3 & 0b111) as usize][(context.machine_code & 0b111) as usize];
        let Some(mne) = context.mnemonic else {
            return Err(MgError::throw_error(MgErrorCode::DevError, context.opcode, context.address, context.machine_code))
        };
        context.string.append_str(mne);

        assert_ne!(context.mnemonic, None);
        Ok(())
    }

    //pcrel
    pub (super) fn addiupc(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        context.mnemonic = Some(MG_MNE_ADDIUPC);
        context.category = Some(MgInstructionCategory::Arithmetic);
        Ok(())
    }
    pub (super) fn lwpc(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        context.mnemonic = Some(MG_MNE_LWPC);
        context.category = Some(MgInstructionCategory::Load);
        Ok(())
    }
    pub (super) fn lwupc(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        context.mnemonic = Some(MG_MNE_LWUPC);
        context.category = Some(MgInstructionCategory::Load);
        Ok(())
    }
    pub (super) fn aluipc(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        context.mnemonic = Some(MG_MNE_ALUIPC);
        context.category = Some(MgInstructionCategory::Logical);
        Ok(())
    }
    pub (super) fn ldpc(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{      //The manual didn't have an entry 
        context.mnemonic = Some(MG_MNE_LDPC);                                                   //for that instruction but it was mentionned in the Table A.13
        context.category = Some(MgInstructionCategory::Load);
        Ok(())
    }
    pub (super) fn auipc(&self, context: &mut MgInstructionContext) -> Result<(), MgError>{
        context.mnemonic = Some(MG_MNE_AUIPC);
        context.category = Some(MgInstructionCategory::Logical);
        Ok(())
    }
}