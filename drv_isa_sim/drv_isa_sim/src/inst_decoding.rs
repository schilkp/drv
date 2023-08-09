use crate::inst::{Instruction, Register};
use anyhow::anyhow;
use bitvec::prelude::*;

// ==== Type Definitions ===========================================================================

type InstBits = BitArray<[u32; 1], Lsb0>;

// ==== Register Parsing ===========================================================================

impl Register {
    pub fn new(i: u32) -> Result<Register, anyhow::Error> {
        match i {
            0 => Ok(Register::X0),
            1 => Ok(Register::X1),
            2 => Ok(Register::X2),
            3 => Ok(Register::X3),
            4 => Ok(Register::X4),
            5 => Ok(Register::X5),
            6 => Ok(Register::X6),
            7 => Ok(Register::X7),
            8 => Ok(Register::X8),
            9 => Ok(Register::X9),
            10 => Ok(Register::X10),
            11 => Ok(Register::X11),
            12 => Ok(Register::X12),
            13 => Ok(Register::X13),
            14 => Ok(Register::X14),
            15 => Ok(Register::X15),
            16 => Ok(Register::Xmpc),
            17 => Ok(Register::Xdpc),
            n => Err(anyhow!("Unknown register {}!", n)),
        }
    }
}

// ==== Base Instruction Format Parsing ============================================================

struct RInstruction {
    funct7: u32,
    rs2: Register,
    rs1: Register,
    funct3: u32,
    rd: Register,
}

impl RInstruction {
    fn new(inst: u32) -> Result<RInstruction, anyhow::Error> {
        let bits: InstBits = BitArray::new([inst]);

        let funct7 = bits[25..=31].load();
        let rs2 = Register::new(bits[20..=24].load())?;
        let rs1 = Register::new(bits[15..=19].load())?;
        let funct3 = bits[12..=14].load();
        let rd = Register::new(bits[7..=11].load())?;

        Ok(RInstruction {
            funct7,
            rs2,
            rs1,
            funct3,
            rd,
        })
    }
}

struct IInstruction {
    imm: u32,
    rs1: Register,
    funct3: u32,
    rd: Register,
}

impl IInstruction {
    fn new(inst: u32) -> Result<IInstruction, anyhow::Error> {
        let bits: InstBits = BitArray::new([inst]);

        let rs1 = Register::new(bits[15..=19].load())?;
        let funct3 = bits[12..=14].load();
        let rd = Register::new(bits[7..=11].load())?;

        // Consutrct I-type immediate value:
        let mut imm: InstBits = BitArray::ZERO;
        for i in 11..=31 {
            imm[i..=i].clone_from_bitslice(&bits[31..=31]);
        }
        imm[0..=10].clone_from_bitslice(&bits[20..=30]);

        Ok(IInstruction {
            imm: imm.load(),
            rs1,
            funct3,
            rd,
        })
    }
}

struct SInstruction {
    imm: u32,
    rs2: Register,
    rs1: Register,
    funct3: u32,
}

impl SInstruction {
    fn new(inst: u32) -> Result<SInstruction, anyhow::Error> {
        let bits: InstBits = BitArray::new([inst]);

        let rs2 = Register::new(bits[20..=24].load())?;
        let rs1 = Register::new(bits[15..=19].load())?;
        let funct3 = bits[12..=14].load();

        // Consutrct S-type immediate value:
        let mut imm: InstBits = BitArray::ZERO;
        for i in 11..=31 {
            imm[i..=i].clone_from_bitslice(&bits[31..=31])
        }
        imm[5..=10].clone_from_bitslice(&bits[25..=30]);
        imm[0..=4].clone_from_bitslice(&bits[7..=11]);

        Ok(SInstruction {
            imm: imm.load(),
            rs2,
            rs1,
            funct3,
        })
    }
}

struct BInstruction {
    imm: u32,
    rs2: Register,
    rs1: Register,
    funct3: u32,
}

