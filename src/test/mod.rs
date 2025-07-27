//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles
#![cfg(test)]

mod test_pop;
mod test_trap;
mod test_special;
mod test_load_store;
mod test_arithmetic;
mod test_branch_jump;

use crate::*;
use crate::operands::*;
use crate::instruction::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

///The purpose of this function is to check if the field that are supposed to be fixed are checked(these ones are supposed to not be 0)
fn check_field_zero(decoder: &MgDisassembler, code: u32, mask: u32, mne: MgMnemonic, pos: u8) -> bool{
    match  decoder.disassemble(code &! (mask << pos), 0){
        Ok(i) => if i.get_mnemonic() != mne{return true},
        Err(_) => return true,
    }
    false
}
///The purpose of this function is to check if the field that are supposed to be fixed are checked(these ones are supposed to not be 0)
fn check_field_zero_assert(decoder: &MgDisassembler, code: u32, mask: u32, mne: MgMnemonic, pos: u8) -> bool{
    assert_eq!(decoder.disassemble(code, 0).unwrap().get_mnemonic(), mne);
    check_field_zero(decoder, code, mask, mne, pos)
}
///The purpose of this function is to check if the field that are supposed to be fixed are checked
fn check_field(decoder: &MgDisassembler, code: u32, mask: u32, mne: MgMnemonic, pos: u8) -> bool{
    assert_eq!(decoder.disassemble(code, 0).unwrap().get_mnemonic(), mne);
    match  decoder.disassemble(code | (mask << pos), 0){
        Ok(i) => if i.get_mnemonic() != mne{return true},
        Err(_) => return true,
    }
    false
}
///The purpose of this function is to check if the number of operands is correct and if they're in a contiguous way inside of the array
fn check_operands(inst: &MgInstruction, op_requiered_num: u8) -> bool{
    let op_num = inst.get_operand_num();
    if op_num as u8 != op_requiered_num{
        return false
    };
    for index in 0..inst.get_operand_num(){
        if inst.get_operand(index).is_none(){return false}
    }
    true
}
///The purpose of this function is to test when the version is correct
fn version_test(code: u32, mne: MgMnemonic, pre6_32: bool, r6_32: bool, pre6_64: bool, r6_64: bool) -> bool{
    let mut decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    match decoder.disassemble(code, 0){
        Ok(i) => if i.get_mnemonic() != mne && r6_32 || i.get_mnemonic() == mne && !r6_32{return false},
        Err(_) => if r6_32{return false},
    };
    decoder.version = MgMipsVersion::M32(MgMips32::MgPreR6);
    match decoder.disassemble(code, 0){
        Ok(i) => if i.get_mnemonic() != mne && pre6_32 || i.get_mnemonic() == mne && !pre6_32{return false},
        Err(_) => if pre6_32{return false},
    };
    decoder.version = MgMipsVersion::M64(MgMips64::MgPreR6);
    match decoder.disassemble(code, 0){
        Ok(i) => if i.get_mnemonic() != mne && pre6_64 || i.get_mnemonic() == mne && !pre6_64{return false},
        Err(_) => if pre6_64{return false},
    };
    decoder.version = MgMipsVersion::M64(MgMips64::MgR6);
    match decoder.disassemble(code, 0){
        Ok(i) =>if i.get_mnemonic() != mne && r6_64 || i.get_mnemonic() == mne && !r6_64{return false},
        Err(_) => if r6_64{return false},
    };
    true
}
///The purpose of this function is to test if we take correctly the immediate
fn imm_limit_reached(disass: &MgDisassembler, mne: MgMnemonic, mut machine_code: u32, bit_pos: u8, mask: u32, operand_index: usize) -> bool{
    //The immediate doesn't take enough bits to make the immediate
    machine_code |= (mask << bit_pos) as u32;
    match disass.disassemble(machine_code, 0x00400000){
        Ok(inst) => {
            assert_eq!(mne, inst.get_mnemonic());
            let Some(MgOperand::MgOpImmediate(imm)) = inst.get_operand(operand_index) else{ //Also check that the operand is immdiate
                return false
            };
            if imm.get_value() as u32 != mask{
                return false
            };
        },
        Err(_) =>(),
    };

    //The immediate takes too much bits to make the immediate
    machine_code += (1 << bit_pos) as u32;
    return match disass.disassemble(machine_code, 0x00400000){
        Ok(inst) => {
            if inst.get_mnemonic() != mne{
                return true
            }
            let Some(MgOperand::MgOpImmediate(imm)) = inst.get_operand(operand_index) else{ //Also check that the operand is immdiate
                return true
            };
            if imm.get_value() != 0{
                return false
            };
            true
        },
        Err(_) =>true,
    }
}
#[test]
fn test_daui(){
    let machine_code = [0x770933f1, 0x740933f1];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M64(MgMips64::MgR6));

    assert_eq!(true, version_test(machine_code[0], MgMnemonic::MgMneDaui, false, false, false, true));

    let daui = decoder.disassemble(machine_code[0], 0).unwrap();
    assert_eq!(MgMnemonic::MgMneDaui, daui.get_mnemonic());
    assert_eq!(MG_MNE_DAUI, daui.get_mnemonic_str());

    assert_eq!(3, daui.get_operand_num());
    assert_eq!(MgInstructionCategory::LargeConstant, daui.get_category());

    let Some(MgOperand::MgOpRegister(_)) = daui.get_operand(0) else {
        panic!();
    };
    let Some(MgOperand::MgOpRegister(_)) = daui.get_operand(1) else {
        panic!();
    };
    assert_eq!(true, imm_limit_reached(&decoder,MgMnemonic::MgMneDaui, machine_code[0], 0, 0xffff, 2));
}