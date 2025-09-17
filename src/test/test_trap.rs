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
    
    assert!(syscall.get_mnemonic() == MgMnemonic::MgMneSyscall);
    assert!(MG_MNE_SYSCALL == syscall.get_mnemonic_str());
    assert!(MG_MNE_SYSCALL == "syscall");

    assert!(!syscall.is_conditional());
    assert!(!syscall.is_relative());
    assert!(!syscall.is_region());

    assert!(0 == syscall.get_opcode());
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSyscall, machine_code, 6, 0xfffff, 0));
    assert!(check_operands(&syscall, 1));
    assert!(version_test(machine_code, MgMnemonic::MgMneSyscall, true, true, true, true));
}
#[test]
fn test_break() {
    let machine_code = 0x0238000D;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let i_break = decoder.disassemble(machine_code, 0).unwrap();
    
    assert!(i_break.get_mnemonic() == MgMnemonic::MgMneBreak);
    assert!(MG_MNE_BREAK == i_break.get_mnemonic_str());
    assert!(MG_MNE_BREAK == "break");

    assert!(!i_break.is_conditional());
    assert!(!i_break.is_relative());
    assert!(!i_break.is_region());

    assert!(0 == i_break.get_opcode());
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneBreak, machine_code, 6, 0xfffff, 0));
    assert!(check_operands(&i_break, 1));
    assert!(version_test(machine_code, MgMnemonic::MgMneBreak, true, true, true, true));
}
#[test]
fn test_sigrie() {
    let machine_code = 0x0417000E;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let sigrie = decoder.disassemble(machine_code, 0).unwrap();
    
    assert!(sigrie.get_mnemonic() == MgMnemonic::MgMneSigrie);
    assert!(MG_MNE_SIGRIE == sigrie.get_mnemonic_str());
    assert!(MG_MNE_SIGRIE == "sigrie");

    assert!(!sigrie.is_conditional());
    assert!(!sigrie.is_relative());
    assert!(!sigrie.is_region());

    assert!(1 == sigrie.get_opcode());
    assert!(check_field(&decoder, machine_code, 0x1f, MgMnemonic::MgMneSigrie, 21));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSigrie, machine_code, 0, 0xffff, 0));
    assert!(check_operands(&sigrie, 1));
    assert!(version_test(machine_code, MgMnemonic::MgMneSigrie, false, true, false, true));
}
#[test]
fn test_sdbbp() {
    let mut machine_code = 0x0238000E;
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let mut sdbbp = decoder.disassemble(machine_code, 0).unwrap();
    
    assert!(sdbbp.get_mnemonic() == MgMnemonic::MgMneSdbbp);
    assert!(MG_MNE_SDBBP == sdbbp.get_mnemonic_str());
    assert!(MG_MNE_SDBBP == "sdbbp");

    assert!(!sdbbp.is_conditional());
    assert!(!sdbbp.is_relative());
    assert!(!sdbbp.is_region());

    assert!(0 == sdbbp.get_opcode());
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSdbbp, machine_code, 6, 0xfffff, 0));
    assert!(check_operands(&sdbbp, 1));
    assert!(version_test(machine_code, MgMnemonic::MgMneSdbbp, false, true, false, true));

    machine_code = 0x70008E3F;
    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);
    sdbbp = decoder.disassemble(machine_code, 0).unwrap();

    assert!(sdbbp.get_mnemonic() == MgMnemonic::MgMneSdbbp);
    assert!(MG_MNE_SDBBP == sdbbp.get_mnemonic_str());

    assert!(0b011100 == sdbbp.get_opcode());
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneSdbbp, machine_code, 6, 0xfffff, 0));
    assert!(check_operands(&sdbbp, 1));
    assert!(version_test(machine_code, MgMnemonic::MgMneSdbbp, true, false, true, false));
}
#[test]
fn test_tne_teq() {
    let machine_code: [u32; 2] = [0x00460036, 0x00400034];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let tne = decoder.disassemble(machine_code[0], 0).unwrap();
    let teq = decoder.disassemble(machine_code[1], 0).unwrap();
    
    //No problem
    assert!(tne.get_mnemonic() == MgMnemonic::MgMneTne);
    assert!(teq.get_mnemonic() == MgMnemonic::MgMneTeq);
    assert!(tne.get_mnemonic_str() == MG_MNE_TNE);
    assert!(teq.get_mnemonic_str() == MG_MNE_TEQ);
    assert!("tne" == MG_MNE_TNE);
    assert!("teq" == MG_MNE_TEQ);

    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneTne, machine_code[0], 6, 0x3ff, 2));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneTeq, machine_code[1], 6, 0x3ff, 2));

    assert!(check_operands(&tne, 3));
    assert!(check_operands(&teq, 3));

    assert!(version_test(machine_code[0], MgMnemonic::MgMneTne, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneTeq, true, true, true, true));
}
#[test]
fn test_teqi_tnei(){
    let machine_code: [u32; 2] = [0x048C0038, 0x048E0038];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let teqi: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let tnei: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneTeqi == teqi.get_mnemonic());
    assert!(MgMnemonic::MgMneTnei == tnei.get_mnemonic());
    assert!("teqi" == MG_MNE_TEQI);
    assert!("tnei" == MG_MNE_TNEI);

    assert!(MG_MNE_TEQI == teqi.get_mnemonic_str());
    assert!(MG_MNE_TNEI == tnei.get_mnemonic_str());

    assert!(check_operands(&teqi, 2));
    assert!(check_operands(&tnei, 2));

    assert!(teqi.is_conditional());
    assert!(!teqi.is_relative());
    assert!(!teqi.is_region());
    assert!(tnei.is_conditional());
    assert!(!tnei.is_relative());
    assert!(!tnei.is_region());

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneTnei, machine_code[1], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneTeqi, machine_code[0], 0, 0xffff, 1));
    assert!(version_test(machine_code[0], MgMnemonic::MgMneTeqi, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneTnei, true, false, true, false));
}
#[test]
fn test_tlti_tltiu(){
    let machine_code: [u32; 2] = [0x048A0038, 0x048B0038];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let tlti: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let tltiu: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneTlti == tlti.get_mnemonic());
    assert!(MgMnemonic::MgMneTltiu == tltiu.get_mnemonic());
    assert!("tlti" == MG_MNE_TLTI);
    assert!("tltiu" == MG_MNE_TLTIU);

    assert!(MG_MNE_TLTI == tlti.get_mnemonic_str());
    assert!(MG_MNE_TLTIU == tltiu.get_mnemonic_str());

    assert!(check_operands(&tlti, 2));
    assert!(check_operands(&tltiu, 2));

    assert!(tlti.is_conditional());
    assert!(!tlti.is_relative());
    assert!(!tlti.is_region());
    assert!(tltiu.is_conditional());
    assert!(!tltiu.is_relative());
    assert!(!tltiu.is_region());

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneTltiu, machine_code[1], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneTlti, machine_code[0], 0, 0xffff, 1));
    assert!(version_test(machine_code[0], MgMnemonic::MgMneTlti, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneTltiu, true, false, true, false));
}
#[test]
fn test_tgei_tgeiu(){
    let machine_code: [u32; 2] = [0x04880038, 0x04890038];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));

    let tgei: MgInstruction= decoder.disassemble(machine_code[0], 0).unwrap();
    let tgeiu: MgInstruction= decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneTgei == tgei.get_mnemonic());
    assert!(MgMnemonic::MgMneTgeiu == tgeiu.get_mnemonic());
    assert!("tgei" == MG_MNE_TGEI);
    assert!("tgeiu" == MG_MNE_TGEIU);

    assert!(MG_MNE_TGEI == tgei.get_mnemonic_str());
    assert!(MG_MNE_TGEIU == tgeiu.get_mnemonic_str());

    assert!(check_operands(&tgei, 2));
    assert!(check_operands(&tgeiu, 2));

    assert!(tgei.is_conditional());
    assert!(!tgei.is_relative());
    assert!(!tgei.is_region());
    assert!(tgeiu.is_conditional());
    assert!(!tgeiu.is_relative());
    assert!(!tgeiu.is_region());

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneTgeiu, machine_code[1], 0, 0xffff, 1));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneTgei, machine_code[0], 0, 0xffff, 1));
    assert!(version_test(machine_code[0], MgMnemonic::MgMneTgei, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneTgeiu, true, false, true, false));
}
#[test]
fn test_tlt_tltu() {
    let machine_code: [u32; 2] = [0x008514F2, 0x008504F3];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let tlt = decoder.disassemble(machine_code[0], 0).unwrap();
    let tltu = decoder.disassemble(machine_code[1], 0).unwrap();
    
    //No problem
    assert!(tlt.get_mnemonic() == MgMnemonic::MgMneTlt);
    assert!(tltu.get_mnemonic() == MgMnemonic::MgMneTltu);
    assert!(tlt.get_mnemonic_str() == MG_MNE_TLT);
    assert!(tltu.get_mnemonic_str() == MG_MNE_TLTU);
    assert!("tlt" == MG_MNE_TLT);
    assert!("tltu" == MG_MNE_TLTU);

    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneTlt, machine_code[0], 6, 0x3ff, 2));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneTltu, machine_code[1], 6, 0x3ff, 2));

    assert!(check_operands(&tlt, 3));
    assert!(check_operands(&tltu, 3));

    assert!(version_test(machine_code[0], MgMnemonic::MgMneTlt, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneTltu, true, true, true, true));
}
#[test]
fn test_tge_tgeu() {
    let machine_code: [u32; 2] = [0x008514F0, 0x008504F1];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let tge = decoder.disassemble(machine_code[0], 0).unwrap();
    let tgeu = decoder.disassemble(machine_code[1], 0).unwrap();
    
    //No problem
    assert!(tge.get_mnemonic() == MgMnemonic::MgMneTge);
    assert!(tgeu.get_mnemonic() == MgMnemonic::MgMneTgeu);
    assert!(tge.get_mnemonic_str() == MG_MNE_TGE);
    assert!(tgeu.get_mnemonic_str() == MG_MNE_TGEU);
    assert!("tge" == MG_MNE_TGE);
    assert!("tgeu" == MG_MNE_TGEU);

    assert!(tge.is_conditional());
    assert!(tgeu.is_conditional());

    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneTge, machine_code[0], 6, 0x3ff, 2));
    assert!(imm_limit_reached(&decoder,MgMnemonic::MgMneTgeu, machine_code[1], 6, 0x3ff, 2));

    assert!(check_operands(&tge, 3));
    assert!(check_operands(&tgeu, 3));

    assert!(version_test(machine_code[0], MgMnemonic::MgMneTge, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneTgeu, true, true, true, true));
}
#[test]
fn test_wait() {
    let machine_code = 0x42000020;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let wait = decoder.disassemble(machine_code, 0).unwrap();
    
    assert!(wait.get_mnemonic() == MgMnemonic::MgMneWait);
    assert!(MG_MNE_WAIT == wait.get_mnemonic_str());
    assert!(MG_MNE_WAIT == "wait");

    assert!(!wait.is_conditional());
    assert!(!wait.is_relative());
    assert!(!wait.is_region());

    assert!(check_field(&decoder, machine_code, 0x7ffff, MgMnemonic::MgMneWait, 6));
    assert!(check_field(&decoder, machine_code, 1, MgMnemonic::MgMneWait, 24));
    assert!(check_operands(&wait, 0));
    assert!(version_test(machine_code, MgMnemonic::MgMneWait, true, true, true, true));
}
#[test]
fn test_eret_deret() {
    let machine_code: [u32; 2] = [0x42000018, 0x4200001F];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let eret = decoder.disassemble(machine_code[0], 0).unwrap();
    let deret = decoder.disassemble(machine_code[1], 0).unwrap();
    
    //No problem
    assert!(eret.get_mnemonic() == MgMnemonic::MgMneEret);
    assert!(deret.get_mnemonic() == MgMnemonic::MgMneDeret);
    assert!(MG_MNE_ERET == eret.get_mnemonic_str());
    assert!(MG_MNE_DERET == deret.get_mnemonic_str());
    assert!(MG_MNE_ERET == "eret");
    assert!(MG_MNE_DERET == "deret");

    assert!(check_field(&decoder, machine_code[0], 0x7ffff, MgMnemonic::MgMneEret, 6));
    assert!(check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneEret, 24));
    assert!(check_field(&decoder, machine_code[1], 0x7ffff, MgMnemonic::MgMneDeret, 6));
    assert!(check_field(&decoder, machine_code[1], 1, MgMnemonic::MgMneDeret, 24));

    assert!(version_test(machine_code[0], MgMnemonic::MgMneEret, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneDeret, true, true, true, true));
}
#[test]
fn test_tlbp() {
    let machine_code = 0x42000008;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let tlbp = decoder.disassemble(machine_code, 0).unwrap();
    
    assert!(tlbp.get_mnemonic() == MgMnemonic::MgMneTlbp);
    assert!(MG_MNE_TLBP == tlbp.get_mnemonic_str());
    assert!(MG_MNE_TLBP == "tlbp");

    assert!(!tlbp.is_conditional());
    assert!(!tlbp.is_relative());
    assert!(!tlbp.is_region());

    assert!(check_field(&decoder, machine_code, 0x7ffff, MgMnemonic::MgMneTlbp, 6));
    assert!(check_field(&decoder, machine_code, 1, MgMnemonic::MgMneTlbp, 24));
    assert!(check_operands(&tlbp, 0));
    assert!(version_test(machine_code, MgMnemonic::MgMneTlbp, true, true, true, true));
}
#[test]
fn test_tlbr_tlbwr() {
    let machine_code: [u32; 2] = [0x42000001, 0x42000006];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let tlbr = decoder.disassemble(machine_code[0], 0).unwrap();
    let tlbwr = decoder.disassemble(machine_code[1], 0).unwrap();
    
    assert!(tlbr.get_mnemonic() == MgMnemonic::MgMneTlbr);
    assert!(tlbwr.get_mnemonic() == MgMnemonic::MgMneTlbwr);
    assert!(MG_MNE_TLBR == tlbr.get_mnemonic_str());
    assert!(MG_MNE_TLBWR == tlbwr.get_mnemonic_str());
    assert!(MG_MNE_TLBR == "tlbr");
    assert!(MG_MNE_TLBWR == "tlbwr");

    assert!(check_field(&decoder, machine_code[0], 0x7ffff, MgMnemonic::MgMneTlbr, 6));
    assert!(check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneTlbr, 24));
    assert!(check_field(&decoder, machine_code[1], 0x7ffff, MgMnemonic::MgMneTlbwr, 6));
    assert!(check_field(&decoder, machine_code[1], 1, MgMnemonic::MgMneTlbwr, 24));

    assert!(version_test(machine_code[0], MgMnemonic::MgMneTlbr, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneTlbwr, true, true, true, true));
}
#[test]
fn test_tlbwi_() {
    let machine_code: [u32; 3] = [0x42000002, 0x42000003, 0x42000004];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let tlbwi = decoder.disassemble(machine_code[0], 0).unwrap();
    let tlbinv = decoder.disassemble(machine_code[1], 0).unwrap();
    let tlbinvf = decoder.disassemble(machine_code[2], 0).unwrap();
    
    assert!(tlbwi.get_mnemonic() == MgMnemonic::MgMneTlbwi);
    assert!(tlbinv.get_mnemonic() == MgMnemonic::MgMneTlbinv);
    assert!(tlbinvf.get_mnemonic() == MgMnemonic::MgMneTlbinvf);
    assert!(MG_MNE_TLBWI == tlbwi.get_mnemonic_str());
    assert!(MG_MNE_TLBINV == tlbinv.get_mnemonic_str());
    assert!(MG_MNE_TLBINVF == tlbinvf.get_mnemonic_str());
    assert!(MG_MNE_TLBWI == "tlbwi");
    assert!(MG_MNE_TLBINV == "tlbinv");
    assert!(MG_MNE_TLBINVF == "tlbinvf");

    assert!(check_field(&decoder, machine_code[0], 0x7ffff, MgMnemonic::MgMneTlbwi, 6));
    assert!(check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneTlbwi, 24));
    assert!(check_field(&decoder, machine_code[1], 0x7ffff, MgMnemonic::MgMneTlbinv, 6));
    assert!(check_field(&decoder, machine_code[1], 1, MgMnemonic::MgMneTlbinv, 24));
    assert!(check_field(&decoder, machine_code[2], 0x7ffff, MgMnemonic::MgMneTlbinvf, 6));
    assert!(check_field(&decoder, machine_code[2], 1, MgMnemonic::MgMneTlbinvf, 24));

    assert!(version_test(machine_code[0], MgMnemonic::MgMneTlbwi, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneTlbinv, true, true, true, true));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneTlbinvf, true, true, true, true));
}
