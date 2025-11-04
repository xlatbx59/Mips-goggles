![](/img/go-goggles.png)
# mips-goggles ![](img/Miniature_Lunettes_Sable_ROSA.png)

|             Brice              |            code             |
| :----------------------------: | :-------------------------: |
| ![](/img/brice-no-goggles.png) |      ![](/img/hex.png)      |
|  ![](/img/brice-goggles.png)   | ![](/img/mips-assembly.png) |

Mips-Goggles is a mips disassembler for the mips instruction set, it is not finished yet but it implement most core instructions except from coprocessor 1, 2 and 3(aka cop1x).

> [!Note]
> You can't turn instructions into strings it has been removed, it's still in the first commit if you want to try, but I'll reimplement it somedays

# Examples
## [Basic linear sweep](examples\linear_sweep\src\main.rs)
```rust
use mips_goggles::{*, disassembler::MgDisassembler};
 
fn main(){
    //Mips machine has to be in big endian
    //This machine code was ripped from a mips crack me from Root-me
    let code = [    0x90ffbd27, 0x6c00bfaf, 0x6800beaf, 0x21f0a003,
                    0x42001c3c, 0x308d9c27, 0x1000bcaf, 0x4000023c,
                    0xc40c4424, 0x2c80998f, 0x00000000, 0x09f82003,
                    0x00000000, 0x1000dc8f, 0x4000023c, 0xe00c4424,
                    0x2c80998f, 0x00000000, 0x09f82003 ];
    let decoder = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
    for machine_code in code{
        println!("{:?}", decoder.disassemble(u32::from_be(machine_code), 0x00400000).unwrap());
    }
}
```

# Completed

- [Opcode map](https://www.cipunited.com/xlx/files/document/202008/1205490289250.pdf#G320.1122743)
- [Special opcode map](https://www.cipunited.com/xlx/files/document/202008/1205490289250.pdf#G320.1123094)
- [Regimm opcode map](https://www.cipunited.com/xlx/files/document/202008/1205490289250.pdf#G320.1096304)
- [Special 2 opcode map](https://www.cipunited.com/xlx/files/document/202008/1205490289250.pdf#G320.1096304)
- [Special 3 opcode map](https://www.cipunited.com/xlx/files/document/202008/1205490289250.pdf#G320.1096549)
- [Pcrel](https://www.cipunited.com/xlx/files/document/202008/1205490289250.pdf#G320.1445147)
- [Cop 0](https://www.cipunited.com/xlx/files/document/202008/1205490289250.pdf#G320.1097182)

# Goals

- [x] Cross platform
- [x] No dependency
- [x] No memory allocation
- [x] Thread safe(there's only one thread)
- [x] No unsafe
- [ ] Assembler???
- [ ] More Mips versions
- [ ] Exceptions
- [ ] Mips Extensions

# Inspiration

- [Zydis](https://github.com/zyantific/zydis)

# Resources

- [Online assembler/disassembler](https://yozan233.github.io/Online-Assembler-Disassembler/) which is basically [Keystone](https://github.com/keystone-engine/keystone)
- MIPS64 Manuals Release 6: [Volume 1](https://www.cipunited.com/xlx/files/document/202008/1205481629410.pdf), [Volume 2](https://s3-eu-west-1.amazonaws.com/downloads-mips/documents/MD00086-2B-MIPS32BIS-AFP-6.06.pdf), [Volume 3](https://s3-eu-west-1.amazonaws.com/downloads-mips/documents/MD00090-2B-MIPS32PRA-AFP-06.02.pdf)