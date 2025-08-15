//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/mips-goggles

/// Gets the mnemonic string when passing it an enum like MgMnemonic::MgMneNop
/// # Example
/// ```rust
/// use mips_goggles::{*, disassembler::MgDisassembler, instruction::mnemonics::*};
/// 
/// let mut decoder = MgDisassembler::new_disassembler(MgMipsVersion::M32(MgMips32::MgPreR6));
/// let instruction = decoder.disassemble(0x00000000, 0x00400000).unwrap();     //nop
/// assert_eq!(get_mnemonic(instruction.get_mnemonic()), MG_MNE_NOP);
/// assert_eq!(get_mnemonic(MgMnemonic::MgMneAddi), MG_MNE_ADDI);
/// ```
pub fn mg_get_mnemonic(mnemonic: MgMnemonic) -> &'static str{
    MG_MNEMONICS[mnemonic as usize]
}

pub const MG_MNE_J: &str = "j"; pub const MG_MNE_JAL: &str = "jal";
pub const MG_MNE_BEQ: &str = "beq"; pub const MG_MNE_BNE: &str = "bne"; pub const MG_MNE_BLEZ: &str = "blez";
pub const MG_MNE_BGTZ: &str = "bgtz"; pub const MG_MNE_ADDI: &str = "addi"; pub const MG_MNE_ADDIU: &str = "addiu";
pub const MG_MNE_SLTI: &str = "slti"; pub const MG_MNE_SLTIU: &str = "sltiu"; pub const MG_MNE_ANDI: &str = "andi";
pub const MG_MNE_ORI: &str = "ori"; pub const MG_MNE_XORI: &str = "xori"; pub const MG_MNE_LUI: &str = "lui";
pub const MG_MNE_BEQL: &str = "beql"; pub const MG_MNE_BNEL: &str = "bnel"; pub const MG_MNE_BLEZL: &str = "blezl";
pub const MG_MNE_BGTZL: &str = "bgtzl"; pub const MG_MNE_JALX: &str = "jalx"; pub const MG_MNE_LB: &str = "lb";
pub const MG_MNE_LH: &str = "lh"; pub const MG_MNE_LWL: &str = "lwl"; pub const MG_MNE_LW: &str = "lw";
pub const MG_MNE_LBU: &str = "lbu"; pub const MG_MNE_LHU: &str = "lhu"; pub const MG_MNE_LWR: &str = "lwr";
pub const MG_MNE_SB: &str = "sb"; pub const MG_MNE_SH: &str = "sh"; pub const MG_MNE_SWL: &str = "swl";
pub const MG_MNE_SW: &str = "sw"; pub const MG_MNE_SWR: &str = "swr"; pub const MG_MNE_CACHE: &str = "cache";
pub const MG_MNE_LL: &str = "ll"; pub const MG_MNE_LWC1: &str = "lwc1"; pub const MG_MNE_LWC2: &str = "lwc2";
pub const MG_MNE_PREF: &str = "pref"; pub const MG_MNE_LDC1: &str = "ldc1"; pub const MG_MNE_LDC2: &str = "ldc2";
pub const MG_MNE_SC: &str = "sc"; pub const MG_MNE_SWC1: &str = "swc1"; pub const MG_MNE_SWC2: &str = "swc2";
pub const MG_MNE_SDC1: &str = "sdc1"; pub const MG_MNE_SDC2: &str = "sdc2"; pub const MG_MNE_BALC: &str = "balc";
pub const MG_MNE_BC: &str = "bc"; pub const MG_MNE_JIC: &str = "jic"; pub const MG_MNE_JIALC: &str = "jialc";
pub const MG_MNE_BOVC: &str = "bovc"; pub const MG_MNE_BNVC: &str = "bnvc"; pub const MG_MNE_BGEUC: &str = "bgeuc";
pub const MG_MNE_BLEZALC: &str = "blezalc"; pub const MG_MNE_BGEZALC: &str = "bgezalc"; pub const MG_MNE_BGTZALC: &str = "bgtzalc";
pub const MG_MNE_BLTZALC: &str = "bltzalc"; pub const MG_MNE_BLTUC: &str = "bltuc";pub const MG_MNE_BLEZC: &str = "blezc";
pub const MG_MNE_BGEZC: &str = "bgezc"; pub const MG_MNE_BGEC: &str = "bgec";pub const MG_MNE_BGTZC: &str = "bgtzc";
pub const MG_MNE_BLTZC: &str = "bltzc"; pub const MG_MNE_BLTC: &str = "bltc";pub const MG_MNE_BEQC: &str = "beqc";
pub const MG_MNE_BEQZALC: &str = "beqzalc";  pub const MG_MNE_BNEZALC: &str = "bnezalc"; pub const MG_MNE_BNEC: &str = "bnec";
pub const MG_MNE_BEQZC: &str = "beqzc"; pub const MG_MNE_BNEZC: &str = "bnezc";pub const MG_MNE_DAUI: &str = "daui";
pub const MG_MNE_LDR: &str = "ldr"; pub const MG_MNE_LDL: &str = "ldl"; pub const MG_MNE_DADDI: &str = "daddi";
pub const MG_MNE_DADDIU: &str = "daddiu";pub const MG_MNE_AUI: &str = "aui"; pub const MG_MNE_LD: &str = "ld";
pub const MG_MNE_SD: &str = "sd"; pub const MG_MNE_SDL: &str = "sdl"; pub const MG_MNE_SDR: &str = "sdr";
pub const MG_MNE_SCD: &str = "scd"; pub const MG_MNE_LLD: &str = "lld"; pub const MG_MNE_LWU: &str = "lwu";

