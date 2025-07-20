//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

#![no_std]
///Self explanatory
pub mod instruction;
///Where all the magic happens but self explanatory ;)
pub mod disassembler;
///Self explanatory
pub mod operands;
///Self explanatory
pub mod error;
mod test;
mod utils;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MgMipsVersion{
    M32(MgMips32), M64(MgMips64)
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MgMips64{
    MgPreR6,MgR6
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MgMips32{
    MgPreR6,MgR6
}