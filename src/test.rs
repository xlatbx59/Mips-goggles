//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use crate::*;
use crate::operands::*;
use crate::instruction::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

///The purpose of this function is to test if we take correctly the immediate
fn imm_limit_reached(disass: &MgDisassembler, mne: MgMnemonic, mut machine_code: u32, bit_pos: u8, mask: u32, operand_index: usize) -> bool{
    //The immediate doesn't take enough bits to make the immediate
    machine_code |= (mask << bit_pos) as u32;
    match disass.disassemble(machine_code, 0x00400000){
        Ok(inst) => {
            assert_eq!(mne, inst.get_mnemonic());
            let Some(MgOperand::MgOpImmediate(imm)) = inst.get_operand(operand_index) else{
                return false
            };
            if imm.get_value() as u32 != mask{
                return false
            };
        },
        Err(_) =>(),
    };

    //The immediate takes too much bits to make the immediate
    machine_code += (1 << bit_pos) as u32;
    return match disass.disassemble(machine_code, 0x00400000){
        Ok(inst) => {
            if inst.get_mnemonic() != mne{
                return true
            }
            let Some(MgOperand::MgOpImmediate(imm)) = inst.get_operand(operand_index) else{
                return true
            };
            if imm.get_value() != 0{
                return false
            };
            true
        },
        Err(_) =>true,
    }
}

#[test]
fn test_lsa_dlsa(){
    let machine_code = [0x00fa18c5, 0x00fa18d5];

    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    assert_eq!(true, decoder.disassemble(machine_code[0], 0).is_err());
    assert_eq!(true, decoder.disassemble(machine_code[1], 0).is_err());

    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);
    assert_eq!(true, decoder.disassemble(machine_code[1], 0).is_err());
    decoder.version = MgMipsVersion::M32(MgMips32::MgR6);
    assert_eq!(true, decoder.disassemble(machine_code[1], 0).is_err());

    decoder.version = MgMipsVersion::M64(MgMips64::MgR6);
    let inst = decoder.disassemble(machine_code[0], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneLsa, inst.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneDlsa, inst1.get_mnemonic());

    assert_eq!(MG_MNE_LSA, inst.get_mnemonic_str());
    assert_eq!(MG_MNE_DLSA, inst1.get_mnemonic_str());

    assert_eq!(MgInstructionCategory::AddressComputation, inst.get_category());
    assert_eq!(MgInstructionCategory::AddressComputation, inst1.get_category());

    assert_eq!(4, inst.get_operand_num());
    assert_eq!(4, inst1.get_operand_num());

    imm_limit_reached(&decoder, MgMnemonic::MgMneLsa, machine_code[0], 6, 3, 3);
    imm_limit_reached(&decoder, MgMnemonic::MgMneDlsa, machine_code[1], 6, 3, 3);

    let Some(MgOperand::MgOpRegister(_)) = inst.get_operand(0) else {
        panic!();
    };
    let Some(MgOperand::MgOpRegister(_)) = inst.get_operand(1) else {
        panic!();
    };
    let Some(MgOperand::MgOpRegister(_)) = inst.get_operand(2) else {
        panic!();
    };
    let Some(MgOperand::MgOpRegister(_)) = inst1.get_operand(0) else {
        panic!();
    };
    let Some(MgOperand::MgOpRegister(_)) = inst1.get_operand(1) else {
        panic!();
    };
    let Some(MgOperand::MgOpRegister(_)) = inst1.get_operand(2) else {
        panic!();
    };
}
#[test]
fn test_ddiv_ddivu(){
    let machine_code = [0x0044001e, 0x000A001f];

    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let ddiv = decoder.disassemble(machine_code[0], 0).unwrap();
    let ddivu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(ddiv.get_category(), MgInstructionCategory::Arithmetic);

    assert_eq!(ddiv.get_operand_num(), 2);
    assert_eq!(ddivu.get_operand_num(), 2);

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneDdiv, machine_code[0], 0, 0x1fffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneDdivu, machine_code[1], 0, 0xffff, 1));

    assert_eq!(ddiv.get_mnemonic(), MgMnemonic::MgMneDdiv);
    assert_eq!(ddivu.get_mnemonic(), MgMnemonic::MgMneDdivu);
    assert_eq!(ddiv.get_mnemonic_str(), MG_MNE_DDIV);
    assert_eq!(ddivu.get_mnemonic_str(), MG_MNE_DDIVU);

    decoder.version = MgMipsVersion::M64(MgMips64::MgR6);
    assert_eq!(decoder.disassemble(machine_code[0], 0).is_err(), true);
    assert_eq!(decoder.disassemble(machine_code[1], 0).is_err(), true);
}