//Special
pub const MG_MNE_SSNOP: &str = "ssnop"; pub const MG_MNE_EHB: &str = "ehb"; pub const MG_MNE_PAUSE: &str = "pause";
pub const MG_MNE_NOP: &str = "nop"; pub const MG_MNE_SLL: &str = "sll"; pub const MG_MNE_SRA: &str = "sra";
pub const MG_MNE_SLLV: &str = "sllv"; pub const MG_MNE_SRAV: &str = "srav"; pub const MG_MNE_JR: &str = "jr";
pub const MG_MNE_JRHB: &str = "jr.hb"; pub const MG_MNE_JALR: &str = "jalr"; pub const MG_MNE_JALRHB: &str = "jalr.hb";
pub const MG_MNE_MOVZ: &str = "movz"; pub const MG_MNE_MOVN: &str = "movn"; pub const MG_MNE_SYSCALL: &str = "syscall";
pub const MG_MNE_BREAK: &str = "break"; pub const MG_MNE_SYNC: &str = "sync"; pub const MG_MNE_MFHI: &str = "mfhi";
pub const MG_MNE_MTHI: &str = "mthi"; pub const MG_MNE_MFLO: &str = "mflo"; pub const MG_MNE_MTLO: &str = "mtlo";
pub const MG_MNE_MULT: &str = "mult"; pub const MG_MNE_MULTU: &str = "multu"; pub const MG_MNE_DIV: &str = "div";
pub const MG_MNE_DIVU: &str = "divu"; pub const MG_MNE_ADD: &str = "add"; pub const MG_MNE_ADDU: &str = "addu";
pub const MG_MNE_SUB: &str = "sub"; pub const MG_MNE_SUBU: &str = "subu"; pub const MG_MNE_AND: &str = "and";
pub const MG_MNE_OR: &str = "or"; pub const MG_MNE_XOR: &str = "xor"; pub const MG_MNE_NOR: &str = "nor";
pub const MG_MNE_SLT: &str = "slt"; pub const MG_MNE_SLTU: &str = "sltu"; pub const MG_MNE_TGE: &str = "tge";
pub const MG_MNE_TGEU: &str = "tgeu"; pub const MG_MNE_TLT: &str = "tlt"; pub const MG_MNE_TLTU: &str = "tltu";
pub const MG_MNE_TEQ: &str = "teq"; pub const MG_MNE_TNE: &str = "tne"; pub const MG_MNE_SRLV: &str = "srlv";
pub const MG_MNE_ROTRV: &str = "rotrv"; pub const MG_MNE_SRL: &str = "srl"; pub const MG_MNE_ROTR: &str = "rotr";
pub const MG_MNE_MOVF: &str = "movf"; pub const MG_MNE_MOVT: &str = "movt";pub const MG_MNE_SELEQZ: &str = "seleqz"; 
pub const MG_MNE_SELNEZ: &str = "selnez"; pub const MG_MNE_DDIV: &str = "ddiv"; pub const MG_MNE_DDIVU: &str = "ddivu";
pub const MG_MNE_LSA: &str = "lsa"; pub const MG_MNE_DLSA: &str = "dlsa"; pub const MG_MNE_DSLLV: &str = "dsllv";
pub const MG_MNE_DSRAV: &str = "dsrav"; pub const MG_MNE_DSRLV: &str = "dsrlv";pub const MG_MNE_DROTRV: &str = "drotrv";
pub const MG_MNE_DSRA: &str = "dsra"; pub const MG_MNE_DSRL: &str = "dsrl";pub const MG_MNE_DROTR: &str = "drotr";
pub const MG_MNE_DSRA32: &str = "dsra32"; pub const MG_MNE_DSRL32: &str = "dsrl32";pub const MG_MNE_DROTR32: &str = "drotr32";
pub const MG_MNE_DSLL: &str = "dsll"; pub const MG_MNE_DSLL32: &str = "dsll32"; pub const MG_MNE_DADD: &str = "dadd";
pub const MG_MNE_DADDU: &str = "daddu"; pub const MG_MNE_DSUBU: &str = "dsubu"; pub const MG_MNE_DSUB: &str = "dsub";
pub const MG_MNE_DMULTU: &str = "dmultu"; pub const MG_MNE_DMULT: &str = "dmult"; pub const MG_MNE_MUH: &str = "muh";
pub const MG_MNE_MULU: &str = "mulu"; pub const MG_MNE_MUHU: &str = "muhu"; pub const MG_MNE_DMUL: &str = "dmul";
pub const MG_MNE_DMUH: &str = "dmuh";pub const MG_MNE_DMULU: &str = "dmulu"; pub const MG_MNE_DMUHU: &str = "dmuhu";
pub const MG_MNE_MOD: &str = "mod";pub const MG_MNE_MODU: &str = "modu";pub const MG_MNE_DMOD: &str = "dmod";
pub const MG_MNE_DMODU: &str = "dmodu";

