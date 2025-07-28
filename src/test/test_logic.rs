//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use super::*;
use crate::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_andi(){
    let machine_code = 0x33640058;
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let andi = decoder.disassemble(machine_code, 0).unwrap();

    assert_eq!(andi.get_mnemonic(), MgMnemonic::MgMneAndi);
    assert_eq!(andi.get_mnemonic_str(), MG_MNE_ANDI);

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

    assert_eq!(true, version_test(machine_code, MgMnemonic::MgMneXori, true, true, true, true));

    assert_eq!(true, check_operands(&xori, 3));
}