#[test]
fn test_pop76(){
    let machine_code = [0xf934794A, 0xf80A794A];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let bnezc = decoder.disassemble(machine_code[0], 0).unwrap();
    let jialc = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(bnezc.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(bnezc.is_region(), false);
    assert_eq!(bnezc.is_relative(), true);
    assert_eq!(bnezc.is_conditional(), true);

    assert_eq!(jialc.is_region(), false);
    assert_eq!(jialc.is_relative(), false);
    assert_eq!(jialc.is_conditional(), false);

    assert_eq!(bnezc.get_mnemonic(), MgMnemonic::MgMneBnezc);
    assert_eq!(jialc.get_mnemonic(), MgMnemonic::MgMneJialc);

    assert_eq!(mg_get_mnemonic(bnezc.get_mnemonic()), MG_MNE_BNEZC);
    assert_eq!(mg_get_mnemonic(jialc.get_mnemonic()), MG_MNE_JIALC);

    assert_eq!(bnezc.get_operand_num(), 2);
    assert_eq!(jialc.get_operand_num(), 2);

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBnezc, machine_code[0], 0, 0x1fffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneJialc, machine_code[1], 0, 0xffff, 1));

    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);

    let beqzc = decoder.disassemble(machine_code[0], 0).unwrap();
    let jic = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_ne!(beqzc.get_mnemonic(), MgMnemonic::MgMneBnezc);
    assert_ne!(jic.get_mnemonic(), MgMnemonic::MgMneJialc);

    assert_ne!(mg_get_mnemonic(beqzc.get_mnemonic()), MG_MNE_BNEZC);
    assert_ne!(mg_get_mnemonic(jic.get_mnemonic()), MG_MNE_JIALC);

    assert_eq!(beqzc.get_operand_num(), 3);
    assert_eq!(jic.get_operand_num(), 3);
}
#[test]
fn test_pop66(){
    let machine_code = [0xd9f4794A, 0xd80A794A];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let beqzc = decoder.disassemble(machine_code[0], 0).unwrap();
    let jic = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(beqzc.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(beqzc.is_region(), false);
    assert_eq!(beqzc.is_relative(), true);
    assert_eq!(beqzc.is_conditional(), true);

    assert_eq!(jic.is_region(), false);
    assert_eq!(jic.is_relative(), false);
    assert_eq!(jic.is_conditional(), false);

    assert_eq!(beqzc.get_mnemonic(), MgMnemonic::MgMneBeqzc);
    assert_eq!(jic.get_mnemonic(), MgMnemonic::MgMneJic);

    assert_eq!(mg_get_mnemonic(beqzc.get_mnemonic()), MG_MNE_BEQZC);
    assert_eq!(mg_get_mnemonic(jic.get_mnemonic()), MG_MNE_JIC);

    assert_eq!(beqzc.get_operand_num(), 2);
    assert_eq!(jic.get_operand_num(), 2);

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBeqzc, machine_code[0], 0, 0x1fffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneJic, machine_code[1], 0, 0xffff, 1));

    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);

    let beqzc = decoder.disassemble(machine_code[0], 0).unwrap();
    let jic = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_ne!(beqzc.get_mnemonic(), MgMnemonic::MgMneBeqzc);
    assert_ne!(jic.get_mnemonic(), MgMnemonic::MgMneJic);

    assert_ne!(mg_get_mnemonic(beqzc.get_mnemonic()), MG_MNE_BEQZC);
    assert_ne!(mg_get_mnemonic(jic.get_mnemonic()), MG_MNE_JIC);

    assert_eq!(beqzc.get_operand_num(), 3);
    assert_eq!(jic.get_operand_num(), 3);
}
#[test]
fn test_pop30(){
    let machine_code = [0x600A794A, 0x6234794A, 0x6000794A];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let bnezalc = decoder.disassemble(machine_code[0], 0).unwrap();
    let bnec = decoder.disassemble(machine_code[1], 0).unwrap();
    let bnvc = decoder.disassemble(machine_code[2], 0).unwrap();

    assert_eq!(bnezalc.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(bnec.is_region(), false);
    assert_eq!(bnec.is_relative(), true);
    assert_eq!(bnec.is_conditional(), true);

    assert_eq!(bnezalc.get_mnemonic(), MgMnemonic::MgMneBnezalc);
    assert_eq!(bnec.get_mnemonic(), MgMnemonic::MgMneBnec);
    assert_eq!(bnvc.get_mnemonic(), MgMnemonic::MgMneBnvc);

    assert_eq!(mg_get_mnemonic(bnezalc.get_mnemonic()), MG_MNE_BNEZALC);
    assert_eq!(mg_get_mnemonic(bnec.get_mnemonic()), MG_MNE_BNEC);
    assert_eq!(mg_get_mnemonic(bnvc.get_mnemonic()), MG_MNE_BNVC);

    assert_eq!(bnezalc.get_operand_num(), 2);
    assert_eq!(bnec.get_operand_num(), 3);
    assert_eq!(bnvc.get_operand_num(), 3);

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBnezalc, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBnec, machine_code[1], 0, 0xffff, 2));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBnvc, machine_code[2], 0, 0xffff, 2));

    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);

    let addi = decoder.disassemble(machine_code[0], 0);
    let addi1 = decoder.disassemble(machine_code[1], 0);
    let addi2 = decoder.disassemble(machine_code[2], 0);

    assert_eq!(addi.is_err(), true);
    assert_eq!(addi1.is_err(), true);
    assert_eq!(addi2.is_err(), true);
}
#[test]
fn test_pop10(){
    let machine_code = [0x200A794A, 0x2234794A, 0x2000794A];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let beqzalc = decoder.disassemble(machine_code[0], 0).unwrap();
    let beqc = decoder.disassemble(machine_code[1], 0).unwrap();
    let bovc = decoder.disassemble(machine_code[2], 0).unwrap();

    assert_eq!(beqzalc.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(beqc.is_region(), false);
    assert_eq!(beqc.is_relative(), true);
    assert_eq!(beqc.is_conditional(), true);

    assert_eq!(beqzalc.get_mnemonic(), MgMnemonic::MgMneBeqzalc);
    assert_eq!(beqc.get_mnemonic(), MgMnemonic::MgMneBeqc);
    assert_eq!(bovc.get_mnemonic(), MgMnemonic::MgMneBovc);

    assert_eq!(mg_get_mnemonic(beqzalc.get_mnemonic()), MG_MNE_BEQZALC);
    assert_eq!(mg_get_mnemonic(beqc.get_mnemonic()), MG_MNE_BEQC);
    assert_eq!(mg_get_mnemonic(bovc.get_mnemonic()), MG_MNE_BOVC);

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBeqzalc, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBeqc, machine_code[1], 0, 0xffff, 2));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBovc, machine_code[2], 0, 0xffff, 2));

    assert_eq!(beqzalc.get_operand_num(), 2);
    assert_eq!(beqc.get_operand_num(), 3);
    assert_eq!(bovc.get_operand_num(), 3);

    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);

    let addi = decoder.disassemble(machine_code[0], 0).unwrap();
    let addi1 = decoder.disassemble(machine_code[1], 0).unwrap();
    let addi2 = decoder.disassemble(machine_code[2], 0).unwrap();

    assert_eq!(addi.get_mnemonic(), MgMnemonic::MgMneAddi);
    assert_eq!(addi1.get_mnemonic(), MgMnemonic::MgMneAddi);
    assert_eq!(addi2.get_mnemonic(), MgMnemonic::MgMneAddi);

    assert_eq!(addi2.get_operand_num(), 3);
    assert_eq!(addi2.get_mnemonic(), MgMnemonic::MgMneAddi);
    assert_eq!(mg_get_mnemonic(addi2.get_mnemonic()), MG_MNE_ADDI);
}
#[test]
fn test_bgtzl_pop27(){
    let machine_code = [0x5C01794A, 0x5E10794A, 0x5D55794A, 0x5E20794A];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[1], 0).unwrap();
    let inst2 = decoder.disassemble(machine_code[2], 0).unwrap();
    let inst3 = decoder.disassemble(machine_code[3], 0);

    assert_eq!(inst0.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(inst1.is_region(), false);
    assert_eq!(inst1.is_relative(), true);
    assert_eq!(inst1.is_conditional(), true);

    assert_eq!(inst3.is_err(), true);

    assert_eq!(inst0.get_mnemonic(), MgMnemonic::MgMneBgtzc);
    assert_eq!(inst1.get_mnemonic(), MgMnemonic::MgMneBltzc);
    assert_eq!(inst2.get_mnemonic(), MgMnemonic::MgMneBltc);

    assert_eq!(mg_get_mnemonic(inst0.get_mnemonic()), MG_MNE_BGTZC);
    assert_eq!(mg_get_mnemonic(inst1.get_mnemonic()), MG_MNE_BLTZC);
    assert_eq!(mg_get_mnemonic(inst2.get_mnemonic()), MG_MNE_BLTC);

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBgtzc, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBltzc, machine_code[1], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBltc, machine_code[2], 0, 0xffff, 2));

    assert_eq!(inst0.get_operand_num(), 2);
    assert_eq!(inst1.get_operand_num(), 2);
    assert_eq!(inst2.get_operand_num(), 3);

    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);

    let inst0 = decoder.disassemble(machine_code[0], 0);
    let inst1 = decoder.disassemble(machine_code[1], 0);
    let inst2 = decoder.disassemble(machine_code[2], 0);
    let inst3 = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(inst0.is_err(), true);
    assert_eq!(inst1.is_err(), true);
    assert_eq!(inst2.is_err(), true);

    assert_eq!(inst3.get_operand_num(), 2);

    assert_eq!(inst3.get_mnemonic(), MgMnemonic::MgMneBgtzl);
    assert_eq!(mg_get_mnemonic(inst3.get_mnemonic()), MG_MNE_BGTZL);
}
#[test]
fn test_blez_pop26(){
    let machine_code = [0x5801794A, 0x5A10794A, 0x5955794A, 0x5A20794A];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[1], 0).unwrap();
    let inst2 = decoder.disassemble(machine_code[2], 0).unwrap();
    let inst3 = decoder.disassemble(machine_code[3], 0);

    assert_eq!(inst0.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(inst1.is_region(), false);
    assert_eq!(inst1.is_relative(), true);
    assert_eq!(inst1.is_conditional(), true);

    assert_eq!(inst3.is_err(), true);

    assert_eq!(inst0.get_mnemonic(), MgMnemonic::MgMneBlezc);
    assert_eq!(inst1.get_mnemonic(), MgMnemonic::MgMneBgezc);
    assert_eq!(inst2.get_mnemonic(), MgMnemonic::MgMneBgec);

    assert_eq!(mg_get_mnemonic(inst0.get_mnemonic()), MG_MNE_BLEZC);
    assert_eq!(mg_get_mnemonic(inst1.get_mnemonic()), MG_MNE_BGEZC);
    assert_eq!(mg_get_mnemonic(inst2.get_mnemonic()), MG_MNE_BGEC);

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBlezc, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBgezc, machine_code[1], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBgec, machine_code[2], 0, 0xffff, 2));

    assert_eq!(inst0.get_operand_num(), 2);
    assert_eq!(inst1.get_operand_num(), 2);
    assert_eq!(inst2.get_operand_num(), 3);

    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);

    let inst0 = decoder.disassemble(machine_code[0], 0);
    let inst1 = decoder.disassemble(machine_code[1], 0);
    let inst2 = decoder.disassemble(machine_code[2], 0);
    let inst3 = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(inst0.is_err(), true);
    assert_eq!(inst1.is_err(), true);
    assert_eq!(inst2.is_err(), true);

    assert_eq!(inst3.get_operand_num(), 2);

    assert_eq!(inst3.get_mnemonic(), MgMnemonic::MgMneBlezl);
    assert_eq!(mg_get_mnemonic(inst3.get_mnemonic()), MG_MNE_BLEZL);
}
#[test]
fn test_pop07(){
    let machine_code: [u32; 4] = [0x1c30FFFF, 0x1c0a0050, 0x1c420050, 0x1c00C011];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[1], 0).unwrap();
    let inst2 = decoder.disassemble(machine_code[2], 0).unwrap();
    let mut inst3 = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(inst0.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(inst1.is_region(), false);
    assert_eq!(inst1.is_relative(), true);
    assert_eq!(inst1.is_conditional(), true);

    assert_eq!(inst0.get_mnemonic(), MgMnemonic::MgMneBltuc);
    assert_eq!(inst1.get_mnemonic(), MgMnemonic::MgMneBltzalc);
    assert_eq!(inst2.get_mnemonic(), MgMnemonic::MgMneBgtzalc);
    assert_eq!(inst3.get_mnemonic(), MgMnemonic::MgMneBgtz);

    assert_eq!(mg_get_mnemonic(inst0.get_mnemonic()), MG_MNE_BLTUC);
    assert_eq!(mg_get_mnemonic(inst1.get_mnemonic()), MG_MNE_BLTZALC);
    assert_eq!(mg_get_mnemonic(inst2.get_mnemonic()), MG_MNE_BGTZALC);
    assert_eq!(mg_get_mnemonic(inst3.get_mnemonic()), MG_MNE_BGTZ);

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBltuc, machine_code[0], 0, 0xffff, 2));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBltzalc, machine_code[1], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBgtzalc, machine_code[2], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBgtz, machine_code[3], 0, 0xffff, 1));

    assert_eq!(inst0.get_operand_num(), 3);
    match (inst0.get_operand(0), inst0.get_operand(1), inst0.get_operand(2)){
        (Some(MgOperand::MgOpRegister(r1)),Some(MgOperand::MgOpRegister(r2)), Some(MgOperand::MgOpImmediate(_))) => assert_ne!(r1, r2),
        _ => panic!(),
    }
    assert_eq!(inst1.get_operand_num(), 2);
    match (inst1.get_operand(0), inst1.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }
    assert_eq!(inst2.get_operand_num(), 2);
    match (inst2.get_operand(0), inst2.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }
    assert_eq!(inst3.get_operand_num(), 2);
    match (inst3.get_operand(0), inst3.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }

    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);
    let inst0 = decoder.disassemble(machine_code[0], 0);
    let inst1 = decoder.disassemble(machine_code[1], 0);
    let inst2 = decoder.disassemble(machine_code[2], 0);
    inst3 = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(inst0.is_err(), true);   //Field value wrong
    assert_eq!(inst1.is_err(), true);   //Field value wrong
    assert_eq!(inst2.is_err(), true);   //Field value wrong

    assert_eq!(inst3.get_mnemonic(), MgMnemonic::MgMneBgtz);
    assert_eq!(mg_get_mnemonic(inst3.get_mnemonic()), MG_MNE_BGTZ);
    assert_eq!(inst3.get_operand_num(), 2);
    match (inst3.get_operand(0), inst3.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }
}
#[test]
fn test_blez_pop06(){
    let machine_code: [u32; 4] = [0x1830FFFF, 0x180a0050, 0x18420050, 0x1800C011];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[1], 0).unwrap();
    let inst2 = decoder.disassemble(machine_code[2], 0).unwrap();
    let mut inst3 = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(inst0.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(inst1.is_region(), false);
    assert_eq!(inst1.is_relative(), true);
    assert_eq!(inst1.is_conditional(), true);

    assert_eq!(inst0.get_mnemonic(), MgMnemonic::MgMneBgeuc);
    assert_eq!(inst1.get_mnemonic(), MgMnemonic::MgMneBlezalc);
    assert_eq!(inst2.get_mnemonic(), MgMnemonic::MgMneBgezalc);
    assert_eq!(inst3.get_mnemonic(), MgMnemonic::MgMneBlez);

    assert_eq!(mg_get_mnemonic(inst0.get_mnemonic()), MG_MNE_BGEUC);
    assert_eq!(mg_get_mnemonic(inst1.get_mnemonic()), MG_MNE_BLEZALC);
    assert_eq!(mg_get_mnemonic(inst2.get_mnemonic()), MG_MNE_BGEZALC);
    assert_eq!(mg_get_mnemonic(inst3.get_mnemonic()), MG_MNE_BLEZ);

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBgeuc, machine_code[0], 0, 0xffff, 2));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBlezalc, machine_code[1], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBgezalc, machine_code[2], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBlez, machine_code[3], 0, 0xffff, 1));

    assert_eq!(inst0.get_operand_num(), 3);
    match (inst0.get_operand(0), inst0.get_operand(1), inst0.get_operand(2)){
        (Some(MgOperand::MgOpRegister(r1)),Some(MgOperand::MgOpRegister(r2)), Some(MgOperand::MgOpImmediate(_))) => assert_ne!(r1, r2),
        _ => panic!(),
    }
    assert_eq!(inst1.get_operand_num(), 2);
    match (inst1.get_operand(0), inst1.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }
    assert_eq!(inst2.get_operand_num(), 2);
    match (inst2.get_operand(0), inst2.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }
    assert_eq!(inst3.get_operand_num(), 2);
    match (inst3.get_operand(0), inst3.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }

    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);
    let inst0 = decoder.disassemble(machine_code[0], 0);
    let inst1 = decoder.disassemble(machine_code[1], 0);
    let inst2 = decoder.disassemble(machine_code[2], 0);
    inst3 = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(inst0.is_err(), true);   //Field value wrong
    assert_eq!(inst1.is_err(), true);   //Field value wrong
    assert_eq!(inst2.is_err(), true);   //Field value wrong

    assert_eq!(inst3.get_mnemonic(), MgMnemonic::MgMneBlez);
    assert_eq!(mg_get_mnemonic(inst3.get_mnemonic()), MG_MNE_BLEZ);
    assert_eq!(inst3.get_operand_num(), 2);
    match (inst3.get_operand(0), inst3.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }
}

