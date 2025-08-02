//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use crate::*;
use super::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_syscall() {
    let machine_code = 0x00008E0C;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let syscall = decoder.disassemble(machine_code, 0).unwrap();
    
    assert_eq!(syscall.get_mnemonic(), MgMnemonic::MgMneSyscall);
    assert_eq!(MG_MNE_SYSCALL, syscall.get_mnemonic_str());

    assert_eq!(false, syscall.is_conditional());
    assert_eq!(false, syscall.is_relative());
    assert_eq!(false, syscall.is_region());

    assert_eq!(0, syscall.get_opcode());
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSyscall, machine_code, 6, 0xfffff, 0));
    assert_eq!(true, check_operands(&syscall, 1));
    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneSyscall, true, true, true, true));
}
#[test]
fn test_break() {
    let machine_code = 0x0238000D;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let i_break = decoder.disassemble(machine_code, 0).unwrap();
    
    assert_eq!(i_break.get_mnemonic(), MgMnemonic::MgMneBreak);
    assert_eq!(MG_MNE_BREAK, i_break.get_mnemonic_str());

    assert_eq!(false, i_break.is_conditional());
    assert_eq!(false, i_break.is_relative());
    assert_eq!(false, i_break.is_region());

    assert_eq!(0, i_break.get_opcode());
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBreak, machine_code, 6, 0xfffff, 0));
    assert_eq!(true, check_operands(&i_break, 1));
    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneBreak, true, true, true, true));
}
#[test]
fn test_sdbbp() {
    let mut machine_code = 0x0238000E;
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let mut sdbbp = decoder.disassemble(machine_code, 0).unwrap();
    
    assert_eq!(sdbbp.get_mnemonic(), MgMnemonic::MgMneSdbbp);
    assert_eq!(MG_MNE_SDBBP, sdbbp.get_mnemonic_str());

    assert_eq!(false, sdbbp.is_conditional());
    assert_eq!(false, sdbbp.is_relative());
    assert_eq!(false, sdbbp.is_region());

    assert_eq!(0, sdbbp.get_opcode());
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSdbbp, machine_code, 6, 0xfffff, 0));
    assert_eq!(true, check_operands(&sdbbp, 1));
    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneSdbbp, false, true, false, true));

    machine_code = 0x70008E3F;
    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);
    sdbbp = decoder.disassemble(machine_code, 0).unwrap();

    assert_eq!(sdbbp.get_mnemonic(), MgMnemonic::MgMneSdbbp);
    assert_eq!(MG_MNE_SDBBP, sdbbp.get_mnemonic_str());

    assert_eq!(0b011100, sdbbp.get_opcode());
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSdbbp, machine_code, 6, 0xfffff, 0));
    assert_eq!(true, check_operands(&sdbbp, 1));
    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneSdbbp, true, false, true, false));
}
#[test]
fn test_tne_teq() {
    let machine_code: [u32; 2] = [0x00460036, 0x00400034];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let tne = decoder.disassemble(machine_code[0], 0).unwrap();
    let teq = decoder.disassemble(machine_code[1], 0).unwrap();
    
    //No problem
    assert_eq!(tne.get_mnemonic(), MgMnemonic::MgMneTne);
    assert_eq!(teq.get_mnemonic(), MgMnemonic::MgMneTeq);

    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneTne, machine_code[0], 6, 0x3ff, 2));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneTeq, machine_code[1], 6, 0x3ff, 2));

    assert_eq!(true, check_operands(&tne, 3));
    assert_eq!(true, check_operands(&teq, 3));

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneTne, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneTeq, true, true, true, true));
}