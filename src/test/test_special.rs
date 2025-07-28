//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::operands::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_lsa_dlsa(){
    let machine_code = [0x00fa18c5, 0x00fa18d5];

    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneLsa, false, true, false, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDlsa, false, false, false, true));

    let lsa = decoder.disassemble(machine_code[0], 0).unwrap();
    let dlsa = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneLsa, lsa.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneDlsa, dlsa.get_mnemonic());

    assert_eq!(true, check_field(&decoder, machine_code[0], 0b111, MgMnemonic::MgMneLsa, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b111, MgMnemonic::MgMneDlsa, 6));

    assert_eq!(MG_MNE_LSA, lsa.get_mnemonic_str());
    assert_eq!(MG_MNE_DLSA, dlsa.get_mnemonic_str());

    assert_eq!(true, check_operands(&lsa, 4));
    assert_eq!(true, check_operands(&dlsa, 4));

    imm_limit_reached(&decoder, MgMnemonic::MgMneLsa, machine_code[0], 6, 3, 3);
    imm_limit_reached(&decoder, MgMnemonic::MgMneDlsa, machine_code[1], 6, 3, 3);

    let Some(MgOperand::MgOpRegister(_)) = lsa.get_operand(0) else {
        panic!();
    };
    let Some(MgOperand::MgOpRegister(_)) = lsa.get_operand(1) else {
        panic!();
    };
    let Some(MgOperand::MgOpRegister(_)) = lsa.get_operand(2) else {
        panic!();
    };
    let Some(MgOperand::MgOpRegister(_)) = dlsa.get_operand(0) else {
        panic!();
    };
    let Some(MgOperand::MgOpRegister(_)) = dlsa.get_operand(1) else {
        panic!();
    };
    let Some(MgOperand::MgOpRegister(_)) = dlsa.get_operand(2) else {
        panic!();
    };
}
#[test]
fn test_seleqz_selnez() {
    let machine_code: [u32; 2] = [0b110101, 0b110111];

    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let seleqz = decoder.disassemble(machine_code[0], 0).unwrap();
    let selnez = decoder.disassemble(machine_code[1], 0).unwrap();

    //No problem
    assert_eq!(seleqz.get_mnemonic(), MgMnemonic::MgMneSeleqz);
    assert_eq!(selnez.get_mnemonic(), MgMnemonic::MgMneSelnez);

    assert_eq!(seleqz.is_conditional(), true);
    assert_eq!(selnez.is_conditional(), true);

    assert_eq!(true, check_operands(&seleqz, 3));
    assert_eq!(true, check_operands(&selnez, 3));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneSeleqz, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneSelnez, 6));

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneSeleqz, false, true, false, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneSelnez, false, true, false, true));
}
#[test]
fn test_dahi_dati(){
    let machine_code = [0x054633f1, 0x055e33f1];

    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDahi, false, false, false, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDati, false, false, false, true));

    decoder.version = MgMipsVersion::M64(MgMips64::MgR6);
    let dahi = decoder.disassemble(machine_code[0], 0).unwrap();
    let dati = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(true, check_operands(&dahi, 2));
    assert_eq!(true, check_operands(&dati, 2));

    assert_eq!(MgMnemonic::MgMneDati, dati.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneDahi, dahi.get_mnemonic());
    assert_eq!(MG_MNE_DATI, dati.get_mnemonic_str());
    assert_eq!(MG_MNE_DAHI, dahi.get_mnemonic_str());

    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneDahi, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneDati, machine_code[1], 0, 0xffff, 1));

    let Some(MgOperand::MgOpRegister(_)) = dahi.get_operand(0) else {
        panic!();
    };
    let Some(MgOperand::MgOpRegister(_)) = dati.get_operand(0) else {
        panic!();
    };
}