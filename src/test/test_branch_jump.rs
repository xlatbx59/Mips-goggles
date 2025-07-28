//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_bne_beq(){
    let machine_code: [u32; 2] = [0x1485C013, 0x1085C013];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let bne: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let beq: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MG_MNE_BNE, bne.get_mnemonic_str());
    assert_eq!(MG_MNE_BEQ, beq.get_mnemonic_str());
    assert_eq!(MgMnemonic::MgMneBne, bne.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneBeq, beq.get_mnemonic());

    assert_eq!(true, bne.is_relative());
    assert_eq!(true, beq.is_relative());

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneBne, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneBeq, true, true, true, true));

    assert_eq!(true, check_operands(&bne, 3));
    assert_eq!(true, check_operands(&beq, 3));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBeq, machine_code[1], 0, 0xffff, 2));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBne, machine_code[0], 0, 0xffff, 2));
}
#[test]
fn test_bnel_beql(){
    let machine_code: [u32; 2] = [0x5485C114, 0x5085C114];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let bnel: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let beql: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MG_MNE_BNEL, bnel.get_mnemonic_str());
    assert_eq!(MG_MNE_BEQL, beql.get_mnemonic_str());

    assert_eq!(MgMnemonic::MgMneBnel, bnel.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneBeql, beql.get_mnemonic());

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneBnel, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneBeql, true, false, true, false));

    assert_eq!(true, bnel.is_relative());
    assert_eq!(true, beql.is_relative());

    assert_eq!(true, check_operands(&bnel, 3));
    assert_eq!(true, check_operands(&beql, 3));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBeql, machine_code[1], 0, 0xffff, 2));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBnel, machine_code[0], 0, 0xffff, 2));
}

#[test]
fn test_jalx(){
    let machine_code = 0x74000115;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let jalx: MgInstruction= decoder.disassemble(machine_code, 0).unwrap();

    assert_eq!(true, check_operands(&jalx, 1));
    assert_eq!(true, jalx.is_region());
    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneJalx, true, false, true, false));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneJalx, machine_code, 0, 0x3ffffff, 0));
}

#[test]
fn test_bc_balc(){
    let machine_code: [u32; 2] = [0xC8020050, 0xE8020050];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let bc = decoder.disassemble(machine_code[0], 0).unwrap();
    let balc = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneBc, false, true, false, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneBalc, false, true, false, true));

    assert_eq!(bc.get_mnemonic(), MgMnemonic::MgMneBc);
    assert_eq!(balc.get_mnemonic(), MgMnemonic::MgMneBalc);

    assert_eq!(mg_get_mnemonic(bc.get_mnemonic()), MG_MNE_BC);
    assert_eq!(mg_get_mnemonic(balc.get_mnemonic()), MG_MNE_BALC);

    assert_eq!(true, check_operands(&bc, 1));
    assert_eq!(true, check_operands(&balc, 1));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBc, machine_code[0], 0, 0x3ffffff, 0));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBalc, machine_code[1], 0, 0x3ffffff, 0));

    assert_eq!(bc.is_conditional(), true);
    assert_ne!(bc.is_region(), true);

    assert_eq!(balc.is_conditional(), true);
    assert_ne!(balc.is_region(), true);
}