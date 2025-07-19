//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

#[cfg(test)]
use crate::*;
#[cfg(test)]
use crate::disassembler::*;
#[cfg(test)]
use crate::instruction::*;
#[cfg(test)]
use crate::operands::*;
#[cfg(test)]
use crate::instruction::mnemonics::*;

#[test]
fn test_pop07(){
    let machine_code: [u32; 4] = [0x1c30FFFF, 0x1c0a0050, 0x1c420050, 0x1c00C011];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[1], 0).unwrap();
    let inst2 = decoder.disassemble(machine_code[2], 0).unwrap();
    let mut inst3 = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(inst0.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(inst1.is_region(), false);
    assert_eq!(inst1.is_relative(), true);
    assert_eq!(inst1.is_conditional(), true);

    assert_eq!(inst0.get_mnemonicid(), Some(MgMnemonic::MgMneBltuc));
    assert_eq!(get_mnemonic(inst0.get_mnemonicid().unwrap()), MG_MNE_BLTUC);
    assert_eq!(inst0.get_operand_num(), 3);
    match (inst0.get_operand(0), inst0.get_operand(1), inst0.get_operand(2)){
        (Some(MgOperand::MgOpRegister(r1)),Some(MgOperand::MgOpRegister(r2)), Some(MgOperand::MgOpImmediate(_))) => assert_ne!(r1, r2),
        _ => panic!(),
    }

    assert_eq!(inst1.get_mnemonicid(), Some(MgMnemonic::MgMneBltzalc));
    assert_eq!(get_mnemonic(inst1.get_mnemonicid().unwrap()), MG_MNE_BLTZALC);
    assert_eq!(inst2.get_mnemonicid(), Some(MgMnemonic::MgMneBgtzalc));
    assert_eq!(get_mnemonic(inst2.get_mnemonicid().unwrap()), MG_MNE_BGTZALC);

    assert_eq!(inst1.get_operand_num(), 2);
    match (inst1.get_operand(0), inst1.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }

    assert_eq!(inst2.get_operand_num(), 2);
    match (inst2.get_operand(0), inst2.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }

    assert_eq!(inst3.get_mnemonicid(), Some(MgMnemonic::MgMneBgtz));
    assert_eq!(get_mnemonic(inst3.get_mnemonicid().unwrap()), MG_MNE_BGTZ);
    assert_eq!(inst3.get_operand_num(), 2);
    match (inst3.get_operand(0), inst3.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }

    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);
    let inst0 = decoder.disassemble(machine_code[0], 0);
    let inst1 = decoder.disassemble(machine_code[1], 0);
    let inst2 = decoder.disassemble(machine_code[2], 0);
    inst3 = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(inst0.is_err(), true);   //Field value wrong
    assert_eq!(inst1.is_err(), true);   //Field value wrong
    assert_eq!(inst2.is_err(), true);   //Field value wrong

    assert_eq!(inst3.get_mnemonicid(), Some(MgMnemonic::MgMneBgtz));
    assert_eq!(get_mnemonic(inst3.get_mnemonicid().unwrap()), MG_MNE_BGTZ);
    assert_eq!(inst3.get_operand_num(), 2);
    match (inst3.get_operand(0), inst3.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }
}
#[test]
fn test_blez_pop06(){
    let machine_code: [u32; 4] = [0x1830FFFF, 0x180a0050, 0x18420050, 0x1800C011];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[1], 0).unwrap();
    let inst2 = decoder.disassemble(machine_code[2], 0).unwrap();
    let mut inst3 = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(inst0.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(inst1.is_region(), false);
    assert_eq!(inst1.is_relative(), true);
    assert_eq!(inst1.is_conditional(), true);

    assert_eq!(inst0.get_mnemonicid(), Some(MgMnemonic::MgMneBgeuc));
    assert_eq!(get_mnemonic(inst0.get_mnemonicid().unwrap()), MG_MNE_BGEUC);
    assert_eq!(inst0.get_operand_num(), 3);
    match (inst0.get_operand(0), inst0.get_operand(1), inst0.get_operand(2)){
        (Some(MgOperand::MgOpRegister(r1)),Some(MgOperand::MgOpRegister(r2)), Some(MgOperand::MgOpImmediate(_))) => assert_ne!(r1, r2),
        _ => panic!(),
    }

    assert_eq!(inst1.get_mnemonicid(), Some(MgMnemonic::MgMneBlezalc));
    assert_eq!(get_mnemonic(inst1.get_mnemonicid().unwrap()), MG_MNE_BLEZALC);
    assert_eq!(inst2.get_mnemonicid(), Some(MgMnemonic::MgMneBgezalc));
    assert_eq!(get_mnemonic(inst2.get_mnemonicid().unwrap()), MG_MNE_BGEZALC);

    assert_eq!(inst1.get_operand_num(), 2);
    match (inst1.get_operand(0), inst1.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }

    assert_eq!(inst2.get_operand_num(), 2);
    match (inst2.get_operand(0), inst2.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }

    assert_eq!(inst3.get_mnemonicid(), Some(MgMnemonic::MgMneBlez));
    assert_eq!(get_mnemonic(inst3.get_mnemonicid().unwrap()), MG_MNE_BLEZ);
    assert_eq!(inst3.get_operand_num(), 2);
    match (inst3.get_operand(0), inst3.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }

    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);
    let inst0 = decoder.disassemble(machine_code[0], 0);
    let inst1 = decoder.disassemble(machine_code[1], 0);
    let inst2 = decoder.disassemble(machine_code[2], 0);
    inst3 = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(inst0.is_err(), true);   //Field value wrong
    assert_eq!(inst1.is_err(), true);   //Field value wrong
    assert_eq!(inst2.is_err(), true);   //Field value wrong

    assert_eq!(inst3.get_mnemonicid(), Some(MgMnemonic::MgMneBlez));
    assert_eq!(get_mnemonic(inst3.get_mnemonicid().unwrap()), MG_MNE_BLEZ);
    assert_eq!(inst3.get_operand_num(), 2);
    match (inst3.get_operand(0), inst3.get_operand(1)){
        (Some(MgOperand::MgOpRegister(_)), Some(MgOperand::MgOpImmediate(_))) => (),
        _ => panic!(),
    }
}

