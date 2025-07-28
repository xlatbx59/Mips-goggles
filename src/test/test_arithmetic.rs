//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_mul(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let machine_code = 0x70f34802;
    let mul = decoder.disassemble(machine_code, 0).unwrap();

    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneMul, true, false, true, false));

    assert_eq!(true, check_operands(&mul, 3));

    assert_eq!(true, check_field(&decoder, machine_code, 0b11111, MgMnemonic::MgMneMul, 6));

    assert_eq!(mul.get_mnemonic(), MgMnemonic::MgMneMul);
    assert_eq!(mul.get_mnemonic_str(), MG_MNE_MUL);
}
#[test]
fn test_ddiv_ddivu(){
    let machine_code = [0x0044001e, 0x000A001f];

    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let ddiv = decoder.disassemble(machine_code[0], 0).unwrap();
    let ddivu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDdiv, false, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDdivu, false, false, true, false));

    assert_eq!(true, check_operands(&ddiv, 2));
    assert_eq!(true, check_operands(&ddivu, 2));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0b1111111111, MgMnemonic::MgMneDdiv, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b1111111111, MgMnemonic::MgMneDdivu, 6));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneDdiv, machine_code[0], 0, 0x1fffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneDdivu, machine_code[1], 0, 0xffff, 1));

    assert_eq!(ddiv.get_mnemonic(), MgMnemonic::MgMneDdiv);
    assert_eq!(ddivu.get_mnemonic(), MgMnemonic::MgMneDdivu);
    assert_eq!(ddiv.get_mnemonic_str(), MG_MNE_DDIV);
    assert_eq!(ddivu.get_mnemonic_str(), MG_MNE_DDIVU);
}
#[test]
fn test_daddi_daddiu(){
    let machine_code: [u32; 2] = [0x63640038, 0x67640038];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let daddi = decoder.disassemble(machine_code[0], 0).unwrap();
    let daddiu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneDaddi, machine_code[0], 0, 0xffff, 2));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneDaddiu, machine_code[1], 0, 0xffff, 2));

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDaddi, false, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDaddiu, false, false, true, true));

    
    assert_eq!(daddi.get_mnemonic(), MgMnemonic::MgMneDaddi);
    assert_eq!(daddiu.get_mnemonic(), MgMnemonic::MgMneDaddiu);

    assert_eq!(daddi.get_mnemonic_str(), MG_MNE_DADDI);
    assert_eq!(daddiu.get_mnemonic_str(), MG_MNE_DADDIU);

    assert_eq!(true, check_operands(&daddi, 3));
    assert_eq!(true, check_operands(&daddiu, 3));
}
#[test]
fn test_addi_addiu(){
    let machine_code: [u32; 2] = [0x23640038, 0x27640038];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let addi = decoder.disassemble(machine_code[0], 0).unwrap();
    let addiu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneAddi, machine_code[0], 0, 0xffff, 2));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneAddiu, machine_code[1], 0, 0xffff, 2));

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneAddi, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneAddiu, true, true, true, true));

    assert_eq!(addi.get_mnemonic(), MgMnemonic::MgMneAddi);
    assert_eq!(addiu.get_mnemonic(), MgMnemonic::MgMneAddiu);

    assert_eq!(addi.get_mnemonic_str(), MG_MNE_ADDI);
    assert_eq!(addiu.get_mnemonic_str(), MG_MNE_ADDIU);

    assert_eq!(true, check_operands(&addi, 3));
    assert_eq!(true, check_operands(&addiu, 3));
}

#[test]
fn test_lui_aui(){
    let machine_code: [u32; 2] = [0x3d1B9c58, 0x3C1B0058];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let aui = decoder.disassemble(machine_code[0], 0).unwrap();
    let lui = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(aui.get_mnemonic(), MgMnemonic::MgMneAui);
    assert_eq!(lui.get_mnemonic(), MgMnemonic::MgMneLui);
    assert_eq!(aui.get_mnemonic_str(), MG_MNE_AUI);
    assert_eq!(lui.get_mnemonic_str(), MG_MNE_LUI);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneAui, false, true, false, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneLui, true, true, true, true));

    assert_eq!(true, check_operands(&aui, 3));
}