impl BInstruction {
    fn new(inst: u32) -> Result<BInstruction, anyhow::Error> {
        let bits: InstBits = BitArray::new([inst]);

        let rs2 = Register::new(bits[20..=24].load())?;
        let rs1 = Register::new(bits[15..=19].load())?;
        let funct3 = bits[12..=14].load();

        // Consutrct B-type immediate value:
        let mut imm: InstBits = BitArray::ZERO;
        for i in 12..=31 {
            imm[i..=i].clone_from_bitslice(&bits[31..=31])
        }
        imm[11..=11].clone_from_bitslice(&bits[7..=7]);
        imm[5..=10].clone_from_bitslice(&bits[25..=30]);
        imm[1..=4].clone_from_bitslice(&bits[8..=11]);

        Ok(BInstruction {
            imm: imm.load(),
            rs2,
            rs1,
            funct3,
        })
    }
}

struct UInstruction {
    imm: u32,
    rd: Register,
}

impl UInstruction {
    fn new(inst: u32) -> Result<UInstruction, anyhow::Error> {
        let bits: InstBits = BitArray::new([inst]);

        let rd = Register::new(bits[7..=11].load())?;

        // Consutrct U-type immediate value:
        let mut imm: InstBits = BitArray::ZERO;
        imm[12..=31].clone_from_bitslice(&bits[12..=31]);

        Ok(UInstruction {
            imm: imm.load(),
            rd,
        })
    }
}

struct JInstruction {
    imm: u32,
    rd: Register,
}

impl JInstruction {
    fn new(inst: u32) -> Result<JInstruction, anyhow::Error> {
        let bits: InstBits = BitArray::new([inst]);

        let rd = Register::new(bits[7..=11].load())?;

        // Consutrct U-type immediate value:
        let mut imm: InstBits = BitArray::ZERO;
        for i in 20..=31 {
            imm[i..=i].clone_from_bitslice(&bits[31..=31]);
        }
        imm[12..=19].clone_from_bitslice(&bits[12..=19]);
        imm[11..=11].clone_from_bitslice(&bits[20..=20]);
        imm[1..=10].clone_from_bitslice(&bits[21..=30]);

        Ok(JInstruction {
            imm: imm.load(),
            rd,
        })
    }
}

// ==== Instruction Decoding =======================================================================

