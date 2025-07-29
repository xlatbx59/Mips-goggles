//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_srl_sra(){
    let machine_code: [u32; 3] = [0x000C7103, 0x000C6902, 0x003f6902];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let sra = decoder.disassemble(machine_code[0], 0).unwrap();
    let srl = decoder.disassemble(machine_code[1], 0).unwrap();
    let rotr = decoder.disassemble(machine_code[2], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneSra, sra.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneSrl, srl.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneRotr, rotr.get_mnemonic());

    assert_eq!(MG_MNE_SRA, sra.get_mnemonic_str());
    assert_eq!(MG_MNE_SRL, srl.get_mnemonic_str());
    assert_eq!(MG_MNE_ROTR, rotr.get_mnemonic_str());

    assert_eq!(3, sra.get_operand_num());
    assert_eq!(3, srl.get_operand_num());
    assert_eq!(3, rotr.get_operand_num());

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneSra, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneSrl, true, false, true, false));
    assert_eq!(true, version_test(machine_code[2], MgMnemonic::MgMneRotr, true, false, true, false));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneSra, 21));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0x1f, MgMnemonic::MgMneSrl, 21));
    assert_eq!(true, check_field(&decoder, machine_code[2], 0b1111, MgMnemonic::MgMneRotr, 22));
    assert_eq!(true, check_field_zero_assert(&decoder, machine_code[2], 1, MgMnemonic::MgMneRotr, 21));

    imm_limit_reached(&decoder, MgMnemonic::MgMneSra, machine_code[0], 6, 0x1f, 2);
    imm_limit_reached(&decoder, MgMnemonic::MgMneSrl, machine_code[1], 6, 0x1f, 2);
}
#[test]
fn test_srlv_srav(){
    let machine_code: [u32; 3] = [0x008C7007, 0x008C6806, 0x008C6846];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let srav = decoder.disassemble(machine_code[0], 0).unwrap();
    let srlv = decoder.disassemble(machine_code[1], 0).unwrap();
    let rotrv = decoder.disassemble(machine_code[2], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneSrav, srav.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneSrlv, srlv.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneRotrv, rotrv.get_mnemonic());

    assert_eq!(MG_MNE_SRAV, srav.get_mnemonic_str());
    assert_eq!(MG_MNE_SRLV, srlv.get_mnemonic_str());
    assert_eq!(MG_MNE_ROTRV, rotrv.get_mnemonic_str());

    assert_eq!(3, srav.get_operand_num());
    assert_eq!(3, srlv.get_operand_num());
    assert_eq!(3, rotrv.get_operand_num());

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneSrav, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneSrlv, true, true, true, true));
    assert_eq!(true, version_test(machine_code[2], MgMnemonic::MgMneRotrv, true, true, true, true));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneSrav, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0x1f, MgMnemonic::MgMneSrlv, 6));
    assert_eq!(true, check_field(&decoder, machine_code[2], 0b1111, MgMnemonic::MgMneRotrv, 7));
    assert_eq!(true, check_field_zero_assert(&decoder, machine_code[2], 1, MgMnemonic::MgMneRotrv, 6));
}
#[test]
fn test_andi(){
    let machine_code = 0x33640058;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let andi = decoder.disassemble(machine_code, 0).unwrap();

    assert_eq!(andi.get_mnemonic(), MgMnemonic::MgMneAndi);
    assert_eq!(andi.get_mnemonic_str(), MG_MNE_ANDI);

    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneAndi, true, true, true, true));

    assert_eq!(true, check_operands(&andi, 3));
}
#[test]
fn test_ori(){
    let machine_code = 0x37640058;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let ori = decoder.disassemble(machine_code, 0).unwrap();

    assert_eq!(ori.get_mnemonic(), MgMnemonic::MgMneOri);
    assert_eq!(ori.get_mnemonic_str(), MG_MNE_ORI);

    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneOri, true, true, true, true));

    assert_eq!(true, check_operands(&ori, 3));
}
#[test]
fn test_xori(){
    let machine_code = 0x3B640058;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let xori = decoder.disassemble(machine_code, 0).unwrap();

    assert_eq!(xori.get_mnemonic(), MgMnemonic::MgMneXori);
    assert_eq!(xori.get_mnemonic_str(), MG_MNE_XORI);

    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneXori, true, true, true, true));

    assert_eq!(true, check_operands(&xori, 3));
}