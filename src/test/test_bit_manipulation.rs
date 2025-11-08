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

    assert!(MgMnemonic::MgMneSeb == seb.get_mnemonic());
    assert!(MgMnemonic::MgMneSeh == seh.get_mnemonic());
    assert!(MG_MNE_SEB == seb.get_mnemonic_str());
    assert!(MG_MNE_SEH == seh.get_mnemonic_str());
    assert!(MG_MNE_SEB == "seb");
    assert!(MG_MNE_SEH == "seh");

    assert!(check_operands(&seb, 2));
    assert!(check_operands(&seh, 2));

    assert!(!seb.is_conditional());
    assert!(!seb.is_relative());
    assert!(!seb.is_region());
    assert!(!seh.is_conditional());
    assert!(!seh.is_relative());
    assert!(!seh.is_region());

    assert!(version_test(machine_code[0], MgMnemonic::MgMneSeb, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneSeh, true, true, true, true));

    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneSeb, 21));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneSeh, 21));
}
#[test]
fn test_wsbh_dsbh(){
    let machine_code: [u32; 2] = [(0b00011111 << 26) | 0b00010100000, (0b00011111 << 26) | 0b00010100100];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let wsbh: MgInstruction = decoder.disassemble(machine_code[0], 0).unwrap();
    let dsbh: MgInstruction = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneWsbh == wsbh.get_mnemonic());
    assert!(MgMnemonic::MgMneDsbh == dsbh.get_mnemonic());
    assert!(MG_MNE_WSBH == wsbh.get_mnemonic_str());
    assert!(MG_MNE_DSBH == dsbh.get_mnemonic_str());
    assert!(MG_MNE_WSBH == "wsbh");
    assert!(MG_MNE_DSBH == "dsbh");

    assert!(check_operands(&wsbh, 2));
    assert!(check_operands(&dsbh, 2));

    assert!(!wsbh.is_conditional());
    assert!(!wsbh.is_relative());
    assert!(!wsbh.is_region());
    assert!(!dsbh.is_conditional());
    assert!(!dsbh.is_relative());
    assert!(!dsbh.is_region());

    assert!(version_test(machine_code[1], MgMnemonic::MgMneDsbh, false, false, true, true));
    assert!(version_test(machine_code[0], MgMnemonic::MgMneWsbh, true, true, true, true));

    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneWsbh, 21));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneDsbh, 21));
}
#[test]
fn test_dshd(){
    let machine_code: u32 = (0b00011111 << 26) | 0b00101100100;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let dshd: MgInstruction = decoder.disassemble(machine_code, 0).unwrap();

    assert!(MgMnemonic::MgMneDshd == dshd.get_mnemonic());
    assert!(MG_MNE_DSHD == dshd.get_mnemonic_str());
    assert!(MG_MNE_DSHD == "dshd");

    assert!(check_operands(&dshd, 2));

    assert!(!dshd.is_conditional());
    assert!(!dshd.is_relative());
    assert!(!dshd.is_region());

    assert!(version_test(machine_code, MgMnemonic::MgMneDshd, false, false, true, true));
    assert!(check_field(&decoder, machine_code, 0b11111, MgMnemonic::MgMneDshd, 21));
}
#[test]
fn test_bitswap_dbitswap(){
    let machine_code: [u32; 2] = [(0b00011111 << 26) | 0b00000100000, (0b00011111 << 26) | 0b00000100100];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let bitswap: MgInstruction = decoder.disassemble(machine_code[0], 0).unwrap();
    let dbitswap: MgInstruction = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneBitswap == bitswap.get_mnemonic());
    assert!(MgMnemonic::MgMneDbitswap == dbitswap.get_mnemonic());
    assert!(MG_MNE_BITSWAP == bitswap.get_mnemonic_str());
    assert!(MG_MNE_DBITSWAP == dbitswap.get_mnemonic_str());
    assert!(MG_MNE_BITSWAP == "bitswap");
    assert!(MG_MNE_DBITSWAP == "dbitswap");

    assert!(check_operands(&bitswap, 2));
    assert!(check_operands(&dbitswap, 2));

    assert!(!bitswap.is_conditional());
    assert!(!bitswap.is_relative());
    assert!(!bitswap.is_region());
    assert!(!dbitswap.is_conditional());
    assert!(!dbitswap.is_relative());
    assert!(!dbitswap.is_region());

    assert!(version_test(machine_code[1], MgMnemonic::MgMneDbitswap, false, false, false, true));
    assert!(version_test(machine_code[0], MgMnemonic::MgMneBitswap, false, true, false, true));

    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneBitswap, 21));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneDbitswap, 21));
}
#[test]
fn test_align_dalign(){
    let machine_code: [u32; 2] = [(0b00011111 << 26) | 0b01010100000, (0b00011111 << 26) | 0b01100100100];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let align: MgInstruction = decoder.disassemble(machine_code[0], 0).unwrap();
    let dalign: MgInstruction = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneAlign == align.get_mnemonic());
    assert!(MgMnemonic::MgMneDalign == dalign.get_mnemonic());
    assert!(MG_MNE_ALIGN == align.get_mnemonic_str());
    assert!(MG_MNE_DALIGN == dalign.get_mnemonic_str());
    assert!(MG_MNE_ALIGN == "align");
    assert!(MG_MNE_DALIGN == "dalign");

    assert!(check_operands(&align, 4));
    assert!(check_operands(&dalign, 4));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneAlign, machine_code[0], 6, 0x3, 3));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneDalign, machine_code[1], 6, 0x7, 3));

    assert!(!align.is_conditional());
    assert!(!align.is_relative());
    assert!(!align.is_region());
    assert!(!dalign.is_conditional());
    assert!(!dalign.is_relative());
    assert!(!dalign.is_region());

    assert!(version_test(machine_code[1], MgMnemonic::MgMneDalign, false, false, false, true));
    assert!(version_test(machine_code[0], MgMnemonic::MgMneAlign, false, true, false, true));
}
#[test]
fn test_rdhwr(){
    let machine_code: u32 = 0x7C04083B;
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let mut rdhwr: MgInstruction = decoder.disassemble(machine_code, 0).unwrap();

    assert!(MgMnemonic::MgMneRdhwr == rdhwr.get_mnemonic());
    assert!(MG_MNE_RDHWR == rdhwr.get_mnemonic_str());
    assert!(MG_MNE_RDHWR == "rdhwr");

    assert!(check_operands(&rdhwr, 2));

    assert!(!rdhwr.is_conditional());
    assert!(!rdhwr.is_relative());
    assert!(!rdhwr.is_region());

    assert!(version_test(machine_code, MgMnemonic::MgMneRdhwr, true, true, true, true));
    assert!(check_field(&decoder, machine_code, 0b11111, MgMnemonic::MgMneRdhwr, 21));
    assert!(check_field(&decoder, machine_code, 0b11111, MgMnemonic::MgMneRdhwr, 6));

    decoder.version = MgMipsVersion::M64(MgMips64::MgR6);
    rdhwr = decoder.disassemble(machine_code, 0).unwrap();

    assert!(MgMnemonic::MgMneRdhwr == rdhwr.get_mnemonic());
    assert!(MG_MNE_RDHWR == rdhwr.get_mnemonic_str());
    assert!(MG_MNE_RDHWR == "rdhwr");

    assert!(check_operands(&rdhwr, 3));

    assert!(!rdhwr.is_conditional());
    assert!(!rdhwr.is_relative());
    assert!(!rdhwr.is_region());

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneRdhwr, machine_code, 6, 0b11, 2));

    assert!(version_test(machine_code, MgMnemonic::MgMneRdhwr, true, true, true, true));
    assert!(check_field(&decoder, machine_code, 0b11111, MgMnemonic::MgMneRdhwr, 21));
    assert!(check_field(&decoder, machine_code, 0b11111, MgMnemonic::MgMneRdhwr, 8));
}
#[test]
fn test_daui(){
    let machine_code = [0x770933f1, 0x740933f1];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));
    let daui = decoder.disassemble(machine_code[0], 0).unwrap();
    assert!(MgMnemonic::MgMneDaui == daui.get_mnemonic());
    assert!(MG_MNE_DAUI == daui.get_mnemonic_str());
    assert!(MG_MNE_DAUI == "daui");

    assert!(version_test(machine_code[0], MgMnemonic::MgMneDaui, false, false, false, true));

    assert!(check_operands(&daui, 3));

    let Some(MgOperand::MgOpRegister(_)) = daui.get_operand(0) else {
        panic!();
    };
    let Some(MgOperand::MgOpRegister(_)) = daui.get_operand(1) else {
        panic!();
    };
    assert!(check_field_zero_assert(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneDaui, 21));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneDaui, machine_code[0], 0, 0xffff, 2));
}
#[test]
fn test_stli_stliu(){
    let machine_code = [0x2B640058, 0x2F640058];

    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let slti = decoder.disassemble(machine_code[0], 0).unwrap();
    let sltiu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(slti.get_mnemonic() == MgMnemonic::MgMneSlti);
    assert!(sltiu.get_mnemonic() == MgMnemonic::MgMneSltiu);
    assert!(slti.get_mnemonic_str() == MG_MNE_SLTI);
    assert!(sltiu.get_mnemonic_str() == MG_MNE_SLTIU);
    assert!("slti" == MG_MNE_SLTI);
    assert!("sltiu" == MG_MNE_SLTIU);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneSlti, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneSltiu, true, true, true, true));

    assert!(check_operands(&slti, 3));
    assert!(check_operands(&sltiu, 3));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneSlti, machine_code[0], 0, 0xffff, 2));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSltiu, machine_code[1], 0, 0xffff, 2));
}
#[test]
fn test_class(){
    let machine_code: [u32; 2] = [0b01000110000000001001101010011011, 0b01000110001000001001101010011011];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let classs = decoder.disassemble(machine_code[0], 0).unwrap();
    let classd = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(classs.get_mnemonic() == MgMnemonic::MgMneClasss);
    assert!(classd.get_mnemonic() == MgMnemonic::MgMneClassd);
    assert!(classs.get_mnemonic_str() == MG_MNE_CLASSS);
    assert!(classd.get_mnemonic_str() == MG_MNE_CLASSD);
    assert!("class.s" == MG_MNE_CLASSS);
    assert!("class.d" == MG_MNE_CLASSD);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneClasss, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneClassd, false, true, false, true));

    assert!(!classd.is_conditional());
    assert!(!classd.is_relative());
    assert!(!classd.is_region());
    assert!(!classs.is_conditional());
    assert!(!classs.is_relative());
    assert!(!classs.is_region());
    
    assert!(check_operands(&classs, 2));
    assert!(check_operands(&classd, 2));

    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneClasss, 16));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneClassd, 16));

    let Some(MgOperand::MgOpRegister(cp1_reg)) = classs.get_operand(0) else {
        panic!("Operand should've been a register")
    };
    let Some(MgOperand::MgOpRegister(cp1_reg_1)) = classs.get_operand(1) else {
        panic!("Operand should've been a register")
    };
    assert!(MgCoprocessor::Cp1 == cp1_reg.get_coprocessor());
    assert!(MgCoprocessor::Cp1 == cp1_reg_1.get_coprocessor());

    let Some(MgOperand::MgOpRegister(cp1_reg)) = classd.get_operand(0) else {
        panic!("Operand should've been a register")
    };
    let Some(MgOperand::MgOpRegister(cp1_reg_1)) = classd.get_operand(1) else {
        panic!("Operand should've been a register")
    };
    assert!(MgCoprocessor::Cp1 == cp1_reg.get_coprocessor());
    assert!(MgCoprocessor::Cp1 == cp1_reg_1.get_coprocessor());
}
