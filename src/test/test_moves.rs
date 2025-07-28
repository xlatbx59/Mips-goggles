//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_mfhi_mflo(){
    let machine_code: [u32; 2] = [0x00002010, 0x00002012];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let mfhi = decoder.disassemble(machine_code[0], 0).unwrap();
    let mflo = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(mfhi.get_mnemonic(), MgMnemonic::MgMneMfhi);
    assert_eq!(mflo.get_mnemonic(), MgMnemonic::MgMneMflo);
    assert_eq!(mfhi.get_mnemonic_str(), MG_MNE_MFHI);
    assert_eq!(mflo.get_mnemonic_str(), MG_MNE_MFLO);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneMfhi, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneMflo, true, false, true, false));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0x3ff, MgMnemonic::MgMneMfhi, 16));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0x3ff, MgMnemonic::MgMneMflo, 16));
    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneMfhi, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneMflo, 6));

    assert_eq!(true, check_operands(&mfhi, 1));
    assert_eq!(true, check_operands(&mflo, 1));
}
#[test]
fn test_mthi_mtlo(){
    let machine_code: [u32; 2] = [0x00800011, 0x00800013];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let mthi = decoder.disassemble(machine_code[0], 0).unwrap();
    let mtlo = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(mthi.get_mnemonic(), MgMnemonic::MgMneMthi);
    assert_eq!(mtlo.get_mnemonic(), MgMnemonic::MgMneMtlo);
    assert_eq!(mthi.get_mnemonic_str(), MG_MNE_MTHI);
    assert_eq!(mtlo.get_mnemonic_str(), MG_MNE_MTLO);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneMthi, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneMtlo, true, false, true, false));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0x7fff, MgMnemonic::MgMneMthi, 16));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0x7fff, MgMnemonic::MgMneMtlo, 16));

    assert_eq!(true, check_operands(&mthi, 1));
    assert_eq!(true, check_operands(&mtlo, 1));
}