//Special2
pub const MG_MNE_MADD: &str = "madd"; pub const MG_MNE_MADDU: &str = "maddu"; pub const MG_MNE_MUL: &str = "mul";
pub const MG_MNE_MSUB: &str = "msub"; pub const MG_MNE_MSUBU: &str = "msubu"; pub const MG_MNE_CLZ: &str = "clz";
pub const MG_MNE_CLO: &str = "clo"; pub const MG_MNE_SDBBP: &str = "sdbbp"; pub const MG_MNE_DCLO: &str = "dclo";
pub const MG_MNE_DCLZ: &str = "dclz";

//Special3
pub const MG_MNE_EXT: &str = "ext"; pub const MG_MNE_INS: &str = "ins"; pub const MG_MNE_WSBH: &str = "wsbh";
pub const MG_MNE_SEB: &str = "seb"; pub const MG_MNE_SEH: &str = "seh"; pub const MG_MNE_RDHWR: &str = "rdhwr";
pub const MG_MNE_DEXT: &str = "dext"; pub const MG_MNE_DEXTU: &str = "dextu";pub const MG_MNE_DEXTM: &str = "dextm";
pub const MG_MNE_DINS: &str = "dins"; pub const MG_MNE_DINSU: &str = "dinsu";pub const MG_MNE_DINSM: &str = "dinsm";
pub const MG_MNE_LWLE: &str = "lwle"; pub const MG_MNE_LWRE: &str = "lwre"; pub const MG_MNE_SWLE: &str = "swle";
pub const MG_MNE_SWRE: &str = "swre"; pub const MG_MNE_PREFE: &str = "prefe";pub const MG_MNE_CACHEE: &str = "cachee";
pub const MG_MNE_SBE: &str = "sbe"; pub const MG_MNE_SHE: &str = "she";pub const MG_MNE_SCE: &str = "sce";
pub const MG_MNE_SWE: &str = "swe"; pub const MG_MNE_LBE: &str = "lbe"; pub const MG_MNE_LHE: &str = "lhe";
pub const MG_MNE_LCE: &str = "lce"; pub const MG_MNE_LWE: &str = "lwe"; pub const MG_MNE_LBUE: &str = "lbue";
pub const MG_MNE_LHUE: &str = "lhue"; pub const MG_MNE_DSBH: &str = "dsbh"; pub const MG_MNE_BITSWAP: &str = "bitswap";
pub const MG_MNE_DBITSWAP: &str = "dbitswap"; pub const MG_MNE_DSHD: &str = "dshd"; pub const MG_MNE_DALIGN: &str = "dalign";
pub const MG_MNE_ALIGN: &str = "align";

