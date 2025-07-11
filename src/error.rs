//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles
use core::fmt;

pub enum MgErrorCode{
    FieldBadValue = 0x80000000, NoInstruction = 0x80000001, DevError = 0x80000002,
    VersionError = 0x80000003,
}

pub struct MgError{
    address: u64,
    machine_code: u32,
    opcode: u8,
    error_code: MgErrorCode,
}

impl MgError{
    pub fn throw_error(error_code: MgErrorCode, opcode: u8, address: u64, machine_code: u32) -> MgError{
        MgError{error_code, address, opcode, machine_code}
    }
}

impl fmt::Display for MgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.error_code{
            MgErrorCode::FieldBadValue => write!(f, "[-]The field of this instruction has a bad value.\r\n\topcode: {:02x}\r\n\taddress: 0x{:08x}\r\n\tmachine code: 0x{:08x}", self.opcode, self.address, self.machine_code),
            MgErrorCode::NoInstruction =>write!(f, "[-]This machine code isn't an instruction.\r\n\topcode: {:02x}\r\n\taddress: 0x{:08x}\r\n\tmachine code: 0x{:08x}", self.opcode, self.address, self.machine_code),
            MgErrorCode::DevError =>write!(f, "[-]I did something wrong again.\r\n\topcode: {:02x}\r\n\taddress: {}\r\n\tmachine code: {}", self.opcode, self.address, self.machine_code),
            MgErrorCode::VersionError =>write!(f, "[-]Instruction was either moved, removed or never appeared in this release.\r\n\topcode: 0x{:02x}\r\n\taddress: 0x{:02x}\r\n\tmachine code: 0x{:02x}", self.opcode, self.address, self.machine_code),
        }
    }
}