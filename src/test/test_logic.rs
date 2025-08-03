//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_sll_dsll(){
    let machine_code: [u32; 2] = [0x000527C0, 0x000527F8];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let sll = decoder.disassemble(machine_code[0], 0).unwrap();
    assert_eq!(MgMnemonic::MgMneSll, sll.get_mnemonic());
    assert_eq!(MG_MNE_SLL, sll.get_mnemonic_str());
    imm_limit_reached(&decoder, MgMnemonic::MgMneSll, machine_code[0], 6, 0x1f, 2);
    assert_eq!(3, sll.get_operand_num());
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneSll, true, true, true, true));
    assert_eq!(true, check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneSll, 21));
    assert_eq!(false, sll.is_conditional());
    assert_eq!(false, sll.is_relative());
    assert_eq!(false, sll.is_region());

    decoder.version = MgMipsVersion::M64(MgMips64::MgPreR6);
    let dsll = decoder.disassemble(machine_code[1], 0).unwrap();
    assert_eq!(MgMnemonic::MgMneDsll, dsll.get_mnemonic());
    assert_eq!(MG_MNE_DSLL, dsll.get_mnemonic_str());
    assert_eq!(3, dsll.get_operand_num());
    imm_limit_reached(&decoder, MgMnemonic::MgMneDsll, machine_code[1], 6, 0x1f, 2);
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDsll, false, false, true, true));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0x1f, MgMnemonic::MgMneDsll, 21));
    assert_eq!(false, dsll.is_conditional());
    assert_eq!(false, dsll.is_relative());
    assert_eq!(false, dsll.is_region());

}
#[test]
fn test_dsrl_dsra(){
    let machine_code: [u32; 3] = [0x000527FB, 0x000527FA, 0x002527FA];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let dsra = decoder.disassemble(machine_code[0], 0).unwrap();
    let dsrl = decoder.disassemble(machine_code[1], 0).unwrap();
    let drotr = decoder.disassemble(machine_code[2], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneDsra, dsra.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneDsrl, dsrl.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneDrotr, drotr.get_mnemonic());

    assert_eq!(MG_MNE_DSRA, dsra.get_mnemonic_str());
    assert_eq!(MG_MNE_DSRL, dsrl.get_mnemonic_str());
    assert_eq!(MG_MNE_DROTR, drotr.get_mnemonic_str());

    assert_eq!(3, dsra.get_operand_num());
    assert_eq!(3, dsrl.get_operand_num());
    assert_eq!(3, drotr.get_operand_num());

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDsra, false, false, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDsrl, false, false, true, true));
    assert_eq!(true, version_test(machine_code[2], MgMnemonic::MgMneDrotr, false, false, true, true));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneDsra, 21));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0x1f, MgMnemonic::MgMneDsrl, 21));
    assert_eq!(true, check_field(&decoder, machine_code[2], 0b1111, MgMnemonic::MgMneDrotr, 22));
    assert_eq!(true, check_field_zero_assert(&decoder, machine_code[2], 1, MgMnemonic::MgMneDrotr, 21));

    imm_limit_reached(&decoder, MgMnemonic::MgMneDsra, machine_code[0], 6, 0x1f, 2);
    imm_limit_reached(&decoder, MgMnemonic::MgMneDsrl, machine_code[1], 6, 0x1f, 2);
    imm_limit_reached(&decoder, MgMnemonic::MgMneDrotr, machine_code[2], 6, 0x1f, 2);
}
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
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneSrl, true, true, true, true));
    assert_eq!(true, version_test(machine_code[2], MgMnemonic::MgMneRotr, true, true, true, true));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneSra, 21));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0x1f, MgMnemonic::MgMneSrl, 21));
    assert_eq!(true, check_field(&decoder, machine_code[2], 0b1111, MgMnemonic::MgMneRotr, 22));
    assert_eq!(true, check_field_zero_assert(&decoder, machine_code[2], 1, MgMnemonic::MgMneRotr, 21));

    imm_limit_reached(&decoder, MgMnemonic::MgMneSra, machine_code[0], 6, 0x1f, 2);
    imm_limit_reached(&decoder, MgMnemonic::MgMneSrl, machine_code[1], 6, 0x1f, 2);
    imm_limit_reached(&decoder, MgMnemonic::MgMneRotr, machine_code[2], 6, 0x1f, 2);
}
#[test]
fn test_dsll32(){
    let machine_code: u32 = 0x000527FC;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let dsll32 = decoder.disassemble(machine_code, 0).unwrap();

    assert_eq!(MgMnemonic::MgMneDsll32, dsll32.get_mnemonic());
    assert_eq!(MG_MNE_DSLL32, dsll32.get_mnemonic_str());
    assert_eq!(3, dsll32.get_operand_num());
    imm_limit_reached(&decoder, MgMnemonic::MgMneDsll32, machine_code, 6, 0x1f, 2);
    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneDsll32, false, false, true, true));
    assert_eq!(true, check_field(&decoder, machine_code, 0x1f, MgMnemonic::MgMneDsll32, 21));
    assert_eq!(false, dsll32.is_conditional());
    assert_eq!(false, dsll32.is_relative());
    assert_eq!(false, dsll32.is_region());
}
#[test]
fn test_sllv_dsllv(){
    let machine_code: [u32; 2] = [0x03652004, 0x03652014];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let sllv = decoder.disassemble(machine_code[0], 0).unwrap();
    assert_eq!(MgMnemonic::MgMneSllv, sllv.get_mnemonic());
    assert_eq!(MG_MNE_SLLV, sllv.get_mnemonic_str());
    assert_eq!(3, sllv.get_operand_num());
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneSllv, true, true, true, true));
    assert_eq!(true, check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneSllv, 6));
    assert_eq!(false, sllv.is_conditional());
    assert_eq!(false, sllv.is_relative());
    assert_eq!(false, sllv.is_region());

    decoder.version = MgMipsVersion::M64(MgMips64::MgPreR6);
    let dsllv = decoder.disassemble(machine_code[1], 0).unwrap();
    assert_eq!(MgMnemonic::MgMneDsllv, dsllv.get_mnemonic());
    assert_eq!(MG_MNE_DSLLV, dsllv.get_mnemonic_str());
    assert_eq!(3, dsllv.get_operand_num());
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDsllv, false, false, true, true));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0x1f, MgMnemonic::MgMneDsllv, 6));
    assert_eq!(false, dsllv.is_conditional());
    assert_eq!(false, dsllv.is_relative());
    assert_eq!(false, dsllv.is_region());

}
#[test]
fn test_dsrl32_dsra32(){
    let machine_code: [u32; 3] = [0x1FFFFF, 0x1FFFFe, 0x3FFFFe];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let dsra32 = decoder.disassemble(machine_code[0], 0).unwrap();
    let dsrl32 = decoder.disassemble(machine_code[1], 0).unwrap();
    let drotr32 = decoder.disassemble(machine_code[2], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneDsra32, dsra32.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneDsrl32, dsrl32.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneDrotr32, drotr32.get_mnemonic());

    assert_eq!(MG_MNE_DSRA32, dsra32.get_mnemonic_str());
    assert_eq!(MG_MNE_DSRL32, dsrl32.get_mnemonic_str());
    assert_eq!(MG_MNE_DROTR32, drotr32.get_mnemonic_str());

    assert_eq!(3, dsra32.get_operand_num());
    assert_eq!(3, dsrl32.get_operand_num());
    assert_eq!(3, drotr32.get_operand_num());

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDsra32, false, false, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDsrl32, false, false, true, true));
    assert_eq!(true, version_test(machine_code[2], MgMnemonic::MgMneDrotr32, false, false, true, true));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneDsra32, 21));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0x1f, MgMnemonic::MgMneDsrl32, 21));
    assert_eq!(true, check_field_zero_assert(&decoder, machine_code[2], 0x1f, MgMnemonic::MgMneDrotr32, 21));

    imm_limit_reached(&decoder, MgMnemonic::MgMneDsra32, machine_code[0], 21, 0x1f, 2);
    imm_limit_reached(&decoder, MgMnemonic::MgMneDsrl32, machine_code[1], 21, 0x1f, 2);
    imm_limit_reached(&decoder, MgMnemonic::MgMneDrotr32, machine_code[2], 21, 0x1f, 2);
}
#[test]
fn test_dsrlv_dsrav(){
    let machine_code: [u32; 3] = [0x03652017, 0x03652016, 0x03652056];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let dsrav = decoder.disassemble(machine_code[0], 0).unwrap();
    let dsrlv = decoder.disassemble(machine_code[1], 0).unwrap();
    let drotrv = decoder.disassemble(machine_code[2], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneDsrav, dsrav.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneDsrlv, dsrlv.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneDrotrv, drotrv.get_mnemonic());

    assert_eq!(MG_MNE_DSRAV, dsrav.get_mnemonic_str());
    assert_eq!(MG_MNE_DSRLV, dsrlv.get_mnemonic_str());
    assert_eq!(MG_MNE_DROTRV, drotrv.get_mnemonic_str());

    assert_eq!(3, dsrav.get_operand_num());
    assert_eq!(3, dsrlv.get_operand_num());
    assert_eq!(3, drotrv.get_operand_num());

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDsrav, false, false, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDsrlv, false, false, true, true));
    assert_eq!(true, version_test(machine_code[2], MgMnemonic::MgMneDrotrv, false, false, true, true));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneDsrav, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0x1f, MgMnemonic::MgMneDsrlv, 6));
    assert_eq!(true, check_field(&decoder, machine_code[2], 0b1111, MgMnemonic::MgMneDrotrv, 7));
    assert_eq!(true, check_field_zero_assert(&decoder, machine_code[2], 1, MgMnemonic::MgMneDrotrv, 6));
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