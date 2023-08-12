// ==== Type Definitions ===========================================================================

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Register {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    Xmpc, // Non-standard DRV extension
    Xdpc, // Non-standard DRV extension
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum Instruction {
    // RV32I:
    LUI {
        imm: u32,
        rd: Register,
    },
    AUIPC {
        imm: u32,
        rd: Register,
    },
    JAL {
        imm: u32,
        rd: Register,
    },
    JALR {
        imm: u32,
        rs1: Register,
        rd: Register,
    },
    BEQ {
        imm: u32,
        rs2: Register,
        rs1: Register,
    },
    BNE {
        imm: u32,
        rs2: Register,
        rs1: Register,
    },
    BLT {
        imm: u32,
        rs2: Register,
        rs1: Register,
    },
    BGE {
        imm: u32,
        rs2: Register,
        rs1: Register,
    },
    BLTU {
        imm: u32,
        rs2: Register,
        rs1: Register,
    },
    BGEU {
        imm: u32,
        rs2: Register,
        rs1: Register,
    },
    LB {
        imm: u32,
        rs1: Register,
        rd: Register,
    },
    LH {
        imm: u32,
        rs1: Register,
        rd: Register,
    },
    LW {
        imm: u32,
        rs1: Register,
        rd: Register,
    },
    LBU {
        imm: u32,
        rs1: Register,
        rd: Register,
    },
    LHU {
        imm: u32,
        rs1: Register,
        rd: Register,
    },
    SB {
        imm: u32,
        rs2: Register,
        rs1: Register,
    },
    SH {
        imm: u32,
        rs2: Register,
        rs1: Register,
    },
    SW {
        imm: u32,
        rs2: Register,
        rs1: Register,
    },
    ADDI {
        imm: u32,
        rs1: Register,
        rd: Register,
    },
    SLTI {
        imm: u32,
        rs1: Register,
        rd: Register,
    },
    SLTIU {
        imm: u32,
        rs1: Register,
        rd: Register,
    },
    XORI {
        imm: u32,
        rs1: Register,
        rd: Register,
    },
    ORI {
        imm: u32,
        rs1: Register,
        rd: Register,
    },
    ANDI {
        imm: u32,
        rs1: Register,
        rd: Register,
    },
    SLLI {
        shamt: u32,
        rs1: Register,
        rd: Register,
    },
    SRLI {
        shamt: u32,
        rs1: Register,
        rd: Register,
    },
    SRAI {
        shamt: u32,
        rs1: Register,
        rd: Register,
    },
    ADD {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    SUB {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    SLL {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    SLT {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    SLTU {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    XOR {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    SRL {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    SRA {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    OR {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    AND {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    FENCE {
        fm: u32,
        pred: u32,
        succ: u32,
    },
    ECALL,
    EBREAK,

    DRET,
    MRET,
}

// ==== Instruction-to-String formatting ===========================================================

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::LUI { imm, rd } => write!(f, "lui {rd:?}, 0x{:x}", imm >> 12),
            Instruction::AUIPC { imm, rd } => write!(f, "auipc {rd:?}, 0x{:x}", imm >> 12),
            Instruction::JAL { imm, rd } => write!(f, "jal {rd:?}, .+0x{imm:x}"),
            Instruction::JALR { imm, rs1, rd } => write!(f, "jalr {rd:?}, 0x{imm:x}({rs1:?})"),
            Instruction::BEQ { imm, rs2, rs1 } => write!(f, "beq {rs1:?}, {rs2:?}, .+0x{imm:x}"),
            Instruction::BNE { imm, rs2, rs1 } => write!(f, "bne {rs1:?}, {rs2:?}, .+0x{imm:x}"),
            Instruction::BLT { imm, rs2, rs1 } => write!(f, "blt {rs1:?}, {rs2:?}, .+0x{imm:x}"),
            Instruction::BGE { imm, rs2, rs1 } => write!(f, "bge {rs1:?}, {rs2:?}, .+0x{imm:x}"),
            Instruction::BLTU { imm, rs2, rs1 } => write!(f, "bltu {rs1:?}, {rs2:?}, .+0x{imm:x}"),
            Instruction::BGEU { imm, rs2, rs1 } => write!(f, "bgeu {rs1:?}, {rs2:?}, .+0x{imm:x}"),
            Instruction::LB { imm, rs1, rd } => write!(f, "lb {rd:?}, 0x{imm:x}({rs1:?})"),
            Instruction::LH { imm, rs1, rd } => write!(f, "lh {rd:?}, 0x{imm:x}({rs1:?})"),
            Instruction::LW { imm, rs1, rd } => write!(f, "lw {rd:?}, 0x{imm:x}({rs1:?})"),
            Instruction::LBU { imm, rs1, rd } => write!(f, "lbu {rd:?}, 0x{imm:x}({rs1:?})"),
            Instruction::LHU { imm, rs1, rd } => write!(f, "lhu {rd:?}, 0x{imm:x}({rs1:?})"),
            Instruction::SB { imm, rs2, rs1 } => write!(f, "sb {rs2:?}, 0x{imm:x}({rs1:?})"),
            Instruction::SH { imm, rs2, rs1 } => write!(f, "sh {rs2:?}, 0x{imm:x}({rs1:?})"),
            Instruction::SW { imm, rs2, rs1 } => write!(f, "sw {rs2:?}, 0x{imm:x}({rs1:?})"),
            Instruction::ADDI { imm, rs1, rd } => write!(f, "addi {rd:?}, {rs1:?}, 0x{imm:x}"),
            Instruction::SLTI { imm, rs1, rd } => write!(f, "slti {rd:?}, {rs1:?}, 0x{imm:x}"),
            Instruction::SLTIU { imm, rs1, rd } => write!(f, "sltiu {rd:?}, {rs1:?}, 0x{imm:x}"),
            Instruction::XORI { imm, rs1, rd } => write!(f, "xori {rd:?}, {rs1:?}, 0x{imm:x}"),
            Instruction::ORI { imm, rs1, rd } => write!(f, "ori {rd:?}, {rs1:?}, 0x{imm:x}"),
            Instruction::ANDI { imm, rs1, rd } => write!(f, "andi {rd:?}, {rs1:?}, 0x{imm:x}"),
            Instruction::SLLI { shamt, rs1, rd } => write!(f, "slli {rd:?}, {rs1:?}, 0x{shamt:x}"),
            Instruction::SRLI { shamt, rs1, rd } => write!(f, "srli {rd:?}, {rs1:?}, 0x{shamt:x}"),
            Instruction::SRAI { shamt, rs1, rd } => write!(f, "srai {rd:?}, {rs1:?}, 0x{shamt:x}"),
            Instruction::ADD { rs2, rs1, rd } => write!(f, "add {rd:?}, {rs1:?}, {rs2:?}"),
            Instruction::SUB { rs2, rs1, rd } => write!(f, "sub {rd:?}, {rs1:?}, {rs2:?}"),
            Instruction::SLL { rs2, rs1, rd } => write!(f, "sll {rd:?}, {rs1:?}, {rs2:?}"),
            Instruction::SLT { rs2, rs1, rd } => write!(f, "slt {rd:?}, {rs1:?}, {rs2:?}"),
            Instruction::SLTU { rs2, rs1, rd } => write!(f, "sltu {rd:?}, {rs1:?}, {rs2:?}"),
            Instruction::XOR { rs2, rs1, rd } => write!(f, "xor {rd:?}, {rs1:?}, {rs2:?}"),
            Instruction::SRL { rs2, rs1, rd } => write!(f, "srl {rd:?}, {rs1:?}, {rs2:?}"),
            Instruction::SRA { rs2, rs1, rd } => write!(f, "sra {rd:?}, {rs1:?}, {rs2:?}"),
            Instruction::OR { rs2, rs1, rd } => write!(f, "or {rd:?}, {rs1:?}, {rs2:?}"),
            Instruction::AND { rs2, rs1, rd } => write!(f, "and {rd:?}, {rs1:?}, {rs2:?}"),
            Instruction::FENCE { fm, pred, succ } => write!(f, "fence f={fm}, p={pred}, s={succ}"),
            Instruction::ECALL => write!(f, "ecall"),
            Instruction::EBREAK => write!(f, "ebreak"),
            Instruction::DRET => write!(f, "dret"),
            Instruction::MRET => write!(f, "mret"),
        }
    }
}
