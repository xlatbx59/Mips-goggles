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
mod test_moves;
mod test_logic_operations;
mod test_bit_manipulation;

use crate::*;
use crate::operands::*;
use crate::instruction::*;
use crate::disassembler::*;
use crate::instruction::mnemonics::*;

///The purpose of this function is to check if the field that are supposed to be fixed are checked(these ones are supposed to not be 0)
fn check_field_zero(decoder: &MgDisassembler, code: u32, mask: u32, mne: MgMnemonic, pos: u8) -> bool{
    match  decoder.disassemble(code & !(mask << pos), 0){
        Ok(i) => if i.get_mnemonic() != mne{return true},
        Err(_) => return true,
    }
    false
}
///The purpose of this function is to check if the field that are supposed to be fixed are checked(these ones are supposed to not be 0)
fn check_field_zero_assert(decoder: &MgDisassembler, code: u32, mask: u32, mne: MgMnemonic, pos: u8) -> bool{
    assert!(decoder.disassemble(code, 0).unwrap().get_mnemonic() == mne);
    check_field_zero(decoder, code, mask, mne, pos)
}
///The purpose of this function is to check if the field that are supposed to be fixed are checked
fn check_field(decoder: &MgDisassembler, code: u32, mask: u32, mne: MgMnemonic, pos: u8) -> bool{
    assert!(decoder.disassemble(code, 0).unwrap().get_mnemonic() == mne);
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
    machine_code |= mask << bit_pos;
    if let Ok(inst) = disass.disassemble(machine_code, 0x00400000){
        assert!(mne == inst.get_mnemonic());
        let Some(MgOperand::MgOpImmediate(imm)) = inst.get_operand(operand_index) else{ //Also check that the operand is immdiate
            return false
        };
        if imm.get_value() as u32 != mask{
            return false
        };
    }

    //The immediate takes too much bits to make the immediate
    machine_code += (1 << bit_pos) as u32;
    if let Ok(inst) = disass.disassemble(machine_code, 0x00400000) {
        if inst.get_mnemonic() != mne{
            true
        }else if let Some(MgOperand::MgOpImmediate(imm)) = inst.get_operand(operand_index) { //Also check that the operand is immdiate
            imm.get_value() == 0
        }else{
            true
        }
    }else{
        true
    }
}

#[test]
fn test_di_ei(){
    let machine_code: [u32; 2] = [0x417C6000, 0x417C6020];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let di = decoder.disassemble(machine_code[0], 0).unwrap();
    let ei = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(di.get_mnemonic() == MgMnemonic::MgMneDi);
    assert!(di.get_mnemonic_str() == MG_MNE_DI);
    assert!("di" == MG_MNE_DI);
    assert!(ei.get_mnemonic() == MgMnemonic::MgMneEi);
    assert!(ei.get_mnemonic_str() == MG_MNE_EI);
    assert!("ei" == MG_MNE_EI);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneDi, true, true, true, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneEi, true, true, true, true));

    assert!(check_field(&decoder, machine_code[0], 7, MgMnemonic::MgMneDi, 0));
    assert!(check_field(&decoder, machine_code[0], 3, MgMnemonic::MgMneDi, 3));
    assert!(check_field(&decoder, machine_code[0], 1, MgMnemonic::MgMneDi, 5));
    assert!(check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneDi, 6));

    assert!(check_field(&decoder, machine_code[1], 7, MgMnemonic::MgMneEi, 0));
    assert!(check_field(&decoder, machine_code[1], 3, MgMnemonic::MgMneEi, 3));
    assert!(check_field_zero_assert(&decoder, machine_code[1], 1, MgMnemonic::MgMneEi, 5));
    assert!(check_field(&decoder, machine_code[1], 0x1f, MgMnemonic::MgMneEi, 6));

    assert!(check_operands(&di, 1));
    assert!(check_operands(&ei, 1));
}
#[test]
fn test_dvp_evp(){
    let machine_code: [u32; 2] = [0x417C0024, 0x417C0004];
    let decoder: MgDisassembler = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgR6));
    let dvp = decoder.disassemble(machine_code[0], 0).unwrap();
    let evp = decoder.disassemble(machine_code[1], 0).unwrap();

    assert!(dvp.get_mnemonic() == MgMnemonic::MgMneDvp);
    assert!(dvp.get_mnemonic_str() == MG_MNE_DVP);
    assert!("dvp" == MG_MNE_DVP);
    assert!(evp.get_mnemonic() == MgMnemonic::MgMneEvp);
    assert!(evp.get_mnemonic_str() == MG_MNE_EVP);
    assert!("evp" == MG_MNE_EVP);

    assert!(version_test(machine_code[0], MgMnemonic::MgMneDvp, false, true, false, true));
    assert!(version_test(machine_code[1], MgMnemonic::MgMneEvp, false, true, false, true));

    assert!(check_field(&decoder, machine_code[0], 0b011, MgMnemonic::MgMneDvp, 0));
    assert!(check_field(&decoder, machine_code[0], 3, MgMnemonic::MgMneDvp, 3));
    assert!(check_field_zero_assert(&decoder, machine_code[0], 1, MgMnemonic::MgMneDvp, 5));
    assert!(check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneDvp, 6));
    assert!(check_field(&decoder, machine_code[0], 0x1f, MgMnemonic::MgMneDvp, 11));

    assert!(check_field(&decoder, machine_code[1], 0b011, MgMnemonic::MgMneEvp, 0));
    assert!(check_field(&decoder, machine_code[1], 3, MgMnemonic::MgMneEvp, 3));
    assert!(check_field(&decoder, machine_code[1], 1, MgMnemonic::MgMneEvp, 5));
    assert!(check_field(&decoder, machine_code[1], 0x1f, MgMnemonic::MgMneEvp, 6));
    assert!(check_field(&decoder, machine_code[1], 0x1f, MgMnemonic::MgMneEvp, 11));

    assert!(check_operands(&dvp, 1));
    assert!(check_operands(&evp, 1));
}