//Regimm
pub const MG_MNE_BLTZ: &str = "bltz"; pub const MG_MNE_BGEZ: &str = "bgez";pub const MG_MNE_BLTZL: &str = "bltzl";
pub const MG_MNE_BGEZL: &str = "bgezl"; pub const MG_MNE_TGEI: &str = "tgei"; pub const MG_MNE_TGEIU: &str = "tgeiu";
pub const MG_MNE_TLTI: &str = "tlti"; pub const MG_MNE_TLTIU: &str = "tltiu"; pub const MG_MNE_TEQI: &str = "teqi";
pub const MG_MNE_TNEI: &str = "tnei"; pub const MG_MNE_BLTZAL: &str = "bltzal"; pub const MG_MNE_BGEZAL: &str = "bgezal";
pub const MG_MNE_BLTZALL: &str = "bltzall";pub const MG_MNE_BGEZALL: &str = "bgezall"; pub const MG_MNE_SYNCI: &str = "synci";
pub const MG_MNE_BAL: &str = "bal"; pub const MG_MNE_DAHI: &str = "dahi"; pub const MG_MNE_DATI: &str = "dati";
pub const MG_MNE_SIGRIE: &str = "sigrie";pub const MG_MNE_NAL: &str = "nal";

//CP0
pub const MG_MNE_MFC0: &str = "mfc0"; pub const MG_MNE_MTC0: &str = "mtc0"; pub const MG_MNE_RDPGPR: &str = "rdpgpr";
pub const MG_MNE_WRPGPR: &str = "wrpgpr"; pub const MG_MNE_DI: &str = "di"; pub const MG_MNE_EI: &str = "ei";
pub const MG_MNE_TLBR: &str = "tlbr"; pub const MG_MNE_TLBWI: &str = "tlbwi"; pub const MG_MNE_TLBWR: &str = "tlbwr";
pub const MG_MNE_TLBP: &str = "tlbp"; pub const MG_MNE_ERET: &str = "eret"; pub const MG_MNE_DERET: &str = "deret";
pub const MG_MNE_WAIT: &str = "wait"; pub const MG_MNE_DMFC0: &str = "dmfc0"; pub const MG_MNE_DMTC0: &str = "dmtc0";
pub const MG_MNE_MFHC0: &str = "mfhc0"; pub const MG_MNE_MTHC0: &str = "mthc0"; pub const MG_MNE_TLBINV: &str = "tlbinv";
pub const MG_MNE_TLBINVF: &str = "tlbinvf"; pub const MG_MNE_DVP: &str = "dvp"; pub const MG_MNE_EVP: &str = "evp";

//PCREL
pub const MG_MNE_ADDIUPC: &str = "addiupc"; pub const MG_MNE_LWPC: &str = "lwpc"; pub const MG_MNE_LDPC: &str = "ldpc";
pub const MG_MNE_AUIPC: &str = "auipc"; pub const MG_MNE_ALUIPC: &str = "aluipc"; pub const MG_MNE_LWUPC: &str = "lwupc";

