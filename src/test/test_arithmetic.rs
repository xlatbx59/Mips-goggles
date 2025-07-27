//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::instruction::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_mul(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let machine_code = 0x70f34802;
    let mul = decoder.disassemble(machine_code, 0).unwrap();

    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneMul, true, false, true, false));
    assert_eq!(mul.get_category(), MgInstructionCategory::Arithmetic);

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

    assert_eq!(ddiv.get_category(), MgInstructionCategory::Arithmetic);

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