#[test]
fn test_lwr_swr_lwl_swl() {
    let machine_code: [u32; 4] = [0x88450050, 0xA8450050, 0x98450050, 0xB8450050];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    
    assert_eq!(decoder.disassemble(machine_code[0], 0).unwrap().get_mnemonicid(), Some(MgMnemonic::MgMneLwl));
    assert_eq!(decoder.disassemble(machine_code[1], 0).unwrap().get_mnemonicid(), Some(MgMnemonic::MgMneSwl));
    assert_eq!(decoder.disassemble(machine_code[2], 0).unwrap().get_mnemonicid(), Some(MgMnemonic::MgMneLwr));
    assert_eq!(decoder.disassemble(machine_code[3], 0).unwrap().get_mnemonicid(), Some(MgMnemonic::MgMneSwr));
    
    decoder.version = MgMipsVersion::M32(MgMips32::MgR6);
    assert_eq!(decoder.disassemble(machine_code[0], 0).is_err(), true);
    assert_eq!(decoder.disassemble(machine_code[1], 0).is_err(), true);
    assert_eq!(decoder.disassemble(machine_code[2], 0).is_err(), true);
    assert_eq!(decoder.disassemble(machine_code[3], 0).is_err(), true);
}

#[test]
fn test_bovc_bnvc(){
    let machine_code: [u32; 2] = [0x209B0030, 0x609B0030];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(inst0.get_mnemonicid(), Some(MgMnemonic::MgMneBovc));
    assert_eq!(get_mnemonic(inst0.get_mnemonicid().unwrap()), MG_MNE_BOVC);
    assert_eq!(inst0.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(inst0.get_format(), MgInstructionFormat::Imm);
    assert_eq!(inst0.is_conditional(), true);
    assert_eq!(inst0.is_relative(), true);
    assert_eq!(inst0.is_region(), false);
    assert_eq!(inst0.get_operand_num(), 3);
    match inst0.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match inst0.get_operand(1){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match inst0.get_operand(2){
        Some(MgOperand::MgOpImmediate(_)) => (),
        _ => panic!(),
    }

    assert_eq!(inst1.get_mnemonicid(), Some(MgMnemonic::MgMneBnvc));
    assert_eq!(inst1.get_format(), MgInstructionFormat::Imm);
    assert_eq!(inst1.is_conditional(), true);
    assert_eq!(inst1.is_relative(), true);
    assert_eq!(inst1.is_region(), false);
    assert_eq!(inst1.get_operand_num(), 3);
    assert_eq!(inst1.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(get_mnemonic(inst1.get_mnemonicid().unwrap()), MG_MNE_BNVC);
    match inst1.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match inst1.get_operand(1){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match inst1.get_operand(2){
        Some(MgOperand::MgOpImmediate(_)) => (),
        _ => panic!(),
    }

    //One is addi and the other is an error
    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);
    let inst0 = decoder.disassemble(machine_code[0], 0).unwrap();

    assert_ne!(inst0.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(decoder.disassemble(machine_code[1], 0).is_err(), true);
    assert_ne!(inst0.is_region(), true);
    assert_ne!(inst0.get_mnemonicid(), Some(MgMnemonic::MgMneAddi));
}

#[test]
fn test_jic_jialc(){
    let machine_code: [u32; 2] = [0xd8020050, 0xf8020050];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(inst0.get_mnemonicid(), Some(MgMnemonic::MgMneJic));
    assert_eq!(get_mnemonic(inst0.get_mnemonicid().unwrap()), MG_MNE_JIC);
    assert_eq!(inst0.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(inst0.is_conditional(), false);
    assert_eq!(inst0.is_region(), false);
    assert_eq!(inst0.get_operand_num(), 2);
    match inst0.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match inst0.get_operand(1){
        Some(MgOperand::MgOpImmediate(_)) => (),
        _ => panic!(),
    }

    assert_eq!(inst1.get_mnemonicid(), Some(MgMnemonic::MgMneJialc));
    assert_eq!(inst1.is_conditional(), false);
    assert_eq!(inst1.is_region(), false);
    assert_eq!(inst1.get_operand_num(), 2);
    assert_eq!(inst1.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(get_mnemonic(inst1.get_mnemonicid().unwrap()), MG_MNE_JIALC);
    match inst1.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match inst1.get_operand(1){
        Some(MgOperand::MgOpImmediate(_)) => (),
        _ => panic!(),
    }

    //Load and store instructions from cop2
    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);
    let inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_ne!(inst0.get_category(), MgInstructionCategory::BranchJump);
    assert_ne!(inst1.get_category(), MgInstructionCategory::BranchJump);
    assert_ne!(inst0.is_conditional(), true);
    assert_ne!(inst0.is_region(), true);
    assert_eq!(inst0.get_mnemonicid(), Some(MgMnemonic::MgMneLdc2));
    assert_eq!(inst1.get_mnemonicid(), Some(MgMnemonic::MgMneSdc2));
}
#[test]
fn test_bc_balc(){
    let machine_code: [u32; 2] = [0xC8020050, 0xE8020050];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));

    let inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(inst0.get_mnemonicid(), Some(MgMnemonic::MgMneBc));
    assert_eq!(get_mnemonic(inst0.get_mnemonicid().unwrap()), MG_MNE_BC);
    assert_eq!(inst0.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(inst0.is_conditional(), true);
    assert_ne!(inst0.is_region(), true);
    assert_eq!(inst0.get_operand_num(), 1);

    assert_eq!(inst1.get_mnemonicid(), Some(MgMnemonic::MgMneBalc));
    assert_eq!(inst1.is_conditional(), true);
    assert_ne!(inst1.is_region(), true);
    assert_eq!(inst1.get_operand_num(), 1);
    assert_eq!(inst1.get_category(), MgInstructionCategory::BranchJump);
    assert_eq!(get_mnemonic(inst1.get_mnemonicid().unwrap()), MG_MNE_BALC);

    //Load and store instructions from cop2
    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);
    let inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_ne!(inst0.get_category(), MgInstructionCategory::BranchJump);
    assert_ne!(inst1.get_category(), MgInstructionCategory::BranchJump);
    assert_ne!(inst0.is_conditional(), true);
    assert_ne!(inst0.is_region(), true);
    assert_eq!(inst0.get_mnemonicid(), Some(MgMnemonic::MgMneLwc2));
    assert_eq!(inst1.get_mnemonicid(), Some(MgMnemonic::MgMneSwc2));
}
#[test]
fn test_sc_ll(){
    let machine_code: [u32; 4] = [0xE0A2FFFF, 0xC0A2FFFF, 0x7fffffa6, 0x7fffffb6];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[1], 0).unwrap();

    assert_eq!(decoder.disassemble(machine_code[2], 0).is_err(), true);
    assert_eq!(decoder.disassemble(machine_code[3], 0).is_err(), true);

    assert_eq!(inst0.is_conditional(), true);
    assert_eq!(inst0.get_mnemonicid().unwrap(), MgMnemonic::MgMneSc);
    assert_eq!(get_mnemonic(inst0.get_mnemonicid().unwrap()), MG_MNE_SC);
    assert_eq!(inst0.get_operand_num(), 3);

    assert_eq!(inst1.is_conditional(), true);
    assert_eq!(inst1.get_mnemonicid().unwrap(), MgMnemonic::MgMneLl);
    assert_eq!(get_mnemonic(inst1.get_mnemonicid().unwrap()), MG_MNE_LL);
    assert_eq!(inst1.get_operand_num(), 3);
    match inst1.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match inst1.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match inst1.get_operand(1){
        Some(MgOperand::MgOpImmediate(i)) => assert!(i.get_value() <= 0xffff),
        _ => panic!(),
    }

    decoder.version = MgMipsVersion::M32(MgMips32::MgR6);

    let inst0 = decoder.disassemble(machine_code[2], 0).unwrap();
    let inst1 = decoder.disassemble(machine_code[3], 0).unwrap();

    assert_eq!(decoder.disassemble(machine_code[0], 0).is_err(), true);
    assert_eq!(decoder.disassemble(machine_code[1], 0).is_err(), true);

    assert_eq!(inst0.is_conditional(), true);
    assert_eq!(inst0.get_mnemonicid().unwrap(), MgMnemonic::MgMneSc);
    assert_eq!(get_mnemonic(inst0.get_mnemonicid().unwrap()), MG_MNE_SC);
    assert_eq!(inst0.get_operand_num(), 3);

    assert_eq!(inst1.is_conditional(), true);
    assert_eq!(inst1.get_mnemonicid().unwrap(), MgMnemonic::MgMneLl);
    assert_eq!(get_mnemonic(inst1.get_mnemonicid().unwrap()), MG_MNE_LL);
    assert_eq!(inst1.get_operand_num(), 3);
    match inst1.get_operand(0){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match inst1.get_operand(2){
        Some(MgOperand::MgOpRegister(_)) => (),
        _ => panic!(),
    }
    match inst1.get_operand(1){
        Some(MgOperand::MgOpImmediate(i)) => assert!(i.get_value() <= 0x1ff),
        _ => panic!(),
    }
}
#[test]
fn test_load_store_cp2(){
    let machine_code: [u32; 8] = [0xC8020050, 0xE8020050, 0xD8020050, 0xF8020050, 0x49C00000,0x49400000, 0x49E00000,0x49600000];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let mut inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    let mut inst1 = decoder.disassemble(machine_code[1], 0).unwrap();
    let mut inst2 = decoder.disassemble(machine_code[2], 0).unwrap();
    let mut inst3 = decoder.disassemble(machine_code[3], 0).unwrap();

    let inst4 = decoder.disassemble(machine_code[4], 0);
    let inst5 = decoder.disassemble(machine_code[5], 0);
    let inst6 = decoder.disassemble(machine_code[6], 0);
    let inst7 = decoder.disassemble(machine_code[7], 0);

    assert_eq!(inst0.get_mnemonicid(), Some(MgMnemonic::MgMneLwc2));
    assert_eq!(inst1.get_mnemonicid(), Some(MgMnemonic::MgMneSwc2));
    assert_eq!(inst2.get_mnemonicid(), Some(MgMnemonic::MgMneLdc2));
    assert_eq!(inst3.get_mnemonicid(), Some(MgMnemonic::MgMneSdc2));

    //Will fail
    assert_eq!(inst4.is_err(), true);    //Ldc2
    assert_eq!(inst5.is_err(), true);    //Lwc2
    assert_eq!(inst6.is_err(), true);    //Sdc2
    assert_eq!(inst7.is_err(), true);    //Swc2

    //The same machine code is used by other instructions in release6
    decoder.version = MgMipsVersion::M32(MgMips32::MgR6);
    inst0 = decoder.disassemble(machine_code[0], 0).unwrap();
    inst1 = decoder.disassemble(machine_code[1], 0).unwrap();
    inst2 = decoder.disassemble(machine_code[2], 0).unwrap();
    inst3 = decoder.disassemble(machine_code[3], 0).unwrap();

    let inst4 = decoder.disassemble(machine_code[4], 0).unwrap();
    let inst5 = decoder.disassemble(machine_code[5], 0).unwrap();
    let inst6 = decoder.disassemble(machine_code[6], 0).unwrap();
    let inst7 = decoder.disassemble(machine_code[7], 0).unwrap();

    assert_eq!(inst0.get_mnemonicid(), Some(MgMnemonic::MgMneBc));
    assert_eq!(inst1.get_mnemonicid(), Some(MgMnemonic::MgMneBalc));
    assert_eq!(inst2.get_mnemonicid(), Some(MgMnemonic::MgMneJic));
    assert_eq!(inst3.get_mnemonicid(), Some(MgMnemonic::MgMneJialc));

    assert_eq!(inst4.get_mnemonicid(), Some(MgMnemonic::MgMneLdc2));
    assert_eq!(inst5.get_mnemonicid(), Some(MgMnemonic::MgMneLwc2));
    assert_eq!(inst6.get_mnemonicid(), Some(MgMnemonic::MgMneSdc2));
    assert_eq!(inst7.get_mnemonicid(), Some(MgMnemonic::MgMneSwc2));
}

#[test]
fn test_tne_teq() {
    let machine_code: [u32; 2] = [0x00460036, 0x00400034];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    
    //No problem
    assert_eq!(decoder.disassemble(machine_code[0], 0).unwrap().get_mnemonicid(), Some(MgMnemonic::MgMneTne));
    assert_eq!(decoder.disassemble(machine_code[1], 0).unwrap().get_mnemonicid(), Some(MgMnemonic::MgMneTeq));

    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);
    assert_eq!(decoder.disassemble(machine_code[0], 0).unwrap().get_mnemonicid(), Some(MgMnemonic::MgMneTne));
    assert_eq!(decoder.disassemble(machine_code[1], 0).unwrap().get_mnemonicid(), Some(MgMnemonic::MgMneTeq));
}

#[test]
fn test_seleqz_selnez() {
    let machine_code: [u32; 4] = [0b110101, 0b110111, 0b1110101,0b10110101];
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    
    //No problem
    assert_eq!(decoder.disassemble(machine_code[0], 0).unwrap().get_mnemonicid(), Some(MgMnemonic::MgMneSeleqz));
    assert_eq!(decoder.disassemble(machine_code[1], 0).unwrap().get_mnemonicid(), Some(MgMnemonic::MgMneSelnez));
    assert_eq!(decoder.disassemble(machine_code[0], 0).unwrap().is_conditional(), true);
    assert_eq!(decoder.disassemble(machine_code[1], 0).unwrap().is_conditional(), true);
    
    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);
    assert_eq!(decoder.disassemble(machine_code[0], 0).is_err(), true);
    assert_eq!(decoder.disassemble(machine_code[1], 0).is_err(), true);

    //Testing if Sa field not set to 0 will fail
    assert_eq!(decoder.disassemble(machine_code[2], 0).is_err(), true);
    assert_eq!(decoder.disassemble(machine_code[3], 0).is_err(), true);
}