//Cp1
pub const MG_MNE_MFC1: &str = "mfc1"; pub const MG_MNE_MTC1: &str = "mtc1"; pub const MG_MNE_DMFC1: &str = "dmfc1";
pub const MG_MNE_DMTC1: &str = "dmtc1"; pub const MG_MNE_CFC1: &str = "cfc1"; pub const MG_MNE_MFHC1: &str = "mfhc1";
pub const MG_MNE_CTC1: &str = "ctc1"; pub const MG_MNE_MTHC1: &str = "mthc1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MgMnemonic {
    MgMneJ, MgMneJal, MgMneBeq, MgMneBne, MgMneBlez, MgMneBgtz, MgMneAddi, MgMneAddiu, MgMneSlti, MgMneSltiu,
    MgMneAndi, MgMneOri, MgMneXori, MgMneLui, MgMneBeql, MgMneBnel, MgMneBlezl, MgMneBgtzl, MgMneJalx, MgMneLb,
    MgMneLh, MgMneLwl, MgMneLw, MgMneLbu, MgMneLhu, MgMneLwr, MgMneSb, MgMneSh, MgMneSwl, MgMneSw,
    MgMneSwr, MgMneCache, MgMneLl, MgMneLwc1, MgMneLwc2, MgMnePref, MgMneLdc1, MgMneLdc2, MgMneSc, MgMneSwc1,
    MgMneSwc2, MgMneSdc1, MgMneSdc2, MgMneSsnop, MgMneEhb, MgMnePause, MgMneNop, MgMneSll, MgMneSra, MgMneSllv,
    MgMneSrav, MgMneJr, MgMneJrhb, MgMneJalr, MgMneJalrhb, MgMneMovz, MgMneMovn, MgMneSyscall, MgMneBreak, MgMneSync,
    MgMneMfhi, MgMneMthi, MgMneMflo, MgMneMtlo, MgMneMult, MgMneMultu, MgMneDiv, MgMneDivu, MgMneAdd, MgMneAddu,
    MgMneSub, MgMneSubu, MgMneAnd, MgMneOr, MgMneXor, MgMneNor, MgMneSlt, MgMneSltu, MgMneTge, MgMneTgeu,
    MgMneTlt, MgMneTltu, MgMneTeq, MgMneTne, MgMneSrlv, MgMneRotrv, MgMneSrl, MgMneRotr, MgMneMovf, MgMneMovt,
    MgMneMadd, MgMneMaddu, MgMneMul, MgMneMsub, MgMneMsubu, MgMneClz, MgMneClo, MgMneSdbbp, MgMneExt, MgMneIns,
    MgMneWsbh, MgMneSeb, MgMneSeh, MgMneRdhwr, MgMneBltz, MgMneBgez, MgMneBltzl, MgMneBgezl, MgMneTgei, MgMneTgeiu,
    MgMneTlti, MgMneTltiu, MgMneTeqi, MgMneTnei, MgMneBltzal, MgMneBgezal, MgMneBltzall, MgMneBgezall, MgMneSynci, MgMneBal,
    MgMneMfc0, MgMneMtc0, MgMneRdpgpr, MgMneWrpgpr, MgMneDi, MgMneEi, MgMneTlbr, MgMneTlbwi, MgMneTlbwr, MgMneTlbp,
    MgMneEret, MgMneDeret, MgMneWait, MgMneAddiupc, MgMneLwpc, MgMneLdpc, MgMneAuipc, MgMneAluipc, MgMneLwupc, MgMneSeleqz,
    MgMneSelnez, MgMneBalc, MgMneBc,MgMneJic,MgMneJialc, MgMneBovc, MgMneBnvc, MgMneBgeuc, MgMneBlezalc, MgMneBgezalc, MgMneBgtzalc,
    MgMneBltzalc, MgMneBltuc, MgMneBlezc, MgMneBgezc, MgMneBgec, MgMneBgtzc, MgMneBltzc, MgMneBltc, MgMneBeqzalc, MgMneBeqc, MgMneBnezalc,
    MgMneBnec, MgMneBeqzc, MgMneBnezc, MgMneDdiv, MgMneDdivu, MgMneLsa, MgMneDlsa, MgMneDaui, MgMneDahi, MgMneDati, MgMneLdr, MgMneLdl,
    MgMneDaddi, MgMneDaddiu, MgMneAui, MgMneLd, MgMneSd, MgMneSdl, MgMneSdr, MgMneLld, MgMneScd, MgMneLwu, MgMneDclz, MgMneDclo,
    MgMneDsllv, MgMneDsrav, MgMneDsrlv, MgMneDrotrv, MgMneDsra, MgMneDsrl, MgMneDrotr, MgMneDsll, MgMneDsll32, MgMneDsra32, MgMneDsrl32,
    MgMneDrotr32, MgMneDadd, MgMneDaddu, MgMneDsub, MgMneDsubu, MgMneDmult, MgMneDmultu, MgMneMuh, MgMneMulu, MgMneMuhu, MgMneDmul, 
    MgMneDmuh, MgMneDmulu,MgMneDmuhu, MgMneMod, MgMneModu, MgMneDmod, MgMneDmodu, MgMneSigrie, MgMneNal, MgMneDext, MgMneDextm, MgMneDextu,
    MgMneDins, MgMneDinsm, MgMneDinsu, MgMneLwle, MgMneLwre, MgMneSwle, MgMneSwre, MgMnePrefe, MgMneCachee, MgMneSbe, MgMneShe, MgMneSce, MgMneSwe,
    MgMneLbe, MgMneLhe, MgMneLce, MgMneLwe, MgMneLbue, MgMneLhue, MgMneDsbh, MgMneBitswap, MgMneDbitswap, MgMneDshd, MgMneAlign, MgMneDalign, MgMneDmfc0,
    MgMneDmtc0, MgMneDmtc1, MgMneDmfc1, MgMneMtc1, MgMneMfc1, MgMneMfhc0, MgMneMthc0, MgMneTlbinv, MgMneTlbinvf, MgMneDvp, MgMneEvp, MgMneCfc1, MgMneMfhc1,
    MgMneCtc1, MgMneMthc1

}

