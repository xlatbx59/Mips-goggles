//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use crate::*;
use super::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

#[test]
fn test_tne_teq() {
    let machine_code: [u32; 2] = [0x00460036, 0x00400034];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let tne = decoder.disassemble(machine_code[0], 0).unwrap();
    let teq = decoder.disassemble(machine_code[1], 0).unwrap();
    
    //No problem
    assert_eq!(tne.get_mnemonic(), MgMnemonic::MgMneTne);
    assert_eq!(teq.get_mnemonic(), MgMnemonic::MgMneTeq);

    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneTne, machine_code[0], 6, 0x3ff, 2));
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneTeq, machine_code[1], 6, 0x3ff, 2));

    assert_eq!(true, check_operands(&tne, 3));
    assert_eq!(true, check_operands(&teq, 3));

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneTne, true, true, true, true));
    assert_eq!(true, version_test(machine_code[1], MgMnemonic::MgMneTeq, true, true, true, true));
}