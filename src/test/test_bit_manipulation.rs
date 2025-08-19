
//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_seh_seb(){
    let machine_code: [u32; 2] = [(0b00011111 << 26) | 0b10000100000, (0b00011111 << 26) | 0b11000100000];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let seb: MgInstruction = decoder.disassemble(machine_code[0], 0).unwrap();
    let seh: MgInstruction = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneSeb, seb.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneSeh, seh.get_mnemonic());
    assert_eq!(MG_MNE_SEB, seb.get_mnemonic_str());
    assert_eq!(MG_MNE_SEH, seh.get_mnemonic_str());
    assert_eq!(MG_MNE_SEB, "seb");
    assert_eq!(MG_MNE_SEH, "seh");

    assert_eq!(2, seb.get_operand_num());
    assert_eq!(2, seh.get_operand_num());

    assert_eq!(false, seb.is_conditional());
    assert_eq!(false, seb.is_relative());
    assert_eq!(false, seb.is_region());
    assert_eq!(false, seh.is_conditional());
    assert_eq!(false, seh.is_relative());
    assert_eq!(false, seh.is_region());

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneSeb, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneSeh, true, true, true, true));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneSeb, 21));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneSeh, 21));
}
#[test]
fn test_wsbh_dsbh(){
    let machine_code: [u32; 2] = [(0b00011111 << 26) | 0b00010100000, (0b00011111 << 26) | 0b00010100100];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let wsbh: MgInstruction = decoder.disassemble(machine_code[0], 0).unwrap();
    let dsbh: MgInstruction = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneWsbh, wsbh.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneDsbh, dsbh.get_mnemonic());
    assert_eq!(MG_MNE_WSBH, wsbh.get_mnemonic_str());
    assert_eq!(MG_MNE_DSBH, dsbh.get_mnemonic_str());
    assert_eq!(MG_MNE_WSBH, "wsbh");
    assert_eq!(MG_MNE_DSBH, "dsbh");

    assert_eq!(2, wsbh.get_operand_num());
    assert_eq!(2, dsbh.get_operand_num());

    assert_eq!(false, wsbh.is_conditional());
    assert_eq!(false, wsbh.is_relative());
    assert_eq!(false, wsbh.is_region());
    assert_eq!(false, dsbh.is_conditional());
    assert_eq!(false, dsbh.is_relative());
    assert_eq!(false, dsbh.is_region());

    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDsbh, false, false, true, true));
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneWsbh, true, true, true, true));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneWsbh, 21));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneDsbh, 21));
}
#[test]
fn test_dshd(){
    let machine_code: u32 = (0b00011111 << 26) | 0b00101100100;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let dshd: MgInstruction = decoder.disassemble(machine_code, 0).unwrap();

    assert_eq!(MgMnemonic::MgMneDshd, dshd.get_mnemonic());
    assert_eq!(MG_MNE_DSHD, dshd.get_mnemonic_str());
    assert_eq!(MG_MNE_DSHD, "dshd");

    assert_eq!(2, dshd.get_operand_num());

    assert_eq!(false, dshd.is_conditional());
    assert_eq!(false, dshd.is_relative());
    assert_eq!(false, dshd.is_region());

    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneDshd, false, false, true, true));
    assert_eq!(true, check_field(&decoder, machine_code, 0b11111, MgMnemonic::MgMneDshd, 21));
}
#[test]
fn test_bitswap_dbitswap(){
    let machine_code: [u32; 2] = [(0b00011111 << 26) | 0b00000100000, (0b00011111 << 26) | 0b00000100100];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let bitswap: MgInstruction = decoder.disassemble(machine_code[0], 0).unwrap();
    let dbitswap: MgInstruction = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneBitswap, bitswap.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneDbitswap, dbitswap.get_mnemonic());
    assert_eq!(MG_MNE_BITSWAP, bitswap.get_mnemonic_str());
    assert_eq!(MG_MNE_DBITSWAP, dbitswap.get_mnemonic_str());
    assert_eq!(MG_MNE_BITSWAP, "bitswap");
    assert_eq!(MG_MNE_DBITSWAP, "dbitswap");

    assert_eq!(2, bitswap.get_operand_num());
    assert_eq!(2, dbitswap.get_operand_num());

    assert_eq!(false, bitswap.is_conditional());
    assert_eq!(false, bitswap.is_relative());
    assert_eq!(false, bitswap.is_region());
    assert_eq!(false, dbitswap.is_conditional());
    assert_eq!(false, dbitswap.is_relative());
    assert_eq!(false, dbitswap.is_region());

    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDbitswap, false, false, false, true));
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneBitswap, false, true, false, true));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneBitswap, 21));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneDbitswap, 21));
}
#[test]
fn test_align_dalign(){
    let machine_code: [u32; 2] = [(0b00011111 << 26) | 0b01010100000, (0b00011111 << 26) | 0b01100100100];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let align: MgInstruction = decoder.disassemble(machine_code[0], 0).unwrap();
    let dalign: MgInstruction = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneAlign, align.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneDalign, dalign.get_mnemonic());
    assert_eq!(MG_MNE_ALIGN, align.get_mnemonic_str());
    assert_eq!(MG_MNE_DALIGN, dalign.get_mnemonic_str());
    assert_eq!(MG_MNE_ALIGN, "align");
    assert_eq!(MG_MNE_DALIGN, "dalign");

    assert_eq!(4, align.get_operand_num());
    assert_eq!(4, dalign.get_operand_num());

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneAlign, machine_code[0], 6, 0x3, 3));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneDalign, machine_code[1], 6, 0x7, 3));

    assert_eq!(false, align.is_conditional());
    assert_eq!(false, align.is_relative());
    assert_eq!(false, align.is_region());
    assert_eq!(false, dalign.is_conditional());
    assert_eq!(false, dalign.is_relative());
    assert_eq!(false, dalign.is_region());

    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDalign, false, false, false, true));
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneAlign, false, true, false, true));
}
#[test]
fn test_rdhwr(){
    let machine_code: u32 = 0x7C04083B;
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let mut rdhwr: MgInstruction = decoder.disassemble(machine_code, 0).unwrap();

    assert_eq!(MgMnemonic::MgMneRdhwr, rdhwr.get_mnemonic());
    assert_eq!(MG_MNE_RDHWR, rdhwr.get_mnemonic_str());
    assert_eq!(MG_MNE_RDHWR, "rdhwr");

    assert_eq!(true, check_operands(&rdhwr, 2));

    assert_eq!(false, rdhwr.is_conditional());
    assert_eq!(false, rdhwr.is_relative());
    assert_eq!(false, rdhwr.is_region());

    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneRdhwr, true, true, true, true));
    assert_eq!(true, check_field(&decoder, machine_code, 0b11111, MgMnemonic::MgMneRdhwr, 21));
    assert_eq!(true, check_field(&decoder, machine_code, 0b11111, MgMnemonic::MgMneRdhwr, 6));

    decoder.version = MgMipsVersion::M64(MgMips64::MgR6);
    rdhwr = decoder.disassemble(machine_code, 0).unwrap();

    assert_eq!(MgMnemonic::MgMneRdhwr, rdhwr.get_mnemonic());
    assert_eq!(MG_MNE_RDHWR, rdhwr.get_mnemonic_str());
    assert_eq!(MG_MNE_RDHWR, "rdhwr");

    assert_eq!(true, check_operands(&rdhwr, 3));

    assert_eq!(false, rdhwr.is_conditional());
    assert_eq!(false, rdhwr.is_relative());
    assert_eq!(false, rdhwr.is_region());

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneRdhwr, machine_code, 6, 0b11, 2));

    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneRdhwr, true, true, true, true));
    assert_eq!(true, check_field(&decoder, machine_code, 0b11111, MgMnemonic::MgMneRdhwr, 21));
    assert_eq!(true, check_field(&decoder, machine_code, 0b11111, MgMnemonic::MgMneRdhwr, 8));
}
#[test]
fn test_daui(){
    let machine_code = [0x770933f1, 0x740933f1];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));
    let daui = decoder.disassemble(machine_code[0], 0).unwrap();
    assert_eq!(MgMnemonic::MgMneDaui, daui.get_mnemonic());
    assert_eq!(MG_MNE_DAUI, daui.get_mnemonic_str());
    assert_eq!(MG_MNE_DAUI, "daui");

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDaui, false, false, false, true));

    assert_eq!(true, check_operands(&daui, 3));

    let Some(MgOperand::MgOpRegister(_)) = daui.get_operand(0) else {
        panic!();
    };
    let Some(MgOperand::MgOpRegister(_)) = daui.get_operand(1) else {
        panic!();
    };
    assert_eq!(true, check_field_zero_assert(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneDaui, 21));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneDaui, machine_code[0], 0, 0xffff, 2));
}
#[test]
fn test_stli_stliu(){
    let machine_code = [0x2B640058, 0x2F640058];

    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let slti = decoder.disassemble(machine_code[0], 0).unwrap();
    let sltiu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(slti.get_mnemonic(), MgMnemonic::MgMneSlti);
    assert_eq!(sltiu.get_mnemonic(), MgMnemonic::MgMneSltiu);
    assert_eq!(slti.get_mnemonic_str(), MG_MNE_SLTI);
    assert_eq!(sltiu.get_mnemonic_str(), MG_MNE_SLTIU);
    assert_eq!("slti", MG_MNE_SLTI);
    assert_eq!("sltiu", MG_MNE_SLTIU);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneSlti, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneSltiu, true, true, true, true));

    assert_eq!(true, check_operands(&slti, 3));
    assert_eq!(true, check_operands(&sltiu, 3));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneSlti, machine_code[0], 0, 0xffff, 2));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSltiu, machine_code[1], 0, 0xffff, 2));
}