pub fn decode_inst(inst: u32) -> Result<Instruction, anyhow::Error> {
    let opcode = inst & 0b1111111;

    match opcode {
        // RV32I:
        0b0110111 => {
            // LUI
            let UInstruction { imm, rd } = UInstruction::new(inst)?;
            Ok(Instruction::LUI { imm, rd })
        }
        0b0010111 => {
            // AUIPC
            let UInstruction { imm, rd } = UInstruction::new(inst)?;
            Ok(Instruction::AUIPC { imm, rd })
        }
        0b1101111 => {
            // JAL
            let JInstruction { imm, rd } = JInstruction::new(inst)?;
            Ok(Instruction::JAL { imm, rd })
        }
        0b1100111 => {
            // JALR
            let IInstruction {
                imm,
                rs1,
                funct3,
                rd,
                ..
            } = IInstruction::new(inst)?;
            if funct3 != 0b000 {
                return Err(anyhow!("JALR funct3 needs to be 0b000, is 0b{funct3:b}"));
            }
            Ok(Instruction::JALR { imm, rs1, rd })
        }
        0b1100011 => {
            // BEQ, BNE, BLT, BGE, BLTU, BGEU
            let BInstruction {
                imm,
                rs2,
                rs1,
                funct3,
                ..
            } = BInstruction::new(inst)?;
            match funct3 {
                0b000 => Ok(Instruction::BEQ { imm, rs2, rs1 }),
                0b001 => Ok(Instruction::BNE { imm, rs2, rs1 }),
                0b100 => Ok(Instruction::BLT { imm, rs2, rs1 }),
                0b101 => Ok(Instruction::BGE { imm, rs2, rs1 }),
                0b110 => Ok(Instruction::BLTU { imm, rs2, rs1 }),
                0b111 => Ok(Instruction::BGEU { imm, rs2, rs1 }),
                n => Err(anyhow!(
                    "Unknown funct3 value for branch instruction 0b{n:b}"
                )),
            }
        }
        0b0000011 => {
            // LB, LH, LW, LBU, LHU
            let IInstruction {
                imm,
                rs1,
                funct3,
                rd,
                ..
            } = IInstruction::new(inst)?;
            match funct3 {
                0b000 => Ok(Instruction::LB { imm, rs1, rd }),
                0b001 => Ok(Instruction::LH { imm, rs1, rd }),
                0b010 => Ok(Instruction::LW { imm, rs1, rd }),
                0b100 => Ok(Instruction::LBU { imm, rs1, rd }),
                0b101 => Ok(Instruction::LHU { imm, rs1, rd }),
                n => Err(anyhow!("Unknown funct3 value for load instruction 0b{n:b}")),
            }
        }
        0b0100011 => {
            // SB, SH, SW
            let SInstruction {
                imm,
                rs2,
                rs1,
                funct3,
                ..
            } = SInstruction::new(inst)?;

            match funct3 {
                0b000 => Ok(Instruction::SB { imm, rs1, rs2 }),
                0b001 => Ok(Instruction::SH { imm, rs1, rs2 }),
                0b010 => Ok(Instruction::SW { imm, rs1, rs2 }),
                n => Err(anyhow!(
                    "Unknown funct3 value for store instruction 0b{n:b}"
                )),
            }
        }
        0b0010011 => {
            // ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI, SRLI, SRAI
            let IInstruction {
                imm,
                rs1,
                funct3,
                rd,
                ..
            } = IInstruction::new(inst)?;

            match funct3 {
                0b000 => Ok(Instruction::ADDI { imm, rs1, rd }), // ADDI
                0b010 => Ok(Instruction::SLTI { imm, rs1, rd }), // SLTI
                0b011 => Ok(Instruction::SLTIU { imm, rs1, rd }), // SLTIU
                0b100 => Ok(Instruction::XORI { imm, rs1, rd }), // XORI
                0b110 => Ok(Instruction::ORI { imm, rs1, rd }),  // ORI
                0b111 => Ok(Instruction::ANDI { imm, rs1, rd }), // ANDI
                0b001 => {
                    // SLLI
                    let shamt = (inst >> 20) & 0b11111;
                    let ctrl = (inst >> 25) & 0b1111111;

                    if ctrl != 0 {
                        return Err(anyhow!("SLLI requires MSBs to be zero, not 0b{ctrl:b}"));
                    }

                    Ok(Instruction::SLLI { shamt, rs1, rd })
                }
                0b101 => {
                    // SRLI, SRAI
                    let shamt = (inst >> 20) & 0b11111;
                    let ctrl = (inst >> 25) & 0b1111111;

                    match ctrl {
                        0b0000000 => Ok(Instruction::SRLI { shamt, rs1, rd }),
                        0b0100000 => Ok(Instruction::SRAI { shamt, rs1, rd }),
                        _ => Err(anyhow!("Unknown MSBs for SRLI/SRAI: 0b{ctrl:b}")),
                    }
                }
                n => Err(anyhow!(
                    "Unknown funct3 for interger register-immediate instructions 0b{n:b}"
                )),
            }
        }
        0b0110011 => {
            // ADD, SUB, SLL, SLT, SLTU, XOR, SRL, SRA, OR, AND
            let RInstruction {
                funct7,
                rs2,
                rs1,
                funct3,
                rd,
                ..
            } = RInstruction::new(inst)?;

            match (funct7, funct3) {
                (0b0000000, 0b000) => Ok(Instruction::ADD { rs1, rs2, rd }),
                (0b0100000, 0b000) => Ok(Instruction::SUB { rs1, rs2, rd }),
                (0b0000000, 0b001) => Ok(Instruction::SLL { rs1, rs2, rd }),
                (0b0000000, 0b010) => Ok(Instruction::SLT { rs1, rs2, rd }),
                (0b0000000, 0b011) => Ok(Instruction::SLTU { rs1, rs2, rd }),
                (0b0000000, 0b100) => Ok(Instruction::XOR { rs1, rs2, rd }),
                (0b0000000, 0b101) => Ok(Instruction::SRL { rs1, rs2, rd }),
                (0b0100000, 0b101) => Ok(Instruction::SRA { rs1, rs2, rd }),
                (0b0000000, 0b110) => Ok(Instruction::OR { rs1, rs2, rd }),
                (0b0000000, 0b111) => Ok(Instruction::AND { rs1, rs2, rd }),
                (_, _)  => Err(anyhow!(
                    "Unknown funct7/3 for interger register-register instructions 0b{funct7:b}/0b{funct3:b}"
                )),
            }
        }
        0b0001111 => {
            // FENCE
            let IInstruction { funct3, .. } = IInstruction::new(inst)?;

            if funct3 != 0 {
                return Err(anyhow!(
                    "Unknown funct3 for fence instructions 0b{funct3:b}"
                ));
            };

            let fm = (inst >> 28) & 0b1111;
            let pred = (inst >> 24) & 0b1111;
            let succ = (inst >> 20) & 0b1111;

            Ok(Instruction::FENCE { fm, pred, succ })
        }

        0b1110011 => {
            // ECALL, EBREAK, MRET, DRET
            match inst {
                0x00000073 => Ok(Instruction::ECALL),
                0x00100073 => Ok(Instruction::EBREAK),
                0x7b200073 => Ok(Instruction::DRET),
                0x30200073 => Ok(Instruction::MRET),
                _ => Err(anyhow!("Invalid ECALL/EBREAK/MRET/DRET-style instruction.")),
            }
        }

        _ => Err(anyhow!("Unknown instruction opcode: 0b{opcode:b}")),
    }
}

