//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

#![no_std]
pub mod instruction;
pub mod disassembler;
pub mod operands;
pub mod error;
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