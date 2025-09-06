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

    assert_eq!(MgMnemonic::MgMneDclz, dclz.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneDclo, dclo.get_mnemonic());
    assert_eq!(MG_MNE_DCLZ, dclz.get_mnemonic_str());
    assert_eq!(MG_MNE_DCLO, dclo.get_mnemonic_str());
    assert_eq!("dclz", MG_MNE_DCLZ);
    assert_eq!("dclo", MG_MNE_DCLO);

    assert_eq!(false, dclo.is_conditional());
    assert_eq!(false, dclo.is_relative());
    assert_eq!(false, dclo.is_region());
    assert_eq!(false, dclz.is_conditional());
    assert_eq!(false, dclz.is_relative());
    assert_eq!(false, dclz.is_region());

    assert_eq!(true, check_operands(&dclz, 2));
    assert_eq!(true, check_operands(&dclo, 2));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11110, MgMnemonic::MgMneDclz, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11110, MgMnemonic::MgMneDclo, 6));
    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneDclz, 16));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneDclo, 16));

    decoder.version = MgMipsVersion::M64(MgMips64::MgPreR6);
    dclz = decoder.disassemble(machine_code[2], 0).unwrap();
    dclo = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneDclz, dclz.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneDclo, dclo.get_mnemonic());
    assert_eq!(MG_MNE_DCLZ, dclz.get_mnemonic_str());
    assert_eq!(MG_MNE_DCLO, dclo.get_mnemonic_str());

    assert_eq!(false, dclo.is_conditional());
    assert_eq!(false, dclo.is_relative());
    assert_eq!(false, dclo.is_region());
    assert_eq!(false, dclz.is_conditional());
    assert_eq!(false, dclz.is_relative());
    assert_eq!(false, dclz.is_region());

    assert_eq!(true, check_operands(&dclz, 2));
    assert_eq!(true, check_operands(&dclo, 2));

    assert_eq!(true, check_field(&decoder, machine_code[2], 0b11111, MgMnemonic::MgMneDclz, 6));
    assert_eq!(true, check_field(&decoder, machine_code[3], 0b11111, MgMnemonic::MgMneDclo, 6));
}
#[test]
fn test_clz_clo(){
    let machine_code: [u32; 4] = [0x00000050, 0x00000051, 0x70A42020, 0x70A42021 ];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));
    
    let mut clz = decoder.disassemble(machine_code[0], 0).unwrap();
    let mut clo = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneClz, clz.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneClo, clo.get_mnemonic());
    assert_eq!(MG_MNE_CLZ, clz.get_mnemonic_str());
    assert_eq!(MG_MNE_CLO, clo.get_mnemonic_str());
    assert_eq!("clz", MG_MNE_CLZ);
    assert_eq!("clo", MG_MNE_CLO);

    assert_eq!(false, clo.is_conditional());
    assert_eq!(false, clo.is_relative());
    assert_eq!(false, clo.is_region());
    assert_eq!(false, clz.is_conditional());
    assert_eq!(false, clz.is_relative());
    assert_eq!(false, clz.is_region());

    assert_eq!(true, check_operands(&clz, 2));
    assert_eq!(true, check_operands(&clo, 2));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11110, MgMnemonic::MgMneClz, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11110, MgMnemonic::MgMneClo, 6));
    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneClz, 16));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneClo, 16));

    decoder.version = MgMipsVersion::M64(MgMips64::MgPreR6);
    clz = decoder.disassemble(machine_code[2], 0).unwrap();
    clo = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(MgMnemonic::MgMneClz, clz.get_mnemonic());
    assert_eq!(MgMnemonic::MgMneClo, clo.get_mnemonic());
    assert_eq!(MG_MNE_CLZ, clz.get_mnemonic_str());
    assert_eq!(MG_MNE_CLO, clo.get_mnemonic_str());

    assert_eq!(false, clo.is_conditional());
    assert_eq!(false, clo.is_relative());
    assert_eq!(false, clo.is_region());
    assert_eq!(false, clz.is_conditional());
    assert_eq!(false, clz.is_relative());
    assert_eq!(false, clz.is_region());

    assert_eq!(true, check_operands(&clz, 2));
    assert_eq!(true, check_operands(&clo, 2));

    assert_eq!(true, check_field(&decoder, machine_code[2], 0b11111, MgMnemonic::MgMneClz, 6));
    assert_eq!(true, check_field(&decoder, machine_code[3], 0b11111, MgMnemonic::MgMneClo, 6));
}
#[test]
fn test_xor_nor(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let machine_code = [0x00BB2026, 0x00BB2027];
    let xor = decoder.disassemble(machine_code[0], 0).unwrap();
    let nor = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(xor.get_mnemonic(), MgMnemonic::MgMneXor);
    assert_eq!(nor.get_mnemonic(), MgMnemonic::MgMneNor);
    assert_eq!(nor.get_mnemonic_str(), MG_MNE_NOR);
    assert_eq!(xor.get_mnemonic_str(), MG_MNE_XOR);
    assert_eq!("nor", MG_MNE_NOR);
    assert_eq!("xor", MG_MNE_XOR);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneXor, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneNor, true, true, true, true));

    assert_eq!(true, check_operands(&xor, 3));
    assert_eq!(true, check_operands(&nor, 3));
    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneXor, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneNor, 6));
}
#[test]
fn test_or_xor(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let machine_code = [0x00BB2025, 0x00BB2024];
    let or = decoder.disassemble(machine_code[0], 0).unwrap();
    let and = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(or.get_mnemonic(), MgMnemonic::MgMneOr);
    assert_eq!(and.get_mnemonic(), MgMnemonic::MgMneAnd);
    assert_eq!(and.get_mnemonic_str(), MG_MNE_AND);
    assert_eq!(or.get_mnemonic_str(), MG_MNE_OR);
    assert_eq!("and", MG_MNE_AND);
    assert_eq!("or", MG_MNE_OR);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneOr, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneAnd, true, true, true, true));

    assert_eq!(true, check_operands(&or, 3));
    assert_eq!(true, check_operands(&and, 3));
    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneOr, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneAnd, 6));
}
#[test]
fn test_sub_subu(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let machine_code = [0x00BB2022, 0x00BB2023];
    let sub = decoder.disassemble(machine_code[0], 0).unwrap();
    let subu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(sub.get_mnemonic(), MgMnemonic::MgMneSub);
    assert_eq!(subu.get_mnemonic(), MgMnemonic::MgMneSubu);
    assert_eq!(subu.get_mnemonic_str(), MG_MNE_SUBU);
    assert_eq!(sub.get_mnemonic_str(), MG_MNE_SUB);
    assert_eq!("subu", MG_MNE_SUBU);
    assert_eq!("sub", MG_MNE_SUB);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneSub, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneSubu, true, true, true, true));

    assert_eq!(true, check_operands(&sub, 3));
    assert_eq!(true, check_operands(&subu, 3));
    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneSub, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneSubu, 6));
}
#[test]
fn test_add_addu(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let machine_code = [0x00BB2020, 0x00BB2021];
    let add = decoder.disassemble(machine_code[0], 0).unwrap();
    let addu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(add.get_mnemonic(), MgMnemonic::MgMneAdd);
    assert_eq!(addu.get_mnemonic(), MgMnemonic::MgMneAddu);
    assert_eq!(addu.get_mnemonic_str(), MG_MNE_ADDU);
    assert_eq!(add.get_mnemonic_str(), MG_MNE_ADD);
    assert_eq!("addu", MG_MNE_ADDU);
    assert_eq!("add", MG_MNE_ADD);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneAdd, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneAddu, true, true, true, true));

    assert_eq!(true, check_operands(&add, 3));
    assert_eq!(true, check_operands(&addu, 3));
    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneAdd, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneAddu, 6));
}
#[test]
fn test_mul(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));
    let machine_code = 0x70f34802;
    let mul = decoder.disassemble(machine_code, 0).unwrap();

    assert_eq!(mul.get_mnemonic(), MgMnemonic::MgMneMul);
    assert_eq!(mul.get_mnemonic_str(), MG_MNE_MUL);
    assert_eq!("mul", MG_MNE_MUL);

    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneMul, true, false, true, false));

    assert_eq!(true, check_operands(&mul, 3));
    assert_eq!(true, check_field(&decoder, machine_code, 0b11111, MgMnemonic::MgMneMul, 6));
}
#[test]
fn sop30(){
    let machine_code = [0x2E75098, 0x2E750D8];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let mul = decoder.disassemble(machine_code[0], 0).unwrap();
    let muh = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(mul.get_mnemonic(), MgMnemonic::MgMneMul);
    assert_eq!(muh.get_mnemonic(), MgMnemonic::MgMneMuh);
    assert_eq!(mul.get_mnemonic_str(), MG_MNE_MUL);
    assert_eq!(muh.get_mnemonic_str(), MG_MNE_MUH);
    assert_eq!("mul", MG_MNE_MUL);
    assert_eq!("muh", MG_MNE_MUH);

    assert_eq!(false, muh.is_conditional());
    assert_eq!(false, muh.is_relative());
    assert_eq!(false, muh.is_region());
    assert_eq!(false, mul.is_conditional());
    assert_eq!(false, mul.is_relative());
    assert_eq!(false, mul.is_region());

    assert_eq!(0b00010, mul.get_machine_code() >> 6 & 0b11111);
    assert_eq!(0b00011, muh.get_machine_code() >> 6 & 0b11111);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneMul, false, true, false, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneMuh, false, true, false, true));

    assert_eq!(true, check_operands(&mul, 3));
    assert_eq!(true, check_operands(&muh, 3));
}
#[test]
fn sop31(){
    let machine_code = [0x2E75099, 0x2E750D9];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let mulu = decoder.disassemble(machine_code[0], 0).unwrap();
    let muhu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(mulu.get_mnemonic(), MgMnemonic::MgMneMulu);
    assert_eq!(muhu.get_mnemonic(), MgMnemonic::MgMneMuhu);
    assert_eq!(mulu.get_mnemonic_str(), MG_MNE_MULU);
    assert_eq!(muhu.get_mnemonic_str(), MG_MNE_MUHU);
    assert_eq!("mulu", MG_MNE_MULU);
    assert_eq!("muhu", MG_MNE_MUHU);

    assert_eq!(false, muhu.is_conditional());
    assert_eq!(false, muhu.is_relative());
    assert_eq!(false, muhu.is_region());
    assert_eq!(false, mulu.is_conditional());
    assert_eq!(false, mulu.is_relative());
    assert_eq!(false, mulu.is_region());

    assert_eq!(0b00010, mulu.get_machine_code() >> 6 & 0b11111);
    assert_eq!(0b00011, muhu.get_machine_code() >> 6 & 0b11111);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneMulu, false, true, false, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneMuhu, false, true, false, true));

    assert_eq!(true, check_operands(&mulu, 3));
    assert_eq!(true, check_operands(&muhu, 3));
}
#[test]
fn sop32(){
    let machine_code = [0x2E7509a, 0x2E750Da];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let div = decoder.disassemble(machine_code[0], 0).unwrap();
    let mod_i = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(div.get_mnemonic(), MgMnemonic::MgMneDiv);
    assert_eq!(mod_i.get_mnemonic(), MgMnemonic::MgMneMod);
    assert_eq!(div.get_mnemonic_str(), MG_MNE_DIV);
    assert_eq!(mod_i.get_mnemonic_str(), MG_MNE_MOD);
    assert_eq!("div", MG_MNE_DIV);
    assert_eq!("mod", MG_MNE_MOD);

    assert_eq!(false, mod_i.is_conditional());
    assert_eq!(false, mod_i.is_relative());
    assert_eq!(false, mod_i.is_region());
    assert_eq!(false, div.is_conditional());
    assert_eq!(false, div.is_relative());
    assert_eq!(false, div.is_region());

    assert_eq!(0b00010, div.get_machine_code() >> 6 & 0b11111);
    assert_eq!(0b00011, mod_i.get_machine_code() >> 6 & 0b11111);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDiv, false, true, false, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneMod, false, true, false, true));

    assert_eq!(true, check_operands(&div, 3));
    assert_eq!(true, check_operands(&mod_i, 3));
}
#[test]
fn sop33(){
    let machine_code = [0x2E7509b, 0x2E750Db];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let divu = decoder.disassemble(machine_code[0], 0).unwrap();
    let modu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(divu.get_mnemonic(), MgMnemonic::MgMneDivu);
    assert_eq!(modu.get_mnemonic(), MgMnemonic::MgMneModu);
    assert_eq!(divu.get_mnemonic_str(), MG_MNE_DIVU);
    assert_eq!(modu.get_mnemonic_str(), MG_MNE_MODU);
    assert_eq!("divu", MG_MNE_DIVU);
    assert_eq!("modu", MG_MNE_MODU);

    assert_eq!(false, modu.is_conditional());
    assert_eq!(false, modu.is_relative());
    assert_eq!(false, modu.is_region());
    assert_eq!(false, divu.is_conditional());
    assert_eq!(false, divu.is_relative());
    assert_eq!(false, divu.is_region());

    assert_eq!(0b00010, divu.get_machine_code() >> 6 & 0b11111);
    assert_eq!(0b00011, modu.get_machine_code() >> 6 & 0b11111);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDivu, false, true, false, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneModu, false, true, false, true));

    assert_eq!(true, check_operands(&divu, 3));
    assert_eq!(true, check_operands(&modu, 3));
}
#[test]
fn sop34(){
    let machine_code = [0x2E7509c, 0x2E750Dc];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let dmul = decoder.disassemble(machine_code[0], 0).unwrap();
    let dmuh = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(dmul.get_mnemonic(), MgMnemonic::MgMneDmul);
    assert_eq!(dmuh.get_mnemonic(), MgMnemonic::MgMneDmuh);
    assert_eq!(dmul.get_mnemonic_str(), MG_MNE_DMUL);
    assert_eq!(dmuh.get_mnemonic_str(), MG_MNE_DMUH);
    assert_eq!("dmul", MG_MNE_DMUL);
    assert_eq!("dmuh", MG_MNE_DMUH);

    assert_eq!(false, dmuh.is_conditional());
    assert_eq!(false, dmuh.is_relative());
    assert_eq!(false, dmuh.is_region());
    assert_eq!(false, dmul.is_conditional());
    assert_eq!(false, dmul.is_relative());
    assert_eq!(false, dmul.is_region());

    assert_eq!(0b00010, dmul.get_machine_code() >> 6 & 0b11111);
    assert_eq!(0b00011, dmuh.get_machine_code() >> 6 & 0b11111);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDmul, false, false, false, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDmuh, false, false, false, true));

    assert_eq!(true, check_operands(&dmul, 3));
    assert_eq!(true, check_operands(&dmuh, 3));
}
#[test]
fn sop35(){
    let machine_code = [0x2E7509d, 0x2E750Dd];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let dmulu = decoder.disassemble(machine_code[0], 0).unwrap();
    let dmuhu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(dmulu.get_mnemonic(), MgMnemonic::MgMneDmulu);
    assert_eq!(dmuhu.get_mnemonic(), MgMnemonic::MgMneDmuhu);
    assert_eq!(dmulu.get_mnemonic_str(), MG_MNE_DMULU);
    assert_eq!(dmuhu.get_mnemonic_str(), MG_MNE_DMUHU);

    assert_eq!(false, dmuhu.is_conditional());
    assert_eq!(false, dmuhu.is_relative());
    assert_eq!(false, dmuhu.is_region());
    assert_eq!(false, dmulu.is_conditional());
    assert_eq!(false, dmulu.is_relative());
    assert_eq!(false, dmulu.is_region());

    assert_eq!(0b00010, dmulu.get_machine_code() >> 6 & 0b11111);
    assert_eq!(0b00011, dmuhu.get_machine_code() >> 6 & 0b11111);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDmulu, false, false, false, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDmuhu, false, false, false, true));

    assert_eq!(true, check_operands(&dmulu, 3));
    assert_eq!(true, check_operands(&dmuhu, 3));
}
#[test]
fn sop36(){
    let machine_code = [0x2E7509e, 0x2E750De];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let ddiv = decoder.disassemble(machine_code[0], 0).unwrap();
    let dmod = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(ddiv.get_mnemonic(), MgMnemonic::MgMneDdiv);
    assert_eq!(dmod.get_mnemonic(), MgMnemonic::MgMneDmod);
    assert_eq!(ddiv.get_mnemonic_str(), MG_MNE_DDIV);
    assert_eq!(dmod.get_mnemonic_str(), MG_MNE_DMOD);
    assert_eq!("ddiv", MG_MNE_DDIV);
    assert_eq!("dmod", MG_MNE_DMOD);

    assert_eq!(false, dmod.is_conditional());
    assert_eq!(false, dmod.is_relative());
    assert_eq!(false, dmod.is_region());
    assert_eq!(false, ddiv.is_conditional());
    assert_eq!(false, ddiv.is_relative());
    assert_eq!(false, ddiv.is_region());

    assert_eq!(0b00010, ddiv.get_machine_code() >> 6 & 0b11111);
    assert_eq!(0b00011, dmod.get_machine_code() >> 6 & 0b11111);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDdiv, false, false, false, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDmod, false, false, false, true));

    assert_eq!(true, check_operands(&ddiv, 3));
    assert_eq!(true, check_operands(&dmod, 3));
}
#[test]
fn sop37(){
    let machine_code = [0x2E7509f, 0x2E750Df];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    let ddivu = decoder.disassemble(machine_code[0], 0).unwrap();
    let dmodu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(ddivu.get_mnemonic(), MgMnemonic::MgMneDdivu);
    assert_eq!(dmodu.get_mnemonic(), MgMnemonic::MgMneDmodu);
    assert_eq!(ddivu.get_mnemonic_str(), MG_MNE_DDIVU);
    assert_eq!(dmodu.get_mnemonic_str(), MG_MNE_DMODU);
    assert_eq!("ddivu", MG_MNE_DDIVU);
    assert_eq!("dmodu", MG_MNE_DMODU);

    assert_eq!(false, dmodu.is_conditional());
    assert_eq!(false, dmodu.is_relative());
    assert_eq!(false, dmodu.is_region());
    assert_eq!(false, ddivu.is_conditional());
    assert_eq!(false, ddivu.is_relative());
    assert_eq!(false, ddivu.is_region());

    assert_eq!(0b00010, ddivu.get_machine_code() >> 6 & 0b11111);
    assert_eq!(0b00011, dmodu.get_machine_code() >> 6 & 0b11111);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDdivu, false, false, false, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDmodu, false, false, false, true));

    assert_eq!(true, check_operands(&ddivu, 3));
    assert_eq!(true, check_operands(&dmodu, 3));
}
#[test]
fn test_dmult_dmultu(){
    let machine_code = [0x0085001C, 0x0085001D];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let dmult = decoder.disassemble(machine_code[0], 0).unwrap();
    let dmultu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(dmult.get_mnemonic(), MgMnemonic::MgMneDmult);
    assert_eq!(dmultu.get_mnemonic(), MgMnemonic::MgMneDmultu);
    assert_eq!(dmult.get_mnemonic_str(), MG_MNE_DMULT);
    assert_eq!(dmultu.get_mnemonic_str(), MG_MNE_DMULTU);
    assert_eq!("dmult", MG_MNE_DMULT);
    assert_eq!("dmultu", MG_MNE_DMULTU);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDmult, false, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDmultu, false, false, true, false));

    assert_eq!(true, check_operands(&dmult, 2));
    assert_eq!(true, check_operands(&dmultu, 2));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0b1111111111, MgMnemonic::MgMneDmult, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b1111111111, MgMnemonic::MgMneDmultu, 6));
}
#[test]
fn test_ddiv_ddivu(){
    let machine_code = [0x0044001e, 0x000A001f];

    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let ddiv = decoder.disassemble(machine_code[0], 0).unwrap();
    let ddivu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(ddiv.get_mnemonic(), MgMnemonic::MgMneDdiv);
    assert_eq!(ddivu.get_mnemonic(), MgMnemonic::MgMneDdivu);
    assert_eq!(ddiv.get_mnemonic_str(), MG_MNE_DDIV);
    assert_eq!(ddivu.get_mnemonic_str(), MG_MNE_DDIVU);
    assert_eq!("ddiv", MG_MNE_DDIV);
    assert_eq!("ddivu", MG_MNE_DDIVU);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDdiv, false, false, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDdivu, false, false, true, true));

    assert_eq!(true, check_operands(&ddiv, 2));
    assert_eq!(true, check_operands(&ddivu, 2));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0b1111111111, MgMnemonic::MgMneDdiv, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b1111111111, MgMnemonic::MgMneDdivu, 6));
}
#[test]
fn test_msub_msubu(){
    let machine_code: [u32; 2] = [0x70850004, 0x70850005];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let msub = decoder.disassemble(machine_code[0], 0).unwrap();
    let msubu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(msub.get_mnemonic(), MgMnemonic::MgMneMsub);
    assert_eq!(msubu.get_mnemonic(), MgMnemonic::MgMneMsubu);
    assert_eq!(msub.get_mnemonic_str(), MG_MNE_MSUB);
    assert_eq!(msubu.get_mnemonic_str(), MG_MNE_MSUBU);
    assert_eq!("msub", MG_MNE_MSUB);
    assert_eq!("msubu", MG_MNE_MSUBU);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneMsub, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneMsubu, true, false, true, false));

    assert_eq!(false, msubu.is_conditional());
    assert_eq!(false, msubu.is_relative());
    assert_eq!(false, msubu.is_region());
    assert_eq!(false, msub.is_conditional());
    assert_eq!(false, msub.is_relative());
    assert_eq!(false, msub.is_region());
    
    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneMsub, 6));
    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneMsub, 11));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneMsubu, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneMsubu, 11));

    assert_eq!(true, check_operands(&msub, 2));
    assert_eq!(true, check_operands(&msubu, 2));
}
#[test]
fn test_dsub_dsubu(){
    let machine_code: [u32; 2] = [0x00A2202E, 0x00A2202F];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let dsub = decoder.disassemble(machine_code[0], 0).unwrap();
    let dsubu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(dsub.get_mnemonic(), MgMnemonic::MgMneDsub);
    assert_eq!(dsubu.get_mnemonic(), MgMnemonic::MgMneDsubu);
    assert_eq!(dsub.get_mnemonic_str(), MG_MNE_DSUB);
    assert_eq!(dsubu.get_mnemonic_str(), MG_MNE_DSUBU);
    assert_eq!("dsub", MG_MNE_DSUB);
    assert_eq!("dsubu", MG_MNE_DSUBU);

    assert_eq!(false, dsubu.is_conditional());
    assert_eq!(false, dsubu.is_relative());
    assert_eq!(false, dsubu.is_region());
    assert_eq!(false, dsub.is_conditional());
    assert_eq!(false, dsub.is_relative());
    assert_eq!(false, dsub.is_region());

    assert_eq!(true, check_operands(&dsub, 3));
    assert_eq!(true, check_operands(&dsubu, 3));
    
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDsub, false, false, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDsubu, false, false, true, true));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneDsub, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneDsubu, 6));
}
#[test]
fn test_dadd_daddu(){
    let machine_code: [u32; 2] = [0x00A2202C, 0x00A2202D];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let dadd = decoder.disassemble(machine_code[0], 0).unwrap();
    let daddu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(dadd.get_mnemonic(), MgMnemonic::MgMneDadd);
    assert_eq!(daddu.get_mnemonic(), MgMnemonic::MgMneDaddu);
    assert_eq!(dadd.get_mnemonic_str(), MG_MNE_DADD);
    assert_eq!(daddu.get_mnemonic_str(), MG_MNE_DADDU);
    assert_eq!("dadd", MG_MNE_DADD);
    assert_eq!("daddu", MG_MNE_DADDU);

    assert_eq!(false, daddu.is_conditional());
    assert_eq!(false, daddu.is_relative());
    assert_eq!(false, daddu.is_region());
    assert_eq!(false, dadd.is_conditional());
    assert_eq!(false, dadd.is_relative());
    assert_eq!(false, dadd.is_region());

    assert_eq!(true, check_operands(&dadd, 3));
    assert_eq!(true, check_operands(&daddu, 3));
    
    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDadd, false, false, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDaddu, false, false, true, true));

    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneDadd, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneDaddu, 6));
}
#[test]
fn test_madd_maddu(){
    let machine_code: [u32; 2] = [0x70850000, 0x70850001];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let madd = decoder.disassemble(machine_code[0], 0).unwrap();
    let maddu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(madd.get_mnemonic(), MgMnemonic::MgMneMadd);
    assert_eq!(maddu.get_mnemonic(), MgMnemonic::MgMneMaddu);
    assert_eq!(madd.get_mnemonic_str(), MG_MNE_MADD);
    assert_eq!(maddu.get_mnemonic_str(), MG_MNE_MADDU);
    assert_eq!("madd", MG_MNE_MADD);
    assert_eq!("maddu", MG_MNE_MADDU);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneMadd, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneMaddu, true, false, true, false));

    assert_eq!(false, maddu.is_conditional());
    assert_eq!(false, maddu.is_relative());
    assert_eq!(false, maddu.is_region());
    assert_eq!(false, madd.is_conditional());
    assert_eq!(false, madd.is_relative());
    assert_eq!(false, madd.is_region());
    
    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneMadd, 6));
    assert_eq!(true, check_field(&decoder, machine_code[0], 0b11111, MgMnemonic::MgMneMadd, 11));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneMaddu, 6));
    assert_eq!(true, check_field(&decoder, machine_code[1], 0b11111, MgMnemonic::MgMneMaddu, 11));

    assert_eq!(true, check_operands(&madd, 2));
    assert_eq!(true, check_operands(&maddu, 2));
}
#[test]
fn test_daddi_daddiu(){
    let machine_code: [u32; 2] = [0x63640038, 0x67640038];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let daddi = decoder.disassemble(machine_code[0], 0).unwrap();
    let daddiu = decoder.disassemble(machine_code[1], 0).unwrap();
    
    assert_eq!(daddi.get_mnemonic(), MgMnemonic::MgMneDaddi);
    assert_eq!(daddiu.get_mnemonic(), MgMnemonic::MgMneDaddiu);
    assert_eq!(daddi.get_mnemonic_str(), MG_MNE_DADDI);
    assert_eq!(daddiu.get_mnemonic_str(), MG_MNE_DADDIU);
    assert_eq!("daddi", MG_MNE_DADDI);
    assert_eq!("daddiu", MG_MNE_DADDIU);

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneDaddi, machine_code[0], 0, 0xffff, 2));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneDaddiu, machine_code[1], 0, 0xffff, 2));

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDaddi, false, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDaddiu, false, false, true, true));

    assert_eq!(true, check_operands(&daddi, 3));
    assert_eq!(true, check_operands(&daddiu, 3));
}
#[test]
fn test_addi_addiu(){
    let machine_code: [u32; 2] = [0x23640038, 0x27640038];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let addi = decoder.disassemble(machine_code[0], 0).unwrap();
    let addiu = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(addi.get_mnemonic(), MgMnemonic::MgMneAddi);
    assert_eq!(addiu.get_mnemonic(), MgMnemonic::MgMneAddiu);
    assert_eq!(addi.get_mnemonic_str(), MG_MNE_ADDI);
    assert_eq!(addiu.get_mnemonic_str(), MG_MNE_ADDIU);
    assert_eq!("addi", MG_MNE_ADDI);
    assert_eq!("addiu", MG_MNE_ADDIU);

    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneAddi, machine_code[0], 0, 0xffff, 2));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneAddiu, machine_code[1], 0, 0xffff, 2));

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneAddi, true, false, true, false));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneAddiu, true, true, true, true));

    assert_eq!(true, check_operands(&addi, 3));
    assert_eq!(true, check_operands(&addiu, 3));
}
#[test]
fn test_lui_aui(){
    let machine_code: [u32; 2] = [0x3d1B9c58, 0x3C1B0058];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let aui = decoder.disassemble(machine_code[0], 0).unwrap();
    let lui = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(aui.get_mnemonic(), MgMnemonic::MgMneAui);
    assert_eq!(lui.get_mnemonic(), MgMnemonic::MgMneLui);
    assert_eq!(aui.get_mnemonic_str(), MG_MNE_AUI);
    assert_eq!(lui.get_mnemonic_str(), MG_MNE_LUI);
    assert_eq!("aui", MG_MNE_AUI);
    assert_eq!("lui", MG_MNE_LUI);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneAui, false, true, false, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneLui, true, true, true, true));

    assert_eq!(true, check_operands(&aui, 3));
}
#[test]
fn test_addiupc(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));
    let machine_code = 0xEF87FFFF;
    let addiupc = decoder.disassemble(machine_code, 0).unwrap();

    assert_eq!(addiupc.get_mnemonic(), MgMnemonic::MgMneAddiupc);
    assert_eq!(addiupc.get_mnemonic_str(), MG_MNE_ADDIUPC);
    assert_eq!("addiupc", MG_MNE_ADDIUPC);

    assert_eq!(true, addiupc.is_relative());
    assert_eq!(false, addiupc.is_region());
    assert_eq!(false, addiupc.is_conditional());

    assert_eq!(true, check_operands(&addiupc, 2));
    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneAddiupc, false, true, false, true));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneAddiupc, machine_code, 0, 0x7ffff, 1));
}
#[test]
fn test_auipc(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));
    let machine_code = 0xEF9EFFFF;
    let auipc = decoder.disassemble(machine_code, 0).unwrap();

    assert_eq!(auipc.get_mnemonic(), MgMnemonic::MgMneAuipc);
    assert_eq!(auipc.get_mnemonic_str(), MG_MNE_AUIPC);
    assert_eq!("auipc", MG_MNE_AUIPC);

    assert_eq!(true, auipc.is_relative());
    assert_eq!(false, auipc.is_region());
    assert_eq!(false, auipc.is_conditional());

    assert_eq!(true, check_operands(&auipc, 2));
    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneAuipc, false, true, false, true));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneAuipc, machine_code, 0, 0xffff, 1));
}
#[test]
fn test_aluipc(){
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));
    let machine_code = 0xEF9FFFFF;
    let aluipc = decoder.disassemble(machine_code, 0).unwrap();

    assert_eq!(aluipc.get_mnemonic(), MgMnemonic::MgMneAluipc);
    assert_eq!(aluipc.get_mnemonic_str(), MG_MNE_ALUIPC);
    assert_eq!("aluipc", MG_MNE_ALUIPC);

    // 0b1110111110000111
    // 0b1110111110011110

    assert_eq!(true, aluipc.is_relative());
    assert_eq!(false, aluipc.is_region());
    assert_eq!(false, aluipc.is_conditional());

    assert_eq!(true, check_operands(&aluipc, 2));
    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneAluipc, false, true, false, true));
    assert_eq!(true, imm_limit_reached(&decoder, MgMnemonic::MgMneAluipc, machine_code, 0, 0xffff, 1));
}
#[test]
fn test_add_cp1(){
    let machine_code: [u32; 3] = [0x46020040, 0x46220040, 0x46c20040];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let adds = decoder.disassemble(machine_code[0], 0).unwrap();
    let addd = decoder.disassemble(machine_code[1], 0).unwrap();
    let addps = decoder.disassemble(machine_code[2], 0).unwrap();

    assert_eq!(adds.get_mnemonic(), MgMnemonic::MgMneAdds);
    assert_eq!(addd.get_mnemonic(), MgMnemonic::MgMneAddd);
    assert_eq!(addps.get_mnemonic(), MgMnemonic::MgMneAddps);
    assert_eq!(adds.get_mnemonic_str(), MG_MNE_ADDS);
    assert_eq!(addd.get_mnemonic_str(), MG_MNE_ADDD);
    assert_eq!(addps.get_mnemonic_str(), MG_MNE_ADDPS);
    assert_eq!("add.s", MG_MNE_ADDS);
    assert_eq!("add.d", MG_MNE_ADDD);
    assert_eq!("add.ps", MG_MNE_ADDPS);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneAdds, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneAddd, true, true, true, true));
    assert_eq!(true, version_test(machine_code[2], MgMnemonic::MgMneAddps, true, false, true, false));

    assert_eq!(false, addd.is_conditional());
    assert_eq!(false, addd.is_relative());
    assert_eq!(false, addd.is_region());
    assert_eq!(false, adds.is_conditional());
    assert_eq!(false, adds.is_relative());
    assert_eq!(false, adds.is_region());
    assert_eq!(false, addps.is_conditional());
    assert_eq!(false, addps.is_relative());
    assert_eq!(false, addps.is_region());
    
    assert_eq!(true, check_operands(&adds, 3));
    assert_eq!(true, check_operands(&addps, 3));
    assert_eq!(true, check_operands(&addd, 3));
}
#[test]
fn test_sub_cp1(){
    let machine_code: [u32; 3] = [0x46020041, 0x46220041, 0x46c20041];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let subs = decoder.disassemble(machine_code[0], 0).unwrap();
    let subd = decoder.disassemble(machine_code[1], 0).unwrap();
    let subps = decoder.disassemble(machine_code[2], 0).unwrap();

    assert_eq!(subs.get_mnemonic(), MgMnemonic::MgMneSubs);
    assert_eq!(subd.get_mnemonic(), MgMnemonic::MgMneSubd);
    assert_eq!(subps.get_mnemonic(), MgMnemonic::MgMneSubps);
    assert_eq!(subs.get_mnemonic_str(), MG_MNE_SUBS);
    assert_eq!(subd.get_mnemonic_str(), MG_MNE_SUBD);
    assert_eq!(subps.get_mnemonic_str(), MG_MNE_SUBPS);
    assert_eq!("sub.s", MG_MNE_SUBS);
    assert_eq!("sub.d", MG_MNE_SUBD);
    assert_eq!("sub.ps", MG_MNE_SUBPS);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneSubs, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneSubd, true, true, true, true));
    assert_eq!(true, version_test(machine_code[2], MgMnemonic::MgMneSubps, true, false, true, false));

    assert_eq!(false, subd.is_conditional());
    assert_eq!(false, subd.is_relative());
    assert_eq!(false, subd.is_region());
    assert_eq!(false, subs.is_conditional());
    assert_eq!(false, subs.is_relative());
    assert_eq!(false, subs.is_region());
    assert_eq!(false, subps.is_conditional());
    assert_eq!(false, subps.is_relative());
    assert_eq!(false, subps.is_region());
    
    assert_eq!(true, check_operands(&subs, 3));
    assert_eq!(true, check_operands(&subps, 3));
    assert_eq!(true, check_operands(&subd, 3));
}
#[test]
fn test_mul_cp1(){
    let machine_code: [u32; 3] = [0x46020042, 0x46220042, 0x46c20042];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let muls = decoder.disassemble(machine_code[0], 0).unwrap();
    let muld = decoder.disassemble(machine_code[1], 0).unwrap();
    let mulps = decoder.disassemble(machine_code[2], 0).unwrap();

    assert_eq!(muls.get_mnemonic(), MgMnemonic::MgMneMuls);
    assert_eq!(muld.get_mnemonic(), MgMnemonic::MgMneMuld);
    assert_eq!(mulps.get_mnemonic(), MgMnemonic::MgMneMulps);
    assert_eq!(muls.get_mnemonic_str(), MG_MNE_MULS);
    assert_eq!(muld.get_mnemonic_str(), MG_MNE_MULD);
    assert_eq!(mulps.get_mnemonic_str(), MG_MNE_MULPS);
    assert_eq!("mul.s", MG_MNE_MULS);
    assert_eq!("mul.d", MG_MNE_MULD);
    assert_eq!("mul.ps", MG_MNE_MULPS);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneMuls, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneMuld, true, true, true, true));
    assert_eq!(true, version_test(machine_code[2], MgMnemonic::MgMneMulps, true, false, true, false));

    assert_eq!(false, muld.is_conditional());
    assert_eq!(false, muld.is_relative());
    assert_eq!(false, muld.is_region());
    assert_eq!(false, muls.is_conditional());
    assert_eq!(false, muls.is_relative());
    assert_eq!(false, muls.is_region());
    assert_eq!(false, mulps.is_conditional());
    assert_eq!(false, mulps.is_relative());
    assert_eq!(false, mulps.is_region());
    
    assert_eq!(true, check_operands(&muls, 3));
    assert_eq!(true, check_operands(&mulps, 3));
    assert_eq!(true, check_operands(&muld, 3));
}
#[test]
fn test_div_cp1(){
    let machine_code: [u32; 3] = [0x46020043, 0x46220043, 0x46c20043];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgPreR6));

    let divs = decoder.disassemble(machine_code[0], 0).unwrap();
    let divd = decoder.disassemble(machine_code[1], 0).unwrap();
    let divps = decoder.disassemble(machine_code[2], 0).unwrap();

    assert_eq!(divs.get_mnemonic(), MgMnemonic::MgMneDivs);
    assert_eq!(divd.get_mnemonic(), MgMnemonic::MgMneDivd);
    assert_eq!(divps.get_mnemonic(), MgMnemonic::MgMneDivps);
    assert_eq!(divs.get_mnemonic_str(), MG_MNE_DIVS);
    assert_eq!(divd.get_mnemonic_str(), MG_MNE_DIVD);
    assert_eq!(divps.get_mnemonic_str(), MG_MNE_DIVPS);
    assert_eq!("div.s", MG_MNE_DIVS);
    assert_eq!("div.d", MG_MNE_DIVD);
    assert_eq!("div.ps", MG_MNE_DIVPS);

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDivs, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneDivd, true, true, true, true));
    assert_eq!(true, version_test(machine_code[2], MgMnemonic::MgMneDivps, true, false, true, false));

    assert_eq!(false, divd.is_conditional());
    assert_eq!(false, divd.is_relative());
    assert_eq!(false, divd.is_region());
    assert_eq!(false, divs.is_conditional());
    assert_eq!(false, divs.is_relative());
    assert_eq!(false, divs.is_region());
    assert_eq!(false, divps.is_conditional());
    assert_eq!(false, divps.is_relative());
    assert_eq!(false, divps.is_region());
    
    assert_eq!(true, check_operands(&divs, 3));
    assert_eq!(true, check_operands(&divps, 3));
    assert_eq!(true, check_operands(&divd, 3));
}