#[test]
fn test_lwr_swr_lwl_swl() {
    let machine_code: [u32; 4] = [0x88450050, 0xA8450050, 0x98450050, 0xB8450050];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    
    let inst = decoder.disassemble(machine_code[0], 0).unwrap();

    assert_eq!(inst.get_operand_num(), 3);

    assert_eq!(inst.get_mnemonic(), MgMnemonic::MgMneLwl);
    assert_eq!(decoder.disassemble(machine_code[1], 0).unwrap().get_mnemonic(), MgMnemonic::MgMneSwl);
    assert_eq!(decoder.disassemble(machine_code[2], 0).unwrap().get_mnemonic(), MgMnemonic::MgMneLwr);
    assert_eq!(decoder.disassemble(machine_code[3], 0).unwrap().get_mnemonic(), MgMnemonic::MgMneSwr);

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLwl, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSwl, machine_code[1], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLwr, machine_code[2], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSwr, machine_code[3], 0, 0xffff, 1));
    
    decoder.version = MgMipsVersion::M32(MgMips32::MgR6);
    assert_eq!(decoder.disassemble(machine_code[0], 0).is_err(), true);
    assert_eq!(decoder.disassemble(machine_code[1], 0).is_err(), true);
    assert_eq!(decoder.disassemble(machine_code[2], 0).is_err(), true);
    assert_eq!(decoder.disassemble(machine_code[3], 0).is_err(), true);
}

