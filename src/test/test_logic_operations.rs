//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_ins_dins(){
    let machine_code: [u32; 2] = [(0b00011111 << 26) | 4, (0b00011111 << 26) | 7];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let ins = decoder.disassemble(machine_code[0], 0).unwrap();
    let dins = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(ins.get_mnemonic(), MgMnemonic::MgMneIns);
    assert_eq!(dins.get_mnemonic(), MgMnemonic::MgMneDins);
    assert_eq!(ins.get_mnemonic_str(), MG_MNE_INS);
    assert_eq!(dins.get_mnemonic_str(), MG_MNE_DINS);
    assert_eq!("ins", MG_MNE_INS);
    assert_eq!("dins", MG_MNE_DINS);

    assert_eq!(false, dins.is_conditional());
    assert_eq!(false, dins.is_relative());
    assert_eq!(false, dins.is_region());
    assert_eq!(false, ins.is_conditional());
    assert_eq!(false, ins.is_relative());
    assert_eq!(false, ins.is_region());

    assert_eq!(true, check_operands(&ins, 4));
    assert_eq!(true, check_operands(&dins, 4));
    
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneIns, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDins, false, false, true, true));
    
    imm_limit_reached(&decoder, MgMnemonic::MgMneIns, machine_code[0], 6, 0x1f, 2);
    imm_limit_reached(&decoder, MgMnemonic::MgMneDins, machine_code[1], 6, 0x1f,2);
    imm_limit_reached(&decoder, MgMnemonic::MgMneIns, machine_code[0], 11, 0x1f,3);
    imm_limit_reached(&decoder, MgMnemonic::MgMneDins, machine_code[1], 11, 0x1f,3);
}
#[test]
fn test_dinsm_dinsu(){
    let machine_code: [u32; 2] = [(0b00011111 << 26) | 5, (0b00011111 << 26) | 6];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let dinsm = decoder.disassemble(machine_code[0], 0).unwrap();
    let dinsu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(dinsm.get_mnemonic(), MgMnemonic::MgMneDinsm);
    assert_eq!(dinsu.get_mnemonic(), MgMnemonic::MgMneDinsu);
    assert_eq!(dinsm.get_mnemonic_str(), MG_MNE_DINSM);
    assert_eq!(dinsu.get_mnemonic_str(), MG_MNE_DINSU);
    assert_eq!("dinsm", MG_MNE_DINSM);
    assert_eq!("dinsu", MG_MNE_DINSU);

    assert_eq!(false, dinsu.is_conditional());
    assert_eq!(false, dinsu.is_relative());
    assert_eq!(false, dinsu.is_region());
    assert_eq!(false, dinsm.is_conditional());
    assert_eq!(false, dinsm.is_relative());
    assert_eq!(false, dinsm.is_region());

    // 0b0111110101101101000010001100000

    assert_eq!(true, check_operands(&dinsm, 4));
    assert_eq!(true, check_operands(&dinsu, 4));
    
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDinsm, false, false, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDinsu, false, false, true, true));
    
    imm_limit_reached(&decoder, MgMnemonic::MgMneDinsm, machine_code[0], 6, 0x1f, 2);
    imm_limit_reached(&decoder, MgMnemonic::MgMneDinsu, machine_code[1], 6, 0x1f,2);
    imm_limit_reached(&decoder, MgMnemonic::MgMneDinsm, machine_code[0], 11, 0x1f,3);
    imm_limit_reached(&decoder, MgMnemonic::MgMneDinsu, machine_code[1], 11, 0x1f,3);
}
#[test]
fn test_dextm_dextu(){
    let machine_code: [u32; 2] = [(0b00011111 << 26) | 1, (0b00011111 << 26) | 2];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let dextm = decoder.disassemble(machine_code[0], 0).unwrap();
    let dextu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(dextm.get_mnemonic(), MgMnemonic::MgMneDextm);
    assert_eq!(dextu.get_mnemonic(), MgMnemonic::MgMneDextu);
    assert_eq!(dextm.get_mnemonic_str(), MG_MNE_DEXTM);
    assert_eq!(dextu.get_mnemonic_str(), MG_MNE_DEXTU);
    assert_eq!("dextm", MG_MNE_DEXTM);
    assert_eq!("dextu", MG_MNE_DEXTU);

    assert_eq!(false, dextu.is_conditional());
    assert_eq!(false, dextu.is_relative());
    assert_eq!(false, dextu.is_region());
    assert_eq!(false, dextm.is_conditional());
    assert_eq!(false, dextm.is_relative());
    assert_eq!(false, dextm.is_region());

    // 0b0111110101101101000010001100000

    assert_eq!(true, check_operands(&dextm, 4));
    assert_eq!(true, check_operands(&dextu, 4));
    
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDextm, false, false, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDextu, false, false, true, true));
    
    imm_limit_reached(&decoder, MgMnemonic::MgMneDextm, machine_code[0], 6, 0x1f, 2);
    imm_limit_reached(&decoder, MgMnemonic::MgMneDextu, machine_code[1], 6, 0x1f,2);
    imm_limit_reached(&decoder, MgMnemonic::MgMneDextm, machine_code[0], 11, 0x1f,3);
    imm_limit_reached(&decoder, MgMnemonic::MgMneDextu, machine_code[1], 11, 0x1f,3);
}
#[test]
fn test_ext_dext(){
    let machine_code: [u32; 2] = [0b00011111 << 26, (0b00011111 << 26) | 3];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let ext = decoder.disassemble(machine_code[0], 0).unwrap();
    let dext = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(ext.get_mnemonic(), MgMnemonic::MgMneExt);
    assert_eq!(dext.get_mnemonic(), MgMnemonic::MgMneDext);
    assert_eq!(ext.get_mnemonic_str(), MG_MNE_EXT);
    assert_eq!(dext.get_mnemonic_str(), MG_MNE_DEXT);
    assert_eq!("ext", MG_MNE_EXT);
    assert_eq!("dext", MG_MNE_DEXT);

    assert_eq!(false, dext.is_conditional());
    assert_eq!(false, dext.is_relative());
    assert_eq!(false, dext.is_region());
    assert_eq!(false, ext.is_conditional());
    assert_eq!(false, ext.is_relative());
    assert_eq!(false, ext.is_region());

    // 0b0111110101101101000010001100000

    assert_eq!(true, check_operands(&ext, 4));
    assert_eq!(true, check_operands(&dext, 4));
    
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneExt, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDext, false, false, true, true));
    
    imm_limit_reached(&decoder, MgMnemonic::MgMneExt, machine_code[0], 6, 0x1f, 2);
    imm_limit_reached(&decoder, MgMnemonic::MgMneDext, machine_code[1], 6, 0x1f,2);
    imm_limit_reached(&decoder, MgMnemonic::MgMneExt, machine_code[0], 11, 0x1f,3);
    imm_limit_reached(&decoder, MgMnemonic::MgMneDext, machine_code[1], 11, 0x1f,3);
}
#[test]
fn test_slt_sltu(){
    let machine_code: [u32; 2] = [0x00A2202A, 0x00A2202B];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let slt = decoder.disassemble(machine_code[0], 0).unwrap();
    let sltu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(slt.get_mnemonic(), MgMnemonic::MgMneSlt);
    assert_eq!(sltu.get_mnemonic(), MgMnemonic::MgMneSltu);
    assert_eq!(slt.get_mnemonic_str(), MG_MNE_SLT);
    assert_eq!(sltu.get_mnemonic_str(), MG_MNE_SLTU);
    assert_eq!("slt", MG_MNE_SLT);
    assert_eq!("sltu", MG_MNE_SLTU);

    assert_eq!(true, sltu.is_conditional());
    assert_eq!(false, sltu.is_relative());
    assert_eq!(false, sltu.is_region());
    assert_eq!(true, slt.is_conditional());
    assert_eq!(false, slt.is_relative());
    assert_eq!(false, slt.is_region());

    assert_eq!(true, check_operands(&slt, 3));
    assert_eq!(true, check_operands(&sltu, 3));
    
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneSlt, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneSltu, true, true, true, true));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneSlt, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneSltu, 6));
}
#[test]
fn test_sll_dsll(){
    let machine_code: [u32; 2] = [0x000527C0, 0x000527F8];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let sll = decoder.disassemble(machine_code[0], 0).unwrap();
    assert_eq!(MgMnemonic::MgMneSll, sll.get_mnemonic());
    assert_eq!(MG_MNE_SLL, "sll");

    imm_limit_reached(&decoder, MgMnemonic::MgMneSll, machine_code[0], 6, 0x1f, 2);
    assert_eq!(true, check_operands(&sll, 3));
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneSll, true, true, true, true));
    assert_eq!(true, check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneSll, 21));
    assert_eq!(false, sll.is_conditional());
    assert_eq!(false, sll.is_relative());
    assert_eq!(false, sll.is_region());

    decoder.version = MgMipsVersion::M64(MgMips64::MgPreR6);
    let dsll = decoder.disassemble(machine_code[1], 0).unwrap();
    assert_eq!(MgMnemonic::MgMneDsll, dsll.get_mnemonic());
    assert_eq!(MG_MNE_DSLL, dsll.get_mnemonic_str());
    assert_eq!(true, check_operands(&dsll, 3));
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
    assert_eq!(MG_MNE_DSRA, "dsra");
    assert_eq!(MG_MNE_DSRL, "dsrl");
    assert_eq!(MG_MNE_DROTR, "drotr");

    assert_eq!(true, check_operands(&dsra, 3));
    assert_eq!(true, check_operands(&dsrl, 3));
    assert_eq!(true, check_operands(&drotr, 3));

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
    assert_eq!(MG_MNE_SRA, "sra");
    assert_eq!(MG_MNE_SRL, "srl");
    assert_eq!(MG_MNE_ROTR, "rotr");

    assert_eq!(true, check_operands(&sra, 3));
    assert_eq!(true, check_operands(&srl, 3));
    assert_eq!(true, check_operands(&rotr, 3));

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
    assert_eq!(MG_MNE_DSLL32, "dsll32");

    assert_eq!(true, check_operands(&dsll32, 3));
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
    assert_eq!(MG_MNE_SLLV, "sllv");
    assert_eq!(true, check_operands(&sllv, 3));
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneSllv, true, true, true, true));
    assert_eq!(true, check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneSllv, 6));
    assert_eq!(false, sllv.is_conditional());
    assert_eq!(false, sllv.is_relative());
    assert_eq!(false, sllv.is_region());

    decoder.version = MgMipsVersion::M64(MgMips64::MgPreR6);
    let dsllv = decoder.disassemble(machine_code[1], 0).unwrap();
    assert_eq!(MgMnemonic::MgMneDsllv, dsllv.get_mnemonic());
    assert_eq!(MG_MNE_DSLLV, dsllv.get_mnemonic_str());
    assert_eq!(true, check_operands(&dsllv, 3));
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
    assert_eq!(MG_MNE_DSRA32, "dsra32");
    assert_eq!(MG_MNE_DSRL32, "dsrl32");
    assert_eq!(MG_MNE_DROTR32, "drotr32");

    assert_eq!(true, check_operands(&dsra32, 3));
    assert_eq!(true, check_operands(&dsrl32, 3));
    assert_eq!(true, check_operands(&drotr32, 3));

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
    assert_eq!(MG_MNE_DSRAV, "dsrav");
    assert_eq!(MG_MNE_DSRLV, "dsrlv");
    assert_eq!(MG_MNE_DROTRV, "drotrv");

    assert_eq!(true, check_operands(&dsrav, 3));
    assert_eq!(true, check_operands(&dsrlv, 3));
    assert_eq!(true, check_operands(&drotrv, 3));

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
    assert_eq!(MG_MNE_SRAV, "srav");
    assert_eq!(MG_MNE_SRLV, "srlv");
    assert_eq!(MG_MNE_ROTRV, "rotrv");

    assert_eq!(true, check_operands(&srav, 3));
    assert_eq!(true, check_operands(&srlv, 3));
    assert_eq!(true, check_operands(&rotrv, 3));

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
    assert_eq!("andi", MG_MNE_ANDI);

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
    assert_eq!("ori", MG_MNE_ORI);

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
    assert_eq!("xori", MG_MNE_XORI);

    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneXori, true, true, true, true));

    assert_eq!(true, check_operands(&xori, 3));
}