//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_dclz_dclo(){
    let machine_code: [u32; 4] = [0x00002052, 0x00800053, 0x70A42024, 0x70A42025];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));
    
    let mut dclz = decoder.disassemble(machine_code[0], 0).unwrap();
    let mut dclo = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneDclz == dclz.get_mnemonic());
    assert!(MgMnemonic::MgMneDclo == dclo.get_mnemonic());
    assert!(MG_MNE_DCLZ == dclz.get_mnemonic_str());
    assert!(MG_MNE_DCLO == dclo.get_mnemonic_str());
    assert!("dclz" == MG_MNE_DCLZ);
    assert!("dclo" == MG_MNE_DCLO);

    assert!(!dclo.is_conditional());
    assert!(!dclo.is_relative());
    assert!(!dclo.is_region());
    assert!(!dclz.is_conditional());
    assert!(!dclz.is_relative());
    assert!(!dclz.is_region());

    assert!(check_operands(&dclz, 2));
    assert!(check_operands(&dclo, 2));

    assert!(check_field(&decoder, machine_code[0], 0b11110, MgMnemonic::MgMneDclz, 6));
    assert!(check_field(&decoder, machine_code[1], 0b11110, MgMnemonic::MgMneDclo, 6));
    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneDclz, 16));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneDclo, 16));

    decoder.version = MgMipsVersion::M64(MgMips64::MgPreR6);
    dclz = decoder.disassemble(machine_code[2], 0).unwrap();
    dclo = decoder.disassemble(machine_code[3], 0).unwrap();

    assert!(MgMnemonic::MgMneDclz == dclz.get_mnemonic());
    assert!(MgMnemonic::MgMneDclo == dclo.get_mnemonic());
    assert!(MG_MNE_DCLZ == dclz.get_mnemonic_str());
    assert!(MG_MNE_DCLO == dclo.get_mnemonic_str());

    assert!(!dclo.is_conditional());
    assert!(!dclo.is_relative());
    assert!(!dclo.is_region());
    assert!(!dclz.is_conditional());
    assert!(!dclz.is_relative());
    assert!(!dclz.is_region());

    assert!(check_operands(&dclz, 2));
    assert!(check_operands(&dclo, 2));

    assert!(check_field(&decoder, machine_code[2], 0b11111, MgMnemonic::MgMneDclz, 6));
    assert!(check_field(&decoder, machine_code[3], 0b11111, MgMnemonic::MgMneDclo, 6));
}
#[test]
fn test_clz_clo(){
    let machine_code: [u32; 4] = [0x00000050, 0x00000051, 0x70A42020, 0x70A42021 ];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));
    
    let mut clz = decoder.disassemble(machine_code[0], 0).unwrap();
    let mut clo = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(MgMnemonic::MgMneClz == clz.get_mnemonic());
    assert!(MgMnemonic::MgMneClo == clo.get_mnemonic());
    assert!(MG_MNE_CLZ == clz.get_mnemonic_str());
    assert!(MG_MNE_CLO == clo.get_mnemonic_str());
    assert!("clz" == MG_MNE_CLZ);
    assert!("clo" == MG_MNE_CLO);

    assert!(!clo.is_conditional());
    assert!(!clo.is_relative());
    assert!(!clo.is_region());
    assert!(!clz.is_conditional());
    assert!(!clz.is_relative());
    assert!(!clz.is_region());

    assert!(check_operands(&clz, 2));
    assert!(check_operands(&clo, 2));

    assert!(check_field(&decoder, machine_code[0], 0b11110, MgMnemonic::MgMneClz, 6));
    assert!(check_field(&decoder, machine_code[1], 0b11110, MgMnemonic::MgMneClo, 6));
    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneClz, 16));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneClo, 16));

    decoder.version = MgMipsVersion::M64(MgMips64::MgPreR6);
    clz = decoder.disassemble(machine_code[2], 0).unwrap();
    clo = decoder.disassemble(machine_code[3], 0).unwrap();

    assert!(MgMnemonic::MgMneClz == clz.get_mnemonic());
    assert!(MgMnemonic::MgMneClo == clo.get_mnemonic());
    assert!(MG_MNE_CLZ == clz.get_mnemonic_str());
    assert!(MG_MNE_CLO == clo.get_mnemonic_str());

    assert!(!clo.is_conditional());
    assert!(!clo.is_relative());
    assert!(!clo.is_region());
    assert!(!clz.is_conditional());
    assert!(!clz.is_relative());
    assert!(!clz.is_region());

    assert!(check_operands(&clz, 2));
    assert!(check_operands(&clo, 2));

    assert!(check_field(&decoder, machine_code[2], 0b11111, MgMnemonic::MgMneClz, 6));
    assert!(check_field(&decoder, machine_code[3], 0b11111, MgMnemonic::MgMneClo, 6));
}
#[test]
fn test_xor_nor(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let machine_code = [0x00BB2026, 0x00BB2027];
    let xor = decoder.disassemble(machine_code[0], 0).unwrap();
    let nor = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(xor.get_mnemonic() == MgMnemonic::MgMneXor);
    assert!(nor.get_mnemonic() == MgMnemonic::MgMneNor);
    assert!(nor.get_mnemonic_str() == MG_MNE_NOR);
    assert!(xor.get_mnemonic_str() == MG_MNE_XOR);
    assert!("nor" == MG_MNE_NOR);
    assert!("xor" == MG_MNE_XOR);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneXor, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneNor, true, true, true, true));

    assert!(check_operands(&xor, 3));
    assert!(check_operands(&nor, 3));
    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneXor, 6));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneNor, 6));
}
#[test]
fn test_or_xor(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let machine_code = [0x00BB2025, 0x00BB2024];
    let or = decoder.disassemble(machine_code[0], 0).unwrap();
    let and = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(or.get_mnemonic() == MgMnemonic::MgMneOr);
    assert!(and.get_mnemonic() == MgMnemonic::MgMneAnd);
    assert!(and.get_mnemonic_str() == MG_MNE_AND);
    assert!(or.get_mnemonic_str() == MG_MNE_OR);
    assert!("and" == MG_MNE_AND);
    assert!("or" == MG_MNE_OR);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneOr, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneAnd, true, true, true, true));

    assert!(check_operands(&or, 3));
    assert!(check_operands(&and, 3));
    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneOr, 6));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneAnd, 6));
}
#[test]
fn test_sub_subu(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let machine_code = [0x00BB2022, 0x00BB2023];
    let sub = decoder.disassemble(machine_code[0], 0).unwrap();
    let subu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(sub.get_mnemonic() == MgMnemonic::MgMneSub);
    assert!(subu.get_mnemonic() == MgMnemonic::MgMneSubu);
    assert!(subu.get_mnemonic_str() == MG_MNE_SUBU);
    assert!(sub.get_mnemonic_str() == MG_MNE_SUB);
    assert!("subu" == MG_MNE_SUBU);
    assert!("sub" == MG_MNE_SUB);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneSub, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneSubu, true, true, true, true));

    assert!(check_operands(&sub, 3));
    assert!(check_operands(&subu, 3));
    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneSub, 6));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneSubu, 6));
}
#[test]
fn test_add_addu(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let machine_code = [0x00BB2020, 0x00BB2021];
    let add = decoder.disassemble(machine_code[0], 0).unwrap();
    let addu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(add.get_mnemonic() == MgMnemonic::MgMneAdd);
    assert!(addu.get_mnemonic() == MgMnemonic::MgMneAddu);
    assert!(addu.get_mnemonic_str() == MG_MNE_ADDU);
    assert!(add.get_mnemonic_str() == MG_MNE_ADD);
    assert!("addu" == MG_MNE_ADDU);
    assert!("add" == MG_MNE_ADD);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneAdd, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneAddu, true, true, true, true));

    assert!(check_operands(&add, 3));
    assert!(check_operands(&addu, 3));
    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneAdd, 6));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneAddu, 6));
}
#[test]
fn test_mul(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let machine_code = 0x70f34802;
    let mul = decoder.disassemble(machine_code, 0).unwrap();

    assert!(mul.get_mnemonic() == MgMnemonic::MgMneMul);
    assert!(mul.get_mnemonic_str() == MG_MNE_MUL);
    assert!("mul" == MG_MNE_MUL);

    assert!(version_test(machine_code, MgMnemonic::MgMneMul, true, false, true, false));

    assert!(check_operands(&mul, 3));
    assert!(check_field(&decoder, machine_code, 0b11111, MgMnemonic::MgMneMul, 6));
}
#[test]
fn sop30(){
    let machine_code = [0x2E75098, 0x2E750D8];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let mul = decoder.disassemble(machine_code[0], 0).unwrap();
    let muh = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(mul.get_mnemonic() == MgMnemonic::MgMneMul);
    assert!(muh.get_mnemonic() == MgMnemonic::MgMneMuh);
    assert!(mul.get_mnemonic_str() == MG_MNE_MUL);
    assert!(muh.get_mnemonic_str() == MG_MNE_MUH);
    assert!("mul" == MG_MNE_MUL);
    assert!("muh" == MG_MNE_MUH);

    assert!(!muh.is_conditional());
    assert!(!muh.is_relative());
    assert!(!muh.is_region());
    assert!(!mul.is_conditional());
    assert!(!mul.is_relative());
    assert!(!mul.is_region());

    assert!(0b00010 == mul.get_machine_code() >> 6 & 0b11111);
    assert!(0b00011 == muh.get_machine_code() >> 6 & 0b11111);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneMul, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneMuh, false, true, false, true));

    assert!(check_operands(&mul, 3));
    assert!(check_operands(&muh, 3));
}
#[test]
fn sop31(){
    let machine_code = [0x2E75099, 0x2E750D9];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let mulu = decoder.disassemble(machine_code[0], 0).unwrap();
    let muhu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(mulu.get_mnemonic() == MgMnemonic::MgMneMulu);
    assert!(muhu.get_mnemonic() == MgMnemonic::MgMneMuhu);
    assert!(mulu.get_mnemonic_str() == MG_MNE_MULU);
    assert!(muhu.get_mnemonic_str() == MG_MNE_MUHU);
    assert!("mulu" == MG_MNE_MULU);
    assert!("muhu" == MG_MNE_MUHU);

    assert!(!muhu.is_conditional());
    assert!(!muhu.is_relative());
    assert!(!muhu.is_region());
    assert!(!mulu.is_conditional());
    assert!(!mulu.is_relative());
    assert!(!mulu.is_region());

    assert!(0b00010 == mulu.get_machine_code() >> 6 & 0b11111);
    assert!(0b00011 == muhu.get_machine_code() >> 6 & 0b11111);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneMulu, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneMuhu, false, true, false, true));

    assert!(check_operands(&mulu, 3));
    assert!(check_operands(&muhu, 3));
}
#[test]
fn sop32(){
    let machine_code = [0x2E7509a, 0x2E750Da];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let div = decoder.disassemble(machine_code[0], 0).unwrap();
    let mod_i = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(div.get_mnemonic() == MgMnemonic::MgMneDiv);
    assert!(mod_i.get_mnemonic() == MgMnemonic::MgMneMod);
    assert!(div.get_mnemonic_str() == MG_MNE_DIV);
    assert!(mod_i.get_mnemonic_str() == MG_MNE_MOD);
    assert!("div" == MG_MNE_DIV);
    assert!("mod" == MG_MNE_MOD);

    assert!(!mod_i.is_conditional());
    assert!(!mod_i.is_relative());
    assert!(!mod_i.is_region());
    assert!(!div.is_conditional());
    assert!(!div.is_relative());
    assert!(!div.is_region());

    assert!(0b00010 == div.get_machine_code() >> 6 & 0b11111);
    assert!(0b00011 == mod_i.get_machine_code() >> 6 & 0b11111);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneDiv, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneMod, false, true, false, true));

    assert!(check_operands(&div, 3));
    assert!(check_operands(&mod_i, 3));
}
#[test]
fn sop33(){
    let machine_code = [0x2E7509b, 0x2E750Db];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let divu = decoder.disassemble(machine_code[0], 0).unwrap();
    let modu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(divu.get_mnemonic() == MgMnemonic::MgMneDivu);
    assert!(modu.get_mnemonic() == MgMnemonic::MgMneModu);
    assert!(divu.get_mnemonic_str() == MG_MNE_DIVU);
    assert!(modu.get_mnemonic_str() == MG_MNE_MODU);
    assert!("divu" == MG_MNE_DIVU);
    assert!("modu" == MG_MNE_MODU);

    assert!(!modu.is_conditional());
    assert!(!modu.is_relative());
    assert!(!modu.is_region());
    assert!(!divu.is_conditional());
    assert!(!divu.is_relative());
    assert!(!divu.is_region());

    assert!(0b00010 == divu.get_machine_code() >> 6 & 0b11111);
    assert!(0b00011 == modu.get_machine_code() >> 6 & 0b11111);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneDivu, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneModu, false, true, false, true));

    assert!(check_operands(&divu, 3));
    assert!(check_operands(&modu, 3));
}
#[test]
fn sop34(){
    let machine_code = [0x2E7509c, 0x2E750Dc];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let dmul = decoder.disassemble(machine_code[0], 0).unwrap();
    let dmuh = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(dmul.get_mnemonic() == MgMnemonic::MgMneDmul);
    assert!(dmuh.get_mnemonic() == MgMnemonic::MgMneDmuh);
    assert!(dmul.get_mnemonic_str() == MG_MNE_DMUL);
    assert!(dmuh.get_mnemonic_str() == MG_MNE_DMUH);
    assert!("dmul" == MG_MNE_DMUL);
    assert!("dmuh" == MG_MNE_DMUH);

    assert!(!dmuh.is_conditional());
    assert!(!dmuh.is_relative());
    assert!(!dmuh.is_region());
    assert!(!dmul.is_conditional());
    assert!(!dmul.is_relative());
    assert!(!dmul.is_region());

    assert!(0b00010 == dmul.get_machine_code() >> 6 & 0b11111);
    assert!(0b00011 == dmuh.get_machine_code() >> 6 & 0b11111);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneDmul, false, false, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneDmuh, false, false, false, true));

    assert!(check_operands(&dmul, 3));
    assert!(check_operands(&dmuh, 3));
}
#[test]
fn sop35(){
    let machine_code = [0x2E7509d, 0x2E750Dd];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let dmulu = decoder.disassemble(machine_code[0], 0).unwrap();
    let dmuhu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(dmulu.get_mnemonic() == MgMnemonic::MgMneDmulu);
    assert!(dmuhu.get_mnemonic() == MgMnemonic::MgMneDmuhu);
    assert!(dmulu.get_mnemonic_str() == MG_MNE_DMULU);
    assert!(dmuhu.get_mnemonic_str() == MG_MNE_DMUHU);

    assert!(!dmuhu.is_conditional());
    assert!(!dmuhu.is_relative());
    assert!(!dmuhu.is_region());
    assert!(!dmulu.is_conditional());
    assert!(!dmulu.is_relative());
    assert!(!dmulu.is_region());

    assert!(0b00010 == dmulu.get_machine_code() >> 6 & 0b11111);
    assert!(0b00011 == dmuhu.get_machine_code() >> 6 & 0b11111);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneDmulu, false, false, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneDmuhu, false, false, false, true));

    assert!(check_operands(&dmulu, 3));
    assert!(check_operands(&dmuhu, 3));
}
#[test]
fn sop36(){
    let machine_code = [0x2E7509e, 0x2E750De];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let ddiv = decoder.disassemble(machine_code[0], 0).unwrap();
    let dmod = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(ddiv.get_mnemonic() == MgMnemonic::MgMneDdiv);
    assert!(dmod.get_mnemonic() == MgMnemonic::MgMneDmod);
    assert!(ddiv.get_mnemonic_str() == MG_MNE_DDIV);
    assert!(dmod.get_mnemonic_str() == MG_MNE_DMOD);
    assert!("ddiv" == MG_MNE_DDIV);
    assert!("dmod" == MG_MNE_DMOD);

    assert!(!dmod.is_conditional());
    assert!(!dmod.is_relative());
    assert!(!dmod.is_region());
    assert!(!ddiv.is_conditional());
    assert!(!ddiv.is_relative());
    assert!(!ddiv.is_region());

    assert!(0b00010 == ddiv.get_machine_code() >> 6 & 0b11111);
    assert!(0b00011 == dmod.get_machine_code() >> 6 & 0b11111);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneDdiv, false, false, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneDmod, false, false, false, true));

    assert!(check_operands(&ddiv, 3));
    assert!(check_operands(&dmod, 3));
}
#[test]
fn sop37(){
    let machine_code = [0x2E7509f, 0x2E750Df];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let ddivu = decoder.disassemble(machine_code[0], 0).unwrap();
    let dmodu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(ddivu.get_mnemonic() == MgMnemonic::MgMneDdivu);
    assert!(dmodu.get_mnemonic() == MgMnemonic::MgMneDmodu);
    assert!(ddivu.get_mnemonic_str() == MG_MNE_DDIVU);
    assert!(dmodu.get_mnemonic_str() == MG_MNE_DMODU);
    assert!("ddivu" == MG_MNE_DDIVU);
    assert!("dmodu" == MG_MNE_DMODU);

    assert!(!dmodu.is_conditional());
    assert!(!dmodu.is_relative());
    assert!(!dmodu.is_region());
    assert!(!ddivu.is_conditional());
    assert!(!ddivu.is_relative());
    assert!(!ddivu.is_region());

    assert!(0b00010 == ddivu.get_machine_code() >> 6 & 0b11111);
    assert!(0b00011 == dmodu.get_machine_code() >> 6 & 0b11111);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneDdivu, false, false, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneDmodu, false, false, false, true));

    assert!(check_operands(&ddivu, 3));
    assert!(check_operands(&dmodu, 3));
}
#[test]
fn test_dmult_dmultu(){
    let machine_code = [0x0085001C, 0x0085001D];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let dmult = decoder.disassemble(machine_code[0], 0).unwrap();
    let dmultu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(dmult.get_mnemonic() == MgMnemonic::MgMneDmult);
    assert!(dmultu.get_mnemonic() == MgMnemonic::MgMneDmultu);
    assert!(dmult.get_mnemonic_str() == MG_MNE_DMULT);
    assert!(dmultu.get_mnemonic_str() == MG_MNE_DMULTU);
    assert!("dmult" == MG_MNE_DMULT);
    assert!("dmultu" == MG_MNE_DMULTU);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneDmult, false, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneDmultu, false, false, true, false));

    assert!(check_operands(&dmult, 2));
    assert!(check_operands(&dmultu, 2));

    assert!(check_field(&decoder, machine_code[0], 0b1111111111, MgMnemonic::MgMneDmult, 6));
    assert!(check_field(&decoder, machine_code[1], 0b1111111111, MgMnemonic::MgMneDmultu, 6));
}
#[test]
fn test_ddiv_ddivu(){
    let machine_code = [0x0044001e, 0x000A001f];

    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let ddiv = decoder.disassemble(machine_code[0], 0).unwrap();
    let ddivu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(ddiv.get_mnemonic() == MgMnemonic::MgMneDdiv);
    assert!(ddivu.get_mnemonic() == MgMnemonic::MgMneDdivu);
    assert!(ddiv.get_mnemonic_str() == MG_MNE_DDIV);
    assert!(ddivu.get_mnemonic_str() == MG_MNE_DDIVU);
    assert!("ddiv" == MG_MNE_DDIV);
    assert!("ddivu" == MG_MNE_DDIVU);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneDdiv, false, false, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneDdivu, false, false, true, true));

    assert!(check_operands(&ddiv, 2));
    assert!(check_operands(&ddivu, 2));

    assert!(check_field(&decoder, machine_code[0], 0b1111111111, MgMnemonic::MgMneDdiv, 6));
    assert!(check_field(&decoder, machine_code[1], 0b1111111111, MgMnemonic::MgMneDdivu, 6));
}
#[test]
fn test_msub_msubu(){
    let machine_code: [u32; 2] = [0x70850004, 0x70850005];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let msub = decoder.disassemble(machine_code[0], 0).unwrap();
    let msubu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(msub.get_mnemonic() == MgMnemonic::MgMneMsub);
    assert!(msubu.get_mnemonic() == MgMnemonic::MgMneMsubu);
    assert!(msub.get_mnemonic_str() == MG_MNE_MSUB);
    assert!(msubu.get_mnemonic_str() == MG_MNE_MSUBU);
    assert!("msub" == MG_MNE_MSUB);
    assert!("msubu" == MG_MNE_MSUBU);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneMsub, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneMsubu, true, false, true, false));

    assert!(!msubu.is_conditional());
    assert!(!msubu.is_relative());
    assert!(!msubu.is_region());
    assert!(!msub.is_conditional());
    assert!(!msub.is_relative());
    assert!(!msub.is_region());
    
    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneMsub, 6));
    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneMsub, 11));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneMsubu, 6));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneMsubu, 11));

    assert!(check_operands(&msub, 2));
    assert!(check_operands(&msubu, 2));
}
#[test]
fn test_dsub_dsubu(){
    let machine_code: [u32; 2] = [0x00A2202E, 0x00A2202F];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let dsub = decoder.disassemble(machine_code[0], 0).unwrap();
    let dsubu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(dsub.get_mnemonic() == MgMnemonic::MgMneDsub);
    assert!(dsubu.get_mnemonic() == MgMnemonic::MgMneDsubu);
    assert!(dsub.get_mnemonic_str() == MG_MNE_DSUB);
    assert!(dsubu.get_mnemonic_str() == MG_MNE_DSUBU);
    assert!("dsub" == MG_MNE_DSUB);
    assert!("dsubu" == MG_MNE_DSUBU);

    assert!(!dsubu.is_conditional());
    assert!(!dsubu.is_relative());
    assert!(!dsubu.is_region());
    assert!(!dsub.is_conditional());
    assert!(!dsub.is_relative());
    assert!(!dsub.is_region());

    assert!(check_operands(&dsub, 3));
    assert!(check_operands(&dsubu, 3));
    
    assert!(version_test(machine_code[0], MgMnemonic::MgMneDsub, false, false, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneDsubu, false, false, true, true));

    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneDsub, 6));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneDsubu, 6));
}
#[test]
fn test_dadd_daddu(){
    let machine_code: [u32; 2] = [0x00A2202C, 0x00A2202D];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let dadd = decoder.disassemble(machine_code[0], 0).unwrap();
    let daddu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(dadd.get_mnemonic() == MgMnemonic::MgMneDadd);
    assert!(daddu.get_mnemonic() == MgMnemonic::MgMneDaddu);
    assert!(dadd.get_mnemonic_str() == MG_MNE_DADD);
    assert!(daddu.get_mnemonic_str() == MG_MNE_DADDU);
    assert!("dadd" == MG_MNE_DADD);
    assert!("daddu" == MG_MNE_DADDU);

    assert!(!daddu.is_conditional());
    assert!(!daddu.is_relative());
    assert!(!daddu.is_region());
    assert!(!dadd.is_conditional());
    assert!(!dadd.is_relative());
    assert!(!dadd.is_region());

    assert!(check_operands(&dadd, 3));
    assert!(check_operands(&daddu, 3));
    
    assert!(version_test(machine_code[0], MgMnemonic::MgMneDadd, false, false, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneDaddu, false, false, true, true));

    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneDadd, 6));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneDaddu, 6));
}
#[test]
fn test_madd_maddu(){
    let machine_code: [u32; 2] = [0x70850000, 0x70850001];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let madd = decoder.disassemble(machine_code[0], 0).unwrap();
    let maddu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(madd.get_mnemonic() == MgMnemonic::MgMneMadd);
    assert!(maddu.get_mnemonic() == MgMnemonic::MgMneMaddu);
    assert!(madd.get_mnemonic_str() == MG_MNE_MADD);
    assert!(maddu.get_mnemonic_str() == MG_MNE_MADDU);
    assert!("madd" == MG_MNE_MADD);
    assert!("maddu" == MG_MNE_MADDU);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneMadd, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneMaddu, true, false, true, false));

    assert!(!maddu.is_conditional());
    assert!(!maddu.is_relative());
    assert!(!maddu.is_region());
    assert!(!madd.is_conditional());
    assert!(!madd.is_relative());
    assert!(!madd.is_region());
    
    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneMadd, 6));
    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneMadd, 11));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneMaddu, 6));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneMaddu, 11));

    assert!(check_operands(&madd, 2));
    assert!(check_operands(&maddu, 2));
}
#[test]
fn test_daddi_daddiu(){
    let machine_code: [u32; 2] = [0x63640038, 0x67640038];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let daddi = decoder.disassemble(machine_code[0], 0).unwrap();
    let daddiu = decoder.disassemble(machine_code[1], 0).unwrap();
    
    assert!(daddi.get_mnemonic() == MgMnemonic::MgMneDaddi);
    assert!(daddiu.get_mnemonic() == MgMnemonic::MgMneDaddiu);
    assert!(daddi.get_mnemonic_str() == MG_MNE_DADDI);
    assert!(daddiu.get_mnemonic_str() == MG_MNE_DADDIU);
    assert!("daddi" == MG_MNE_DADDI);
    assert!("daddiu" == MG_MNE_DADDIU);

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneDaddi, machine_code[0], 0, 0xffff, 2));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneDaddiu, machine_code[1], 0, 0xffff, 2));

    assert!(version_test(machine_code[0], MgMnemonic::MgMneDaddi, false, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneDaddiu, false, false, true, true));

    assert!(check_operands(&daddi, 3));
    assert!(check_operands(&daddiu, 3));
}
#[test]
fn test_addi_addiu(){
    let machine_code: [u32; 2] = [0x23640038, 0x27640038];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let addi = decoder.disassemble(machine_code[0], 0).unwrap();
    let addiu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(addi.get_mnemonic() == MgMnemonic::MgMneAddi);
    assert!(addiu.get_mnemonic() == MgMnemonic::MgMneAddiu);
    assert!(addi.get_mnemonic_str() == MG_MNE_ADDI);
    assert!(addiu.get_mnemonic_str() == MG_MNE_ADDIU);
    assert!("addi" == MG_MNE_ADDI);
    assert!("addiu" == MG_MNE_ADDIU);

    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneAddi, machine_code[0], 0, 0xffff, 2));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneAddiu, machine_code[1], 0, 0xffff, 2));

    assert!(version_test(machine_code[0], MgMnemonic::MgMneAddi, true, false, true, false));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneAddiu, true, true, true, true));

    assert!(check_operands(&addi, 3));
    assert!(check_operands(&addiu, 3));
}
#[test]
fn test_lui_aui(){
    let machine_code: [u32; 2] = [0x3d1B9c58, 0x3C1B0058];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let aui = decoder.disassemble(machine_code[0], 0).unwrap();
    let lui = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(aui.get_mnemonic() == MgMnemonic::MgMneAui);
    assert!(lui.get_mnemonic() == MgMnemonic::MgMneLui);
    assert!(aui.get_mnemonic_str() == MG_MNE_AUI);
    assert!(lui.get_mnemonic_str() == MG_MNE_LUI);
    assert!("aui" == MG_MNE_AUI);
    assert!("lui" == MG_MNE_LUI);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneAui, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneLui, true, true, true, true));

    assert!(check_operands(&aui, 3));
}
#[test]
fn test_addiupc(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));
    let machine_code = 0xEF87FFFF;
    let addiupc = decoder.disassemble(machine_code, 0).unwrap();

    assert!(addiupc.get_mnemonic() == MgMnemonic::MgMneAddiupc);
    assert!(addiupc.get_mnemonic_str() == MG_MNE_ADDIUPC);
    assert!("addiupc" == MG_MNE_ADDIUPC);

    assert!(addiupc.is_relative());
    assert!(!addiupc.is_region());
    assert!(!addiupc.is_conditional());

    assert!(check_operands(&addiupc, 2));
    assert!(version_test(machine_code, MgMnemonic::MgMneAddiupc, false, true, false, true));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneAddiupc, machine_code, 0, 0x7ffff, 1));
}
#[test]
fn test_auipc(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));
    let machine_code = 0xEF9EFFFF;
    let auipc = decoder.disassemble(machine_code, 0).unwrap();

    assert!(auipc.get_mnemonic() == MgMnemonic::MgMneAuipc);
    assert!(auipc.get_mnemonic_str() == MG_MNE_AUIPC);
    assert!("auipc" == MG_MNE_AUIPC);

    assert!(auipc.is_relative());
    assert!(!auipc.is_region());
    assert!(!auipc.is_conditional());

    assert!(check_operands(&auipc, 2));
    assert!(version_test(machine_code, MgMnemonic::MgMneAuipc, false, true, false, true));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneAuipc, machine_code, 0, 0xffff, 1));
}
#[test]
fn test_aluipc(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));
    let machine_code = 0xEF9FFFFF;
    let aluipc = decoder.disassemble(machine_code, 0).unwrap();

    assert!(aluipc.get_mnemonic() == MgMnemonic::MgMneAluipc);
    assert!(aluipc.get_mnemonic_str() == MG_MNE_ALUIPC);
    assert!("aluipc" == MG_MNE_ALUIPC);

    // 0b1110111110000111
    // 0b1110111110011110

    assert!(aluipc.is_relative());
    assert!(!aluipc.is_region());
    assert!(!aluipc.is_conditional());

    assert!(check_operands(&aluipc, 2));
    assert!(version_test(machine_code, MgMnemonic::MgMneAluipc, false, true, false, true));
    assert!(imm_limit_reached(&decoder, MgMnemonic::MgMneAluipc, machine_code, 0, 0xffff, 1));
}
#[test]
fn test_add_cp1(){
    let machine_code: [u32; 3] = [0x46020040, 0x46220040, 0x46c20040];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let adds = decoder.disassemble(machine_code[0], 0).unwrap();
    let addd = decoder.disassemble(machine_code[1], 0).unwrap();
    let addps = decoder.disassemble(machine_code[2], 0).unwrap();

    assert!(adds.get_mnemonic() == MgMnemonic::MgMneAdds);
    assert!(addd.get_mnemonic() == MgMnemonic::MgMneAddd);
    assert!(addps.get_mnemonic() == MgMnemonic::MgMneAddps);
    assert!(adds.get_mnemonic_str() == MG_MNE_ADDS);
    assert!(addd.get_mnemonic_str() == MG_MNE_ADDD);
    assert!(addps.get_mnemonic_str() == MG_MNE_ADDPS);
    assert!("add.s" == MG_MNE_ADDS);
    assert!("add.d" == MG_MNE_ADDD);
    assert!("add.ps" == MG_MNE_ADDPS);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneAdds, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneAddd, true, true, true, true));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneAddps, true, false, true, false));

    assert!(!addd.is_conditional());
    assert!(!addd.is_relative());
    assert!(!addd.is_region());
    assert!(!adds.is_conditional());
    assert!(!adds.is_relative());
    assert!(!adds.is_region());
    assert!(!addps.is_conditional());
    assert!(!addps.is_relative());
    assert!(!addps.is_region());
    
    assert!(check_operands(&adds, 3));
    assert!(check_operands(&addps, 3));
    assert!(check_operands(&addd, 3));
}
#[test]
fn test_sub_cp1(){
    let machine_code: [u32; 3] = [0x46020041, 0x46220041, 0x46c20041];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let subs = decoder.disassemble(machine_code[0], 0).unwrap();
    let subd = decoder.disassemble(machine_code[1], 0).unwrap();
    let subps = decoder.disassemble(machine_code[2], 0).unwrap();

    assert!(subs.get_mnemonic() == MgMnemonic::MgMneSubs);
    assert!(subd.get_mnemonic() == MgMnemonic::MgMneSubd);
    assert!(subps.get_mnemonic() == MgMnemonic::MgMneSubps);
    assert!(subs.get_mnemonic_str() == MG_MNE_SUBS);
    assert!(subd.get_mnemonic_str() == MG_MNE_SUBD);
    assert!(subps.get_mnemonic_str() == MG_MNE_SUBPS);
    assert!("sub.s" == MG_MNE_SUBS);
    assert!("sub.d" == MG_MNE_SUBD);
    assert!("sub.ps" == MG_MNE_SUBPS);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneSubs, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneSubd, true, true, true, true));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneSubps, true, false, true, false));

    assert!(!subd.is_conditional());
    assert!(!subd.is_relative());
    assert!(!subd.is_region());
    assert!(!subs.is_conditional());
    assert!(!subs.is_relative());
    assert!(!subs.is_region());
    assert!(!subps.is_conditional());
    assert!(!subps.is_relative());
    assert!(!subps.is_region());
    
    assert!(check_operands(&subs, 3));
    assert!(check_operands(&subps, 3));
    assert!(check_operands(&subd, 3));
}
#[test]
fn test_mul_cp1(){
    let machine_code: [u32; 3] = [0x46020042, 0x46220042, 0x46c20042];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let muls = decoder.disassemble(machine_code[0], 0).unwrap();
    let muld = decoder.disassemble(machine_code[1], 0).unwrap();
    let mulps = decoder.disassemble(machine_code[2], 0).unwrap();

    assert!(muls.get_mnemonic() == MgMnemonic::MgMneMuls);
    assert!(muld.get_mnemonic() == MgMnemonic::MgMneMuld);
    assert!(mulps.get_mnemonic() == MgMnemonic::MgMneMulps);
    assert!(muls.get_mnemonic_str() == MG_MNE_MULS);
    assert!(muld.get_mnemonic_str() == MG_MNE_MULD);
    assert!(mulps.get_mnemonic_str() == MG_MNE_MULPS);
    assert!("mul.s" == MG_MNE_MULS);
    assert!("mul.d" == MG_MNE_MULD);
    assert!("mul.ps" == MG_MNE_MULPS);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneMuls, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneMuld, true, true, true, true));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneMulps, true, false, true, false));

    assert!(!muld.is_conditional());
    assert!(!muld.is_relative());
    assert!(!muld.is_region());
    assert!(!muls.is_conditional());
    assert!(!muls.is_relative());
    assert!(!muls.is_region());
    assert!(!mulps.is_conditional());
    assert!(!mulps.is_relative());
    assert!(!mulps.is_region());
    
    assert!(check_operands(&muls, 3));
    assert!(check_operands(&mulps, 3));
    assert!(check_operands(&muld, 3));
}
#[test]
fn test_div_cp1(){
    let machine_code: [u32; 3] = [0x46020043, 0x46220043, 0x46c20043];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let divs = decoder.disassemble(machine_code[0], 0).unwrap();
    let divd = decoder.disassemble(machine_code[1], 0).unwrap();
    let divps = decoder.disassemble(machine_code[2], 0).unwrap();

    assert!(divs.get_mnemonic() == MgMnemonic::MgMneDivs);
    assert!(divd.get_mnemonic() == MgMnemonic::MgMneDivd);
    assert!(divps.get_mnemonic() == MgMnemonic::MgMneDivps);
    assert!(divs.get_mnemonic_str() == MG_MNE_DIVS);
    assert!(divd.get_mnemonic_str() == MG_MNE_DIVD);
    assert!(divps.get_mnemonic_str() == MG_MNE_DIVPS);
    assert!("div.s" == MG_MNE_DIVS);
    assert!("div.d" == MG_MNE_DIVD);
    assert!("div.ps" == MG_MNE_DIVPS);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneDivs, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneDivd, true, true, true, true));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneDivps, true, false, true, false));

    assert!(!divd.is_conditional());
    assert!(!divd.is_relative());
    assert!(!divd.is_region());
    assert!(!divs.is_conditional());
    assert!(!divs.is_relative());
    assert!(!divs.is_region());
    assert!(!divps.is_conditional());
    assert!(!divps.is_relative());
    assert!(!divps.is_region());
    
    assert!(check_operands(&divs, 3));
    assert!(check_operands(&divps, 3));
    assert!(check_operands(&divd, 3));
}
#[test]
fn test_sqrt_cp1(){
    let machine_code: [u32; 3] = [0x46001044, 0x46201004, 0x46c01004];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let sqrts = decoder.disassemble(machine_code[0], 0).unwrap();
    let sqrtd = decoder.disassemble(machine_code[1], 0).unwrap();
    let sqrtps = decoder.disassemble(machine_code[2], 0).unwrap();

    assert!(sqrts.get_mnemonic() == MgMnemonic::MgMneSqrts);
    assert!(sqrtd.get_mnemonic() == MgMnemonic::MgMneSqrtd);
    assert!(sqrtps.get_mnemonic() == MgMnemonic::MgMneSqrtps);
    assert!(sqrts.get_mnemonic_str() == MG_MNE_SQRTS);
    assert!(sqrtd.get_mnemonic_str() == MG_MNE_SQRTD);
    assert!(sqrtps.get_mnemonic_str() == MG_MNE_SQRTPS);
    assert!("sqrt.s" == MG_MNE_SQRTS);
    assert!("sqrt.d" == MG_MNE_SQRTD);
    assert!("sqrt.ps" == MG_MNE_SQRTPS);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneSqrts, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneSqrtd, true, true, true, true));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneSqrtps, true, false, true, false));

    assert!(!sqrtd.is_conditional());
    assert!(!sqrtd.is_relative());
    assert!(!sqrtd.is_region());
    assert!(!sqrts.is_conditional());
    assert!(!sqrts.is_relative());
    assert!(!sqrts.is_region());
    assert!(!sqrtps.is_conditional());
    assert!(!sqrtps.is_relative());
    assert!(!sqrtps.is_region());
    
    assert!(check_operands(&sqrts, 2));
    assert!(check_operands(&sqrtps, 2));
    assert!(check_operands(&sqrtd, 2));
}
#[test]
fn test_mov_cp1(){
    let machine_code: [u32; 3] = [0x46001046, 0x46201006, 0x46c01046];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let movs = decoder.disassemble(machine_code[0], 0).unwrap();
    let movd = decoder.disassemble(machine_code[1], 0).unwrap();
    let movps = decoder.disassemble(machine_code[2], 0).unwrap();

    assert!(movs.get_mnemonic() == MgMnemonic::MgMneMovs);
    assert!(movd.get_mnemonic() == MgMnemonic::MgMneMovd);
    assert!(movps.get_mnemonic() == MgMnemonic::MgMneMovps);
    assert!(movs.get_mnemonic_str() == MG_MNE_MOVS);
    assert!(movd.get_mnemonic_str() == MG_MNE_MOVD);
    assert!(movps.get_mnemonic_str() == MG_MNE_MOVPS);
    assert!("mov.s" == MG_MNE_MOVS);
    assert!("mov.d" == MG_MNE_MOVD);
    assert!("mov.ps" == MG_MNE_MOVPS);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneMovs, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneMovd, true, true, true, true));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneMovps, true, false, true, false));

    assert!(!movd.is_conditional());
    assert!(!movd.is_relative());
    assert!(!movd.is_region());
    assert!(!movs.is_conditional());
    assert!(!movs.is_relative());
    assert!(!movs.is_region());
    assert!(!movps.is_conditional());
    assert!(!movps.is_relative());
    assert!(!movps.is_region());
    
    assert!(check_operands(&movs, 2));
    assert!(check_operands(&movps, 2));
    assert!(check_operands(&movd, 2));
}
#[test]
fn test_abs_cp1(){
    let machine_code: [u32; 3] = [0x46001045, 0x46201005, 0x46c01045];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let abss = decoder.disassemble(machine_code[0], 0).unwrap();
    let absd = decoder.disassemble(machine_code[1], 0).unwrap();
    let absps = decoder.disassemble(machine_code[2], 0).unwrap();

    assert!(abss.get_mnemonic() == MgMnemonic::MgMneAbss);
    assert!(absd.get_mnemonic() == MgMnemonic::MgMneAbsd);
    assert!(absps.get_mnemonic() == MgMnemonic::MgMneAbsps);
    assert!(abss.get_mnemonic_str() == MG_MNE_ABSS);
    assert!(absd.get_mnemonic_str() == MG_MNE_ABSD);
    assert!(absps.get_mnemonic_str() == MG_MNE_ABSPS);
    assert!("abs.s" == MG_MNE_ABSS);
    assert!("abs.d" == MG_MNE_ABSD);
    assert!("abs.ps" == MG_MNE_ABSPS);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneAbss, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneAbsd, true, true, true, true));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneAbsps, true, false, true, false));

    assert!(!absd.is_conditional());
    assert!(!absd.is_relative());
    assert!(!absd.is_region());
    assert!(!abss.is_conditional());
    assert!(!abss.is_relative());
    assert!(!abss.is_region());
    assert!(!absps.is_conditional());
    assert!(!absps.is_relative());
    assert!(!absps.is_region());
    
    assert!(check_operands(&abss, 2));
    assert!(check_operands(&absps, 2));
    assert!(check_operands(&absd, 2));
}
#[test]
fn test_neg_cp1(){
    let machine_code: [u32; 3] = [0x46001047, 0x46201007, 0x46c01047];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let negs = decoder.disassemble(machine_code[0], 0).unwrap();
    let negd = decoder.disassemble(machine_code[1], 0).unwrap();
    let negps = decoder.disassemble(machine_code[2], 0).unwrap();

    assert!(negs.get_mnemonic() == MgMnemonic::MgMneNegs);
    assert!(negd.get_mnemonic() == MgMnemonic::MgMneNegd);
    assert!(negps.get_mnemonic() == MgMnemonic::MgMneNegps);
    assert!(negs.get_mnemonic_str() == MG_MNE_NEGS);
    assert!(negd.get_mnemonic_str() == MG_MNE_NEGD);
    assert!(negps.get_mnemonic_str() == MG_MNE_NEGPS);
    assert!("neg.s" == MG_MNE_NEGS);
    assert!("neg.d" == MG_MNE_NEGD);
    assert!("neg.ps" == MG_MNE_NEGPS);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneNegs, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneNegd, true, true, true, true));
    assert!(version_test(machine_code[2], MgMnemonic::MgMneNegps, true, false, true, false));

    assert!(!negd.is_conditional());
    assert!(!negd.is_relative());
    assert!(!negd.is_region());
    assert!(!negs.is_conditional());
    assert!(!negs.is_relative());
    assert!(!negs.is_region());
    assert!(!negps.is_conditional());
    assert!(!negps.is_relative());
    assert!(!negps.is_region());
    
    assert!(check_operands(&negs, 2));
    assert!(check_operands(&negps, 2));
    assert!(check_operands(&negd, 2));
}
#[test]
fn test_round_cp1(){
    let machine_code: [u32; 3] = [0x46001047, 0x46201007, 0x46c01047];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let roundls = decoder.disassemble(machine_code[0], 0).unwrap();
    let roundld = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(roundls.get_mnemonic() == MgMnemonic::MgMneRoundls);
    assert!(roundld.get_mnemonic() == MgMnemonic::MgMneRoundld);
    assert!(roundls.get_mnemonic_str() == MG_MNE_ROUNDLS);
    assert!(roundld.get_mnemonic_str() == MG_MNE_ROUNDLD);
    assert!("round.l.s" == MG_MNE_ROUNDLS);
    assert!("round.l.d" == MG_MNE_ROUNDLD);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneRoundls, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneRoundld, true, true, true, true));

    assert!(!roundld.is_conditional());
    assert!(!roundld.is_relative());
    assert!(!roundld.is_region());
    assert!(!roundls.is_conditional());
    assert!(!roundls.is_relative());
    assert!(!roundls.is_region());
    
    assert!(check_operands(&roundls, 2));
    assert!(check_operands(&roundld, 2));

    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneRoundls, 16));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneRoundld, 16));

    let Some(MgOperand::MgOpRegister(cp1_reg)) = roundls.get_operand(0) else {
        panic!("Operand should've been a register")
    };
    let Some(MgOperand::MgOpRegister(cp1_reg_1)) = roundls.get_operand(1) else {
        panic!("Operand should've been a register")
    };
    assert!(MgCoprocessor::Cp1 == cp1_reg.get_coprocessor());
    assert!(MgCoprocessor::Cp1 == cp1_reg_1.get_coprocessor());

    let Some(MgOperand::MgOpRegister(cp1_reg)) = roundld.get_operand(0) else {
        panic!("Operand should've been a register")
    };
    let Some(MgOperand::MgOpRegister(cp1_reg_1)) = roundld.get_operand(1) else {
        panic!("Operand should've been a register")
    };
    assert!(MgCoprocessor::Cp1 == cp1_reg.get_coprocessor());
    assert!(MgCoprocessor::Cp1 == cp1_reg_1.get_coprocessor());
}
#[test]
fn test_trunc_cp1(){
    let machine_code: [u32; 3] = [0x46001047, 0x46201007, 0x46c01047];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let truncls = decoder.disassemble(machine_code[0], 0).unwrap();
    let truncld = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(truncls.get_mnemonic() == MgMnemonic::MgMneTruncls);
    assert!(truncld.get_mnemonic() == MgMnemonic::MgMneTruncld);
    assert!(truncls.get_mnemonic_str() == MG_MNE_ROUNDLS);
    assert!(truncld.get_mnemonic_str() == MG_MNE_ROUNDLD);
    assert!("trunc.l.s" == MG_MNE_ROUNDLS);
    assert!("trunc.l.d" == MG_MNE_ROUNDLD);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneTruncls, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneTruncld, true, true, true, true));

    assert!(!truncld.is_conditional());
    assert!(!truncld.is_relative());
    assert!(!truncld.is_region());
    assert!(!truncls.is_conditional());
    assert!(!truncls.is_relative());
    assert!(!truncls.is_region());
    
    assert!(check_operands(&truncls, 2));
    assert!(check_operands(&truncld, 2));

    assert!(check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneTruncls, 16));
    assert!(check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneTruncld, 16));

    let Some(MgOperand::MgOpRegister(cp1_reg)) = truncls.get_operand(0) else {
        panic!("Operand should've been a register")
    };
    let Some(MgOperand::MgOpRegister(cp1_reg_1)) = truncls.get_operand(1) else {
        panic!("Operand should've been a register")
    };
    assert!(MgCoprocessor::Cp1 == cp1_reg.get_coprocessor());
    assert!(MgCoprocessor::Cp1 == cp1_reg_1.get_coprocessor());

    let Some(MgOperand::MgOpRegister(cp1_reg)) = truncld.get_operand(0) else {
        panic!("Operand should've been a register")
    };
    let Some(MgOperand::MgOpRegister(cp1_reg_1)) = truncld.get_operand(1) else {
        panic!("Operand should've been a register")
    };
    assert!(MgCoprocessor::Cp1 == cp1_reg.get_coprocessor());
    assert!(MgCoprocessor::Cp1 == cp1_reg_1.get_coprocessor());
}
