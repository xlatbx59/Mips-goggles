//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_sc_ll(){
    let machine_code: [u32; 4] = [0xE0A2FFFF, 0xC0A2FFFF, 0x7fffffa6, 0x7fffffb6];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(inst0.get_mnemonic(), MgMnemonic::MgMneSc);
    assert_eq!(inst1.get_mnemonic(), MgMnemonic::MgMneLl);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneSc, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneLl, true, false, true, false));
    assert_eq!(true, version_test(machine_code[2], MgMnemonic::MgMneSc, false, true, false, true));
    assert_eq!(true, version_test(machine_code[3], MgMnemonic::MgMneLl, false, true, false, true));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneSc, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneLl, machine_code[1], 0, 0xffff, 1));

    assert_eq!(true, check_operands(&inst0, 3));
    assert_eq!(true, check_operands(&inst1, 3));

    assert_eq!(inst0.is_conditional(), true);
    assert_eq!(mg_get_mnemonic(inst0.get_mnemonic()), MG_MNE_SC);

    assert_eq!(inst1.is_conditional(), true);
    assert_eq!(mg_get_mnemonic(inst1.get_mnemonic()), MG_MNE_LL);
    match inst1.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match inst1.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match inst1.get_operand(1){
        Some(MgOperand::MgOpImmediate(i)) => assert!(i.get_value() <= 0xffff),
        _ => panic!(),
    }

    decoder.version = MgMipsVersion::M32(MgMips32::MgR6);

    assert_eq!(true, check_field(&decoder, machine_code[3], 1, MgMnemonic::MgMneLl, 6));
    assert_eq!(true, check_field(&decoder, machine_code[2], 1, MgMnemonic::MgMneSc, 6));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneSc, machine_code[2], 7, 0b111111111, 1));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLl, machine_code[3], 7, 0b111111111, 1));
}
#[test]
fn test_load_store_cp2(){
    let machine_code: [u32; 8] = [0xC8020050, 0xE8020050, 0xD8020050, 0xF8020050, 0x49C00000,0x49400000, 0x49E00000,0x49600000];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let mut lwc2 = decoder.disassemble(machine_code[0], 0).unwrap();
    let mut swc2 = decoder.disassemble(machine_code[1], 0).unwrap();
    let mut ldc2 = decoder.disassemble(machine_code[2], 0).unwrap();
    let mut sdc2 = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneLwc2, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneSwc2, true, false, true, false));
    assert_eq!(true, version_test(machine_code[2], MgMnemonic::MgMneLdc2, true, false, true, false));
    assert_eq!(true, version_test(machine_code[3], MgMnemonic::MgMneSdc2, true, false, true, false));
    assert_eq!(true, version_test(machine_code[4], MgMnemonic::MgMneLdc2, false, true, false, true));
    assert_eq!(true, version_test(machine_code[5], MgMnemonic::MgMneLwc2, false, true, false, true));
    assert_eq!(true, version_test(machine_code[6], MgMnemonic::MgMneSdc2, false, true, false, true));
    assert_eq!(true, version_test(machine_code[7], MgMnemonic::MgMneSwc2, false, true, false, true));

    assert_eq!(lwc2.get_mnemonic(), MgMnemonic::MgMneLwc2);
    assert_eq!(swc2.get_mnemonic(), MgMnemonic::MgMneSwc2);
    assert_eq!(ldc2.get_mnemonic(), MgMnemonic::MgMneLdc2);
    assert_eq!(sdc2.get_mnemonic(), MgMnemonic::MgMneSdc2);

    assert_eq!(true, check_operands(&lwc2, 3));
    assert_eq!(true, check_operands(&swc2, 3));
    assert_eq!(true, check_operands(&ldc2, 3));
    assert_eq!(true, check_operands(&sdc2, 3));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLwc2, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSwc2, machine_code[1], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLdc2, machine_code[2], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSdc2, machine_code[3], 0, 0xffff, 1));

    //The same machine code is used by other instructions in release6
    decoder.version = MgMipsVersion::M32(MgMips32::MgR6);
    ldc2 = decoder.disassemble(machine_code[4], 0).unwrap();
    lwc2 = decoder.disassemble(machine_code[5], 0).unwrap();
    sdc2 = decoder.disassemble(machine_code[6], 0).unwrap();
    swc2 = decoder.disassemble(machine_code[7], 0).unwrap();

    assert_eq!(ldc2.get_mnemonic(), MgMnemonic::MgMneLdc2);
    assert_eq!(lwc2.get_mnemonic(), MgMnemonic::MgMneLwc2);
    assert_eq!(sdc2.get_mnemonic(), MgMnemonic::MgMneSdc2);
    assert_eq!(swc2.get_mnemonic(), MgMnemonic::MgMneSwc2);

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLdc2, machine_code[4], 7, 0b111111111, 1));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLwc2, machine_code[5], 7, 0b111111111, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSdc2, machine_code[6], 7, 0b111111111, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSwc2, machine_code[7], 7, 0b111111111, 1));

    assert_eq!(true, check_operands(&lwc2, 3));
    assert_eq!(true, check_operands(&swc2, 3));
    assert_eq!(true, check_operands(&ldc2, 3));
    assert_eq!(true, check_operands(&sdc2, 3));
}

#[test]
fn test_ldr_ldl(){
    let machine_code: [u32; 2] = [0x6CA40050, 0x68A40050];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let ldr = decoder.disassemble(machine_code[0], 0).unwrap();
    let ldl = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLdr, machine_code[0], 0, 0xffff, 2));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLdl, machine_code[1], 0, 0xffff, 2));

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneLdr, false, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneLdl, false, false, true, false));

    assert_eq!(ldr.get_mnemonic(), MgMnemonic::MgMneLdr);
    assert_eq!(ldl.get_mnemonic(), MgMnemonic::MgMneLdl);

    assert_eq!(ldr.get_mnemonic_str(), MG_MNE_LDR);
    assert_eq!(ldl.get_mnemonic_str(), MG_MNE_LDL);

    assert_eq!(true, check_operands(&ldr, 3));
    assert_eq!(true, check_operands(&ldl, 3));
}
#[test]
fn test_lwr_swr_lwl_swl() {
    let machine_code: [u32; 4] = [0x88450050, 0xA8450050, 0x98450050, 0xB8450050];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    
    let inst = decoder.disassemble(machine_code[0], 0).unwrap();

    assert_eq!(inst.get_operand_num(), 3);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneLwl, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneSwl, true, false, true, false));
    assert_eq!(true, version_test(machine_code[2], MgMnemonic::MgMneLwr, true, false, true, false));
    assert_eq!(true, version_test(machine_code[3], MgMnemonic::MgMneSwr, true, false, true, false));

    assert_eq!(inst.get_mnemonic(), MgMnemonic::MgMneLwl);
    assert_eq!(decoder.disassemble(machine_code[1], 0).unwrap().get_mnemonic(), MgMnemonic::MgMneSwl);
    assert_eq!(decoder.disassemble(machine_code[2], 0).unwrap().get_mnemonic(), MgMnemonic::MgMneLwr);
    assert_eq!(decoder.disassemble(machine_code[3], 0).unwrap().get_mnemonic(), MgMnemonic::MgMneSwr);

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLwl, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSwl, machine_code[1], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLwr, machine_code[2], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSwr, machine_code[3], 0, 0xffff, 1));
}