//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_pop76(){
    let machine_code = [0xf934794A, 0xf80A794A];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let bnezc = decoder.disassemble(machine_code[0], 0).unwrap();
    let jialc = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(bnezc.get_mnemonic() == MgMnemonic::MgMneBnezc);
    assert!(jialc.get_mnemonic() == MgMnemonic::MgMneJialc);
    assert!(mg_get_mnemonic(bnezc.get_mnemonic()) == MG_MNE_BNEZC);
    assert!(mg_get_mnemonic(jialc.get_mnemonic()) == MG_MNE_JIALC);
    assert!("bnezc" == MG_MNE_BNEZC);
    assert!("jialc" == MG_MNE_JIALC);

    assert!(check_operands(&bnezc, 2));
    assert!(check_operands(&jialc, 2));

    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneJialc, 21));
    assert!(check_field_zero_assert(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneBnezc, 21));

    assert!(version_test(machine_code[0], MgMnemonic::MgMneBnezc, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneJialc, false, true, false, true));

    assert!(!bnezc.is_region());
    assert!(bnezc.is_relative());
    assert!(bnezc.is_conditional());

    assert!(!jialc.is_region());
    assert!(!jialc.is_relative());
    assert!(!jialc.is_conditional());

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBnezc, machine_code[0], 0, 0x1fffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneJialc, machine_code[1], 0, 0xffff, 1));
}
#[test]
fn test_pop66(){
    let machine_code = [0xd9f4794A, 0xd80A794A];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let beqzc = decoder.disassemble(machine_code[0], 0).unwrap();
    let jic = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(beqzc.get_mnemonic() == MgMnemonic::MgMneBeqzc);
    assert!(jic.get_mnemonic() == MgMnemonic::MgMneJic);
    assert!(mg_get_mnemonic(beqzc.get_mnemonic()) == MG_MNE_BEQZC);
    assert!(mg_get_mnemonic(jic.get_mnemonic()) == MG_MNE_JIC);
    assert!("beqzc" == MG_MNE_BEQZC);
    assert!("jic" == MG_MNE_JIC);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneBeqzc, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneJic, false, true, false, true));

    assert!(!beqzc.is_region());
    assert!(beqzc.is_relative());
    assert!(beqzc.is_conditional());

    assert!(!jic.is_region());
    assert!(!jic.is_relative());
    assert!(!jic.is_conditional());

    assert!(check_operands(&beqzc, 2));
    assert!(check_operands(&jic, 2));

    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneJic, 21));
    assert!(check_field_zero_assert(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneBeqzc, 21));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBeqzc, machine_code[0], 0, 0x1fffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneJic, machine_code[1], 0, 0xffff, 1));
}
#[test]
fn test_pop30(){
    let machine_code = [0x600A794A, 0x6234794A, 0x6000794A];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let bnezalc = decoder.disassemble(machine_code[0], 0).unwrap();
    let bnec = decoder.disassemble(machine_code[1], 0).unwrap();
    let bnvc = decoder.disassemble(machine_code[2], 0).unwrap();

    assert!(bnezalc.get_mnemonic() == MgMnemonic::MgMneBnezalc);
    assert!(bnec.get_mnemonic() == MgMnemonic::MgMneBnec);
    assert!(bnvc.get_mnemonic() == MgMnemonic::MgMneBnvc);
    assert!(mg_get_mnemonic(bnezalc.get_mnemonic()) == MG_MNE_BNEZALC);
    assert!(mg_get_mnemonic(bnec.get_mnemonic()) == MG_MNE_BNEC);
    assert!(mg_get_mnemonic(bnvc.get_mnemonic()) == MG_MNE_BNVC);
    assert!("bnezalc" == MG_MNE_BNEZALC);
    assert!("bnec" == MG_MNE_BNEC);
    assert!("bnvc" == MG_MNE_BNVC);

    assert!(check_operands(&bnezalc, 2));
    assert!(check_operands(&bnec, 3));
    assert!(check_operands(&bnvc, 3));

    assert!(check_field_zero_assert(&decoder, machine_code[0], 0b1111111111, MgMnemonic::MgMneBnezalc, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneBnezalc, 16));
    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneBnezalc, 21));
    assert!(check_field_zero(&decoder, machine_code[0] ^ 0x3e000000, 0b1111100000, MgMnemonic::MgMneBnezalc, 16));

    assert!(check_field_zero_assert(&decoder, machine_code[1], 0b1111111111, MgMnemonic::MgMneBnec, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneBnec, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneBnec, 20));

    //The third one is kind off a catch all so I didn't test it's fields

    assert!(version_test(machine_code[0], MgMnemonic::MgMneBnezalc, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneBnec, false, true, false, true));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneBnvc, false, true, false, true));

    assert!(!bnec.is_region());
    assert!(bnec.is_relative());
    assert!(bnec.is_conditional());

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBnezalc, machine_code[0], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneBnec, machine_code[1], 0, 0xffff, 2));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneBnvc, machine_code[2], 0, 0xffff, 2));
}
#[test]
fn test_pop10(){
    let machine_code = [0x200A794A, 0x2234794A, 0x2000794A];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let beqzalc = decoder.disassemble(machine_code[0], 0).unwrap();
    let beqc = decoder.disassemble(machine_code[1], 0).unwrap();
    let bovc = decoder.disassemble(machine_code[2], 0).unwrap();

    assert!(beqzalc.get_mnemonic() == MgMnemonic::MgMneBeqzalc);
    assert!(beqc.get_mnemonic() == MgMnemonic::MgMneBeqc);
    assert!(bovc.get_mnemonic() == MgMnemonic::MgMneBovc);
    assert!(mg_get_mnemonic(beqzalc.get_mnemonic()) == MG_MNE_BEQZALC);
    assert!(mg_get_mnemonic(beqc.get_mnemonic()) == MG_MNE_BEQC);
    assert!(mg_get_mnemonic(bovc.get_mnemonic()) == MG_MNE_BOVC);
    assert!("beqzalc" == MG_MNE_BEQZALC);
    assert!("beqc" == MG_MNE_BEQC);
    assert!("bovc" == MG_MNE_BOVC);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneBeqzalc, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneBeqc, false, true, false, true));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneBovc, false, true, false, true));

    assert!(!beqc.is_region());
    assert!(beqc.is_relative());
    assert!(beqc.is_conditional());

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBeqzalc, machine_code[0], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneBeqc, machine_code[1], 0, 0xffff, 2));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneBovc, machine_code[2], 0, 0xffff, 2));

    assert!(check_operands(&beqzalc, 2));
    assert!(check_operands(&beqc, 3));
    assert!(check_operands(&bovc, 3));

    assert!(check_field_zero_assert(&decoder, machine_code[0], 0b1111111111, MgMnemonic::MgMneBeqzalc, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneBeqzalc, 16));
    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneBeqzalc, 21));
    assert!(check_field_zero(&decoder, machine_code[0] ^ 0x3e000000, 0b1111100000, MgMnemonic::MgMneBeqzalc, 16));

    assert!(check_field_zero_assert(&decoder, machine_code[1], 0b1111111111, MgMnemonic::MgMneBeqc, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneBeqc, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneBeqc, 20));
}
#[test]
fn test_bgtzl_pop27(){
    let machine_code = [0x5C01794A, 0x5E10794A, 0x5D55794A, 0x5E20794A];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let bgtzc = decoder.disassemble(machine_code[0], 0).unwrap();
    let bltzc = decoder.disassemble(machine_code[1], 0).unwrap();
    let bltc = decoder.disassemble(machine_code[2], 0).unwrap();

    assert!(bgtzc.get_mnemonic() == MgMnemonic::MgMneBgtzc);
    assert!(bltzc.get_mnemonic() == MgMnemonic::MgMneBltzc);
    assert!(bltc.get_mnemonic() == MgMnemonic::MgMneBltc);
    assert!(mg_get_mnemonic(bgtzc.get_mnemonic()) == MG_MNE_BGTZC);
    assert!(mg_get_mnemonic(bltzc.get_mnemonic()) == MG_MNE_BLTZC);
    assert!(mg_get_mnemonic(bltc.get_mnemonic()) == MG_MNE_BLTC);
    assert!("bgtzc" == MG_MNE_BGTZC);
    assert!("bltzc" == MG_MNE_BLTZC);
    assert!("bltc" == MG_MNE_BLTC);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneBgtzc, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneBltzc, false, true, false, true));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneBltc, false, true, false, true));

    assert!(!bltzc.is_region());
    assert!(bltzc.is_relative());
    assert!(bltzc.is_conditional());

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBgtzc, machine_code[0], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneBltzc, machine_code[1], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneBltc, machine_code[2], 0, 0xffff, 2));

    assert!(check_operands(&bgtzc, 2));
    assert!(check_operands(&bltzc, 2));
    assert!(check_operands(&bltc, 3));

    assert!(check_field_zero_assert(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneBgtzc, 16));
    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneBgtzc, 21));
    assert!(check_field_zero(&decoder, machine_code[0] ^ 0x3e000000, 0b1111100000, MgMnemonic::MgMneBgtzc, 16));

    assert!(check_field_zero_assert(&decoder, machine_code[1], 0b1111111111, MgMnemonic::MgMneBltzc, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneBltzc, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneBltzc, 21));
    assert!(check_field_zero_assert(&decoder, machine_code[1], 0b11110, MgMnemonic::MgMneBltzc, 21));
    assert!(check_field_zero_assert(&decoder, machine_code[1], 0b11110, MgMnemonic::MgMneBltzc, 16));
    
    assert!(check_field_zero_assert(&decoder, machine_code[2], 0b1111111111, MgMnemonic::MgMneBltc, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[2], 0b1111011110, MgMnemonic::MgMneBltc, 16));

    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);
    let bgtzl = decoder.disassemble(machine_code[3], 0).unwrap();

    assert!(version_test(machine_code[3], MgMnemonic::MgMneBgtzl, true, false, true, false));
    assert!(check_operands(&bgtzl, 2));
    assert!(check_field(&decoder, machine_code[3], 0b11111, MgMnemonic::MgMneBgtzl, 16));

    assert!(bgtzl.get_mnemonic() == MgMnemonic::MgMneBgtzl);
    assert!(mg_get_mnemonic(bgtzl.get_mnemonic()) == MG_MNE_BGTZL);
}
#[test]
fn test_blez_pop26(){
    let machine_code = [0x5801794A, 0x5A10794A, 0x5955794A, 0x5A20794A];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let blezc = decoder.disassemble(machine_code[0], 0).unwrap();
    let bgezc = decoder.disassemble(machine_code[1], 0).unwrap();
    let bgec = decoder.disassemble(machine_code[2], 0).unwrap();

    assert!(blezc.get_mnemonic() == MgMnemonic::MgMneBlezc);
    assert!(bgezc.get_mnemonic() == MgMnemonic::MgMneBgezc);
    assert!(bgec.get_mnemonic() == MgMnemonic::MgMneBgec);
    assert!(mg_get_mnemonic(blezc.get_mnemonic()) == MG_MNE_BLEZC);
    assert!(mg_get_mnemonic(bgezc.get_mnemonic()) == MG_MNE_BGEZC);
    assert!(mg_get_mnemonic(bgec.get_mnemonic()) == MG_MNE_BGEC);
    assert!("blezc" == MG_MNE_BLEZC);
    assert!("bgezc" == MG_MNE_BGEZC);
    assert!("bgec" == MG_MNE_BGEC);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneBlezc, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneBgezc, false, true, false, true));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneBgec, false, true, false, true));

    assert!(!bgezc.is_region());
    assert!(bgezc.is_relative());
    assert!(bgezc.is_conditional());

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBlezc, machine_code[0], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneBgezc, machine_code[1], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneBgec, machine_code[2], 0, 0xffff, 2));

    assert!(check_operands(&blezc, 2));
    assert!(check_operands(&bgezc, 2));
    assert!(check_operands(&bgec, 3));

    assert!(check_field_zero_assert(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneBlezc, 16));
    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneBlezc, 21));
    assert!(check_field_zero(&decoder, machine_code[0] ^ 0x3e000000, 0b1111100000, MgMnemonic::MgMneBlezc, 16));

    assert!(check_field_zero_assert(&decoder, machine_code[1], 0b1111111111, MgMnemonic::MgMneBgezc, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneBgezc, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneBgezc, 21));
    assert!(check_field_zero_assert(&decoder, machine_code[1], 0b11110, MgMnemonic::MgMneBgezc, 21));
    assert!(check_field_zero_assert(&decoder, machine_code[1], 0b11110, MgMnemonic::MgMneBgezc, 16));
    
    assert!(check_field_zero_assert(&decoder, machine_code[2], 0b1111111111, MgMnemonic::MgMneBgec, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[2], 0b1111011110, MgMnemonic::MgMneBgec, 16));

    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);
    let blezl = decoder.disassemble(machine_code[3], 0).unwrap();

    assert!(version_test(machine_code[3], MgMnemonic::MgMneBlezl, true, false, true, false));

    assert!(check_operands(&blezl, 2));
    assert!(check_field(&decoder, machine_code[3], 0b11111, MgMnemonic::MgMneBlezl, 16));

    assert!(blezl.get_mnemonic() == MgMnemonic::MgMneBlezl);
    assert!(mg_get_mnemonic(blezl.get_mnemonic()) == MG_MNE_BLEZL);
}
#[test]
fn test_pop07(){
    let machine_code: [u32; 4] = [0x1c30FFFF, 0x1c0a0050, 0x1c420050, 0x1c00C011];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let bltuc = decoder.disassemble(machine_code[0], 0).unwrap();
    let bgtzalc = decoder.disassemble(machine_code[1], 0).unwrap();
    let bltzalc = decoder.disassemble(machine_code[2], 0).unwrap();
    let bgtz = decoder.disassemble(machine_code[3], 0).unwrap();

    assert!(bltuc.get_mnemonic() == MgMnemonic::MgMneBltuc);
    assert!(bltzalc.get_mnemonic() == MgMnemonic::MgMneBltzalc);
    assert!(bgtzalc.get_mnemonic() == MgMnemonic::MgMneBgtzalc);
    assert!(bgtz.get_mnemonic() == MgMnemonic::MgMneBgtz);
    assert!(mg_get_mnemonic(bltuc.get_mnemonic()) == MG_MNE_BLTUC);
    assert!(mg_get_mnemonic(bltzalc.get_mnemonic()) == MG_MNE_BLTZALC);
    assert!(mg_get_mnemonic(bgtzalc.get_mnemonic()) == MG_MNE_BGTZALC);
    assert!(mg_get_mnemonic(bgtz.get_mnemonic()) == MG_MNE_BGTZ);
    assert!("bltuc" == MG_MNE_BLTUC);
    assert!("bltzalc" == MG_MNE_BLTZALC);
    assert!("bgtzalc" == MG_MNE_BGTZALC);
    assert!("bgtz" == MG_MNE_BGTZ);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneBltuc, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneBgtzalc, false, true, false, true));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneBltzalc, false, true, false, true));
    assert!(version_test(machine_code[3], MgMnemonic::MgMneBgtz, true, true, true, true));

    assert!(!bltzalc.is_region());
    assert!(bltzalc.is_relative());
    assert!(bltzalc.is_conditional());

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBltuc, machine_code[0], 0, 0xffff, 2));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBgtzalc, machine_code[1], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneBltzalc, machine_code[2], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneBgtz, machine_code[3], 0, 0xffff, 1));

    assert!(check_operands(&bgtzalc, 2));
    assert!(check_operands(&bltzalc, 2));
    assert!(check_operands(&bltuc, 3));
    assert!(check_operands(&bgtz, 2));

    assert!(check_field_zero_assert(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneBgtzalc, 16));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneBgtzalc, 21));
    assert!(check_field_zero(&decoder, machine_code[1] ^ 0x3e000000, 0b1111100000, MgMnemonic::MgMneBgtzalc, 16));

    assert!(check_field_zero_assert(&decoder, machine_code[2], 0b1111111111, MgMnemonic::MgMneBltzalc, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[2], 0b11111, MgMnemonic::MgMneBltzalc, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[2], 0b11111, MgMnemonic::MgMneBltzalc, 21));
    assert!(check_field_zero_assert(&decoder, machine_code[2], 0b11110, MgMnemonic::MgMneBltzalc, 21));
    assert!(check_field_zero_assert(&decoder, machine_code[2], 0b11110, MgMnemonic::MgMneBltzalc, 16));

    assert!(check_field(&decoder, machine_code[3], 0b11111, MgMnemonic::MgMneBgtz, 16));

    assert!(check_field_zero_assert(&decoder, machine_code[0], 0b1111111111, MgMnemonic::MgMneBltuc, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[0], 0b1111011110, MgMnemonic::MgMneBltuc, 16));

    match (bltuc.get_operand(0), bltuc.get_operand(1), bltuc.get_operand(2)){
        (Some(MgOperand::MgOpRegister(r1)),Some(MgOperand::MgOpRegister(r2)), Some(MgOperand::MgOpImmediate(_))) => assert_ne!(r1, r2),
        _ => panic!(),
    }
    assert!(check_operands(&bltzalc, 2));
    match (bltzalc.get_operand(0), bltzalc.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }
    assert!(check_operands(&bgtzalc, 2));
    match (bgtzalc.get_operand(0), bgtzalc.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }
    assert!(check_operands(&bgtz, 2));
    match (bgtz.get_operand(0), bgtz.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }
}
#[test]
fn test_blez_pop06(){
    let machine_code: [u32; 4] = [0x1830FFFF, 0x180a0050, 0x18420050, 0x1800C011];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let bgeuc = decoder.disassemble(machine_code[0], 0).unwrap();
    let blezalc = decoder.disassemble(machine_code[1], 0).unwrap();
    let bgezalc = decoder.disassemble(machine_code[2], 0).unwrap();
    let blez = decoder.disassemble(machine_code[3], 0).unwrap();

    assert!(bgeuc.get_mnemonic() == MgMnemonic::MgMneBgeuc);
    assert!(blezalc.get_mnemonic() == MgMnemonic::MgMneBlezalc);
    assert!(bgezalc.get_mnemonic() == MgMnemonic::MgMneBgezalc);
    assert!(blez.get_mnemonic() == MgMnemonic::MgMneBlez);
    assert!(mg_get_mnemonic(bgeuc.get_mnemonic()) == MG_MNE_BGEUC);
    assert!(mg_get_mnemonic(blezalc.get_mnemonic()) == MG_MNE_BLEZALC);
    assert!(mg_get_mnemonic(bgezalc.get_mnemonic()) == MG_MNE_BGEZALC);
    assert!(mg_get_mnemonic(blez.get_mnemonic()) == MG_MNE_BLEZ);
    assert!("bgeuc" == MG_MNE_BGEUC);
    assert!("blezalc" == MG_MNE_BLEZALC);
    assert!("bgezalc" == MG_MNE_BGEZALC);
    assert!("blez" == MG_MNE_BLEZ);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneBgeuc, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneBlezalc, false, true, false, true));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneBgezalc, false, true, false, true));
    assert!(version_test(machine_code[3], MgMnemonic::MgMneBlez, true, true, true, true));

    assert!(!blezalc.is_region());
    assert!(blezalc.is_relative());
    assert!(blezalc.is_conditional());

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBgeuc, machine_code[0], 0, 0xffff, 2));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneBlezalc, machine_code[1], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBgezalc, machine_code[2], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneBlez, machine_code[3], 0, 0xffff, 1));

    assert!(check_operands(&blezalc, 2));
    assert!(check_operands(&bgezalc, 2));
    assert!(check_operands(&bgeuc, 3));
    assert!(check_operands(&blez, 2));

    assert!(check_field_zero_assert(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneBlezalc, 16));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneBlezalc, 21));
    assert!(check_field_zero(&decoder, machine_code[1] ^ 0x3e000000, 0b1111100000, MgMnemonic::MgMneBlezalc, 16));

    assert!(check_field_zero_assert(&decoder, machine_code[2], 0b1111111111, MgMnemonic::MgMneBgezalc, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[2], 0b11111, MgMnemonic::MgMneBgezalc, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[2], 0b11111, MgMnemonic::MgMneBgezalc, 21));
    assert!(check_field_zero_assert(&decoder, machine_code[2], 0b11110, MgMnemonic::MgMneBgezalc, 21));
    assert!(check_field_zero_assert(&decoder, machine_code[2], 0b11110, MgMnemonic::MgMneBgezalc, 16));

    assert!(check_field(&decoder, machine_code[3], 0b11111, MgMnemonic::MgMneBlez, 16));

    assert!(check_field_zero_assert(&decoder, machine_code[0], 0b1111111111, MgMnemonic::MgMneBgeuc, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[0], 0b1111011110, MgMnemonic::MgMneBgeuc, 16));

    match (bgeuc.get_operand(0), bgeuc.get_operand(1), bgeuc.get_operand(2)){
        (Some(MgOperand::MgOpRegister(r1)),Some(MgOperand::MgOpRegister(r2)), Some(MgOperand::MgOpImmediate(_))) => assert_ne!(r1, r2),
        _ => panic!(),
    }
    match (blezalc.get_operand(0), blezalc.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }
    match (bgezalc.get_operand(0), bgezalc.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }
    match (blez.get_operand(0), blez.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }
}
