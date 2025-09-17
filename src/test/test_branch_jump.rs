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

    assert!(MG_MNE_BNE == "bne");
    assert!(MG_MNE_BEQ == "beq");
    assert!(MG_MNE_BNE == bne.get_mnemonic_str());
    assert!(MG_MNE_BEQ == beq.get_mnemonic_str());
    assert!(MgMnemonic::MgMneBne == bne.get_mnemonic());
    assert!(MgMnemonic::MgMneBeq == beq.get_mnemonic());

    assert!(bne.is_relative());
    assert!(beq.is_relative());

    assert!(version_test(machine_code[0], MgMnemonic::MgMneBne, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneBeq, true, true, true, true));

    assert!(check_operands(&bne, 3));
    assert!(check_operands(&beq, 3));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBeq, machine_code[1], 0, 0xffff, 2));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBne, machine_code[0], 0, 0xffff, 2));
}
#[test]
fn test_bnel_beql(){
    let machine_code: [u32; 2] = [0x5485C114, 0x5085C114];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let bnel: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let beql: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MG_MNE_BNEL == "bnel");
    assert!(MG_MNE_BEQL == "beql");
    assert!(MG_MNE_BNEL == bnel.get_mnemonic_str());
    assert!(MG_MNE_BEQL == beql.get_mnemonic_str());
    assert!(MgMnemonic::MgMneBnel == bnel.get_mnemonic());
    assert!(MgMnemonic::MgMneBeql == beql.get_mnemonic());

    assert!(version_test(machine_code[0], MgMnemonic::MgMneBnel, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneBeql, true, false, true, false));

    assert!(bnel.is_relative());
    assert!(beql.is_relative());

    assert!(check_operands(&bnel, 3));
    assert!(check_operands(&beql, 3));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBeql, machine_code[1], 0, 0xffff, 2));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBnel, machine_code[0], 0, 0xffff, 2));
}
#[test]
fn test_jr(){
    let machine_code = [0x00800008, 0x00800408];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let jr: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let jrhb: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneJrhb == jrhb.get_mnemonic());
    assert!(MgMnemonic::MgMneJr == jr.get_mnemonic());
    assert!("jr.hb" == MG_MNE_JRHB);
    assert!("jr" == MG_MNE_JR);

    assert!(MG_MNE_JRHB == jrhb.get_mnemonic_str());
    assert!(MG_MNE_JR == jr.get_mnemonic_str());

    assert!(check_operands(&jr, 1));
    assert!(check_operands(&jrhb, 1));

    assert!(!jr.is_relative());
    assert!(!jrhb.is_relative());
    assert!(!jr.is_region());
    assert!(!jrhb.is_region());

    assert!(version_test(machine_code[0], MgMnemonic::MgMneJr, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneJrhb, true, false, true, false));
}
#[test]
fn test_bltzall_bgezall(){
    let machine_code: [u32; 2] = [0x0492C00D, 0x0493C00D];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let bltzall: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let bgezall: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneBltzall == bltzall.get_mnemonic());
    assert!(MgMnemonic::MgMneBgezall == bgezall.get_mnemonic());
    assert!("bltzall" == MG_MNE_BLTZALL);
    assert!("bgezall" == MG_MNE_BGEZALL);

    assert!(MG_MNE_BLTZALL == bltzall.get_mnemonic_str());
    assert!(MG_MNE_BGEZALL == bgezall.get_mnemonic_str());

    assert!(check_operands(&bltzall, 2));
    assert!(check_operands(&bgezall, 2));

    assert!(bltzall.is_conditional());
    assert!(bltzall.is_relative());
    assert!(!bltzall.is_region());
    assert!(bgezall.is_conditional());
    assert!(bgezall.is_relative());
    assert!(!bgezall.is_region());

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBgezall, machine_code[1], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBltzall, machine_code[0], 0, 0xffff, 1));
    assert!(version_test(machine_code[0], MgMnemonic::MgMneBltzall, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneBgezall, true, false, true, false));
}
#[test]
fn test_nal_bal(){
    let machine_code: [u32; 2] = [0x0410C00D, 0x0411C00D];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let nal: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let bal: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneNal == nal.get_mnemonic());
    assert!(MgMnemonic::MgMneBal == bal.get_mnemonic());
    assert!("nal" == MG_MNE_NAL);
    assert!("bal" == MG_MNE_BAL);

    assert!(MG_MNE_NAL == nal.get_mnemonic_str());
    assert!(MG_MNE_BAL == bal.get_mnemonic_str());

    assert!(check_operands(&nal, 1));
    assert!(check_operands(&bal, 1));

    assert!(!nal.is_conditional());
    assert!(nal.is_relative());
    assert!(!nal.is_region());
    assert!(!bal.is_conditional());
    assert!(bal.is_relative());
    assert!(!bal.is_region());

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBal, machine_code[1], 0, 0xffff, 0));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneNal, machine_code[0], 0, 0xffff, 0));
    assert!(check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneNal, 21));
    assert!(check_field(&decoder, machine_code[1], 0x1f, MgMnemonic::MgMneBal, 21));
    assert!(version_test(machine_code[0], MgMnemonic::MgMneNal, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneBal, false, true, false, true));
}
#[test]
fn test_bltzal_bgezal(){
    let machine_code: [u32; 2] = [0x0490C00D, 0x0491C00D];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let bltzal: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let bgezal: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneBltzal == bltzal.get_mnemonic());
    assert!(MgMnemonic::MgMneBgezal == bgezal.get_mnemonic());
    assert!("bltzal" == MG_MNE_BLTZAL);
    assert!("bgezal" == MG_MNE_BGEZAL);

    assert!(MG_MNE_BLTZAL == bltzal.get_mnemonic_str());
    assert!(MG_MNE_BGEZAL == bgezal.get_mnemonic_str());

    assert!(check_operands(&bltzal, 2));
    assert!(check_operands(&bgezal, 2));

    assert!(bltzal.is_conditional());
    assert!(bltzal.is_relative());
    assert!(!bltzal.is_region());
    assert!(bgezal.is_conditional());
    assert!(bgezal.is_relative());
    assert!(!bgezal.is_region());

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBgezal, machine_code[1], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBltzal, machine_code[0], 0, 0xffff, 1));
    assert!(version_test(machine_code[0], MgMnemonic::MgMneBltzal, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneBgezal, true, false, true, false));
}
#[test]
fn test_bltz_bgez(){
    let machine_code: [u32; 2] = [0x0480C00D, 0x0481C00D];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let bltz: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let bgez: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneBltz == bltz.get_mnemonic());
    assert!(MgMnemonic::MgMneBgez == bgez.get_mnemonic());
    assert!("bltz" == MG_MNE_BLTZ);
    assert!("bgez" == MG_MNE_BGEZ);

    assert!(MG_MNE_BLTZ == bltz.get_mnemonic_str());
    assert!(MG_MNE_BGEZ == bgez.get_mnemonic_str());

    assert!(check_operands(&bltz, 2));
    assert!(check_operands(&bgez, 2));

    assert!(bltz.is_conditional());
    assert!(bltz.is_relative());
    assert!(!bltz.is_region());
    assert!(bgez.is_conditional());
    assert!(bgez.is_relative());
    assert!(!bgez.is_region());

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBgez, machine_code[1], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBltz, machine_code[0], 0, 0xffff, 1));
    assert!(version_test(machine_code[0], MgMnemonic::MgMneBltz, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneBgez, true, false, true, false));
}
#[test]
fn test_bltzl_bgezl(){
    let machine_code: [u32; 2] = [0x0482C00D, 0x0483C00D];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let bltzl: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let bgezl: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneBltzl == bltzl.get_mnemonic());
    assert!(MgMnemonic::MgMneBgezl == bgezl.get_mnemonic());
    assert!("bltzl" == MG_MNE_BLTZL);
    assert!("bgezl" == MG_MNE_BGEZL);

    assert!(MG_MNE_BLTZL == bltzl.get_mnemonic_str());
    assert!(MG_MNE_BGEZL == bgezl.get_mnemonic_str());

    assert!(check_operands(&bltzl, 2));
    assert!(check_operands(&bgezl, 2));

    assert!(bltzl.is_conditional());
    assert!(bltzl.is_relative());
    assert!(!bltzl.is_region());
    assert!(bgezl.is_conditional());
    assert!(bgezl.is_relative());
    assert!(!bgezl.is_region());

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBgezl, machine_code[1], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBltzl, machine_code[0], 0, 0xffff, 1));
    assert!(version_test(machine_code[0], MgMnemonic::MgMneBltzl, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneBgezl, true, false, true, false));
}
#[test]
fn test_jalr(){
    let machine_code = [0x03602009, 0x03602409];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let jalr: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let jalrhb: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneJalrhb == jalrhb.get_mnemonic());
    assert!(MgMnemonic::MgMneJalr == jalr.get_mnemonic());
    assert!("jalr.hb" == MG_MNE_JALRHB);
    assert!("jalr" == MG_MNE_JALR);

    assert!(MG_MNE_JALRHB == jalrhb.get_mnemonic_str());
    assert!(MG_MNE_JALR == jalr.get_mnemonic_str());

    assert!(check_operands(&jalr, 2));
    assert!(check_operands(&jalrhb, 2));

    assert!(!jalr.is_relative());
    assert!(!jalrhb.is_relative());
    assert!(!jalr.is_region());
    assert!(!jalrhb.is_region());

    assert!(version_test(machine_code[0], MgMnemonic::MgMneJalr, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneJalrhb, true, true, true, true));
}
#[test]
fn test_jalx(){
    let machine_code = 0x74000115;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let jalx: MgInstruction= decoder.disassemble(machine_code, 0).unwrap();

    assert!(MgMnemonic::MgMneJalx == jalx.get_mnemonic());
    assert!(MG_MNE_JALX == jalx.get_mnemonic_str());
    assert!(MG_MNE_JALX == "jalx");

    assert!(check_operands(&jalx, 1));
    assert!(jalx.is_region());
    assert!(version_test(machine_code, MgMnemonic::MgMneJalx, true, false, true, false));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneJalx, machine_code, 0, 0x3ffffff, 0));
}
#[test]
fn test_bc_balc(){
    let machine_code: [u32; 2] = [0xC8020050, 0xE8020050];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let bc = decoder.disassemble(machine_code[0], 0).unwrap();
    let balc = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(bc.get_mnemonic() == MgMnemonic::MgMneBc);
    assert!(balc.get_mnemonic() == MgMnemonic::MgMneBalc);
    assert!(bc.get_mnemonic_str() == MG_MNE_BC);
    assert!(balc.get_mnemonic_str() == MG_MNE_BALC);
    assert!("bc" == MG_MNE_BC);
    assert!("balc" == MG_MNE_BALC);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneBc, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneBalc, false, true, false, true));

    assert!(check_operands(&bc, 1));
    assert!(check_operands(&balc, 1));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBc, machine_code[0], 0, 0x3ffffff, 0));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneBalc, machine_code[1], 0, 0x3ffffff, 0));

    assert!(bc.is_conditional());
    assert!(!bc.is_region());

    assert!(balc.is_conditional());
    assert!(!balc.is_region());
}
#[test]
fn test_bc1f_bc1t(){
    let machine_code: [u32; 2] = [0x4518C00D, 0x4519C00D];

    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let bc1f = decoder.disassemble(machine_code[0], 0x00400000).unwrap();
    let bc1t = decoder.disassemble(machine_code[1], 0x00400000).unwrap();

    assert!(bc1f.get_mnemonic() == MgMnemonic::MgMneBc1f);
    assert!(bc1t.get_mnemonic() == MgMnemonic::MgMneBc1t);
    assert!(bc1f.get_mnemonic_str() == MG_MNE_BC1F);
    assert!(bc1t.get_mnemonic_str() == MG_MNE_BC1T);
    assert!("bc1f" == MG_MNE_BC1F);
    assert!("bc1t" == MG_MNE_BC1T);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneBc1f, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneBc1t, true, false, true, false));

    assert!(check_operands(&bc1f, 2));
    assert!(check_operands(&bc1t, 2));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBc1f, machine_code[0], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneBc1t, machine_code[1], 0, 0xffff, 1));

    assert!(check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneBc1f, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[1], 1, MgMnemonic::MgMneBc1t, 16));
    assert!(check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneBc1f, 17));
    assert!(check_field(&decoder, machine_code[1], 1, MgMnemonic::MgMneBc1t, 17));

    assert!(bc1f.is_conditional());
    assert!(bc1f.is_relative());
    assert!(!bc1f.is_region());

    assert!(bc1t.is_conditional());
    assert!(!bc1t.is_region());
    assert!(bc1t.is_relative());
}
#[test]
fn test_bc1fl_bc1tl(){
    let machine_code: [u32; 2] = [0x451AC00D, 0x451BC00D];

    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let bc1fl = decoder.disassemble(machine_code[0], 0x00400000).unwrap();
    let bc1tl = decoder.disassemble(machine_code[1], 0x00400000).unwrap();

    assert!(bc1fl.get_mnemonic() == MgMnemonic::MgMneBc1fl);
    assert!(bc1tl.get_mnemonic() == MgMnemonic::MgMneBc1tl);
    assert!(bc1fl.get_mnemonic_str() == MG_MNE_BC1FL);
    assert!(bc1tl.get_mnemonic_str() == MG_MNE_BC1TL);
    assert!("bc1fl" == MG_MNE_BC1FL);
    assert!("bc1tl" == MG_MNE_BC1TL);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneBc1fl, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneBc1tl, true, false, true, false));

    assert!(check_operands(&bc1fl, 2));
    assert!(check_operands(&bc1tl, 2));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBc1fl, machine_code[0], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneBc1tl, machine_code[1], 0, 0xffff, 1));

    assert!(check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneBc1fl, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[1], 1, MgMnemonic::MgMneBc1tl, 16));
    assert!(check_field_zero_assert(&decoder, machine_code[0], 1, MgMnemonic::MgMneBc1fl, 17));
    assert!(check_field_zero_assert(&decoder, machine_code[1], 1, MgMnemonic::MgMneBc1tl, 17));

    assert!(bc1fl.is_conditional());
    assert!(bc1fl.is_relative());
    assert!(!bc1fl.is_region());

    assert!(bc1tl.is_conditional());
    assert!(!bc1tl.is_region());
    assert!(bc1tl.is_relative());
}
#[test]
fn test_bc1eqz_bc1nez(){
    let machine_code: [u32; 2] = [0x453bC00D, 0x45bBC00D];
    // 0b0100010100111011
    // 0b0100010110111011

    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let bc1eqz = decoder.disassemble(machine_code[0], 0x00400000).unwrap();
    let bc1nez = decoder.disassemble(machine_code[1], 0x00400000).unwrap();

    assert!(bc1eqz.get_mnemonic() == MgMnemonic::MgMneBc1eqz);
    assert!(bc1nez.get_mnemonic() == MgMnemonic::MgMneBc1nez);
    assert!(bc1eqz.get_mnemonic_str() == MG_MNE_BC1EQZ);
    assert!(bc1nez.get_mnemonic_str() == MG_MNE_BC1NEZ);
    assert!("bc1eqz" == MG_MNE_BC1EQZ);
    assert!("bc1nez" == MG_MNE_BC1NEZ);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneBc1eqz, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneBc1nez, false, true, false, true));

    assert!(check_operands(&bc1eqz, 2));
    assert!(check_operands(&bc1nez, 2));

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneBc1eqz, machine_code[0], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneBc1nez, machine_code[1], 0, 0xffff, 1));

    assert!(bc1eqz.is_conditional());
    assert!(bc1eqz.is_relative());
    assert!(!bc1eqz.is_region());

    assert!(bc1nez.is_conditional());
    assert!(!bc1nez.is_region());
    assert!(bc1nez.is_relative());
}
