//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_synci() {
    let machine_code = 0x041f000f;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let synci = decoder.disassemble(machine_code, 0).unwrap();
    
    assert!(synci.get_mnemonic() == MgMnemonic::MgMneSynci);
    assert!(MG_MNE_SYNCI == synci.get_mnemonic_str());
    assert!(MG_MNE_SYNCI == "synci");

    assert!(false == synci.is_conditional());
    assert!(false == synci.is_relative());
    assert!(false == synci.is_region());

    assert!(1 == synci.get_opcode());
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSynci, machine_code, 0, 0xffff, 0));
    assert!(check_operands(&synci, 2));
    assert!(version_test(machine_code, MgMnemonic::MgMneSynci, true, true, true, true));
}
#[test]
fn test_sync() {
    let machine_code = 0x0000000f;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let sync = decoder.disassemble(machine_code, 0).unwrap();
    
    assert!(sync.get_mnemonic() == MgMnemonic::MgMneSync);
    assert!(MG_MNE_SYNC == sync.get_mnemonic_str());

    assert!(false == sync.is_conditional());
    assert!(false == sync.is_relative());
    assert!(false == sync.is_region());

    assert!(0 == sync.get_opcode());
    assert!(check_field(&decoder, machine_code, 0x7fff, MgMnemonic::MgMneSync, 11));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSync, machine_code, 6, 0x1f, 0));
    assert!(check_operands(&sync, 1));
    assert!(version_test(machine_code, MgMnemonic::MgMneSync, true, true, true, true));
}
#[test]
fn test_scd_lld(){
    let machine_code: [u32; 4] = [0xF3640050, 0xD3640050, 0x7DAD6B27, 0x7DAD6B37];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let mut scd = decoder.disassemble(machine_code[0], 0).unwrap();
    let mut lld = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(scd.get_mnemonic() == MgMnemonic::MgMneScd);
    assert!(lld.get_mnemonic() == MgMnemonic::MgMneLld);
    assert!(mg_get_mnemonic(scd.get_mnemonic()) == MG_MNE_SCD);
    assert!(mg_get_mnemonic(lld.get_mnemonic()) == MG_MNE_LLD);
    assert!("scd" == MG_MNE_SCD);
    assert!("lld" == MG_MNE_LLD);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneScd, false, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneLld, false, false, true, false));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneScd, false, false, false, true));
    assert!(version_test(machine_code[3], MgMnemonic::MgMneLld, false, false, false, true));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneScd, machine_code[0], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneLld, machine_code[1], 0, 0xffff, 1));

    assert!(check_operands(&scd, 3));
    assert!(check_operands(&lld, 3));
    
    assert!(false == scd.is_relative());
    assert!(false == lld.is_relative());

    assert!(scd.is_conditional());
    assert!(false == lld.is_conditional());

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

    assert!(check_field(&decoder, machine_code[2], 1, MgMnemonic::MgMneScd, 6));
    assert!(check_field(&decoder, machine_code[3], 1, MgMnemonic::MgMneLld, 6));
    
    assert!(check_operands(&lld, 3));
    assert!(check_operands(&scd, 3));

    assert!(false == scd.is_relative());
    assert!(false == lld.is_relative());

    assert!(scd.is_conditional());
    assert!(false == lld.is_conditional());

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneScd, machine_code[2], 7, 0b111111111, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLld, machine_code[3], 7, 0b111111111, 1));

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
fn pref(){
    let machine_code: [u32; 2] = [0xCCA1FFFF, (0b011111 << 26) | 0x35];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let mut pref = decoder.disassemble(machine_code[0], 0).unwrap();

    assert!(pref.get_mnemonic() == MgMnemonic::MgMnePref);
    assert!(mg_get_mnemonic(pref.get_mnemonic()) == MG_MNE_PREF);
    assert!("pref" == MG_MNE_PREF);

    assert!(check_operands(&pref, 3));
    assert!(false == pref.is_relative());
    assert!(false == pref.is_conditional());

    assert!(version_test(machine_code[0], MgMnemonic::MgMnePref, true, false, true, false));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMnePref, machine_code[0], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMnePref, machine_code[0], 16, 0x1f, 0));

    //Special3
    decoder.version = MgMipsVersion::M64(MgMips64::MgR6);
    pref = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(pref.get_mnemonic() == MgMnemonic::MgMnePref);
    assert!(mg_get_mnemonic(pref.get_mnemonic()) == MG_MNE_PREF);
    assert!("pref" == MG_MNE_PREF);

    assert!(false == pref.is_conditional());
    assert!(false == pref.is_relative());
    assert!(check_operands(&pref, 3));

    assert!(check_field(&decoder, machine_code[1], 1, MgMnemonic::MgMnePref, 6));
    assert!(version_test(machine_code[1], MgMnemonic::MgMnePref, false, true, false, true));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMnePref, machine_code[1], 7, 0x1ff, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMnePref, machine_code[1], 16, 0x1f, 0));

    match pref.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
}
#[test]
fn prefe(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let machine_code: u32 = (0b011111 << 26) | 0x23;
    let prefe = decoder.disassemble(machine_code, 0).unwrap();

    assert!(prefe.get_mnemonic() == MgMnemonic::MgMnePrefe);
    assert!(mg_get_mnemonic(prefe.get_mnemonic()) == MG_MNE_PREFE);
    assert!("prefe" == MG_MNE_PREFE);

    assert!(false == prefe.is_conditional());
    assert!(false == prefe.is_relative());
    assert!(check_operands(&prefe, 3));

    assert!(check_field(&decoder, machine_code, 1, MgMnemonic::MgMnePrefe, 6));
    assert!(version_test(machine_code, MgMnemonic::MgMnePrefe, true, true, true, true));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMnePrefe, machine_code, 7, 0x1ff, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMnePrefe, machine_code, 16, 0x1f, 0));

    match prefe.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
}
#[test]
fn cachee(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let machine_code: u32 = (0b011111 << 26) | 0b011011;
    let cachee = decoder.disassemble(machine_code, 0).unwrap();

    assert!(cachee.get_mnemonic() == MgMnemonic::MgMneCachee);
    assert!(mg_get_mnemonic(cachee.get_mnemonic()) == MG_MNE_CACHEE);
    assert!("cachee" == MG_MNE_CACHEE);

    assert!(false == cachee.is_conditional());
    assert!(false == cachee.is_relative());
    assert!(check_operands(&cachee, 3));

    assert!(check_field(&decoder, machine_code, 1, MgMnemonic::MgMneCachee, 6));
    assert!(version_test(machine_code, MgMnemonic::MgMneCachee, true, true, true, true));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneCachee, machine_code, 7, 0x1ff, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneCachee, machine_code, 16, 0x1f, 0));

    match cachee.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
}
#[test]
fn cache(){
    let machine_code: [u32; 2] = [0b101111 << 26, (0b011111 << 26) | 0x25];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let mut cache = decoder.disassemble(machine_code[0], 0).unwrap();

    assert!(cache.get_mnemonic() == MgMnemonic::MgMneCache);
    assert!(mg_get_mnemonic(cache.get_mnemonic()) == MG_MNE_CACHE);
    assert!("cache" == MG_MNE_CACHE);

    assert!(check_operands(&cache, 3));
    assert!(false == cache.is_relative());
    assert!(false == cache.is_conditional());

    assert!(version_test(machine_code[0], MgMnemonic::MgMneCache, true, false, true, false));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneCache, machine_code[0], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneCache, machine_code[0], 16, 0x1f, 0));

    //Special3
    decoder.version = MgMipsVersion::M64(MgMips64::MgR6);
    cache = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(cache.get_mnemonic() == MgMnemonic::MgMneCache);
    assert!(mg_get_mnemonic(cache.get_mnemonic()) == MG_MNE_CACHE);
    assert!("cache" == MG_MNE_CACHE);

    assert!(false == cache.is_conditional());
    assert!(false == cache.is_relative());
    assert!(check_operands(&cache, 3));

    assert!(check_field(&decoder, machine_code[1], 1, MgMnemonic::MgMneCache, 6));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneCache, false, true, false, true));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneCache, machine_code[1], 7, 0x1ff, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneCache, machine_code[1], 16, 0x1f, 0));

    match cache.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
}#[test]
fn test_sce_swe(){
    let machine_code: [u32; 2] = [(0b00011111 << 26) | 0x1e, (0b00011111 << 26) | 0x1f];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let sce = decoder.disassemble(machine_code[0], 0).unwrap();
    let swe = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(sce.get_mnemonic() == MgMnemonic::MgMneSce);
    assert!(swe.get_mnemonic() == MgMnemonic::MgMneSwe);
    assert!(mg_get_mnemonic(sce.get_mnemonic()) == MG_MNE_SCE);
    assert!(mg_get_mnemonic(swe.get_mnemonic()) == MG_MNE_SWE);
    assert!("sce" == MG_MNE_SCE);
    assert!("swe" == MG_MNE_SWE);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneSce, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneSwe, true, true, true, true));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneSce, machine_code[0], 7, 0x1ff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSwe, machine_code[1], 7, 0x1ff, 1));

    assert!(check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneSce, 6));
    assert!(check_field(&decoder, machine_code[1], 1, MgMnemonic::MgMneSwe, 6));

    assert!(check_operands(&sce, 3));
    assert!(check_operands(&swe, 3));

    assert!(false == sce.is_relative());
    assert!(false == swe.is_relative());
    assert!(false == sce.is_conditional());
    assert!(false == swe.is_conditional());

    match swe.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match swe.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
}
#[test]
fn test_sbe_she(){
    let machine_code: [u32; 2] = [(0b00011111 << 26) | 0x1c, (0b00011111 << 26) | 0x1d];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let sbe = decoder.disassemble(machine_code[0], 0).unwrap();
    let she = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(sbe.get_mnemonic() == MgMnemonic::MgMneSbe);
    assert!(she.get_mnemonic() == MgMnemonic::MgMneShe);
    assert!(mg_get_mnemonic(sbe.get_mnemonic()) == MG_MNE_SBE);
    assert!(mg_get_mnemonic(she.get_mnemonic()) == MG_MNE_SHE);
    assert!("sbe" == MG_MNE_SBE);
    assert!("she" == MG_MNE_SHE);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneSbe, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneShe, true, true, true, true));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneSbe, machine_code[0], 7, 0x1ff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneShe, machine_code[1], 7, 0x1ff, 1));

    assert!(check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneSbe, 6));
    assert!(check_field(&decoder, machine_code[1], 1, MgMnemonic::MgMneShe, 6));

    assert!(check_operands(&sbe, 3));
    assert!(check_operands(&she, 3));

    assert!(false == sbe.is_relative());
    assert!(false == she.is_relative());
    assert!(false == sbe.is_conditional());
    assert!(false == she.is_conditional());

    match she.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match she.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
}
#[test]
fn test_lce_lwe(){
    let machine_code: [u32; 2] = [(0b00011111 << 26) | 0x2e, (0b00011111 << 26) | 0x2f];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let lce = decoder.disassemble(machine_code[0], 0).unwrap();
    let lwe = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(lce.get_mnemonic() == MgMnemonic::MgMneLce);
    assert!(lwe.get_mnemonic() == MgMnemonic::MgMneLwe);
    assert!(mg_get_mnemonic(lce.get_mnemonic()) == MG_MNE_LCE);
    assert!(mg_get_mnemonic(lwe.get_mnemonic()) == MG_MNE_LWE);
    assert!("lce" == MG_MNE_LCE);
    assert!("lwe" == MG_MNE_LWE);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneLce, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneLwe, true, true, true, true));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLce, machine_code[0], 7, 0x1ff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneLwe, machine_code[1], 7, 0x1ff, 1));

    assert!(check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneLce, 6));
    assert!(check_field(&decoder, machine_code[1], 1, MgMnemonic::MgMneLwe, 6));

    assert!(check_operands(&lce, 3));
    assert!(check_operands(&lwe, 3));

    assert!(false == lce.is_relative());
    assert!(false == lwe.is_relative());
    assert!(false == lce.is_conditional());
    assert!(false == lwe.is_conditional());

    match lwe.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match lwe.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
}
#[test]
fn test_lbe_lhe(){
    let machine_code: [u32; 2] = [(0b00011111 << 26) | 0x2c, (0b00011111 << 26) | 0x2d];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let lbe = decoder.disassemble(machine_code[0], 0).unwrap();
    let lhe = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(lbe.get_mnemonic() == MgMnemonic::MgMneLbe);
    assert!(lhe.get_mnemonic() == MgMnemonic::MgMneLhe);
    assert!(mg_get_mnemonic(lbe.get_mnemonic()) == MG_MNE_LBE);
    assert!(mg_get_mnemonic(lhe.get_mnemonic()) == MG_MNE_LHE);
    assert!("lbe" == MG_MNE_LBE);
    assert!("lhe" == MG_MNE_LHE);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneLbe, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneLhe, true, true, true, true));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLbe, machine_code[0], 7, 0x1ff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneLhe, machine_code[1], 7, 0x1ff, 1));

    assert!(check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneLbe, 6));
    assert!(check_field(&decoder, machine_code[1], 1, MgMnemonic::MgMneLhe, 6));

    assert!(check_operands(&lbe, 3));
    assert!(check_operands(&lhe, 3));

    assert!(false == lbe.is_relative());
    assert!(false == lhe.is_relative());
    assert!(false == lbe.is_conditional());
    assert!(false == lhe.is_conditional());

    match lhe.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match lhe.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
}
#[test]
fn test_lbue_lhue(){
    let machine_code: [u32; 2] = [(0b00011111 << 26) | 0x28, (0b00011111 << 26) | 0x29];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let lbue = decoder.disassemble(machine_code[0], 0).unwrap();
    let lhue = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(lbue.get_mnemonic() == MgMnemonic::MgMneLbue);
    assert!(lhue.get_mnemonic() == MgMnemonic::MgMneLhue);
    assert!(mg_get_mnemonic(lbue.get_mnemonic()) == MG_MNE_LBUE);
    assert!(mg_get_mnemonic(lhue.get_mnemonic()) == MG_MNE_LHUE);
    assert!("lbue" == MG_MNE_LBUE);
    assert!("lhue" == MG_MNE_LHUE);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneLbue, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneLhue, true, true, true, true));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLbue, machine_code[0], 7, 0x1ff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneLhue, machine_code[1], 7, 0x1ff, 1));

    assert!(check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneLbue, 6));
    assert!(check_field(&decoder, machine_code[1], 1, MgMnemonic::MgMneLhue, 6));

    assert!(check_operands(&lbue, 3));
    assert!(check_operands(&lhue, 3));

    assert!(false == lbue.is_relative());
    assert!(false == lhue.is_relative());
    assert!(false == lbue.is_conditional());
    assert!(false == lhue.is_conditional());

    match lhue.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match lhue.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
}
#[test]
fn test_swle_swre(){
    let machine_code: [u32; 2] = [(0b00011111 << 26) | 0x21, (0b00011111 << 26) | 0x22];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let swle = decoder.disassemble(machine_code[0], 0).unwrap();
    let swre = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(swle.get_mnemonic() == MgMnemonic::MgMneSwle);
    assert!(swre.get_mnemonic() == MgMnemonic::MgMneSwre);
    assert!(mg_get_mnemonic(swle.get_mnemonic()) == MG_MNE_SWLE);
    assert!(mg_get_mnemonic(swre.get_mnemonic()) == MG_MNE_SWRE);
    assert!("swle" == MG_MNE_SWLE);
    assert!("swre" == MG_MNE_SWRE);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneSwle, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneSwre, true, false, true, false));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneSwle, machine_code[0], 7, 0x1ff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSwre, machine_code[1], 7, 0x1ff, 1));

    assert!(check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneSwle, 6));
    assert!(check_field(&decoder, machine_code[1], 1, MgMnemonic::MgMneSwre, 6));

    assert!(check_operands(&swle, 3));
    assert!(check_operands(&swre, 3));

    assert!(false == swle.is_relative());
    assert!(false == swre.is_relative());

    assert!(false == swle.is_conditional());
    assert!(false == swre.is_conditional());

    match swre.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match swre.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
}
#[test]
fn test_lwle_lwre(){
    let machine_code: [u32; 2] = [(0b00011111 << 26) | 0x19, (0b00011111 << 26) | 0x1a];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let lwle = decoder.disassemble(machine_code[0], 0).unwrap();
    let lwre = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(lwle.get_mnemonic() == MgMnemonic::MgMneLwle);
    assert!(lwre.get_mnemonic() == MgMnemonic::MgMneLwre);
    assert!(mg_get_mnemonic(lwle.get_mnemonic()) == MG_MNE_LWLE);
    assert!(mg_get_mnemonic(lwre.get_mnemonic()) == MG_MNE_LWRE);
    assert!("lwle" == MG_MNE_LWLE);
    assert!("lwre" == MG_MNE_LWRE);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneLwle, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneLwre, true, false, true, false));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLwle, machine_code[0], 7, 0x1ff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneLwre, machine_code[1], 7, 0x1ff, 1));

    assert!(check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneLwle, 6));
    assert!(check_field(&decoder, machine_code[1], 1, MgMnemonic::MgMneLwre, 6));

    assert!(check_operands(&lwle, 3));
    assert!(check_operands(&lwre, 3));

    assert!(false == lwle.is_relative());
    assert!(false == lwre.is_relative());

    assert!(false == lwle.is_conditional());
    assert!(false == lwre.is_conditional());

    match lwre.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match lwre.get_operand(2){
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


    assert!(sdr.get_mnemonic() == MgMnemonic::MgMneSdr);
    assert!(sdl.get_mnemonic() == MgMnemonic::MgMneSdl);
    assert!(mg_get_mnemonic(sdr.get_mnemonic()) == MG_MNE_SDR);
    assert!(mg_get_mnemonic(sdl.get_mnemonic()) == MG_MNE_SDL);
    assert!("sdr" == MG_MNE_SDR);
    assert!("sdl" == MG_MNE_SDL);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneSdr, false, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneSdl, false, false, true, false));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneSdr, machine_code[0], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSdl, machine_code[1], 0, 0xffff, 1));

    assert!(check_operands(&sdr, 3));
    assert!(check_operands(&sdl, 3));

    assert!(false == sdr.is_relative());
    assert!(false == sdl.is_relative());

    assert!(false == sdr.is_conditional());
    assert!(false == sdl.is_conditional());

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

    assert!(lwu.get_mnemonic() == MgMnemonic::MgMneLwu);
    assert!(mg_get_mnemonic(lwu.get_mnemonic()) == MG_MNE_LWU);
    assert!("lwu" == MG_MNE_LWU);

    assert!(version_test(machine_code, MgMnemonic::MgMneLwu, false, false, true, true));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLwu, machine_code, 0, 0xffff, 1));
    assert!(check_operands(&lwu, 3));


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

    assert!(ld.get_mnemonic() == MgMnemonic::MgMneLd);
    assert!(sd.get_mnemonic() == MgMnemonic::MgMneSd);
    assert!(mg_get_mnemonic(ld.get_mnemonic()) == MG_MNE_LD);
    assert!(mg_get_mnemonic(sd.get_mnemonic()) == MG_MNE_SD);
    assert!("ld" == MG_MNE_LD);
    assert!("sd" == MG_MNE_SD);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneLd, false, false, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneSd, false, false, true, true));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLd, machine_code[0], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSd, machine_code[1], 0, 0xffff, 1));

    assert!(check_operands(&ld, 3));
    assert!(check_operands(&sd, 3));

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
    let sc = decoder.disassemble(machine_code[0], 0).unwrap();
    let ll = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(sc.get_mnemonic() == MgMnemonic::MgMneSc);
    assert!(ll.get_mnemonic() == MgMnemonic::MgMneLl);
    assert!(mg_get_mnemonic(sc.get_mnemonic()) == MG_MNE_SC);
    assert!(mg_get_mnemonic(ll.get_mnemonic()) == MG_MNE_LL);
    assert!("sc" == MG_MNE_SC);
    assert!("ll" == MG_MNE_LL);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneSc, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneLl, true, false, true, false));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneSc, false, true, false, true));
    assert!(version_test(machine_code[3], MgMnemonic::MgMneLl, false, true, false, true));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneSc, machine_code[0], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneLl, machine_code[1], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneSc, machine_code[2], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneLl, machine_code[3], 0, 0xffff, 1));

    assert!(check_operands(&sc, 3));
    assert!(check_operands(&ll, 3));

    assert!(sc.is_conditional());

    assert!(!ll.is_conditional());
    match ll.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match ll.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }

    decoder.version = MgMipsVersion::M32(MgMips32::MgR6);

    assert!(check_field(&decoder, machine_code[3], 1, MgMnemonic::MgMneLl, 6));
    assert!(check_field(&decoder, machine_code[2], 1, MgMnemonic::MgMneSc, 6));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneSc, machine_code[2], 7, 0b111111111, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLl, machine_code[3], 7, 0b111111111, 1));
}
#[test]
fn test_load_store_cp1(){
    let machine_code: [u32; 4] = [0xC4410454, 0xE4410454, 0xD4400454, 0xF4400454];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let lwc1 = decoder.disassemble(machine_code[0], 0).unwrap();
    let swc1 = decoder.disassemble(machine_code[1], 0).unwrap();
    let ldc1 = decoder.disassemble(machine_code[2], 0).unwrap();
    let sdc1 = decoder.disassemble(machine_code[3], 0).unwrap();

    assert!(lwc1.get_mnemonic() == MgMnemonic::MgMneLwc1);
    assert!(swc1.get_mnemonic() == MgMnemonic::MgMneSwc1);
    assert!(ldc1.get_mnemonic() == MgMnemonic::MgMneLdc1);
    assert!(sdc1.get_mnemonic() == MgMnemonic::MgMneSdc1);
    assert!(lwc1.get_mnemonic_str() == MG_MNE_LWC1);
    assert!(swc1.get_mnemonic_str() == MG_MNE_SWC1);
    assert!(ldc1.get_mnemonic_str() == MG_MNE_LDC1);
    assert!(sdc1.get_mnemonic_str() == MG_MNE_SDC1);
    assert!("lwc1" == MG_MNE_LWC1);
    assert!("swc1" == MG_MNE_SWC1);
    assert!("ldc1" == MG_MNE_LDC1);
    assert!("sdc1" == MG_MNE_SDC1);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneLwc1, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneSwc1, true, true, true, true));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneLdc1, true, true, true, true));
    assert!(version_test(machine_code[3], MgMnemonic::MgMneSdc1, true, true, true, true));

    assert!(check_operands(&lwc1, 3));
    assert!(check_operands(&swc1, 3));
    assert!(check_operands(&ldc1, 3));
    assert!(check_operands(&sdc1, 3));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLwc1, machine_code[0], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSwc1, machine_code[1], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLdc1, machine_code[2], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSdc1, machine_code[3], 0, 0xffff, 1));
}
#[test]
fn test_load_store_cp2(){
    let machine_code: [u32; 8] = [0xC8020050, 0xE8020050, 0xD8020050, 0xF8020050, 0x49C00000,0x49400000, 0x49E00000,0x49600000];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let mut lwc2 = decoder.disassemble(machine_code[0], 0).unwrap();
    let mut swc2 = decoder.disassemble(machine_code[1], 0).unwrap();
    let mut ldc2 = decoder.disassemble(machine_code[2], 0).unwrap();
    let mut sdc2 = decoder.disassemble(machine_code[3], 0).unwrap();

    assert!(lwc2.get_mnemonic() == MgMnemonic::MgMneLwc2);
    assert!(swc2.get_mnemonic() == MgMnemonic::MgMneSwc2);
    assert!(ldc2.get_mnemonic() == MgMnemonic::MgMneLdc2);
    assert!(sdc2.get_mnemonic() == MgMnemonic::MgMneSdc2);
    assert!(lwc2.get_mnemonic_str() == MG_MNE_LWC2);
    assert!(swc2.get_mnemonic_str() == MG_MNE_SWC2);
    assert!(ldc2.get_mnemonic_str() == MG_MNE_LDC2);
    assert!(sdc2.get_mnemonic_str() == MG_MNE_SDC2);
    assert!("lwc2" == MG_MNE_LWC2);
    assert!("swc2" == MG_MNE_SWC2);
    assert!("ldc2" == MG_MNE_LDC2);
    assert!("sdc2" == MG_MNE_SDC2);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneLwc2, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneSwc2, true, false, true, false));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneLdc2, true, false, true, false));
    assert!(version_test(machine_code[3], MgMnemonic::MgMneSdc2, true, false, true, false));
    assert!(version_test(machine_code[4], MgMnemonic::MgMneLdc2, false, true, false, true));
    assert!(version_test(machine_code[5], MgMnemonic::MgMneLwc2, false, true, false, true));
    assert!(version_test(machine_code[6], MgMnemonic::MgMneSdc2, false, true, false, true));
    assert!(version_test(machine_code[7], MgMnemonic::MgMneSwc2, false, true, false, true));

    assert!(check_operands(&lwc2, 3));
    assert!(check_operands(&swc2, 3));
    assert!(check_operands(&ldc2, 3));
    assert!(check_operands(&sdc2, 3));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLwc2, machine_code[0], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSwc2, machine_code[1], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLdc2, machine_code[2], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSdc2, machine_code[3], 0, 0xffff, 1));

    //The same machine code is used by other instructions in release6
    decoder.version = MgMipsVersion::M32(MgMips32::MgR6);
    ldc2 = decoder.disassemble(machine_code[4], 0).unwrap();
    lwc2 = decoder.disassemble(machine_code[5], 0).unwrap();
    sdc2 = decoder.disassemble(machine_code[6], 0).unwrap();
    swc2 = decoder.disassemble(machine_code[7], 0).unwrap();

    assert!(ldc2.get_mnemonic() == MgMnemonic::MgMneLdc2);
    assert!(lwc2.get_mnemonic() == MgMnemonic::MgMneLwc2);
    assert!(sdc2.get_mnemonic() == MgMnemonic::MgMneSdc2);
    assert!(swc2.get_mnemonic() == MgMnemonic::MgMneSwc2);

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLdc2, machine_code[4], 7, 0b111111111, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLwc2, machine_code[5], 7, 0b111111111, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSdc2, machine_code[6], 7, 0b111111111, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSwc2, machine_code[7], 7, 0b111111111, 1));

    assert!(check_operands(&lwc2, 3));
    assert!(check_operands(&swc2, 3));
    assert!(check_operands(&ldc2, 3));
    assert!(check_operands(&sdc2, 3));
}
#[test]
fn test_ldr_ldl(){
    let machine_code: [u32; 2] = [0x6CA40050, 0x68A40050];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let ldr = decoder.disassemble(machine_code[0], 0).unwrap();
    let ldl = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(ldr.get_mnemonic() == MgMnemonic::MgMneLdr);
    assert!(ldl.get_mnemonic() == MgMnemonic::MgMneLdl);
    assert!(ldr.get_mnemonic_str() == MG_MNE_LDR);
    assert!(ldl.get_mnemonic_str() == MG_MNE_LDL);
    assert!("ldr" == MG_MNE_LDR);
    assert!("ldl" == MG_MNE_LDL);

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLdr, machine_code[0], 0, 0xffff, 2));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLdl, machine_code[1], 0, 0xffff, 2));

    assert!(version_test(machine_code[0], MgMnemonic::MgMneLdr, false, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneLdl, false, false, true, false));

    assert!(check_operands(&ldr, 3));
    assert!(check_operands(&ldl, 3));
}
#[test]
fn test_lwupc(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));
    let machine_code = 0xEF97FFFF;
    let lwupc = decoder.disassemble(machine_code, 0).unwrap();

    assert!(lwupc.get_mnemonic() == MgMnemonic::MgMneLwupc);
    assert!(lwupc.get_mnemonic_str() == MG_MNE_LWUPC);
    assert!("lwupc" == MG_MNE_LWUPC);

    assert!(lwupc.is_relative());
    assert!(false == lwupc.is_region());
    assert!(false == lwupc.is_conditional());

    assert!(check_operands(&lwupc, 2));
    assert!(version_test(machine_code, MgMnemonic::MgMneLwupc, false, true, false, true));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLwupc, machine_code, 0, 0x7ffff, 1));
}
#[test]
fn test_ldpc(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));
    let machine_code = 0xEF9bFFFF;
    let ldpc = decoder.disassemble(machine_code, 0).unwrap();

    assert!(ldpc.get_mnemonic() == MgMnemonic::MgMneLdpc);
    assert!(ldpc.get_mnemonic_str() == MG_MNE_LDPC);
    assert!("ldpc" == MG_MNE_LDPC);

    assert!(ldpc.is_relative());
    assert!(false == ldpc.is_region());
    assert!(false == ldpc.is_conditional());

    assert!(check_operands(&ldpc, 2));
    assert!(version_test(machine_code, MgMnemonic::MgMneLdpc, false, true, false, true));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLdpc, machine_code, 0, 0x3ffff, 1));
}
#[test]
fn test_lwpc(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));
    let machine_code = 0xEF8fFFFF;
    let lwpc = decoder.disassemble(machine_code, 0).unwrap();

    assert!(lwpc.get_mnemonic() == MgMnemonic::MgMneLwpc);
    assert!(lwpc.get_mnemonic_str() == MG_MNE_LWPC);
    assert!("lwpc" == MG_MNE_LWPC);

    assert!(lwpc.is_relative());
    assert!(false == lwpc.is_region());
    assert!(false == lwpc.is_conditional());

    assert!(check_operands(&lwpc, 2));
    assert!(version_test(machine_code, MgMnemonic::MgMneLwpc, false, true, false, true));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLwpc, machine_code, 0, 0x7ffff, 1));
}
#[test]
fn test_lwr_swr_lwl_swl() {
    let machine_code: [u32; 4] = [0x88450050, 0xA8450050, 0x98450050, 0xB8450050];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    
    let inst = decoder.disassemble(machine_code[0], 0).unwrap();

    assert!(inst.get_mnemonic() == MgMnemonic::MgMneLwl);
    assert!(inst.get_mnemonic_str() == MG_MNE_LWL);
    assert!("lwl" == MG_MNE_LWL);

    assert!(inst.get_operand_num() == 3);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneLwl, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneSwl, true, false, true, false));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneLwr, true, false, true, false));
    assert!(version_test(machine_code[3], MgMnemonic::MgMneSwr, true, false, true, false));

    assert!(decoder.disassemble(machine_code[1], 0).unwrap().get_mnemonic() == MgMnemonic::MgMneSwl);
    assert!(decoder.disassemble(machine_code[2], 0).unwrap().get_mnemonic() == MgMnemonic::MgMneLwr);
    assert!(decoder.disassemble(machine_code[3], 0).unwrap().get_mnemonic() == MgMnemonic::MgMneSwr);

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLwl, machine_code[0], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSwl, machine_code[1], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLwr, machine_code[2], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSwr, machine_code[3], 0, 0xffff, 1));
}
#[test]
fn test_lwxc1_ldxc1(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let machine_code = [0x4EDC05C0, 0x4EDC05C1];
    let lwxc1 = decoder.disassemble(machine_code[0], 0).unwrap();
    let ldxc1 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(lwxc1.get_mnemonic() == MgMnemonic::MgMneLwxc1);
    assert!(ldxc1.get_mnemonic() == MgMnemonic::MgMneLdxc1);
    assert!(lwxc1.get_mnemonic_str() == MG_MNE_LWXC1);
    assert!(ldxc1.get_mnemonic_str() == MG_MNE_LDXC1);
    assert!("lwxc1" == MG_MNE_LWXC1);
    assert!("ldxc1" == MG_MNE_LDXC1);

    assert!(MgCoprocessor::Cp1x == lwxc1.get_coprocessor());
    assert!(MgCoprocessor::Cp1x == ldxc1.get_coprocessor());
    
    assert!(false == lwxc1.is_relative());
    assert!(false == ldxc1.is_relative());
    assert!(false == lwxc1.is_region());
    assert!(false == ldxc1.is_region());
    assert!(false == lwxc1.is_conditional());
    assert!(false == ldxc1.is_conditional());

    assert!(check_operands(&lwxc1, 3));
    assert!(check_operands(&ldxc1, 3));

    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneLwxc1, 11));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneLdxc1, 11));

    assert!(version_test(machine_code[0], MgMnemonic::MgMneLwxc1, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneLdxc1, true, false, true, false));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLwxc1, machine_code[0], 16, 0x1f, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneLdxc1, machine_code[1], 16, 0x1f, 1));

    let Some(MgOperand::MgOpRegister(cpx_reg)) = lwxc1.get_operand(0) else {
        panic!("Operand should've been a register")
    };
    let Some(MgOperand::MgOpRegister(cpu_reg)) = lwxc1.get_operand(2) else {
        panic!("Operand should've been a register")
    };
    assert!(MgCoprocessor::Cpu == cpu_reg.get_coprocessor());
    assert!(MgCoprocessor::Cp1 == cpx_reg.get_coprocessor());

    let Some(MgOperand::MgOpRegister(cpx_reg)) = ldxc1.get_operand(0) else {
        panic!("Operand should've been a register")
    };
    let Some(MgOperand::MgOpRegister(cpu_reg)) = ldxc1.get_operand(2) else {
        panic!("Operand should've been a register")
    };
    assert!(MgCoprocessor::Cpu == cpu_reg.get_coprocessor());
    assert!(MgCoprocessor::Cp1 == cpx_reg.get_coprocessor());
}