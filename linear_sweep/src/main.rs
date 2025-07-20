use mips_goggles::{*, disassembler::MgDisassembler};
use mips_goggles::instruction::mnemonics::MG_MNE_NOP;
use mips_goggles::instruction::mnemonics::*;
 
fn main(){
    let decoder = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    let instruction = decoder.disassemble(0x00000000, 0x00400000).unwrap();     //nop
    assert_eq!(get_mnemonic(instruction.get_mnemonic()), MG_MNE_NOP);
}