// ==== Instruction Decoding Tests =================================================================

#[cfg(test)]
mod tests {
    use crate::inst_decoding::*;

    #[test]
    fn inst_decoding() {
        let input = vec![
            (0x123457b7_u32, "lui x15, 0x12345"),
            (0x1beef517_u32, "auipc x10, 0x1beef"),
            (0x004000ef_u32, "jal x1, .+0x4"),
            (0x004605e7_u32, "jalr x11, 0x4(x12)"),
            (0x00100263_u32, "beq x0, x1, .+0x4"),
            (0x00311463_u32, "bne x2, x3, .+0x8"),
            (0x00524963_u32, "blt x4, x5, .+0x12"),
            (0x00735b63_u32, "bge x6, x7, .+0x16"),
            (0x02946063_u32, "bltu x8, x9, .+0x20"),
            (0x02b57263_u32, "bgeu x10, x11, .+0x24"),
            (0x0ff68603_u32, "lb x12, 0xFF(x13)"),
            (0x0ff69603_u32, "lh x12, 0xFF(x13)"),
            (0x0ff6a603_u32, "lw x12, 0xFF(x13)"),
            (0x0ff6c603_u32, "lbu x12, 0xFF(x13)"),
            (0x0ff6d603_u32, "lhu x12, 0xFF(x13)"),
            (0x0a1005a3_u32, "sb x1, 0xAB(x0)"),
            (0x0a2095a3_u32, "sh x2, 0xAB(x1)"),
            (0x0a3125a3_u32, "sw x3, 0xAB(x2)"),
            (0x00130293_u32, "addi x5, x6, 0x1"),
            (0x00132293_u32, "slti x5, x6, 0x1"),
            (0x00133293_u32, "sltiu x5, x6, 0x1"),
            (0x00134293_u32, "xori x5, x6, 0x1"),
            (0x00136293_u32, "ori x5, x6, 0x1"),
            (0x00137293_u32, "andi x5, x6, 0x1"),
            (0x01f29313_u32, "slli x6, x5, 0x1f"),
            (0x01445393_u32, "srli x7, x8, 0x14"),
            (0x40a15093_u32, "srai x1, x2, 0xa"),
            (0x003100b3_u32, "add x1, x2, x3"),
            (0x40628233_u32, "sub x4, x5, x6"),
            (0x009413b3_u32, "sll x7, x8, x9"),
            (0x00c5a533_u32, "slt x10, x11, x12"),
            (0x00c5b533_u32, "sltu x10, x11, x12"),
            (0x003140b3_u32, "xor x1, x2, x3"),
            (0x005251b3_u32, "srl x3, x4, x5"),
            (0x405251b3_u32, "sra x3, x4, x5"),
            (0x003160b3_u32, "or x1, x2, x3"),
            (0x003170b3_u32, "and x1, x2, x3"),
            (0x00000073_u32, "ecall"),
            (0x00100073_u32, "ebreak"),
            (0x7b200073_u32, "dret"),
            (0x30200073_u32, "mret"),
        ];

        for (binary, orig) in input {
            println!("Decoding 0x{binary:x} (objdump_output)");
            let inst = decode_inst(binary).unwrap();
            let inst = format!("{:?}", inst).to_lowercase();
            let orig = orig.to_lowercase();
            assert_eq!(inst, orig);
        }

        assert!(matches!(
            decode_inst(0x0ff0000f_u32).unwrap(),
            Instruction::FENCE { .. }
        ));
    }
}
