//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_sync() {
    let machine_code = 0x0000000f;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let sync = decoder.disassemble(machine_code, 0).unwrap();
    
    assert_eq!(sync.get_mnemonic(), MgMnemonic::MgMneSync);
    assert_eq!(MG_MNE_SYNC, sync.get_mnemonic_str());

    assert_eq!(false, sync.is_conditional());
    assert_eq!(false, sync.is_relative());
    assert_eq!(false, sync.is_region());

    assert_eq!(0, sync.get_opcode());
    assert_eq!(true, check_field(&decoder, machine_code, 0x7fff, MgMnemonic::MgMneSync, 11));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSync, machine_code, 6, 0x1f, 0));
    assert_eq!(true, check_operands(&sync, 1));
    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneSync, true, true, true, true));
}
#[test]
fn test_scd_lld(){
    let machine_code: [u32; 4] = [0xF3640050, 0xD3640050, 0x7DAD6B27, 0x7DAD6B37];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let mut scd = decoder.disassemble(machine_code[0], 0).unwrap();
    let mut lld = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(scd.get_mnemonic(), MgMnemonic::MgMneScd);
    assert_eq!(lld.get_mnemonic(), MgMnemonic::MgMneLld);

    assert_eq!(mg_get_mnemonic(scd.get_mnemonic()), MG_MNE_SCD);
    assert_eq!(mg_get_mnemonic(lld.get_mnemonic()), MG_MNE_LLD);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneScd, false, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneLld, false, false, true, false));
    assert_eq!(true, version_test(machine_code[2], MgMnemonic::MgMneScd, false, false, false, true));
    assert_eq!(true, version_test(machine_code[3], MgMnemonic::MgMneLld, false, false, false, true));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneScd, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneLld, machine_code[1], 0, 0xffff, 1));

    assert_eq!(true, check_operands(&scd, 3));
    assert_eq!(true, check_operands(&lld, 3));
    
    assert_eq!(false, scd.is_relative());
    assert_eq!(false, lld.is_relative());

    assert_eq!(true, scd.is_conditional());
    assert_eq!(false, lld.is_conditional());

    match scd.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match scd.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match lld.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match lld.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }

    decoder.version = MgMipsVersion::M64(MgMips64::MgR6);
    scd = decoder.disassemble(machine_code[2], 0).unwrap();
    lld = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(true, check_field(&decoder, machine_code[2], 1, MgMnemonic::MgMneScd, 6));
    assert_eq!(true, check_field(&decoder, machine_code[3], 1, MgMnemonic::MgMneLld, 6));
    
    assert_eq!(true, check_operands(&lld, 3));
    assert_eq!(true, check_operands(&scd, 3));

    assert_eq!(false, scd.is_relative());
    assert_eq!(false, lld.is_relative());

    assert_eq!(true, scd.is_conditional());
    assert_eq!(false, lld.is_conditional());

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneScd, machine_code[2], 7, 0b111111111, 1));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLld, machine_code[3], 7, 0b111111111, 1));

    match scd.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match scd.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match lld.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match lld.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
}
#[test]
fn test_sdr_sdl(){
    let machine_code: [u32; 2] = [0xB7640050, 0xB3640050];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let sdr = decoder.disassemble(machine_code[0], 0).unwrap();
    let sdl = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(sdr.get_mnemonic(), MgMnemonic::MgMneSdr);
    assert_eq!(sdl.get_mnemonic(), MgMnemonic::MgMneSdl);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneSdr, false, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneSdl, false, false, true, false));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneSdr, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSdl, machine_code[1], 0, 0xffff, 1));

    assert_eq!(true, check_operands(&sdr, 3));
    assert_eq!(true, check_operands(&sdl, 3));

    assert_eq!(mg_get_mnemonic(sdr.get_mnemonic()), MG_MNE_SDR);
    assert_eq!(mg_get_mnemonic(sdl.get_mnemonic()), MG_MNE_SDL);

    assert_eq!(false, sdr.is_relative());
    assert_eq!(false, sdl.is_relative());

    assert_eq!(false, sdr.is_conditional());
    assert_eq!(false, sdl.is_conditional());

    match sdl.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match sdl.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
}
#[test]
fn test_lwu(){
    let machine_code: u32 = 0x9CA40004;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let lwu= decoder.disassemble(machine_code, 0).unwrap();

    assert_eq!(lwu.get_mnemonic(), MgMnemonic::MgMneLwu);
    assert_eq!(mg_get_mnemonic(lwu.get_mnemonic()), MG_MNE_LWU);
    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneLwu, false, false, true, true));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLwu, machine_code, 0, 0xffff, 1));
    assert_eq!(true, check_operands(&lwu, 3));


    match lwu.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match lwu.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
}
#[test]
fn test_ld_sd(){
    let machine_code: [u32; 2] = [0xDF640050, 0xFF640050];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let ld = decoder.disassemble(machine_code[0], 0).unwrap();
    let sd = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(ld.get_mnemonic(), MgMnemonic::MgMneLd);
    assert_eq!(sd.get_mnemonic(), MgMnemonic::MgMneSd);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneLd, false, false, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneSd, false, false, true, true));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLd, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSd, machine_code[1], 0, 0xffff, 1));

    assert_eq!(true, check_operands(&ld, 3));
    assert_eq!(true, check_operands(&sd, 3));

    assert_eq!(mg_get_mnemonic(ld.get_mnemonic()), MG_MNE_LD);
    assert_eq!(mg_get_mnemonic(sd.get_mnemonic()), MG_MNE_SD);

    match sd.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match sd.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
}
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
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneSc, machine_code[2], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneLl, machine_code[3], 0, 0xffff, 1));

    assert_eq!(true, check_operands(&inst0, 3));
    assert_eq!(true, check_operands(&inst1, 3));

    assert_eq!(inst0.is_conditional(), true);
    assert_eq!(mg_get_mnemonic(inst0.get_mnemonic()), MG_MNE_SC);

    assert_eq!(inst1.is_conditional(), false);
    assert_eq!(mg_get_mnemonic(inst1.get_mnemonic()), MG_MNE_LL);
    match inst1.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match inst1.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }

    decoder.version = MgMipsVersion::M32(MgMips32::MgR6);

    assert_eq!(true, check_field(&decoder, machine_code[3], 1, MgMnemonic::MgMneLl, 6));
    assert_eq!(true, check_field(&decoder, machine_code[2], 1, MgMnemonic::MgMneSc, 6));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneSc, machine_code[2], 7, 0b111111111, 1));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLl, machine_code[3], 7, 0b111111111, 1));
}
#[test]
fn test_load_store_cp1(){
    let machine_code: [u32; 4] = [0xC4410454, 0xE4410454, 0xD4400454, 0xF4400454];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let lwc1 = decoder.disassemble(machine_code[0], 0).unwrap();
    let swc1 = decoder.disassemble(machine_code[1], 0).unwrap();
    let ldc1 = decoder.disassemble(machine_code[2], 0).unwrap();
    let sdc1 = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(lwc1.get_mnemonic(), MgMnemonic::MgMneLwc1);
    assert_eq!(swc1.get_mnemonic(), MgMnemonic::MgMneSwc1);
    assert_eq!(ldc1.get_mnemonic(), MgMnemonic::MgMneLdc1);
    assert_eq!(sdc1.get_mnemonic(), MgMnemonic::MgMneSdc1);

    assert_eq!(lwc1.get_mnemonic_str(), MG_MNE_LWC1);
    assert_eq!(swc1.get_mnemonic_str(), MG_MNE_SWC1);
    assert_eq!(ldc1.get_mnemonic_str(), MG_MNE_LDC1);
    assert_eq!(sdc1.get_mnemonic_str(), MG_MNE_SDC1);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneLwc1, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneSwc1, true, true, true, true));
    assert_eq!(true, version_test(machine_code[2], MgMnemonic::MgMneLdc1, true, true, true, true));
    assert_eq!(true, version_test(machine_code[3], MgMnemonic::MgMneSdc1, true, true, true, true));

    assert_eq!(true, check_operands(&lwc1, 3));
    assert_eq!(true, check_operands(&swc1, 3));
    assert_eq!(true, check_operands(&ldc1, 3));
    assert_eq!(true, check_operands(&sdc1, 3));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLwc1, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSwc1, machine_code[1], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneLdc1, machine_code[2], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneSdc1, machine_code[3], 0, 0xffff, 1));
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

    assert_eq!(lwc2.get_mnemonic_str(), MG_MNE_LWC2);
    assert_eq!(swc2.get_mnemonic_str(), MG_MNE_SWC2);
    assert_eq!(ldc2.get_mnemonic_str(), MG_MNE_LDC2);
    assert_eq!(sdc2.get_mnemonic_str(), MG_MNE_SDC2);

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