#[test]
fn test_bc_balc(){
    let machine_code: [u32; 2] = [0xC8020050, 0xE8020050];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(inst0.get_mnemonic(), MgMnemonic::MgMneBc);
    assert_eq!(inst1.get_mnemonic(), MgMnemonic::MgMneBalc);

    assert_eq!(mg_get_mnemonic(inst0.get_mnemonic()), MG_MNE_BC);
    assert_eq!(mg_get_mnemonic(inst1.get_mnemonic()), MG_MNE_BALC);

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBc, machine_code[0], 0, 0x3ffffff, 0));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBalc, machine_code[1], 0, 0x3ffffff, 0));

    assert_eq!(inst0.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(inst0.is_conditional(), true);
    assert_eq!(inst0.get_operand_num(), 1);
    assert_ne!(inst0.is_region(), true);

    assert_eq!(inst1.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(inst1.is_conditional(), true);
    assert_eq!(inst1.get_operand_num(), 1);
    assert_ne!(inst1.is_region(), true);

    //Load and store instructions from cop2
    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);
    let inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_ne!(inst0.get_category(), MgInstructionCategory::BranchJump);
    assert_ne!(inst1.get_category(), MgInstructionCategory::BranchJump);
    assert_ne!(inst0.is_conditional(), true);
    assert_ne!(inst0.is_region(), true);
    assert_eq!(inst0.get_mnemonic(), MgMnemonic::MgMneLwc2);
    assert_eq!(inst1.get_mnemonic(), MgMnemonic::MgMneSwc2);
}
#[test]
fn test_sc_ll(){
    let machine_code: [u32; 4] = [0xE0A2FFFF, 0xC0A2FFFF, 0x7fffffa6, 0x7fffffb6];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(inst0.get_mnemonic(), MgMnemonic::MgMneSc);
    assert_eq!(inst1.get_mnemonic(), MgMnemonic::MgMneLl);

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneSc, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneLl, machine_code[1], 0, 0xffff, 1));

    assert_eq!(decoder.disassemble(machine_code[2], 0).is_err(), true);
    assert_eq!(decoder.disassemble(machine_code[3], 0).is_err(), true);

    assert_eq!(inst0.is_conditional(), true);
    assert_eq!(mg_get_mnemonic(inst0.get_mnemonic()), MG_MNE_SC);
    assert_eq!(inst0.get_operand_num(), 3);

    assert_eq!(inst1.is_conditional(), true);
    assert_eq!(mg_get_mnemonic(inst1.get_mnemonic()), MG_MNE_LL);
    assert_eq!(inst1.get_operand_num(), 3);
    match inst1.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match inst1.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match inst1.get_operand(1){
        Some(MgOperand::MgOpImmediate(i)) => assert!(i.get_value() <= 0xffff),
        _ => panic!(),
    }

    decoder.version = MgMipsVersion::M32(MgMips32::MgR6);

    let inst0 = decoder.disassemble(machine_code[2], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(decoder.disassemble(machine_code[0], 0).is_err(), true);
    assert_eq!(decoder.disassemble(machine_code[1], 0).is_err(), true);

    assert_eq!(inst0.is_conditional(), true);
    assert_eq!(inst0.get_mnemonic(), MgMnemonic::MgMneSc);
    assert_eq!(mg_get_mnemonic(inst0.get_mnemonic()), MG_MNE_SC);
    assert_eq!(inst0.get_operand_num(), 3);

    assert_eq!(inst1.is_conditional(), true);
    assert_eq!(inst1.get_mnemonic(), MgMnemonic::MgMneLl);
    assert_eq!(mg_get_mnemonic(inst1.get_mnemonic()), MG_MNE_LL);
    assert_eq!(inst1.get_operand_num(), 3);
    match inst1.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match inst1.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match inst1.get_operand(1){
        Some(MgOperand::MgOpImmediate(i)) => assert!(i.get_value() <= 0x1ff),
        _ => panic!(),
    }
}
#[test]
fn test_load_store_cp2(){
    let machine_code: [u32; 8] = [0xC8020050, 0xE8020050, 0xD8020050, 0xF8020050, 0x49C00000,0x49400000, 0x49E00000,0x49600000];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let mut inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let mut inst1 = decoder.disassemble(machine_code[1], 0).unwrap();
    let mut inst2 = decoder.disassemble(machine_code[2], 0).unwrap();
    let mut inst3 = decoder.disassemble(machine_code[3], 0).unwrap();

    let inst4 = decoder.disassemble(machine_code[4], 0);
    let inst5 = decoder.disassemble(machine_code[5], 0);
    let inst6 = decoder.disassemble(machine_code[6], 0);
    let inst7 = decoder.disassemble(machine_code[7], 0);

    assert_eq!(inst0.get_mnemonic(), MgMnemonic::MgMneLwc2);
    assert_eq!(inst1.get_mnemonic(), MgMnemonic::MgMneSwc2);
    assert_eq!(inst2.get_mnemonic(), MgMnemonic::MgMneLdc2);
    assert_eq!(inst3.get_mnemonic(), MgMnemonic::MgMneSdc2);

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLwc2, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSwc2, machine_code[1], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLdc2, machine_code[2], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSdc2, machine_code[3], 0, 0xffff, 1));

    //Will fail
    assert_eq!(inst4.is_err(), true);    //Ldc2
    assert_eq!(inst5.is_err(), true);    //Lwc2
    assert_eq!(inst6.is_err(), true);    //Sdc2
    assert_eq!(inst7.is_err(), true);    //Swc2

    //The same machine code is used by other instructions in release6
    decoder.version = MgMipsVersion::M32(MgMips32::MgR6);
    inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    inst1 = decoder.disassemble(machine_code[1], 0).unwrap();
    inst2 = decoder.disassemble(machine_code[2], 0).unwrap();
    inst3 = decoder.disassemble(machine_code[3], 0).unwrap();

    let inst4 = decoder.disassemble(machine_code[4], 0).unwrap();
    let inst5 = decoder.disassemble(machine_code[5], 0).unwrap();
    let inst6 = decoder.disassemble(machine_code[6], 0).unwrap();
    let inst7 = decoder.disassemble(machine_code[7], 0).unwrap();

    assert_eq!(inst0.get_mnemonic(), MgMnemonic::MgMneBc);
    assert_eq!(inst1.get_mnemonic(), MgMnemonic::MgMneBalc);
    assert_eq!(inst2.get_mnemonic(), MgMnemonic::MgMneJic);
    assert_eq!(inst3.get_mnemonic(), MgMnemonic::MgMneJialc);

    assert_eq!(inst4.get_mnemonic(), MgMnemonic::MgMneLdc2);
    assert_eq!(inst5.get_mnemonic(), MgMnemonic::MgMneLwc2);
    assert_eq!(inst6.get_mnemonic(), MgMnemonic::MgMneSdc2);
    assert_eq!(inst7.get_mnemonic(), MgMnemonic::MgMneSwc2);
}

