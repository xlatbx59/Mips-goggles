//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

use mips_goggles::{*, disassembler::MgDisassembler};
 
fn main(){
    let code = [ 0x90ffbd27, 0x6c00bfaf, 0x6800beaf, 0x21f0a003,
                            0x42001c3c, 0x308d9c27, 0x1000bcaf, 0x4000023c,
                            0xc40c4424, 0x2c80998f, 0x00000000, 0x09f82003,
                            0x00000000, 0x1000dc8f, 0x4000023c, 0xe00c4424,
                            0x2c80998f, 0x00000000, 0x09f82003 ];
    let decoder = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    for machine_code in code{
        println!("{:?}", decoder.disassemble(u32::from_be(machine_code), 0x00400000).unwrap());
    }
}