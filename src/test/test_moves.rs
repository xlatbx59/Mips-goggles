//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_movn_movz(){
    let machine_code: [u32; 2] = [0x00BB200B, 0x00BB200A];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let movn = decoder.disassemble(machine_code[0], 0).unwrap();
    let movz = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneMovz, movz.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneMovn, movn.get_mnemonic());

    assert_eq!(true, movz.is_conditional());
    assert_eq!(false, movz.is_relative());
    assert_eq!(false, movz.is_region());
    assert_eq!(true, movn.is_conditional());
    assert_eq!(false, movn.is_relative());
    assert_eq!(false, movn.is_region());

    assert_eq!(MG_MNE_MOVN, movn.get_mnemonic_str());
    assert_eq!(MG_MNE_MOVZ, movz.get_mnemonic_str());

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneMovn, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneMovz, true, false, true, false));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneMovn, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0x1f, MgMnemonic::MgMneMovz, 6));

    assert_eq!(true, check_operands(&movn, 3));
    assert_eq!(true, check_operands(&movz, 3));
}
#[test]
fn test_movci(){
    let machine_code: [u32; 2] = [0x01204001,0x01214001];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let movf = decoder.disassemble(machine_code[0], 0).unwrap();
    let movt = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneMovt, movt.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneMovf, movf.get_mnemonic());

    assert_eq!(MG_MNE_MOVF, movf.get_mnemonic_str());
    assert_eq!(MG_MNE_MOVT, movt.get_mnemonic_str());

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneMovf, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneMovt, true, false, true, false));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneMovf, 6));
    assert_eq!(true, check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneMovf, 17));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0x1f, MgMnemonic::MgMneMovt, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 1, MgMnemonic::MgMneMovt, 17));

    assert_eq!(true, check_operands(&movf, 3));
    assert_eq!(true, check_operands(&movt, 3));
}
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