pub(super)const MG_MNEMONICS: &[&str] = &[
    MG_MNE_J, MG_MNE_JAL, MG_MNE_BEQ, MG_MNE_BNE, MG_MNE_BLEZ, MG_MNE_BGTZ, MG_MNE_ADDI, MG_MNE_ADDIU, MG_MNE_SLTI, MG_MNE_SLTIU,
    MG_MNE_ANDI, MG_MNE_ORI, MG_MNE_XORI, MG_MNE_LUI, MG_MNE_BEQL, MG_MNE_BNEL, MG_MNE_BLEZL, MG_MNE_BGTZL, MG_MNE_JALX, MG_MNE_LB,
    MG_MNE_LH, MG_MNE_LWL, MG_MNE_LW, MG_MNE_LBU, MG_MNE_LHU, MG_MNE_LWR, MG_MNE_SB, MG_MNE_SH, MG_MNE_SWL, MG_MNE_SW,
    MG_MNE_SWR, MG_MNE_CACHE, MG_MNE_LL, MG_MNE_LWC1, MG_MNE_LWC2, MG_MNE_PREF, MG_MNE_LDC1, MG_MNE_LDC2, MG_MNE_SC, MG_MNE_SWC1,
    MG_MNE_SWC2, MG_MNE_SDC1, MG_MNE_SDC2, MG_MNE_SSNOP, MG_MNE_EHB, MG_MNE_PAUSE, MG_MNE_NOP, MG_MNE_SLL, MG_MNE_SRA, MG_MNE_SLLV,
    MG_MNE_SRAV, MG_MNE_JR, MG_MNE_JRHB, MG_MNE_JALR, MG_MNE_JALRHB, MG_MNE_MOVZ, MG_MNE_MOVN, MG_MNE_SYSCALL, MG_MNE_BREAK, MG_MNE_SYNC,
    MG_MNE_MFHI, MG_MNE_MTHI, MG_MNE_MFLO, MG_MNE_MTLO, MG_MNE_MULT, MG_MNE_MULTU, MG_MNE_DIV, MG_MNE_DIVU, MG_MNE_ADD, MG_MNE_ADDU,
    MG_MNE_SUB, MG_MNE_SUBU, MG_MNE_AND, MG_MNE_OR, MG_MNE_XOR, MG_MNE_NOR, MG_MNE_SLT, MG_MNE_SLTU, MG_MNE_TGE, MG_MNE_TGEU,
    MG_MNE_TLT, MG_MNE_TLTU, MG_MNE_TEQ, MG_MNE_TNE, MG_MNE_SRLV, MG_MNE_ROTRV, MG_MNE_SRL, MG_MNE_ROTR, MG_MNE_MOVF, MG_MNE_MOVT,
    MG_MNE_MADD, MG_MNE_MADDU, MG_MNE_MUL, MG_MNE_MSUB, MG_MNE_MSUBU, MG_MNE_CLZ, MG_MNE_CLO, MG_MNE_SDBBP, MG_MNE_EXT, MG_MNE_INS,
    MG_MNE_WSBH, MG_MNE_SEB, MG_MNE_SEH, MG_MNE_RDHWR, MG_MNE_BLTZ, MG_MNE_BGEZ, MG_MNE_BLTZL, MG_MNE_BGEZL, MG_MNE_TGEI, MG_MNE_TGEIU,
    MG_MNE_TLTI, MG_MNE_TLTIU, MG_MNE_TEQI, MG_MNE_TNEI, MG_MNE_BLTZAL, MG_MNE_BGEZAL, MG_MNE_BLTZALL, MG_MNE_BGEZALL, MG_MNE_SYNCI, MG_MNE_BAL,
    MG_MNE_MFC0, MG_MNE_MTC0, MG_MNE_RDPGPR, MG_MNE_WRPGPR, MG_MNE_DI, MG_MNE_EI, MG_MNE_TLBR, MG_MNE_TLBWI, MG_MNE_TLBWR, MG_MNE_TLBP,
    MG_MNE_ERET, MG_MNE_DERET, MG_MNE_WAIT, MG_MNE_ADDIUPC, MG_MNE_LWPC, MG_MNE_LDPC, MG_MNE_AUIPC, MG_MNE_ALUIPC, MG_MNE_LWUPC, MG_MNE_SELEQZ,
    MG_MNE_SELNEZ, MG_MNE_BALC, MG_MNE_BC, MG_MNE_JIC, MG_MNE_JIALC, MG_MNE_BOVC, MG_MNE_BNVC, MG_MNE_BGEUC, MG_MNE_BLEZALC,MG_MNE_BGEZALC,
    MG_MNE_BGTZALC,MG_MNE_BLTZALC, MG_MNE_BLTUC, MG_MNE_BLEZC, MG_MNE_BGEZC, MG_MNE_BGEC, MG_MNE_BGTZC, MG_MNE_BLTZC, MG_MNE_BLTC, MG_MNE_BEQZALC,
    MG_MNE_BEQC, MG_MNE_BNEZALC, MG_MNE_BNEC, MG_MNE_BEQZC,MG_MNE_BNEZC, MG_MNE_DDIV, MG_MNE_DDIVU, MG_MNE_LSA, MG_MNE_DLSA, MG_MNE_DAUI,
    MG_MNE_DAHI, MG_MNE_DATI, MG_MNE_LDR, MG_MNE_LDL, MG_MNE_DADDI, MG_MNE_DADDIU, MG_MNE_AUI, MG_MNE_LD, MG_MNE_SD, MG_MNE_SDL, MG_MNE_SDR,
    MG_MNE_LLD, MG_MNE_SCD, MG_MNE_LWU, MG_MNE_DCLZ, MG_MNE_DCLO, MG_MNE_DSLLV, MG_MNE_DSRAV, MG_MNE_DSRLV, MG_MNE_DROTRV, MG_MNE_DSRA, MG_MNE_DSRL,
    MG_MNE_DROTR, MG_MNE_DSLL, MG_MNE_DSLL32, MG_MNE_DSRA32, MG_MNE_DSRL32, MG_MNE_DROTR32, MG_MNE_DADD, MG_MNE_DADDU, MG_MNE_DSUB, MG_MNE_DSUBU,
    MG_MNE_DMULT, MG_MNE_DMULTU, MG_MNE_MUH, MG_MNE_MULU, MG_MNE_MUHU, MG_MNE_DMUL, MG_MNE_DMUH, MG_MNE_DMULU, MG_MNE_DMUHU, MG_MNE_MOD, MG_MNE_MODU,
    MG_MNE_DMOD, MG_MNE_DMODU, MG_MNE_SIGRIE, MG_MNE_NAL, MG_MNE_DEXT, MG_MNE_DEXTM, MG_MNE_DEXTU, MG_MNE_DINS, MG_MNE_DINSM, MG_MNE_DINSU, MG_MNE_LWLE, MG_MNE_LWRE,
    MG_MNE_SWLE, MG_MNE_SWRE, MG_MNE_PREFE, MG_MNE_CACHEE, MG_MNE_SBE, MG_MNE_SHE, MG_MNE_SCE, MG_MNE_SWE, MG_MNE_LBE, MG_MNE_LHE, MG_MNE_LCE, MG_MNE_LWE,
    MG_MNE_LBUE, MG_MNE_LHUE, MG_MNE_DSBH, MG_MNE_BITSWAP, MG_MNE_DBITSWAP, MG_MNE_DSHD, MG_MNE_ALIGN, MG_MNE_DALIGN, MG_MNE_DMFC0, MG_MNE_DMTC0, MG_MNE_DMTC1, MG_MNE_DMFC1,
    MG_MNE_MTC1, MG_MNE_MFC1, MG_MNE_MFHC0, MG_MNE_MTHC0, MG_MNE_TLBINV, MG_MNE_TLBINVF, MG_MNE_DVP, MG_MNE_EVP, MG_MNE_CFC1, MG_MNE_MFHC1, MG_MNE_CTC1, MG_MNE_MTHC1
];