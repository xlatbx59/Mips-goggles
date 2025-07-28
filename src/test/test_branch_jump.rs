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

    assert_eq!(true, check_operands(&bne, 3));
    assert_eq!(true, check_operands(&beq, 3));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBeq, machine_code[1], 0, 0xffff, 2));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBne, machine_code[0], 0, 0xffff, 2));
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