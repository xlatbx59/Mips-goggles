//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use crate::instruction::*;
use crate::instruction::mnemonics::*;
use crate::operands::*;
use crate::disassembler::*;
use crate::{MgMips32, MgMips64};
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
        MgDisassembler::sll,  MgDisassembler::movci,  MgDisassembler::srl_sra,  MgDisassembler::srl_sra,  MgDisassembler::sllv_dsllv,  MgDisassembler::lsa_dlsa,  MgDisassembler::srlv_srav,  MgDisassembler::srlv_srav,
        MgDisassembler::jr,  MgDisassembler::jalr,  MgDisassembler::movn_movz,  MgDisassembler::movn_movz,  MgDisassembler::syscall_break,  MgDisassembler::syscall_break,  MgDisassembler::sdbbp,  MgDisassembler::sync,
        MgDisassembler::mfhi_mthi,  MgDisassembler::mfhi_mthi,  MgDisassembler::mflo_mtlo,  MgDisassembler::mflo_mtlo,  MgDisassembler::sllv_dsllv,  MgDisassembler::lsa_dlsa,  MgDisassembler::dsrlv_dsrav,  MgDisassembler::dsrlv_dsrav,
        MgDisassembler::sop,  MgDisassembler::sop,  MgDisassembler::sop,  MgDisassembler::sop,  MgDisassembler::sop,  MgDisassembler::sop,  MgDisassembler::sop,  MgDisassembler::sop,
        MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,  MgDisassembler::add_addu_sub_subu_and_or_xor_nor,
        MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::slt_sltu,  MgDisassembler::slt_sltu,  MgDisassembler::dadd_daddu,  MgDisassembler::dadd_daddu,  MgDisassembler::dsub_dsubu,  MgDisassembler::dsub_dsubu,
        MgDisassembler::tge_tgeu_tlt_tltu,  MgDisassembler::tge_tgeu_tlt_tltu,  MgDisassembler::tge_tgeu_tlt_tltu,  MgDisassembler::tge_tgeu_tlt_tltu,  MgDisassembler::teq_tne,  MgDisassembler::seleqz_selnez,  MgDisassembler::teq_tne,  MgDisassembler::seleqz_selnez,
        MgDisassembler::dsll,  MgDisassembler::no_instructions,  MgDisassembler::dsrl_dsra,  MgDisassembler::dsrl_dsra,  MgDisassembler::dsll32,  MgDisassembler::no_instructions,  MgDisassembler::dsrl32_dsra32,  MgDisassembler::dsrl32_dsra32 ];

        SPECIAL_MAP[(prototype.machine_code & 0b111111) as usize](self, prototype)
    }
    pub(super) fn dahi_dati(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        if prototype.machine_code >> 21 & 0b11111 == 0{
            return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
        };
        let MgMipsVersion::M64(MgMips64::MgR6) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };

        prototype.mnemonic = match prototype.machine_code >> 16 & 0b11111{
            0b00110 => Some(MgMnemonic::MgMneDahi),
            0b11110 =>Some(MgMnemonic::MgMneDati),
            _ => return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
        };

        self.imm_format(prototype, Some(FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg)), None, Some(FieldInfos::default_imm_field(1)))
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

        if prototype.machine_code >> 16 & 0b11111 == 0b00110 ||
        prototype.machine_code >> 16 & 0b11111 == 0b11110{
            return self.dahi_dati(prototype)
        };

        prototype.mnemonic = MENMONICS[(prototype.machine_code >> 19 & 0b11) as usize][(prototype.machine_code >> 16 & 0b111) as usize];
        Some(match prototype.machine_code >> 19 & 3{
            3 => (),
            1 => {
                prototype.is_conditional = true;
            },
            _ => {
                prototype.is_relative = true;
                prototype.is_conditional = true;
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
        if let MgMipsVersion::M32(MgMips32::MgR6) | MgMipsVersion::M64(MgMips64::MgR6) = self.version{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };
        static SPECIAL2_MAP: [fn(disassembler: &MgDisassembler, &mut MgInstructionPrototype) -> Result<(), MgError>; 64] = 
            [   MgDisassembler::madd_maddu,  MgDisassembler::madd_maddu,  MgDisassembler::mul,  MgDisassembler::no_instructions,  MgDisassembler::msub_msubu,  MgDisassembler::msub_msubu,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::clz_clo,  MgDisassembler::clz_clo,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::dclz_dclo,  MgDisassembler::dclz_dclo,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
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
                MgDisassembler::bshfl,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::cache_pref,  MgDisassembler::sc_ll,  MgDisassembler::scd_lld,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,
                MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::no_instructions,  MgDisassembler::cache_pref,  MgDisassembler::sc_ll,  MgDisassembler::scd_lld,
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
        if let MgMipsVersion::M32(MgMips32::MgPreR6) | MgMipsVersion::M64(MgMips64::MgPreR6) = self.version{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };
        static PCREL_MAP: [fn(disassembler: &MgDisassembler, &mut MgInstructionPrototype) -> Result<(), MgError>; 32] =[   
            MgDisassembler::addiupc,  MgDisassembler::addiupc,  MgDisassembler::lwpc,  MgDisassembler::lwpc,  MgDisassembler::lwupc,  MgDisassembler::lwupc,  MgDisassembler::ldpc,  MgDisassembler::no_instructions,
            MgDisassembler::addiupc,  MgDisassembler::addiupc,  MgDisassembler::lwpc,  MgDisassembler::lwpc,  MgDisassembler::lwupc,  MgDisassembler::lwupc,  MgDisassembler::ldpc,  MgDisassembler::no_instructions,
            MgDisassembler::addiupc,  MgDisassembler::addiupc,  MgDisassembler::lwpc,  MgDisassembler::lwpc,  MgDisassembler::lwupc,  MgDisassembler::lwupc,  MgDisassembler::ldpc,  MgDisassembler::auipc,
            MgDisassembler::addiupc,  MgDisassembler::addiupc,  MgDisassembler::lwpc,  MgDisassembler::lwpc,  MgDisassembler::lwupc,  MgDisassembler::lwupc,  MgDisassembler::ldpc,  MgDisassembler::aluipc
        ];

        prototype.is_relative = true;
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
        prototype.mnemonic = Some(MgMnemonic::MgMneBeq);
        prototype.is_conditional = true;
        
        return MgDisassembler::imm_format(self, prototype, Some(rs), Some(rt), Some(FieldInfos::default_imm_field(2)));
    }
    pub(super) fn bne(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    

        prototype.is_relative = true;
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
        (rt, rs, imm) = match self.version{
            MgMipsVersion::M64(MgMips64::MgPreR6) => return self.daddi_daddiu(prototype),
            MgMipsVersion::M32(MgMips32::MgPreR6) => return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code)),
            _ => {

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
            }
        };       
        return MgDisassembler::imm_format(self, prototype, rs, rt, imm);
    }
    pub(super) fn pop10(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: Option<FieldInfos>;
        let rt: Option<FieldInfos>;    
        let imm: Option<FieldInfos>;    

        prototype.is_relative = true;
        prototype.is_conditional = true;

        (rt, rs, imm) = match self.version{
            MgMipsVersion::M64(MgMips64::MgPreR6) | MgMipsVersion::M32(MgMips32::MgPreR6) =>return self.addi_addiu(prototype),
            _ =>{
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
            }
        };        
        return MgDisassembler::imm_format(self, prototype, rs, rt, imm);
    }
    pub(super) fn blez_pop06(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: Option<FieldInfos>;
        let rt: Option<FieldInfos>;    
        let imm: Option<FieldInfos>;    

        prototype.is_relative = true;
        prototype.is_conditional = true;

        (rt, rs, imm) = if let MgMipsVersion::M32(MgMips32::MgPreR6) | MgMipsVersion::M64(MgMips64::MgPreR6) = self.version{
            if prototype.machine_code >> 0x10 & 0b11111 != 0{
                return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            }
            prototype.mnemonic = Some(MgMnemonic::MgMneBlez);      
            (None, Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(1)))
        }else /*if let MgMipsVersion::M32(MgMips32::MgR6) = self.version*/{
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
        };
        
        return MgDisassembler::imm_format(self, prototype, rs, rt, imm);
    }
    pub(super) fn bgtz_pop07(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: Option<FieldInfos>;
        let rt: Option<FieldInfos>;    
        let imm: Option<FieldInfos>;    

        prototype.is_relative = true;
        prototype.is_conditional = true;

        (rt, rs, imm) = if let MgMipsVersion::M32(MgMips32::MgPreR6) | MgMipsVersion::M64(MgMips64::MgPreR6) = self.version{
            if prototype.machine_code >> 0x10 & 0b11111 != 0{
                return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            }
            prototype.mnemonic = Some(MgMnemonic::MgMneBgtz);      
            (None, Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(1)))
        }else /*if let MgMipsVersion::M32(MgMips32::MgR6) = self.version*/{
            if prototype.machine_code >> 0x15 & 0b11111 == prototype.machine_code >> 0x10 & 0b11111 && prototype.machine_code >> 0x10 & 0b11111 != 0{
                prototype.mnemonic = Some(MgMnemonic::MgMneBltzalc);
                (Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)),     //rt
                None, Some(FieldInfos::default_imm_field(1)))   //rs
            }else if prototype.machine_code >> 0x15 & 0b11111 != prototype.machine_code >> 0x10 & 0b11111 && 
            prototype.machine_code >> 0x10 & 0b11111 != 0 && prototype.machine_code >> 0x15 & 0b11111 != 0{
                prototype.mnemonic = Some(MgMnemonic::MgMneBltuc);
                (Some(FieldInfos::default_reg_field(1, MgCoprocessor::Cpu)),     //rt
                Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(2)))   //rs
            }else if prototype.machine_code >> 0x15 & 0b11111 == 0x00 && prototype.machine_code >> 0x10 & 0b11111 != 0x00{
                prototype.mnemonic = Some(MgMnemonic::MgMneBgtzalc);
                (Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), None, Some(FieldInfos::default_imm_field(1)))
            }else {
                prototype.mnemonic = Some(MgMnemonic::MgMneBgtz);      
                (None,     //rt
                Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(1)))   //rs
            }
        };
        
        return MgDisassembler::imm_format(self, prototype, rs, rt, imm);
    }
    pub(super) fn ldr_ldl(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        if let MgMipsVersion::M32(_) | MgMipsVersion::M64(MgMips64::MgR6) = self.version{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };

        prototype.mnemonic = if prototype.opcode & 1 == 1{
            Some(MgMnemonic::MgMneLdr)
        }else {
            Some(MgMnemonic::MgMneLdl)
        };
        return MgDisassembler::imm_format(self, prototype, Some(rs), Some(rt), Some(sa));
    }
    pub(super) fn daddi_daddiu(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        if let MgMipsVersion::M32(_) = self.version{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        }

        prototype.mnemonic = if prototype.opcode & 1 == 1{
            Some(MgMnemonic::MgMneDaddiu)
        }else {
            if let MgMipsVersion::M64(MgMips64::MgR6) = self.version {
                return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            }
            Some(MgMnemonic::MgMneDaddi)
        };
        return MgDisassembler::imm_format(self, prototype, Some(rs), Some(rt), Some(sa));
    }
    pub(super) fn addi_addiu(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        prototype.mnemonic = if prototype.opcode & 1 == 1{
            Some(MgMnemonic::MgMneAddiu)
        }else {
            if let MgMipsVersion::M32(MgMips32::MgR6) | MgMipsVersion::M64(MgMips64::MgR6) = self.version {
                return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            }
            Some(MgMnemonic::MgMneAddi)
        };
        return MgDisassembler::imm_format(self, prototype, Some(rs), Some(rt), Some(sa));
    }
    pub(super) fn slti_sltiu(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

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

        return MgDisassembler::imm_format(self, prototype, Some(rs), Some(rt), Some(sa));
    }
    pub(super) fn ori(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        prototype.mnemonic = Some(MgMnemonic::MgMneOri);
        
        return MgDisassembler::imm_format(self, prototype, Some(rs), Some(rt), Some(sa));
    }
    pub(super) fn xori(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        prototype.mnemonic = Some(MgMnemonic::MgMneXori);
        
        return MgDisassembler::imm_format(self, prototype, Some(rs), Some(rt), Some(sa));
    }
    pub(super) fn lui_aui(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let rs: Option<FieldInfos>;
        let sa: FieldInfos;

        rs = if prototype.machine_code >> 21 & 0b11111 != 0{
            if let MgMipsVersion::M32(MgMips32::MgPreR6) | MgMipsVersion::M64(MgMips64::MgPreR6) = self.version{
                return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            };
            prototype.mnemonic = Some(MgMnemonic::MgMneAui);
            sa = FieldInfos::default_imm_field(2);
            Some(FieldInfos::default_reg_field(1, MgCoprocessor::Cpu))
        }else{
            sa = FieldInfos::default_imm_field(1);
            prototype.mnemonic = Some(MgMnemonic::MgMneLui);
            None
        };
        return MgDisassembler::imm_format(self, prototype, rs, Some(rt), Some(sa));
    }
    pub(super) fn beql(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        if let MgMipsVersion::M64(MgMips64::MgR6) | MgMipsVersion::M32(MgMips32::MgR6) = self.version{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };
        let rs: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let imm: FieldInfos = FieldInfos::default_imm_field(2);

        prototype.is_relative = true;
        prototype.mnemonic = Some(MgMnemonic::MgMneBeql);
        prototype.is_conditional = true;
        
        return MgDisassembler::imm_format(self, prototype, Some(rs), Some(rt), Some(imm));
    }
    pub(super) fn bnel(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        if let MgMipsVersion::M64(MgMips64::MgR6) | MgMipsVersion::M32(MgMips32::MgR6) = self.version{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };
        let rs: FieldInfos = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(1, MgCoprocessor::Cpu);    
        let imm: FieldInfos = FieldInfos::default_imm_field(2);

        prototype.is_relative = true;
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

        (rt, rs, imm) = if let MgMipsVersion::M32(MgMips32::MgPreR6) | MgMipsVersion::M64(MgMips64::MgPreR6) = self.version{
            if prototype.machine_code >> 0x10 & 0b11111 != 0{
                return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            }
            prototype.mnemonic = Some(MgMnemonic::MgMneBlezl);      
            (None, Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(1)))
        }else /* if let MgMipsVersion::M32(MgMips32::MgR6) | MgMipsVersion::M64(MgMips64::MgR6) = self.version*/{
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
        };        
        return MgDisassembler::imm_format(self, prototype, rs, rt, imm);
    }
    pub(super) fn bgtzl_pop27(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: Option<FieldInfos>;
        let rt: Option<FieldInfos>;    
        let imm: Option<FieldInfos>;    

        prototype.is_relative = true;
        prototype.is_conditional = true;

        (rt, rs, imm) = if let MgMipsVersion::M32(MgMips32::MgPreR6) | MgMipsVersion::M64(MgMips64::MgPreR6) = self.version{
            if prototype.machine_code >> 0x10 & 0b11111 != 0{
                return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            }
            prototype.mnemonic = Some(MgMnemonic::MgMneBgtzl);
            (None, Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(1)))
        }else /*if let MgMipsVersion::M32(MgMips32::MgR6) = self.version*/{
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
        };        
        return MgDisassembler::imm_format(self, prototype, rs, rt, imm);
    }
    pub(super) fn jalx(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        if let  MgMipsVersion::M64(MgMips64::MgR6) = self.version {
            return self.daui(prototype)
        }else if let MgMipsVersion::M32(MgMips32::MgR6) = self.version{
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
            MgMipsVersion::M32(MgMips32::MgPreR6) | MgMipsVersion::M64(MgMips64::MgPreR6)=> (),
            MgMipsVersion::M32(MgMips32::MgR6) | MgMipsVersion::M64(MgMips64::MgR6) => return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code)),
            // _ => unimplemented!(),
        }
                
        prototype.mnemonic = if prototype.opcode >> 3 & 1 == 0{
            mnemonics_id[(prototype.opcode >> 2 & 1) as usize][0]
        }else{
            mnemonics_id[(prototype.opcode >> 2 & 1) as usize][1]
        };

        return MgDisassembler::imm_format(self, prototype, Some(base), Some(rt), Some(FieldInfos::default_imm_field(1)))
    }
    pub(super) fn pop76(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let imm: FieldInfos;
        let rs: FieldInfos;
        let rt: Option<FieldInfos>;
        if let MgMipsVersion::M32(MgMips32::MgPreR6) | MgMipsVersion::M64(MgMips64::MgPreR6) = self.version{
            return self.load_store_cp2(prototype)
        }
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
        if let MgMipsVersion::M32(MgMips32::MgPreR6) | MgMipsVersion::M64(MgMips64::MgPreR6) = self.version{
            return self.load_store_cp2(prototype)
        }
        (rs, rt, imm) = if prototype.machine_code >> 21 & 0b11111 != 0{
            prototype.is_relative = true;
            prototype.is_conditional = true;
            prototype.mnemonic = Some(MgMnemonic::MgMneBeqzc);
            (FieldInfos::default_reg_field(0, MgCoprocessor::Cpu), None, FieldInfos::imm_field(1, 0x1fffff))
        }else{
            prototype.mnemonic = Some(MgMnemonic::MgMneJic);
            (FieldInfos::default_fixed_field(), Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), FieldInfos::default_imm_field(1))
        };
 
        return MgDisassembler::imm_format(self, prototype, Some(rs), rt, Some(imm))
    }
    pub(super) fn bc_balc(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        if let MgMipsVersion::M32(MgMips32::MgPreR6) | MgMipsVersion::M64(MgMips64::MgPreR6) = self.version{
            return self.load_store_cp2(prototype)
        };

        prototype.mnemonic = if prototype.opcode >> 3 & 1 == 1{
             Some(MgMnemonic::MgMneBalc)
        }else{
            Some(MgMnemonic::MgMneBc)
        };

        //Some attributes about the instruction
        prototype.operand_num = 1 ;
        prototype.is_conditional = true;
        prototype.operand[0] = Some(MgOpImmediate::new_imm_opreand((prototype.machine_code & 0x3FFFFFF) as u64));

        Ok(())
    }
    pub(super) fn load_store_cp1(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let mnemonic: [[Option<MgMnemonic>; 2]; 2] = [[Some(MgMnemonic::MgMneLwc1), Some(MgMnemonic::MgMneSwc1)], [Some(MgMnemonic::MgMneLdc1), Some(MgMnemonic::MgMneSdc1)]];
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let rt = FieldInfos::default_reg_field(0, MgCoprocessor::Cp1);
        let imm = Some(FieldInfos::default_imm_field(1));

        prototype.mnemonic = mnemonic[(prototype.opcode >> 2 & 1) as usize][(prototype.opcode >> 3 & 1) as usize];

        return MgDisassembler::imm_format(self, prototype, Some(base), Some(rt), imm)
    }
    pub(super) fn load_store_cp2(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let mnemonic: [[Option<MgMnemonic>; 2]; 2] = [[Some(MgMnemonic::MgMneLwc2), Some(MgMnemonic::MgMneSwc2)], [Some(MgMnemonic::MgMneLdc2), Some(MgMnemonic::MgMneSdc2)]];
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let rt = FieldInfos::default_reg_field(0, MgCoprocessor::Cp2);
        let imm: Option<FieldInfos>;

        imm = match self.version{
            MgMipsVersion::M32(MgMips32::MgR6) | MgMipsVersion::M64(MgMips64::MgR6)=> {
                if prototype.opcode != 0b010010{
                    return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
                }
                prototype.mnemonic = mnemonic[(prototype.machine_code >> 23 & 1) as usize][(prototype.machine_code >> 21 & 1) as usize];
                prototype.operand[1] = Some(MgOpImmediate::new_imm_opreand((prototype.machine_code >> 7 & 0b111111111) as u64));
                prototype.operand_num+=1;
                None
            },
            MgMipsVersion::M32(MgMips32::MgPreR6) | MgMipsVersion::M64(MgMips64::MgPreR6) => {
                if prototype.opcode == 0b010010{
                    return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
                }
                prototype.mnemonic = mnemonic[(prototype.opcode >> 2 & 1) as usize][(prototype.opcode >> 3 & 1) as usize];
                Some(FieldInfos::default_imm_field(1))
            }
        };

        return MgDisassembler::imm_format(self, prototype, Some(base), Some(rt), imm)
    }
    pub (super) fn sdl_sdr(&self, prototype : &mut MgInstructionPrototype) -> Result<(), MgError>{
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let rt = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);
        let imm: Option<FieldInfos> = Some(FieldInfos::default_imm_field(1));

        let MgMipsVersion::M64(MgMips64::MgPreR6) = self.version else {
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };

        prototype.mnemonic = if prototype.opcode & 1 == 1{
            Some(MgMnemonic::MgMneSdr)
        }else {
            Some(MgMnemonic::MgMneSdl)
        };

        return MgDisassembler::imm_format(self, prototype, Some(base), Some(rt), imm)
    }
    pub (super) fn sd_ld(&self, prototype : &mut MgInstructionPrototype) -> Result<(), MgError>{
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let rt = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);
        let imm: Option<FieldInfos> = Some(FieldInfos::default_imm_field(1));

        if let MgMipsVersion::M32(_) = self.version{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        }

        prototype.mnemonic = if prototype.opcode >> 3 & 1 == 1{
            Some(MgMnemonic::MgMneSd)
        }else {
            Some(MgMnemonic::MgMneLd)
        };

        return MgDisassembler::imm_format(self, prototype, Some(base), Some(rt), imm)
    }
    pub (super) fn scd_lld(&self, prototype : &mut MgInstructionPrototype) -> Result<(), MgError>{
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let rt = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);
        let imm: Option<FieldInfos>;

        prototype.mnemonic = match self.version{
            MgMipsVersion::M64(MgMips64::MgPreR6) => {
                if 0b011111 == prototype.opcode{
                    return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
                }
                imm = Some(FieldInfos::default_imm_field(1));
                if prototype.opcode >> 3 & 1 == 1{
                    prototype.is_conditional = true;
                    Some(MgMnemonic::MgMneScd)
                }else {
                    Some(MgMnemonic::MgMneLld)
                }
            },
            MgMipsVersion::M64(MgMips64::MgR6)=>{
                if 0b011111 != prototype.opcode{
                    return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
                };
                if prototype.machine_code >> 6 & 1 == 1{
                    return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
                }
                imm = None;
                prototype.operand[1] = Some(MgOpImmediate::new_imm_opreand((prototype.machine_code >> 7 & 0x1ff) as u64));
                prototype.operand_num += 1;
                if prototype.machine_code >> 4 & 1 != 1{
                    prototype.is_conditional = true;
                    Some(MgMnemonic::MgMneScd)
                }else {
                    Some(MgMnemonic::MgMneLld)
                }
            },
            _ => return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };

        return MgDisassembler::imm_format(self, prototype, Some(base), Some(rt), imm)
    }
    pub (super) fn sc_ll(&self, prototype : &mut MgInstructionPrototype) -> Result<(), MgError>{
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let rt = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);
        let imm: Option<FieldInfos>;

        prototype.mnemonic = match self.version{
            MgMipsVersion::M32(MgMips32::MgPreR6) | MgMipsVersion::M64(MgMips64::MgPreR6) => {
                if 0b011111 == prototype.opcode{
                    return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
                }else {
                    imm = Some(FieldInfos::default_imm_field(1));
                    if prototype.opcode >> 3 & 1 == 1{
                        prototype.is_conditional = true;
                        Some(MgMnemonic::MgMneSc)
                    }else {
                        Some(MgMnemonic::MgMneLl)
                    }
                }
            },
            MgMipsVersion::M32(MgMips32::MgR6) | MgMipsVersion::M64(MgMips64::MgR6)=>{
                if 0b011111 != prototype.opcode{
                    return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
                }else{
                    if prototype.machine_code >> 6 & 1 == 1{
                        return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
                    }
                    imm = None;
                    prototype.operand[1] = Some(MgOpImmediate::new_imm_opreand((prototype.machine_code >> 7 & 0b111111111) as u64));
                    prototype.operand_num += 1;
                    if prototype.machine_code >> 4 & 1 != 1{
                        prototype.is_conditional = true;
                        Some(MgMnemonic::MgMneSc)
                    }else {
                        Some(MgMnemonic::MgMneLl)
                    }
                }
            },
        };

        return MgDisassembler::imm_format(self, prototype, Some(base), Some(rt), imm)
    }
    pub(super) fn lwu(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let rt = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);
        if let MgMipsVersion::M32(_) = self.version {
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        }
        prototype.mnemonic = Some(MgMnemonic::MgMneLwu);
        return MgDisassembler::imm_format(self, prototype, Some(base), Some(rt), Some(FieldInfos::default_imm_field(1)))
    }
    pub(super) fn cpu_loadstore(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let rt = FieldInfos::default_reg_field(0, MgCoprocessor::Cpu);
        let mnemonics: [[Option<MgMnemonic>; 7]; 4] = [
            [Some(MgMnemonic::MgMneLb), Some(MgMnemonic::MgMneLh), None, Some(MgMnemonic::MgMneLw), Some(MgMnemonic::MgMneLbu), Some(MgMnemonic::MgMneLhu), None],
            [Some(MgMnemonic::MgMneSb), Some(MgMnemonic::MgMneSh), None, Some(MgMnemonic::MgMneSw), None, None, None],
            [None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None]
        ];
        prototype.mnemonic = mnemonics[(prototype.machine_code >> 29 & 3) as usize][(prototype.machine_code >> 26 & 7) as usize];

        return MgDisassembler::imm_format(self, prototype, Some(base), Some(rt), Some(FieldInfos::default_imm_field(1)))
    }
    pub(super) fn cache_pref(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let base: FieldInfos = FieldInfos::default_reg_field(2, MgCoprocessor::Cpu);
        let op: FieldInfos = FieldInfos::imm_field(0, 0b11111);

        return if let MgMipsVersion::M32(MgMips32::MgPreR6) = self.version{
            if prototype.opcode == 0b011111{
                Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            } else{
                prototype.mnemonic = if prototype.opcode == 0b110011{
                    Some(MgMnemonic::MgMnePref)
                }else if prototype.opcode == 0b101111{
                    Some(MgMnemonic::MgMneCache)
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
                prototype.mnemonic = if prototype.machine_code >> 4 & 1 == 1{
                    Some(MgMnemonic::MgMnePref)
                } else if prototype.machine_code >> 4 & 1 == 0{
                    Some(MgMnemonic::MgMneCache)
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
    pub(super) fn daui(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        if prototype.machine_code >> 21 & 0b11111 == 0{
            return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
        };
        prototype.mnemonic = Some(MgMnemonic::MgMneDaui);

        self.imm_format(prototype, Some(FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg)), Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_imm_field(2)))
    }

    //Special
    pub(super) fn dsll32(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_fixed_field();
        let rt = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let sa = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Imm);

        let MgMipsVersion::M64(_) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };

        prototype.mnemonic = Some(MgMnemonic::MgMneDsll32);
        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), Some(sa))
    }
    pub(super) fn dsll(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::default_fixed_field();
        let rt = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let sa = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Imm);

        let MgMipsVersion::M64(_) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };

        prototype.mnemonic = Some(MgMnemonic::MgMneDsll);
        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), Some(sa))
    }
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
        }
        else{
            prototype.mnemonic = Some(MgMnemonic::MgMneSll);

            rt = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
            rd = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
            sa = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Imm);
        }
        
        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), Some(sa))
    }
    pub(super) fn lsa_dlsa(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = match self.version{
            MgMipsVersion::M32(MgMips32::MgR6) => {
                if prototype.machine_code >> 4 & 1 != 0{
                    return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
                }
                Some(MgMnemonic::MgMneLsa)
            },
            MgMipsVersion::M64(MgMips64::MgR6) => {
                if prototype.machine_code >> 4 & 1 == 0{
                    Some(MgMnemonic::MgMneLsa)
                }else {
                    Some(MgMnemonic::MgMneDlsa)
                }
            },
            _ => return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };
        if prototype.machine_code >> 8 & 0b111 != 0{
            return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
        }
        MgDisassembler::reg_format(self, prototype, Some(FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg)), Some(FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg)), Some(FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg)), Some(FieldInfos::imm_field(3, 3)))
    }
    pub(super) fn sop(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let mnemonics = 
        [/*30*/[Some(MgMnemonic::MgMneMul), Some(MgMnemonic::MgMneMuh)],    /*31*/[Some(MgMnemonic::MgMneMulu), Some(MgMnemonic::MgMneMuhu)],
        /*32*/[Some(MgMnemonic::MgMneDiv), Some(MgMnemonic::MgMneMod)],     /*33*/[Some(MgMnemonic::MgMneDivu), Some(MgMnemonic::MgMneModu)],
        /*34*/[Some(MgMnemonic::MgMneDmul), Some(MgMnemonic::MgMneDmuh)],   /*35*/[Some(MgMnemonic::MgMneDmulu), Some(MgMnemonic::MgMneDmuhu)],
        /*36*/[Some(MgMnemonic::MgMneDdiv), Some(MgMnemonic::MgMneDmod)],   /*37*/[Some(MgMnemonic::MgMneDdivu), Some(MgMnemonic::MgMneDmodu)],];

        if prototype.machine_code >> 2 & 1 == 0{
            if let MgMipsVersion::M32(MgMips32::MgPreR6) | MgMipsVersion::M64(MgMips64::MgPreR6) = self.version{
                return self.mult_multu_div_divu(prototype)
            }
        }else{
            if let MgMipsVersion::M64(MgMips64::MgPreR6) = self.version{
                return self.dmult_dmulu_ddiv_ddivu(prototype)
            }else if let MgMipsVersion::M32(_) = self.version{
                return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            }
        }

        prototype.mnemonic = mnemonics[(prototype.machine_code & 7) as usize][(prototype.machine_code >> 6 & 1) as usize];
        self.reg_format(prototype, Some(rs), Some(rt), Some(rd), None)
    }
    pub(super) fn dmult_dmulu_ddiv_ddivu(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let MgMipsVersion::M64(MgMips64::MgPreR6) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };

        prototype.mnemonic = match prototype.machine_code & 3{
            0 => Some(MgMnemonic::MgMneDmult),
            1 => Some(MgMnemonic::MgMneDmultu),
            2 => Some(MgMnemonic::MgMneDdiv),
            _ => Some(MgMnemonic::MgMneDdivu),
        };

        self.reg_format(prototype, Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu)), Some(FieldInfos::default_reg_field(1, MgCoprocessor::Cpu)), Some(FieldInfos::default_fixed_field()), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn movci(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        //Reserved Instruction, Coprocessor Unusable
        if let MgMipsVersion::M32(MgMips32::MgR6) | MgMipsVersion::M64(MgMips64::MgR6) = self.version{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };
        if (prototype.machine_code >> 6 & 0b11111) != 0
        ||(prototype.machine_code >> 17 & 1) != 0{
            return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
        }
        let mnemonics = [Some(MgMnemonic::MgMneMovf), Some(MgMnemonic::MgMneMovt)];
        let registers: [&str; 8] = [ MG_REG_FCC0, MG_REG_FCC1, MG_REG_FCC2, MG_REG_FCC3, MG_REG_FCC4, MG_REG_FCC5, MG_REG_FCC6, MG_REG_FCC7,];
        
        prototype.mnemonic = mnemonics[(prototype.machine_code >> 16 & 1) as usize];

        prototype.operand_num = 3;
        prototype.operand[0] = Some(MgOpRegister::new_reg_opreand((prototype.machine_code >> 11 & 0b11111) as u8, MgCoprocessor::Cpu));
        prototype.operand[1] = Some(MgOpRegister::new_reg_opreand((prototype.machine_code >> 21 & 0b11111) as u8, MgCoprocessor::Cpu));
        prototype.operand[2] = Some(MgOpRegister::new_reg_operand_str(registers[(prototype.machine_code >> 18 & 0b111) as usize], MgCoprocessor::Cp1));

        Ok(())
    }
    pub(super) fn dsrl32_dsra32(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let sa: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Imm);

        let MgMipsVersion::M64(_) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };

        if prototype.machine_code >> 22 & 0xf != 0{
            return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
        }
        if prototype.machine_code & 1 == 1{
            prototype.mnemonic = Some(MgMnemonic::MgMneDsra32);
        }else if prototype.machine_code >> 21 & 1 == 1{
            prototype.mnemonic = Some(MgMnemonic::MgMneDrotr32);
        }else{
            prototype.mnemonic = Some(MgMnemonic::MgMneDsrl32);
        };

        return MgDisassembler::reg_format(self, prototype, None, Some(rt), Some(rd), Some(sa))
    }
    pub(super) fn dsrl_dsra(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let sa: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Imm);

        let MgMipsVersion::M64(_) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };

        if prototype.machine_code >> 22 & 0xf != 0{
            return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
        }
        if prototype.machine_code & 1 == 1{
            prototype.mnemonic = Some(MgMnemonic::MgMneDsra);
        }else if prototype.machine_code >> 21 & 1 == 1{
            prototype.mnemonic = Some(MgMnemonic::MgMneDrotr);
        }else{
            prototype.mnemonic = Some(MgMnemonic::MgMneDsrl);
        };

        return MgDisassembler::reg_format(self, prototype, None, Some(rt), Some(rd), Some(sa))
    }
    pub(super) fn srl_sra(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let sa: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Imm);


        if prototype.machine_code >> 22 & 0xf != 0{
            return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
        }
        if prototype.machine_code & 1 == 1{
            prototype.mnemonic = Some(MgMnemonic::MgMneSra);
        }else if prototype.machine_code >> 21 & 1 == 1{
            prototype.mnemonic = Some(MgMnemonic::MgMneRotr);
        }else{
            prototype.mnemonic = Some(MgMnemonic::MgMneSrl);
        };

        return MgDisassembler::reg_format(self, prototype, None, Some(rt), Some(rd), Some(sa))
    }
    pub(super) fn sllv_dsllv(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let sa: FieldInfos = FieldInfos::default_fixed_field();
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);

        prototype.mnemonic = if prototype.machine_code >> 4 & 1 == 1{
            let MgMipsVersion::M64(_) = self.version else{
                return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
            };
            Some(MgMnemonic::MgMneDsllv)
        }else{
            Some(MgMnemonic::MgMneSllv)
        };

        return MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), Some(sa))
    }
    pub(super) fn dsrlv_dsrav(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Imm);

        let MgMipsVersion::M64(_) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };

        prototype.mnemonic = if prototype.machine_code & 1 == 1{
            if prototype.machine_code >> 6 & 0x1f != 0{
                return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
            }
            Some(MgMnemonic::MgMneDsrav)
        }else if prototype.machine_code >> 6 & 1 == 1{
            if prototype.machine_code >> 7 & 0xf != 0{
                return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
            }
            Some(MgMnemonic::MgMneDrotrv)
        }else{
            if prototype.machine_code >> 6 & 0x1f != 0{
                return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
            }
            Some(MgMnemonic::MgMneDsrlv)
        };

        return MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), None)
    }
    pub(super) fn srlv_srav(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Imm);

        prototype.mnemonic = if prototype.machine_code & 1 == 1{
            if prototype.machine_code >> 6 & 0x1f != 0{
                return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
            }
            Some(MgMnemonic::MgMneSrav)
        }else if prototype.machine_code >> 6 & 1 == 1{
            if prototype.machine_code >> 7 & 0xf != 0{
                return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
            }
            Some(MgMnemonic::MgMneRotrv)
        }else{
            if prototype.machine_code >> 6 & 0x1f != 0{
                return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
            }
            Some(MgMnemonic::MgMneSrlv)
        };

        return MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), None)
    }
    pub(super) fn jr(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::fixed_field(4, 0b1111111111);
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);


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

        if let MgMipsVersion::M32(MgMips32::MgR6) | MgMipsVersion::M64(MgMips64::MgR6) = self.version{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };

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
        prototype.mnemonic = if prototype.machine_code & 1 == 1{
            Some(MgMnemonic::MgMneBreak)
        }else{
            Some(MgMnemonic::MgMneSyscall)
        };
        prototype.operand[0] = Some(MgOpImmediate::new_imm_opreand(((prototype.machine_code >> 6) & 0xFFFFF) as u64));
        prototype.operand_num = 1;
        Ok(())
    }
    pub(super) fn sync(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::fixed_field(4, 0b111111111111111);
        let sa: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Imm);

        prototype.mnemonic = Some(MgMnemonic::MgMneSync);
        MgDisassembler::reg_format(self, prototype, None, None, Some(rd), Some(sa))
    }

    pub(super) fn mflo_mtlo(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        if let MgMipsVersion::M32(MgMips32::MgR6) | MgMipsVersion::M64(MgMips64::MgR6) = self.version{
            return self.dclz_dclo(prototype)
        }
        let rd: Option<FieldInfos>;
        let rt: Option<FieldInfos>;
        let rs: Option<FieldInfos>;
        let sa: Option<FieldInfos>;

        prototype.mnemonic = if prototype.machine_code & 1 == 1{
            rd = None;
            rt = None;
            sa = Some(FieldInfos::fixed_field(4, 0b111111111111111));
            rs = Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu));
            Some(MgMnemonic::MgMneMtlo)
        }else{
            rs = None;
            rd = Some(FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg));
            rt = Some(FieldInfos::fixed_field(4, 0b1111111111));
            sa = Some(FieldInfos::default_fixed_field());
            Some(MgMnemonic::MgMneMflo)
        };
        MgDisassembler::reg_format(self, prototype, rs, rt, rd, sa)
    }
    pub(super) fn mfhi_mthi(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        if let MgMipsVersion::M32(MgMips32::MgR6) | MgMipsVersion::M64(MgMips64::MgR6) = self.version{
            return self.clz_clo(prototype)
        }
        let rd: Option<FieldInfos>;
        let rt: Option<FieldInfos>;
        let rs: Option<FieldInfos>;
        let sa: Option<FieldInfos>;

        prototype.mnemonic = if prototype.machine_code & 1 == 1{
            rd = None;
            rt = None;
            sa = Some(FieldInfos::fixed_field(4, 0b111111111111111));
            rs = Some(FieldInfos::default_reg_field(0, MgCoprocessor::Cpu));
            Some(MgMnemonic::MgMneMthi)
        }else{
            rs = None;
            rd = Some(FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg));
            rt = Some(FieldInfos::fixed_field(4, 0b1111111111));
            sa = Some(FieldInfos::default_fixed_field());
            Some(MgMnemonic::MgMneMfhi)
        };
        MgDisassembler::reg_format(self, prototype, rs, rt, rd, sa)
    }
    pub(super) fn mult_multu_div_divu(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let mnemonics = [[Some(MgMnemonic::MgMneMult), Some(MgMnemonic::MgMneMultu)], [Some(MgMnemonic::MgMneDiv), Some(MgMnemonic::MgMneDivu)]];

        prototype.mnemonic = mnemonics[(prototype.machine_code >> 1 & 1) as usize][(prototype.machine_code & 1) as usize];
        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), None, Some(FieldInfos::fixed_field(4, 0b1111111111)))
    }
    pub(super) fn dsub_dsubu(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);

        let MgMipsVersion::M64(_) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };

        prototype.mnemonic = if prototype.machine_code & 1 == 1{
            Some(MgMnemonic::MgMneDsubu)
        }
        else{
            Some(MgMnemonic::MgMneDsub)
        };

        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn dadd_daddu(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);

        let MgMipsVersion::M64(_) = self.version else{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        };

        prototype.mnemonic = if prototype.machine_code & 1 == 1{
            Some(MgMnemonic::MgMneDaddu)
        }
        else{
            Some(MgMnemonic::MgMneDadd)
        };

        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn add_addu_sub_subu_and_or_xor_nor(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        prototype.mnemonic = [[[Some(MgMnemonic::MgMneAdd), Some(MgMnemonic::MgMneAddu)], [Some(MgMnemonic::MgMneSub), Some(MgMnemonic::MgMneSubu)]], [[Some(MgMnemonic::MgMneAnd), Some(MgMnemonic::MgMneOr)], [Some(MgMnemonic::MgMneXor), Some(MgMnemonic::MgMneNor)]]][(prototype.machine_code >> 2 & 1) as usize][(prototype.machine_code >> 1 & 1) as usize][(prototype.machine_code & 1) as usize];
        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn slt_sltu(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let mnemonics = [Some(MgMnemonic::MgMneSlt), Some(MgMnemonic::MgMneSltu)];

        prototype.is_conditional = true;
        prototype.mnemonic = mnemonics[(prototype.machine_code & 1) as usize];

        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn tge_tgeu_tlt_tltu(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let mnemonics = [[Some(MgMnemonic::MgMneTge), Some(MgMnemonic::MgMneTgeu)], [Some(MgMnemonic::MgMneTlt), Some(MgMnemonic::MgMneTltu)]];
        
        prototype.mnemonic = mnemonics[(prototype.machine_code >> 1 & 1) as usize][(prototype.machine_code & 1) as usize];
        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), None, Some(FieldInfos::imm_field(2, 0b1111111111)))
    }
    pub(super) fn seleqz_selnez(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);

        if let MgMipsVersion::M32(MgMips32::MgPreR6) | MgMipsVersion::M64(MgMips64::MgPreR6)= self.version{
            return Err(MgError::throw_error(MgErrorCode::VersionError, prototype.opcode, prototype.address, prototype.machine_code))
        }

        prototype.is_conditional = true;

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

        prototype.mnemonic = if prototype.machine_code & 1 == 0{
            Some(MgMnemonic::MgMneMadd)
        }else{
            Some(MgMnemonic::MgMneMaddu)
        };

        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), None, Some(FieldInfos::fixed_field(4, 0b1111111111)))
    }
    pub(super) fn mul(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);

        prototype.mnemonic = Some(MgMnemonic::MgMneMul);

        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn msub_msubu(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rs: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);

        prototype.mnemonic = if prototype.machine_code & 1 == 0{
            Some(MgMnemonic::MgMneMsub)
        }else{
            Some(MgMnemonic::MgMneMsubu)
        };

        MgDisassembler::reg_format(self, prototype, Some(rs), Some(rt), None, Some(FieldInfos::fixed_field(4, 0b1111111111)))
    }
    pub(super) fn dclz_dclo(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);

        prototype.mnemonic = if prototype.machine_code & 1 == 0{
            Some(MgMnemonic::MgMneDclz)
        }else{
            Some(MgMnemonic::MgMneDclo)
        };

        return if let MgMipsVersion::M32(MgMips32::MgR6) | MgMipsVersion::M64(MgMips64::MgR6) = self.version{
            if prototype.machine_code >> 6 & 0b11111 != 1{
                Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
            }else{
                MgDisassembler::reg_format(self, prototype, Some(rs), Some(FieldInfos::default_fixed_field()), Some(rd), None)
            }
        }else{
            MgDisassembler::reg_format(self, prototype, Some(rs), None, Some(rd), Some(FieldInfos::default_fixed_field()))
        }
    }
    pub(super) fn clz_clo(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);

        prototype.mnemonic = if prototype.machine_code & 1 == 0{
            Some(MgMnemonic::MgMneClz)
        }else{
            Some(MgMnemonic::MgMneClo)
        };

        return if let MgMipsVersion::M32(MgMips32::MgR6) | MgMipsVersion::M64(MgMips64::MgR6) = self.version{
            if prototype.machine_code >> 6 & 0b11111 != 1 || prototype.machine_code >> 16 & 0b11111 != 0{
                Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
            }else{
                MgDisassembler::reg_format(self, prototype, Some(rs), Some(FieldInfos::default_fixed_field()), Some(rd), None)
            }
        }else{
            MgDisassembler::reg_format(self, prototype, Some(rs), None, Some(rd), Some(FieldInfos::default_fixed_field()))
        }
    }
    pub(super) fn sdbbp(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        if let MgMipsVersion::M32(MgMips32::MgR6) | MgMipsVersion::M64(MgMips64::MgR6) = self.version{
            if prototype.opcode != 0{
                return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
            }
        }else if let MgMipsVersion::M32(MgMips32::MgPreR6) | MgMipsVersion::M64(MgMips64::MgPreR6) = self.version{
            if prototype.opcode != 0b011100{
                return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
            }
        };
        prototype.mnemonic = Some(MgMnemonic::MgMneSdbbp);
        prototype.operand[0] = Some(MgOpImmediate::new_imm_opreand(((prototype.machine_code >> 6) & 0xFFFFF) as u64));
        prototype.operand_num = 1;

        Ok(())
    }

    //Special3 They need some testing
    pub(super) fn ext(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = Some(MgMnemonic::MgMneExt);

        prototype.operand_num = 4;
        prototype.operand[0] = Some(MgOpRegister::new_reg_opreand((prototype.machine_code >> 16 & 0b11111) as u8, MgCoprocessor::Cpu));
        prototype.operand[1] = Some(MgOpRegister::new_reg_opreand((prototype.machine_code >> 21 & 0b11111) as u8, MgCoprocessor::Cpu));
        prototype.operand[2] = Some(MgOpImmediate::new_imm_opreand((prototype.machine_code >> 6 & 0b11111) as u64));
        prototype.operand[3] = Some(MgOpImmediate::new_imm_opreand((prototype.machine_code >> 11 & 0b11111) as u64));
        Ok(())
    }
    pub(super) fn ins(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = Some(MgMnemonic::MgMneIns);

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

        prototype.mnemonic = match prototype.machine_code >> 6 & 0b11111{
            0b00010 => Some(MgMnemonic::MgMneWsbh),
            0b10000 => Some(MgMnemonic::MgMneSeb),
            0b11000 => Some(MgMnemonic::MgMneSeh),
            _ => return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
        };
        
        MgDisassembler::reg_format(self, prototype, Some(FieldInfos::default_fixed_field()), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }
    pub(super) fn rdhwr(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let rt: FieldInfos = FieldInfos::reg_field(0, MgCoprocessor::Cpu, MgOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(1, MgCoprocessor::Cpu, MgOperandType::Reg);
        
        prototype.mnemonic = Some(MgMnemonic::MgMneRdhwr);
        MgDisassembler::reg_format(self, prototype, Some(FieldInfos::default_fixed_field()), Some(rt), Some(rd), Some(FieldInfos::default_fixed_field()))
    }

    //CP0
    pub(super) fn mov_cp0(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let mnemonics = [Some(MgMnemonic::MgMneMfc0), Some(MgMnemonic::MgMneMtc0)];
        if (prototype.machine_code >> 3 & 0b11111111) != 0{
            return Err(MgError::throw_error(MgErrorCode::FieldBadValue, prototype.opcode, prototype.address, prototype.machine_code))
        }

        prototype.mnemonic = mnemonics[(prototype.machine_code >> 23 & 1) as usize];
        prototype.operand_num = 3;

        prototype.operand[0] = Some(MgOpRegister::new_reg_opreand((prototype.machine_code >> 16 & 0b11111) as u8, MgCoprocessor::Cpu));
        prototype.operand[1] = Some(MgOpRegister::new_reg_opreand((prototype.machine_code >> 11 & 0b11111) as u8, MgCoprocessor::Cpu));
        prototype.operand[2] = Some(MgOpImmediate::new_imm_opreand((prototype.machine_code & 7) as u64));
        Ok(())
    }
    pub(super) fn gpr_shadowset(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        let mnemonics = [Some(MgMnemonic::MgMneRdpgpr), Some(MgMnemonic::MgMneWrpgpr)];

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

        prototype.mnemonic = mnemonics[(prototype.machine_code >> 3 & 0b111) as usize][(prototype.machine_code & 0b111) as usize];
        Ok(())
    }

    //pcrel
    pub (super) fn addiupc(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = Some(MgMnemonic::MgMneAddiupc);
        Ok(())
    }
    pub (super) fn lwpc(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = Some(MgMnemonic::MgMneLwpc);
        Ok(())
    }
    pub (super) fn lwupc(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = Some(MgMnemonic::MgMneLwupc);
        Ok(())
    }
    pub (super) fn aluipc(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = Some(MgMnemonic::MgMneAluipc);
        Ok(())
    }
    pub (super) fn ldpc(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{      //The manual didn't have an entry 
        prototype.mnemonic = Some(MgMnemonic::MgMneLdpc);                                                   //for that instruction but it was mentionned in the Table A.13
        Ok(())
    }
    pub (super) fn auipc(&self, prototype: &mut MgInstructionPrototype) -> Result<(), MgError>{
        prototype.mnemonic = Some(MgMnemonic::MgMneAuipc);
        Ok(())
    }
}