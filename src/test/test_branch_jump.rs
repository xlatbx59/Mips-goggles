//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_bne_beq(){
    let machine_code: [u32; 2] = [0x1485C013, 0x1085C013];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let bne: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let beq: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MG_MNE_BNE, "bne");
    assert_eq!(MG_MNE_BEQ, "beq");
    assert_eq!(MG_MNE_BNE, bne.get_mnemonic_str());
    assert_eq!(MG_MNE_BEQ, beq.get_mnemonic_str());
    assert_eq!(MgMnemonic::MgMneBne, bne.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneBeq, beq.get_mnemonic());

    assert_eq!(true, bne.is_relative());
    assert_eq!(true, beq.is_relative());

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneBne, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneBeq, true, true, true, true));

    assert_eq!(true, check_operands(&bne, 3));
    assert_eq!(true, check_operands(&beq, 3));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBeq, machine_code[1], 0, 0xffff, 2));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBne, machine_code[0], 0, 0xffff, 2));
}
#[test]
fn test_bnel_beql(){
    let machine_code: [u32; 2] = [0x5485C114, 0x5085C114];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let bnel: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let beql: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MG_MNE_BNEL, "bnel");
    assert_eq!(MG_MNE_BEQL, "beql");
    assert_eq!(MG_MNE_BNEL, bnel.get_mnemonic_str());
    assert_eq!(MG_MNE_BEQL, beql.get_mnemonic_str());
    assert_eq!(MgMnemonic::MgMneBnel, bnel.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneBeql, beql.get_mnemonic());

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneBnel, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneBeql, true, false, true, false));

    assert_eq!(true, bnel.is_relative());
    assert_eq!(true, beql.is_relative());

    assert_eq!(true, check_operands(&bnel, 3));
    assert_eq!(true, check_operands(&beql, 3));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBeql, machine_code[1], 0, 0xffff, 2));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBnel, machine_code[0], 0, 0xffff, 2));
}
#[test]
fn test_jr(){
    let machine_code = [0x00800008, 0x00800408];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let jr: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let jrhb: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneJrhb, jrhb.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneJr, jr.get_mnemonic());
    assert_eq!("jr.hb", MG_MNE_JRHB);
    assert_eq!("jr", MG_MNE_JR);

    assert_eq!(MG_MNE_JRHB, jrhb.get_mnemonic_str());
    assert_eq!(MG_MNE_JR, jr.get_mnemonic_str());

    assert_eq!(true, check_operands(&jr, 1));
    assert_eq!(true, check_operands(&jrhb, 1));

    assert_eq!(false, jr.is_relative());
    assert_eq!(false, jrhb.is_relative());
    assert_eq!(false, jr.is_region());
    assert_eq!(false, jrhb.is_region());

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneJr, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneJrhb, true, false, true, false));
}
#[test]
fn test_bltzall_bgezall(){
    let machine_code: [u32; 2] = [0x0492C00D, 0x0493C00D];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let bltzall: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let bgezall: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneBltzall, bltzall.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneBgezall, bgezall.get_mnemonic());
    assert_eq!("bltzall", MG_MNE_BLTZALL);
    assert_eq!("bgezall", MG_MNE_BGEZALL);

    assert_eq!(MG_MNE_BLTZALL, bltzall.get_mnemonic_str());
    assert_eq!(MG_MNE_BGEZALL, bgezall.get_mnemonic_str());

    assert_eq!(true, check_operands(&bltzall, 2));
    assert_eq!(true, check_operands(&bgezall, 2));

    assert_eq!(true, bltzall.is_conditional());
    assert_eq!(true, bltzall.is_relative());
    assert_eq!(false, bltzall.is_region());
    assert_eq!(true, bgezall.is_conditional());
    assert_eq!(true, bgezall.is_relative());
    assert_eq!(false, bgezall.is_region());

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBgezall, machine_code[1], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBltzall, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneBltzall, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneBgezall, true, false, true, false));
}
#[test]
fn test_nal_bal(){
    let machine_code: [u32; 2] = [0x0410C00D, 0x0411C00D];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let nal: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let bal: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneNal, nal.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneBal, bal.get_mnemonic());
    assert_eq!("nal", MG_MNE_NAL);
    assert_eq!("bal", MG_MNE_BAL);

    assert_eq!(MG_MNE_NAL, nal.get_mnemonic_str());
    assert_eq!(MG_MNE_BAL, bal.get_mnemonic_str());

    assert_eq!(true, check_operands(&nal, 1));
    assert_eq!(true, check_operands(&bal, 1));

    assert_eq!(false, nal.is_conditional());
    assert_eq!(true, nal.is_relative());
    assert_eq!(false, nal.is_region());
    assert_eq!(false, bal.is_conditional());
    assert_eq!(true, bal.is_relative());
    assert_eq!(false, bal.is_region());

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBal, machine_code[1], 0, 0xffff, 0));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneNal, machine_code[0], 0, 0xffff, 0));
    assert_eq!(true, check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneNal, 21));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0x1f, MgMnemonic::MgMneBal, 21));
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneNal, false, true, false, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneBal, false, true, false, true));
}
#[test]
fn test_bltzal_bgezal(){
    let machine_code: [u32; 2] = [0x0490C00D, 0x0491C00D];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let bltzal: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let bgezal: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneBltzal, bltzal.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneBgezal, bgezal.get_mnemonic());
    assert_eq!("bltzal", MG_MNE_BLTZAL);
    assert_eq!("bgezal", MG_MNE_BGEZAL);

    assert_eq!(MG_MNE_BLTZAL, bltzal.get_mnemonic_str());
    assert_eq!(MG_MNE_BGEZAL, bgezal.get_mnemonic_str());

    assert_eq!(true, check_operands(&bltzal, 2));
    assert_eq!(true, check_operands(&bgezal, 2));

    assert_eq!(true, bltzal.is_conditional());
    assert_eq!(true, bltzal.is_relative());
    assert_eq!(false, bltzal.is_region());
    assert_eq!(true, bgezal.is_conditional());
    assert_eq!(true, bgezal.is_relative());
    assert_eq!(false, bgezal.is_region());

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBgezal, machine_code[1], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBltzal, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneBltzal, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneBgezal, true, false, true, false));
}
#[test]
fn test_bltz_bgez(){
    let machine_code: [u32; 2] = [0x0480C00D, 0x0481C00D];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let bltz: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let bgez: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneBltz, bltz.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneBgez, bgez.get_mnemonic());
    assert_eq!("bltz", MG_MNE_BLTZ);
    assert_eq!("bgez", MG_MNE_BGEZ);

    assert_eq!(MG_MNE_BLTZ, bltz.get_mnemonic_str());
    assert_eq!(MG_MNE_BGEZ, bgez.get_mnemonic_str());

    assert_eq!(true, check_operands(&bltz, 2));
    assert_eq!(true, check_operands(&bgez, 2));

    assert_eq!(true, bltz.is_conditional());
    assert_eq!(true, bltz.is_relative());
    assert_eq!(false, bltz.is_region());
    assert_eq!(true, bgez.is_conditional());
    assert_eq!(true, bgez.is_relative());
    assert_eq!(false, bgez.is_region());

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBgez, machine_code[1], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBltz, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneBltz, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneBgez, true, false, true, false));
}
#[test]
fn test_bltzl_bgezl(){
    let machine_code: [u32; 2] = [0x0482C00D, 0x0483C00D];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let bltzl: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let bgezl: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneBltzl, bltzl.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneBgezl, bgezl.get_mnemonic());
    assert_eq!("bltzl", MG_MNE_BLTZL);
    assert_eq!("bgezl", MG_MNE_BGEZL);

    assert_eq!(MG_MNE_BLTZL, bltzl.get_mnemonic_str());
    assert_eq!(MG_MNE_BGEZL, bgezl.get_mnemonic_str());

    assert_eq!(true, check_operands(&bltzl, 2));
    assert_eq!(true, check_operands(&bgezl, 2));

    assert_eq!(true, bltzl.is_conditional());
    assert_eq!(true, bltzl.is_relative());
    assert_eq!(false, bltzl.is_region());
    assert_eq!(true, bgezl.is_conditional());
    assert_eq!(true, bgezl.is_relative());
    assert_eq!(false, bgezl.is_region());

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBgezl, machine_code[1], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBltzl, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneBltzl, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneBgezl, true, false, true, false));
}
#[test]
fn test_jalr(){
    let machine_code = [0x03602009, 0x03602409];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let jalr: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let jalrhb: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneJalrhb, jalrhb.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneJalr, jalr.get_mnemonic());
    assert_eq!("jalr.hb", MG_MNE_JALRHB);
    assert_eq!("jalr", MG_MNE_JALR);

    assert_eq!(MG_MNE_JALRHB, jalrhb.get_mnemonic_str());
    assert_eq!(MG_MNE_JALR, jalr.get_mnemonic_str());

    assert_eq!(true, check_operands(&jalr, 2));
    assert_eq!(true, check_operands(&jalrhb, 2));

    assert_eq!(false, jalr.is_relative());
    assert_eq!(false, jalrhb.is_relative());
    assert_eq!(false, jalr.is_region());
    assert_eq!(false, jalrhb.is_region());

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneJalr, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneJalrhb, true, true, true, true));
}
#[test]
fn test_jalx(){
    let machine_code = 0x74000115;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let jalx: MgInstruction= decoder.disassemble(machine_code, 0).unwrap();

    assert_eq!(MgMnemonic::MgMneJalx, jalx.get_mnemonic());
    assert_eq!(MG_MNE_JALX, jalx.get_mnemonic_str());
    assert_eq!(MG_MNE_JALX, "jalx");

    assert_eq!(true, check_operands(&jalx, 1));
    assert_eq!(true, jalx.is_region());
    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneJalx, true, false, true, false));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneJalx, machine_code, 0, 0x3ffffff, 0));
}
#[test]
fn test_bc_balc(){
    let machine_code: [u32; 2] = [0xC8020050, 0xE8020050];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let bc = decoder.disassemble(machine_code[0], 0).unwrap();
    let balc = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(bc.get_mnemonic(), MgMnemonic::MgMneBc);
    assert_eq!(balc.get_mnemonic(), MgMnemonic::MgMneBalc);
    assert_eq!(bc.get_mnemonic_str(), MG_MNE_BC);
    assert_eq!(balc.get_mnemonic_str(), MG_MNE_BALC);
    assert_eq!("bc", MG_MNE_BC);
    assert_eq!("balc", MG_MNE_BALC);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneBc, false, true, false, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneBalc, false, true, false, true));

    assert_eq!(true, check_operands(&bc, 1));
    assert_eq!(true, check_operands(&balc, 1));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBc, machine_code[0], 0, 0x3ffffff, 0));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBalc, machine_code[1], 0, 0x3ffffff, 0));

    assert_eq!(bc.is_conditional(), true);
    assert_ne!(bc.is_region(), true);

    assert_eq!(balc.is_conditional(), true);
    assert_ne!(balc.is_region(), true);
}
#[test]
fn test_bc1f_bc1t(){
    let machine_code: [u32; 2] = [0x4518C00D, 0x4519C00D];

    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let bc1f = decoder.disassemble(machine_code[0], 0x00400000).unwrap();
    let bc1t = decoder.disassemble(machine_code[1], 0x00400000).unwrap();

    assert_eq!(bc1f.get_mnemonic(), MgMnemonic::MgMneBc1f);
    assert_eq!(bc1t.get_mnemonic(), MgMnemonic::MgMneBc1t);
    assert_eq!(bc1f.get_mnemonic_str(), MG_MNE_BC1F);
    assert_eq!(bc1t.get_mnemonic_str(), MG_MNE_BC1T);
    assert_eq!("bc1f", MG_MNE_BC1F);
    assert_eq!("bc1t", MG_MNE_BC1T);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneBc1f, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneBc1t, true, false, true, false));

    assert_eq!(true, check_operands(&bc1f, 2));
    assert_eq!(true, check_operands(&bc1t, 2));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBc1f, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBc1t, machine_code[1], 0, 0xffff, 1));

    assert_eq!(true, check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneBc1f, 16));
    assert_eq!(true, check_field_zero_assert(&decoder, machine_code[1], 1, MgMnemonic::MgMneBc1t, 16));
    assert_eq!(true, check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneBc1f, 17));
    assert_eq!(true, check_field(&decoder, machine_code[1], 1, MgMnemonic::MgMneBc1t, 17));

    assert_eq!(bc1f.is_conditional(), true);
    assert_eq!(bc1f.is_relative(), true);
    assert_eq!(bc1f.is_region(), false);

    assert_eq!(bc1t.is_conditional(), true);
    assert_eq!(bc1t.is_region(), false);
    assert_eq!(bc1t.is_relative(), true);
}
#[test]
fn test_bc1fl_bc1tl(){
    let machine_code: [u32; 2] = [0x451AC00D, 0x451BC00D];

    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let bc1fl = decoder.disassemble(machine_code[0], 0x00400000).unwrap();
    let bc1tl = decoder.disassemble(machine_code[1], 0x00400000).unwrap();

    assert_eq!(bc1fl.get_mnemonic(), MgMnemonic::MgMneBc1fl);
    assert_eq!(bc1tl.get_mnemonic(), MgMnemonic::MgMneBc1tl);
    assert_eq!(bc1fl.get_mnemonic_str(), MG_MNE_BC1FL);
    assert_eq!(bc1tl.get_mnemonic_str(), MG_MNE_BC1TL);
    assert_eq!("bc1fl", MG_MNE_BC1FL);
    assert_eq!("bc1tl", MG_MNE_BC1TL);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneBc1fl, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneBc1tl, true, false, true, false));

    assert_eq!(true, check_operands(&bc1fl, 2));
    assert_eq!(true, check_operands(&bc1tl, 2));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBc1fl, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBc1tl, machine_code[1], 0, 0xffff, 1));

    assert_eq!(true, check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneBc1fl, 16));
    assert_eq!(true, check_field_zero_assert(&decoder, machine_code[1], 1, MgMnemonic::MgMneBc1tl, 16));
    assert_eq!(true, check_field_zero_assert(&decoder, machine_code[0], 1, MgMnemonic::MgMneBc1fl, 17));
    assert_eq!(true, check_field_zero_assert(&decoder, machine_code[1], 1, MgMnemonic::MgMneBc1tl, 17));

    assert_eq!(bc1fl.is_conditional(), true);
    assert_eq!(bc1fl.is_relative(), true);
    assert_eq!(bc1fl.is_region(), false);

    assert_eq!(bc1tl.is_conditional(), true);
    assert_eq!(bc1tl.is_region(), false);
    assert_eq!(bc1tl.is_relative(), true);
}
#[test]
fn test_bc1eqz_bc1nez(){
    let machine_code: [u32; 2] = [0x453bC00D, 0x45bBC00D];
    // 0b0100010100111011
    // 0b0100010110111011

    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let bc1eqz = decoder.disassemble(machine_code[0], 0x00400000).unwrap();
    let bc1nez = decoder.disassemble(machine_code[1], 0x00400000).unwrap();

    assert_eq!(bc1eqz.get_mnemonic(), MgMnemonic::MgMneBc1eqz);
    assert_eq!(bc1nez.get_mnemonic(), MgMnemonic::MgMneBc1nez);
    assert_eq!(bc1eqz.get_mnemonic_str(), MG_MNE_BC1EQZ);
    assert_eq!(bc1nez.get_mnemonic_str(), MG_MNE_BC1NEZ);
    assert_eq!("bc1eqz", MG_MNE_BC1EQZ);
    assert_eq!("bc1nez", MG_MNE_BC1NEZ);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneBc1eqz, false, true, false, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneBc1nez, false, true, false, true));

    assert_eq!(true, check_operands(&bc1eqz, 2));
    assert_eq!(true, check_operands(&bc1nez, 2));

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneBc1eqz, machine_code[0], 0, 0xffff, 1));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneBc1nez, machine_code[1], 0, 0xffff, 1));

    assert_eq!(bc1eqz.is_conditional(), true);
    assert_eq!(bc1eqz.is_relative(), true);
    assert_eq!(bc1eqz.is_region(), false);

    assert_eq!(bc1nez.is_conditional(), true);
    assert_eq!(bc1nez.is_region(), false);
    assert_eq!(bc1nez.is_relative(), true);
}