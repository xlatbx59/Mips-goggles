//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_dmtc0_mtc0(){
    let machine_code: [u32; 2] = [0x40A20803, 0x40820803];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let dmtc0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let mtc0 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneMtc0 == mtc0.get_mnemonic());
    assert!(MgMnemonic::MgMneDmtc0 == dmtc0.get_mnemonic());

    assert!(MG_MNE_DMTC0 == dmtc0.get_mnemonic_str());
    assert!(MG_MNE_MTC0 == mtc0.get_mnemonic_str());
    assert!(MG_MNE_DMTC0 == "dmtc0");
    assert!(MG_MNE_MTC0 == "mtc0");

    assert!(false == mtc0.is_conditional());
    assert!(false == mtc0.is_relative());
    assert!(false == mtc0.is_region());
    assert!(false == dmtc0.is_conditional());
    assert!(false == dmtc0.is_relative());
    assert!(false == dmtc0.is_region());

    assert!(version_test(machine_code[1], MgMnemonic::MgMneMtc0, true, true, true, true));
    assert!(version_test(machine_code[0], MgMnemonic::MgMneDmtc0, false, false, true, true));

    assert!(check_field(&decoder, machine_code[0], 0xff, MgMnemonic::MgMneDmtc0, 3));
    assert!(check_field(&decoder, machine_code[1], 0xff, MgMnemonic::MgMneMtc0, 3));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneDmtc0, machine_code[0], 0, 7, 2));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneMtc0, machine_code[1], 0, 7, 2));

    assert!(check_operands(&dmtc0, 3));
    assert!(check_operands(&mtc0, 3));
}
#[test]
fn test_dmfc0_mfc0(){
    let machine_code: [u32; 2] = [0x40220803, 0x40020803];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let dmfc0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let mfc0 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneMfc0 == mfc0.get_mnemonic());
    assert!(MgMnemonic::MgMneDmfc0 == dmfc0.get_mnemonic());

    assert!(MG_MNE_DMFC0 == dmfc0.get_mnemonic_str());
    assert!(MG_MNE_MFC0 == mfc0.get_mnemonic_str());
    assert!(MG_MNE_DMFC0 == "dmfc0");
    assert!(MG_MNE_MFC0 == "mfc0");

    assert!(false == mfc0.is_conditional());
    assert!(false == mfc0.is_relative());
    assert!(false == mfc0.is_region());
    assert!(false == dmfc0.is_conditional());
    assert!(false == dmfc0.is_relative());
    assert!(false == dmfc0.is_region());

    assert!(version_test(machine_code[1], MgMnemonic::MgMneMfc0, true, true, true, true));
    assert!(version_test(machine_code[0], MgMnemonic::MgMneDmfc0, false, false, true, true));

    assert!(check_field(&decoder, machine_code[0], 0xff, MgMnemonic::MgMneDmfc0, 3));
    assert!(check_field(&decoder, machine_code[1], 0xff, MgMnemonic::MgMneMfc0, 3));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneDmfc0, machine_code[0], 0, 7, 2));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneMfc0, machine_code[1], 0, 7, 2));

    assert!(check_operands(&dmfc0, 3));
    assert!(check_operands(&mfc0, 3));
}
#[test]
fn test_mfhc0_mthc0(){
    let machine_code: [u32; 2] = [0b01000000010111000001100000000111, 0b01000000110111000001100000000111];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let mfhc0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let mthc0 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneMthc0 == mthc0.get_mnemonic());
    assert!(MgMnemonic::MgMneMfhc0 == mfhc0.get_mnemonic());
    assert!(MG_MNE_MFHC0 == mfhc0.get_mnemonic_str());
    assert!(MG_MNE_MTHC0 == mthc0.get_mnemonic_str());
    assert!(MG_MNE_MFHC0 == "mfhc0");
    assert!(MG_MNE_MTHC0 == "mthc0");

    assert!(false == mthc0.is_conditional());
    assert!(false == mthc0.is_relative());
    assert!(false == mthc0.is_region());
    assert!(false == mfhc0.is_conditional());
    assert!(false == mfhc0.is_relative());
    assert!(false == mfhc0.is_region());

    assert!(version_test(machine_code[1], MgMnemonic::MgMneMthc0, true, true, true, true));
    assert!(version_test(machine_code[0], MgMnemonic::MgMneMfhc0, true, true, true, true));

    assert!(check_field(&decoder, machine_code[0], 0xff, MgMnemonic::MgMneMfhc0, 3));
    assert!(check_field(&decoder, machine_code[1], 0xff, MgMnemonic::MgMneMthc0, 3));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneMfhc0, machine_code[0], 0, 7, 2));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneMthc0, machine_code[1], 0, 7, 2));

    assert!(check_operands(&mfhc0, 3));
    assert!(check_operands(&mthc0, 3));
}
#[test]
fn test_dmtc1_mtc1(){
    let machine_code: [u32; 2] = [0x44A20800, 0x44820800];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let dmtc1 = decoder.disassemble(machine_code[0], 0).unwrap();
    let mtc1 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneMtc1 == mtc1.get_mnemonic());
    assert!(MgMnemonic::MgMneDmtc1 == dmtc1.get_mnemonic());

    assert!(MG_MNE_DMTC1 == dmtc1.get_mnemonic_str());
    assert!(MG_MNE_MTC1 == mtc1.get_mnemonic_str());
    assert!(MG_MNE_DMTC1 == "dmtc1");
    assert!(MG_MNE_MTC1 == "mtc1");

    assert!(false == mtc1.is_conditional());
    assert!(false == mtc1.is_relative());
    assert!(false == mtc1.is_region());
    assert!(false == dmtc1.is_conditional());
    assert!(false == dmtc1.is_relative());
    assert!(false == dmtc1.is_region());

    assert!(version_test(machine_code[1], MgMnemonic::MgMneMtc1, true, true, true, true));
    assert!(version_test(machine_code[0], MgMnemonic::MgMneDmtc1, false, false, true, true));

    assert!(check_field(&decoder, machine_code[0], 0x7ff, MgMnemonic::MgMneDmtc1, 0));
    assert!(check_field(&decoder, machine_code[1], 0x7ff, MgMnemonic::MgMneMtc1, 0));

    assert!(check_operands(&dmtc1, 2));
    assert!(check_operands(&mtc1, 2));
}
#[test]
fn test_dmfc1_mfc1(){
    let machine_code: [u32; 2] = [0x44220800, 0x44020800];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let dmfc1 = decoder.disassemble(machine_code[0], 0).unwrap();
    let mfc1 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneMfc1 == mfc1.get_mnemonic());
    assert!(MgMnemonic::MgMneDmfc1 == dmfc1.get_mnemonic());

    assert!(MG_MNE_DMFC1 == dmfc1.get_mnemonic_str());
    assert!(MG_MNE_MFC1 == mfc1.get_mnemonic_str());
    assert!(MG_MNE_DMFC1 == "dmfc1");
    assert!(MG_MNE_MFC1 == "mfc1");

    assert!(false == mfc1.is_conditional());
    assert!(false == mfc1.is_relative());
    assert!(false == mfc1.is_region());
    assert!(false == dmfc1.is_conditional());
    assert!(false == dmfc1.is_relative());
    assert!(false == dmfc1.is_region());

    assert!(version_test(machine_code[1], MgMnemonic::MgMneMfc1, true, true, true, true));
    assert!(version_test(machine_code[0], MgMnemonic::MgMneDmfc1, false, false, true, true));

    assert!(check_field(&decoder, machine_code[0], 0x7ff, MgMnemonic::MgMneDmfc1, 0));
    assert!(check_field(&decoder, machine_code[1], 0x7ff, MgMnemonic::MgMneMfc1, 0));

    assert!(check_operands(&dmfc1, 2));
    assert!(check_operands(&mfc1, 2));
}
#[test]
fn test_movn_movz(){
    let machine_code: [u32; 2] = [0x00BB200B, 0x00BB200A];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let movn = decoder.disassemble(machine_code[0], 0).unwrap();
    let movz = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneMovz == movz.get_mnemonic());
    assert!(MgMnemonic::MgMneMovn == movn.get_mnemonic());
    assert!(MG_MNE_MOVZ == movz.get_mnemonic_str());
    assert!(MG_MNE_MOVN == movn.get_mnemonic_str());
    assert!(MG_MNE_MOVZ == "movz");
    assert!(MG_MNE_MOVN == "movn");

    assert!(movz.is_conditional());
    assert!(false == movz.is_relative());
    assert!(false == movz.is_region());
    assert!(movn.is_conditional());
    assert!(false == movn.is_relative());
    assert!(false == movn.is_region());

    assert!(MG_MNE_MOVN == movn.get_mnemonic_str());
    assert!(MG_MNE_MOVZ == movz.get_mnemonic_str());

    assert!(version_test(machine_code[0], MgMnemonic::MgMneMovn, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneMovz, true, false, true, false));

    assert!(check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneMovn, 6));
    assert!(check_field(&decoder, machine_code[1], 0x1f, MgMnemonic::MgMneMovz, 6));

    assert!(check_operands(&movn, 3));
    assert!(check_operands(&movz, 3));
}
#[test]
fn test_movci(){
    let machine_code: [u32; 2] = [0x01204001,0x01214001];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let movf = decoder.disassemble(machine_code[0], 0).unwrap();
    let movt = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneMovt == movt.get_mnemonic());
    assert!(MgMnemonic::MgMneMovf == movf.get_mnemonic());
    assert!(MG_MNE_MOVF == movf.get_mnemonic_str());
    assert!(MG_MNE_MOVT == movt.get_mnemonic_str());
    assert!(MG_MNE_MOVF == "movf");
    assert!(MG_MNE_MOVT == "movt");

    assert!(version_test(machine_code[0], MgMnemonic::MgMneMovf, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneMovt, true, false, true, false));

    assert!(check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneMovf, 6));
    assert!(check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneMovf, 17));
    assert!(check_field(&decoder, machine_code[1], 0x1f, MgMnemonic::MgMneMovt, 6));
    assert!(check_field(&decoder, machine_code[1], 1, MgMnemonic::MgMneMovt, 17));

    assert!(check_operands(&movf, 3));
    assert!(check_operands(&movt, 3));
}
#[test]
fn test_mfhi_mflo(){
    let machine_code: [u32; 2] = [0x00002010, 0x00002012];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let mfhi = decoder.disassemble(machine_code[0], 0).unwrap();
    let mflo = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(mfhi.get_mnemonic() == MgMnemonic::MgMneMfhi);
    assert!(mflo.get_mnemonic() == MgMnemonic::MgMneMflo);
    assert!(mfhi.get_mnemonic_str() == MG_MNE_MFHI);
    assert!(mflo.get_mnemonic_str() == MG_MNE_MFLO);
    assert!("mfhi" == MG_MNE_MFHI);
    assert!("mflo" == MG_MNE_MFLO);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneMfhi, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneMflo, true, false, true, false));

    assert!(check_field(&decoder, machine_code[0], 0x3ff, MgMnemonic::MgMneMfhi, 16));
    assert!(check_field(&decoder, machine_code[1], 0x3ff, MgMnemonic::MgMneMflo, 16));
    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneMfhi, 6));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneMflo, 6));

    assert!(check_operands(&mfhi, 1));
    assert!(check_operands(&mflo, 1));
}
#[test]
fn test_mthi_mtlo(){
    let machine_code: [u32; 2] = [0x00800011, 0x00800013];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let mthi = decoder.disassemble(machine_code[0], 0).unwrap();
    let mtlo = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(mthi.get_mnemonic() == MgMnemonic::MgMneMthi);
    assert!(mtlo.get_mnemonic() == MgMnemonic::MgMneMtlo);
    assert!(mthi.get_mnemonic_str() == MG_MNE_MTHI);
    assert!(mtlo.get_mnemonic_str() == MG_MNE_MTLO);
    assert!("mthi" == MG_MNE_MTHI);
    assert!("mtlo" == MG_MNE_MTLO);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneMthi, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneMtlo, true, false, true, false));

    assert!(check_field(&decoder, machine_code[0], 0x7fff, MgMnemonic::MgMneMthi, 16));
    assert!(check_field(&decoder, machine_code[1], 0x7fff, MgMnemonic::MgMneMtlo, 16));

    assert!(check_operands(&mthi, 1));
    assert!(check_operands(&mtlo, 1));
}
#[test]
fn test_rdpgpr_wrpgpr(){
    let machine_code: [u32; 2] = [0x415C1800, 0x41DC1800];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let rdpgpr = decoder.disassemble(machine_code[0], 0).unwrap();
    let wdpgpr = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneWrpgpr == wdpgpr.get_mnemonic());
    assert!(MgMnemonic::MgMneRdpgpr == rdpgpr.get_mnemonic());

    assert!(MG_MNE_RDPGPR == rdpgpr.get_mnemonic_str());
    assert!(MG_MNE_WRPGPR == wdpgpr.get_mnemonic_str());
    assert!(MG_MNE_RDPGPR == "rdpgpr");
    assert!(MG_MNE_WRPGPR == "wrpgpr");

    assert!(false == wdpgpr.is_conditional());
    assert!(false == wdpgpr.is_relative());
    assert!(false == wdpgpr.is_region());
    assert!(false == rdpgpr.is_conditional());
    assert!(false == rdpgpr.is_relative());
    assert!(false == rdpgpr.is_region());

    assert!(version_test(machine_code[1], MgMnemonic::MgMneWrpgpr, true, true, true, true));
    assert!(version_test(machine_code[0], MgMnemonic::MgMneRdpgpr, true, true, true, true));

    assert!(check_field(&decoder, machine_code[0], 0x7ff, MgMnemonic::MgMneRdpgpr, 0));
    assert!(check_field(&decoder, machine_code[1], 0x7ff, MgMnemonic::MgMneWrpgpr, 0));

    assert!(check_operands(&rdpgpr, 2));
    assert!(check_operands(&wdpgpr, 2));
}
#[test]
fn test_cfc1_mfhc1(){
    let machine_code: [u32; 2] = [0x44440800, 0x447f0800];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let cfc1 = decoder.disassemble(machine_code[0], 0).unwrap();
    let mfhc1 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneCfc1 == cfc1.get_mnemonic());
    assert!(MgMnemonic::MgMneMfhc1 == mfhc1.get_mnemonic());

    assert!(MG_MNE_MFHC1 == mfhc1.get_mnemonic_str());
    assert!(MG_MNE_CFC1 == cfc1.get_mnemonic_str());
    assert!(MG_MNE_MFHC1 == "mfhc1");
    assert!(MG_MNE_CFC1 == "cfc1");

    assert!(false == cfc1.is_conditional());
    assert!(false == cfc1.is_relative());
    assert!(false == cfc1.is_region());
    assert!(false == mfhc1.is_conditional());
    assert!(false == mfhc1.is_relative());
    assert!(false == mfhc1.is_region());

    assert!(version_test(machine_code[0], MgMnemonic::MgMneCfc1, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneMfhc1, true, true, true, true));

    assert!(check_field(&decoder, machine_code[1], 0x7ff, MgMnemonic::MgMneMfhc1, 0));
    assert!(check_field(&decoder, machine_code[0], 0x7ff, MgMnemonic::MgMneCfc1, 0));

    assert!(check_operands(&mfhc1, 2));
    assert!(check_operands(&cfc1, 2));
}
#[test]
fn test_ctc1_mthc1(){
    let machine_code: [u32; 2] = [0x44C40800, 0x44E40800];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let ctc1 = decoder.disassemble(machine_code[0], 0).unwrap();
    let mthc1 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneCtc1 == ctc1.get_mnemonic());
    assert!(MgMnemonic::MgMneMthc1 == mthc1.get_mnemonic());

    assert!(MG_MNE_MTHC1 == mthc1.get_mnemonic_str());
    assert!(MG_MNE_CTC1 == ctc1.get_mnemonic_str());
    assert!(MG_MNE_MTHC1 == "mthc1");
    assert!(MG_MNE_CTC1 == "ctc1");

    assert!(!ctc1.is_conditional());
    assert!(!ctc1.is_relative());
    assert!(!ctc1.is_region());
    assert!(!mthc1.is_conditional());
    assert!(!mthc1.is_relative());
    assert!(!mthc1.is_region());

    assert!(version_test(machine_code[0], MgMnemonic::MgMneCtc1, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneMthc1, true, true, true, true));

    assert!(check_field(&decoder, machine_code[1], 0x7ff, MgMnemonic::MgMneMthc1, 0));
    assert!(check_field(&decoder, machine_code[0], 0x7ff, MgMnemonic::MgMneCtc1, 0));

    assert!(check_operands(&mthc1, 2));
    assert!(check_operands(&ctc1, 2));
}
#[test]
fn test_sel_cp1(){
    let machine_code: [u32; 2] = [0b01000110000000010001000011010000, 0b01000110001000010001000011010000];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let sels = decoder.disassemble(machine_code[0], 0).unwrap();
    let seld = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(sels.get_mnemonic() == MgMnemonic::MgMneSels);
    assert!(seld.get_mnemonic() == MgMnemonic::MgMneSeld);
    assert!(sels.get_mnemonic_str() == MG_MNE_SELS);
    assert!(seld.get_mnemonic_str() == MG_MNE_SELD);
    assert!("sel.s" == MG_MNE_SELS);
    assert!("sel.d" == MG_MNE_SELD);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneSels, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneSeld, false, true, false, true));

    assert!(seld.is_conditional());
    assert!(!seld.is_relative());
    assert!(!seld.is_region());

    assert!(sels.is_conditional());
    assert!(!sels.is_relative());
    assert!(!sels.is_region());
    
    assert!(check_operands(&sels, 3));
    assert!(check_operands(&seld, 3));

    let (Some(MgOperand::MgOpRegister(cp1_reg_1)), Some(MgOperand::MgOpRegister(cp1_reg_2)), Some(MgOperand::MgOpRegister(cp1_reg_3))) = (sels.get_operand(0), sels.get_operand(1),  sels.get_operand(2) )else {
        panic!("Operands should've been registers")
    };
    assert!(MgCoprocessor::Cp1 == cp1_reg_1.get_coprocessor());
    assert!(MgCoprocessor::Cp1 == cp1_reg_2.get_coprocessor());
    assert!(MgCoprocessor::Cp1 == cp1_reg_3.get_coprocessor());

    let (Some(MgOperand::MgOpRegister(cp1_reg_1)), Some(MgOperand::MgOpRegister(cp1_reg_2)), Some(MgOperand::MgOpRegister(cp1_reg_3))) = (seld.get_operand(0), seld.get_operand(1),  seld.get_operand(2) )else {
        panic!("Operands should've been registers")
    };
    assert!(MgCoprocessor::Cp1 == cp1_reg_1.get_coprocessor());
    assert!(MgCoprocessor::Cp1 == cp1_reg_2.get_coprocessor());
    assert!(MgCoprocessor::Cp1 == cp1_reg_3.get_coprocessor());
}