#[test]
fn test_tne_teq() {
    let machine_code: [u32; 2] = [0x00460036, 0x00400034];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    
    //No problem
    assert_eq!(decoder.disassemble(machine_code[0], 0).unwrap().get_mnemonic(), MgMnemonic::MgMneTne);
    assert_eq!(decoder.disassemble(machine_code[1], 0).unwrap().get_mnemonic(), MgMnemonic::MgMneTeq);

    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneTne, machine_code[0], 6, 0x3ff, 2));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneTeq, machine_code[1], 6, 0x3ff, 2));

    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);
    assert_eq!(decoder.disassemble(machine_code[0], 0).unwrap().get_mnemonic(), MgMnemonic::MgMneTne);
    assert_eq!(decoder.disassemble(machine_code[1], 0).unwrap().get_mnemonic(), MgMnemonic::MgMneTeq);
}

#[test]
fn test_seleqz_selnez() {
    let machine_code: [u32; 4] = [0b110101, 0b110111, 0b1110101,0b10110101];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    
    //No problem
    assert_eq!(decoder.disassemble(machine_code[0], 0).unwrap().get_mnemonic(), MgMnemonic::MgMneSeleqz);
    assert_eq!(decoder.disassemble(machine_code[1], 0).unwrap().get_mnemonic(), MgMnemonic::MgMneSelnez);

    assert_eq!(decoder.disassemble(machine_code[0], 0).unwrap().is_conditional(), true);
    assert_eq!(decoder.disassemble(machine_code[1], 0).unwrap().is_conditional(), true);
    
    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);
    assert_eq!(decoder.disassemble(machine_code[0], 0).is_err(), true);
    assert_eq!(decoder.disassemble(machine_code[1], 0).is_err(), true);

    //Testing if Sa field not set to 0 will fail
    assert_eq!(decoder.disassemble(machine_code[2], 0).is_err(), true);
    assert_eq!(decoder.disassemble(machine_code[3], 0).is_err(), true);
}