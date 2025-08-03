//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_dclz_dclo(){
    let machine_code: [u32; 4] = [0x00002052, 0x00800053, 0x70A42024, 0x70A42025];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));
    
    let mut dclz = decoder.disassemble(machine_code[0], 0).unwrap();
    let mut dclo = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(false, dclo.is_conditional());
    assert_eq!(false, dclo.is_relative());
    assert_eq!(false, dclo.is_region());
    assert_eq!(false, dclz.is_conditional());
    assert_eq!(false, dclz.is_relative());
    assert_eq!(false, dclz.is_region());

    assert_eq!(MgMnemonic::MgMneDclz, dclz.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneDclo, dclo.get_mnemonic());
    assert_eq!(MG_MNE_DCLZ, dclz.get_mnemonic_str());
    assert_eq!(MG_MNE_DCLO, dclo.get_mnemonic_str());

    assert_eq!(true, check_operands(&dclz, 2));
    assert_eq!(true, check_operands(&dclo, 2));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11110, MgMnemonic::MgMneDclz, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11110, MgMnemonic::MgMneDclo, 6));
    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneDclz, 16));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneDclo, 16));

    decoder.version = MgMipsVersion::M64(MgMips64::MgPreR6);
    dclz = decoder.disassemble(machine_code[2], 0).unwrap();
    dclo = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneDclz, dclz.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneDclo, dclo.get_mnemonic());
    assert_eq!(MG_MNE_DCLZ, dclz.get_mnemonic_str());
    assert_eq!(MG_MNE_DCLO, dclo.get_mnemonic_str());

    assert_eq!(false, dclo.is_conditional());
    assert_eq!(false, dclo.is_relative());
    assert_eq!(false, dclo.is_region());
    assert_eq!(false, dclz.is_conditional());
    assert_eq!(false, dclz.is_relative());
    assert_eq!(false, dclz.is_region());

    assert_eq!(true, check_operands(&dclz, 2));
    assert_eq!(true, check_operands(&dclo, 2));

    assert_eq!(true, check_field(&decoder, machine_code[2], 0b11111, MgMnemonic::MgMneDclz, 6));
    assert_eq!(true, check_field(&decoder, machine_code[3], 0b11111, MgMnemonic::MgMneDclo, 6));
}
#[test]
fn test_clz_clo(){
    let machine_code: [u32; 4] = [0x00000050, 0x00000051, 0x70A42020, 0x70A42021 ];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));
    
    let mut clz = decoder.disassemble(machine_code[0], 0).unwrap();
    let mut clo = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(false, clo.is_conditional());
    assert_eq!(false, clo.is_relative());
    assert_eq!(false, clo.is_region());
    assert_eq!(false, clz.is_conditional());
    assert_eq!(false, clz.is_relative());
    assert_eq!(false, clz.is_region());

    assert_eq!(MgMnemonic::MgMneClz, clz.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneClo, clo.get_mnemonic());
    assert_eq!(MG_MNE_CLZ, clz.get_mnemonic_str());
    assert_eq!(MG_MNE_CLO, clo.get_mnemonic_str());

    assert_eq!(true, check_operands(&clz, 2));
    assert_eq!(true, check_operands(&clo, 2));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11110, MgMnemonic::MgMneClz, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11110, MgMnemonic::MgMneClo, 6));
    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneClz, 16));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneClo, 16));

    decoder.version = MgMipsVersion::M64(MgMips64::MgPreR6);
    clz = decoder.disassemble(machine_code[2], 0).unwrap();
    clo = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneClz, clz.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneClo, clo.get_mnemonic());
    assert_eq!(MG_MNE_CLZ, clz.get_mnemonic_str());
    assert_eq!(MG_MNE_CLO, clo.get_mnemonic_str());

    assert_eq!(false, clo.is_conditional());
    assert_eq!(false, clo.is_relative());
    assert_eq!(false, clo.is_region());
    assert_eq!(false, clz.is_conditional());
    assert_eq!(false, clz.is_relative());
    assert_eq!(false, clz.is_region());

    assert_eq!(true, check_operands(&clz, 2));
    assert_eq!(true, check_operands(&clo, 2));

    assert_eq!(true, check_field(&decoder, machine_code[2], 0b11111, MgMnemonic::MgMneClz, 6));
    assert_eq!(true, check_field(&decoder, machine_code[3], 0b11111, MgMnemonic::MgMneClo, 6));
}
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

    assert_eq!(ddiv.get_mnemonic(), MgMnemonic::MgMneDdiv);
    assert_eq!(ddivu.get_mnemonic(), MgMnemonic::MgMneDdivu);
    assert_eq!(ddiv.get_mnemonic_str(), MG_MNE_DDIV);
    assert_eq!(ddivu.get_mnemonic_str(), MG_MNE_DDIVU);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDdiv, false, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDdivu, false, false, true, false));

    assert_eq!(true, check_operands(&ddiv, 2));
    assert_eq!(true, check_operands(&ddivu, 2));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0b1111111111, MgMnemonic::MgMneDdiv, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b1111111111, MgMnemonic::MgMneDdivu, 6));
}
#[test]
fn test_msub_msubu(){
    let machine_code: [u32; 2] = [0x70850004, 0x70850005];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let msub = decoder.disassemble(machine_code[0], 0).unwrap();
    let msubu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneMsub, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneMsubu, true, false, true, false));

    assert_eq!(false, msubu.is_conditional());
    assert_eq!(false, msubu.is_relative());
    assert_eq!(false, msubu.is_region());
    assert_eq!(false, msub.is_conditional());
    assert_eq!(false, msub.is_relative());
    assert_eq!(false, msub.is_region());
    
    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneMsub, 6));
    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneMsub, 11));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneMsubu, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneMsubu, 11));

    assert_eq!(msub.get_mnemonic(), MgMnemonic::MgMneMsub);
    assert_eq!(msubu.get_mnemonic(), MgMnemonic::MgMneMsubu);

    assert_eq!(msub.get_mnemonic_str(), MG_MNE_MSUB);
    assert_eq!(msubu.get_mnemonic_str(), MG_MNE_MSUBU);

    assert_eq!(true, check_operands(&msub, 2));
    assert_eq!(true, check_operands(&msubu, 2));
}
#[test]
fn test_madd_maddu(){
    let machine_code: [u32; 2] = [0x70850000, 0x70850001];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let madd = decoder.disassemble(machine_code[0], 0).unwrap();
    let maddu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneMadd, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneMaddu, true, false, true, false));

    assert_eq!(false, maddu.is_conditional());
    assert_eq!(false, maddu.is_relative());
    assert_eq!(false, maddu.is_region());
    assert_eq!(false, madd.is_conditional());
    assert_eq!(false, madd.is_relative());
    assert_eq!(false, madd.is_region());
    
    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneMadd, 6));
    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneMadd, 11));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneMaddu, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneMaddu, 11));

    assert_eq!(madd.get_mnemonic(), MgMnemonic::MgMneMadd);
    assert_eq!(maddu.get_mnemonic(), MgMnemonic::MgMneMaddu);

    assert_eq!(madd.get_mnemonic_str(), MG_MNE_MADD);
    assert_eq!(maddu.get_mnemonic_str(), MG_MNE_MADDU);

    assert_eq!(true, check_operands(&madd, 2));
    assert_eq!(true, check_operands(&maddu, 2));
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