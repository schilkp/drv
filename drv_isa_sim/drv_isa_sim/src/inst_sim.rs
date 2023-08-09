use crate::{inst::Instruction, inst_decoding::decode_inst, inst_log::InstLog, DRVSim};

// ==== Instruction Implementation =================================================================

impl DRVSim {
    pub fn step(&mut self) -> Result<InstLog, anyhow::Error> {
        // Fetch & decode instruction:
        let inst = self.read_w(self.pc)?.val;
        let inst = decode_inst(inst)?;

        // Track of the instruction branched and already updated the program counter,
        // or if the program counter needs to be incremented to the next instruction:
        let mut did_branch = false;

        // Keep track of details for logging:
        let log_pc = self.pc;
        let log_handling_trap = false; // TODO
        let log_debug_mode = false; // TODO

        // Track all values read and commited by this instruciton for logging:
        let mut log_input_values = vec![];
        let mut log_commit_values = vec![];

        // Execute the fetched instruction:
        match inst {
            Instruction::LUI { imm, rd } => {
                // LUI rd, imm:
                // Places immediate value in MSBs of rd, leaving the rest zero.
                log_commit_values.push(self.write_register(rd, imm));
            }

            Instruction::AUIPC { imm, rd } => {
                // AUIPC rd, imm:
                // Adds the immediate value to the PC of this instruction, placing the result
                // in rd while ignoring overflows.
                let result = u32::wrapping_add(self.pc, imm);
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::JAL { imm, rd } => {
                // JAL rd, imm:
                // Adds the immediate value to the PC of this instruction, and branches to
                // that instruction. The address of the following instruction (pc+4) is stored
                // in rd.
                let pc = self.pc;

                did_branch = true;
                self.pc = u32::wrapping_add(pc, imm);

                let next_inst = u32::wrapping_add(pc, 4);
                log_commit_values.push(self.write_register(rd, next_inst));
            }

            Instruction::JALR { imm, rs1, rd } => {
                // JALR rd, imm(rs1):
                // First, add the immediate to rs1, while ignoring overflows. Set LSB
                // of the obtain value to zero and jump to this instruction. The
                // address of the following instruction (pc+4) is stored in rd.
                let pc = self.pc;

                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                did_branch = true;
                self.pc = u32::wrapping_add(inp_rs1.val, imm) & (!0x1);

                let next_inst = u32::wrapping_add(pc, 4);
                log_commit_values.push(self.write_register(rd, next_inst));
            }

            Instruction::BEQ { imm, rs2, rs1 } => {
                // BEQ rs1, rs2, imm
                // If rs1 equals rs2, branch to the sum of the address of this
                // instruction and the immediate value, otherwise advance to pc+4
                // as normal.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                if inp_rs1.val == inp_rs2.val {
                    did_branch = true;
                    self.pc = u32::wrapping_add(self.pc, imm);
                }
            }

            Instruction::BNE { imm, rs2, rs1 } => {
                // BNE rs1, rs2, imm
                // If rs1 does not equal rs2, branch to the sum of the address of this
                // instruction and the immediate value, otherwise advance to pc+4
                // as normal.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                if inp_rs1.val != inp_rs2.val {
                    did_branch = true;
                    self.pc = u32::wrapping_add(self.pc, imm);
                }
            }

            Instruction::BLT { imm, rs2, rs1 } => {
                // BLT rs1, rs2, imm
                // Interpret rs1 and rs2 as signed values.
                // If rs1 is strictly less than rs2, branch to the sum of the address of this
                // instruction and the immediate value, otherwise advance to pc+4 as normal.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                if (inp_rs1.val as i32) < (inp_rs2.val as i32) {
                    did_branch = true;
                    self.pc = u32::wrapping_add(self.pc, imm);
                }
            }

            Instruction::BGE { imm, rs2, rs1 } => {
                // BGE rs1, rs2, imm
                // Interpret rs1 and rs2 as signed values.
                // If rs1 is greater or equal to rs2, branch to the sum of the address of this
                // instruction and the immediate value, otherwise advance to pc+4 as normal.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                if (inp_rs1.val as i32) >= (inp_rs2.val as i32) {
                    did_branch = true;
                    self.pc = u32::wrapping_add(self.pc, imm);
                }
            }

            Instruction::BLTU { imm, rs2, rs1 } => {
                // BLTU rs1, rs2, imm
                // Interpret rs1 and rs2 as unsigned values.
                // If rs1 is strictly less than rs2, branch to the sum of the address of this
                // instruction and the immediate value, otherwise advance to pc+4 as normal.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                if inp_rs1.val < inp_rs2.val {
                    did_branch = true;
                    self.pc = u32::wrapping_add(self.pc, imm);
                }
            }

            Instruction::BGEU { imm, rs2, rs1 } => {
                // BGEU rs1, rs2, imm
                // Interpret rs1 and rs2 as unsigned values.
                // If rs1 is greater or equal to rs2, branch to the sum of the address of this
                // instruction and the immediate value, otherwise advance to pc+4 as normal.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                if inp_rs1.val >= inp_rs2.val {
                    did_branch = true;
                    self.pc = u32::wrapping_add(self.pc, imm);
                }
            }

            Instruction::LB { imm, rs1, rd } => {
                // LB rd, imm(rs1)
                // First add rs1 to the immediate value to obtain the memory address.
                // Then load a byte from the memory location at that address, sign-extend
                // it to 32 bit, and store it in rd.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);

                let adr = u32::wrapping_add(inp_rs1.val, imm);
                let mem_val = self.read_b(adr)?;
                log_input_values.push(mem_val);

                let result = ((((mem_val.val & 0xFF) as u8) as i8) as i32) as u32;
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::LH { imm, rs1, rd } => {
                // LH rd, imm(rs1)
                // First add rs1 to the immediate value to obtain the memory address.
                // Then load a half-word from the memory location at that address, sign-extend
                // it to 32 bit, and store it in rd.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);

                let adr = u32::wrapping_add(inp_rs1.val, imm);
                let mem_val = self.read_h(adr)?;
                log_input_values.push(mem_val);

                let result = ((((mem_val.val & 0xFFFF) as u16) as i16) as i32) as u32;
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::LW { imm, rs1, rd } => {
                // LW rd, imm(rs1)
                // First add rs1 to the immediate value to obtain the memory address.
                // Then load a word from the memory location at that address, and store it in rd.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);

                let adr = u32::wrapping_add(inp_rs1.val, imm);
                let mem_val = self.read_w(adr)?;
                log_input_values.push(mem_val);

                log_commit_values.push(self.write_register(rd, mem_val.val));
            }

            Instruction::LBU { imm, rs1, rd } => {
                // LBU rd, imm(rs1)
                // First add rs1 to the immediate value to obtain the memory address.
                // Then load a byte from the memory location at that address, zero-
                // extend it to 32 bits, and store in rd.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);

                let adr = u32::wrapping_add(inp_rs1.val, imm);
                let mem_val = self.read_b(adr)?;
                log_input_values.push(mem_val);

                let result = mem_val.val & 0xFF;
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::LHU { imm, rs1, rd } => {
                // LHU rd, imm(rs1)
                // First add rs1 to the immediate value to obtain the memory address.
                // Then load a half word from the memory location at that address, zero-
                // extend it to 32 bits, and store in rd.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);

                let adr = u32::wrapping_add(inp_rs1.val, imm);
                let mem_val = self.read_h(adr)?;
                log_input_values.push(mem_val);

                let result = mem_val.val & 0xFFFF;
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::SB { imm, rs2, rs1 } => {
                // SB rd, rs2, imm(rs1)
                // First add rs1 to the immediate value to obtain the memory address.
                // Then store the lowest byte from rs2 to memory at that address.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                let adr = u32::wrapping_add(inp_rs1.val, imm);
                log_commit_values.push(self.write_b(adr, (inp_rs2.val & 0xFF) as u8)?);
            }

            Instruction::SH { imm, rs2, rs1 } => {
                // SH rd, rs2, imm(rs1)
                // First add rs1 to the immediate value to obtain the memory address.
                // Then store the lowest half-word from rs2 to memory at that address.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                let adr = u32::wrapping_add(inp_rs1.val, imm);
                log_commit_values.push(self.write_h(adr, (inp_rs2.val & 0xFFFF) as u16)?);
            }

            Instruction::SW { imm, rs2, rs1 } => {
                // SW rd, rs2, imm(rs1)
                // First add rs1 to the immediate value to obtain the memory address.
                // Then store rs2 to memory at that address.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                let adr = u32::wrapping_add(inp_rs1.val, imm);
                log_commit_values.push(self.write_w(adr, inp_rs2.val)?);
            }

            Instruction::ADDI { imm, rs1, rd } => {
                // ADDI rd, rs1, imm:
                // Add imm to rs1, place result in rd, ignoring overflows.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);

                let result = u32::wrapping_add(inp_rs1.val, imm);
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::SLTI { imm, rs1, rd } => {
                // SLTI rd, rs1, imm:
                // Interpret rs1 and imm as signed values. Set rd to 1 if rs1 < imm, otherwise
                // set rd to 0.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);

                if (inp_rs1.val as i32) < (imm as i32) {
                    log_commit_values.push(self.write_register(rd, 1));
                } else {
                    log_commit_values.push(self.write_register(rd, 0));
                }
            }

            Instruction::SLTIU { imm, rs1, rd } => {
                // SLTIU rd, rs1, imm:
                // Interpret rs1 and imm as unsigned values, noting that imm still sign extended.
                // Set rd to 1 if rs1 < imm, otherwise set rd to 0.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);

                if inp_rs1.val < imm {
                    log_commit_values.push(self.write_register(rd, 1));
                } else {
                    log_commit_values.push(self.write_register(rd, 0));
                }
            }

            Instruction::XORI { imm, rs1, rd } => {
                // XORI rd, rs1, imm:
                // Calculate the bitwise XOR of rs1 and imm, store the result in rd.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);

                let result = inp_rs1.val ^ imm;
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::ORI { imm, rs1, rd } => {
                // ORI rd, rs1, imm:
                // Calculate the bitwise OR of rs1 and imm, store the result in rd.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);

                let result = inp_rs1.val | imm;
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::ANDI { imm, rs1, rd } => {
                // ANDI rd, rs1, imm:
                // Calculate the bitwise AND of rs1 and imm, store the result in rd.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);

                let result = inp_rs1.val & imm;
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::SLLI { shamt, rs1, rd } => {
                // SLLI rd, rs1, shamt:
                // Logical-shift rs1 left by the immediate shamt, placing the result
                // in rd.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);

                let result = inp_rs1.val << shamt;
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::SRLI { shamt, rs1, rd } => {
                // SRLI rd, rs1, shamt:
                // Logical-shift rs1 right by the immediate shamt, placing the result
                // in rd.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);

                let result = inp_rs1.val >> shamt;
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::SRAI { shamt, rs1, rd } => {
                // SRAI rd, rs1, shamt:
                // Arithmetic-shift rs1 right by the immediate shamt, placing the result
                // in rd.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);

                let result = ((inp_rs1.val as i32) >> shamt) as u32;
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::ADD { rs2, rs1, rd } => {
                // ADD rd, rs1, rs2:
                // Add rs1 to rs2, place result in rd, ignoring overflows.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                let result = u32::wrapping_add(inp_rs1.val, inp_rs2.val);
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::SUB { rs2, rs1, rd } => {
                // SUB rd, rs1, rs2:
                // Subtract rs2 from rs1, place result in rd, ignoring overflows.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                let result = u32::wrapping_sub(inp_rs1.val, inp_rs2.val);
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::SLL { rs2, rs1, rd } => {
                // SLL rd, rs1, rs2:
                // Logical-shift rs1 left by the amount in the lower 5 bits of rs2, placing
                // the result in rd.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                let result = inp_rs1.val << (inp_rs2.val & 0x1F);
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::SLT { rs2, rs1, rd } => {
                // SLT rd, rs1, rs2:
                // Interpret rs1 and rs2 as signed values. Set rd to 1 if rs1 < rs2, otherwise
                // set rd to 0.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                if (inp_rs1.val as i32) < (inp_rs2.val as i32) {
                    log_commit_values.push(self.write_register(rd, 1));
                } else {
                    log_commit_values.push(self.write_register(rd, 0));
                }
            }

            Instruction::SLTU { rs2, rs1, rd } => {
                // SLTU rd, rs1, rs2:
                // Interpret rs1 and rs2 as unsigned values. Set rd to 1 if rs1 < rs2, otherwise
                // set rd to 0.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                if inp_rs1.val < inp_rs2.val {
                    log_commit_values.push(self.write_register(rd, 1));
                } else {
                    log_commit_values.push(self.write_register(rd, 0));
                }
            }

            Instruction::XOR { rs2, rs1, rd } => {
                // XOR rd, rs1, rs2:
                // Calculate the bitwise XOR of rs1 and rs2, store the result in rd.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                let result = inp_rs1.val ^ inp_rs2.val;
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::SRL { rs2, rs1, rd } => {
                // SRL rd, rs1, rs2:
                // Logical-shift rs1 right by the amount in the lower 5 bits of rs2, placing
                // the result in rd.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                let result = inp_rs1.val >> (inp_rs2.val & 0x1F);
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::SRA { rs2, rs1, rd } => {
                // SRL rd, rs1, rs2:
                // Arithmetic-shift rs1 right by the amount in the lower 5 bits of rs2, placing
                // the result in rd.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                let result = ((inp_rs1.val as i32) >> (inp_rs2.val & 0x1F)) as u32;
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::OR { rs2, rs1, rd } => {
                // OR rd, rs1, rs2:
                // Calculate the bitwise OR of rs1 and rs2, store the result in rd.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                let result = inp_rs1.val | inp_rs2.val;
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::AND { rs2, rs1, rd } => {
                // AND rd, rs1, rs2:
                // Calculate the bitwise AND of rs1 and rs2, store the result in rd.
                let inp_rs1 = self.read_register(rs1)?;
                log_input_values.push(inp_rs1);
                let inp_rs2 = self.read_register(rs2)?;
                log_input_values.push(inp_rs2);

                let result = inp_rs1.val & inp_rs2.val;
                log_commit_values.push(self.write_register(rd, result));
            }

            Instruction::FENCE { .. } => {
                // Do nothing.
                // All instructions, including their memory access, are atomic.
            }

            Instruction::ECALL => todo!(),
            Instruction::EBREAK => todo!(),
            Instruction::DRET => todo!(),
            Instruction::MRET => todo!(),
        };

        if !did_branch {
            self.pc = u32::wrapping_add(self.pc, 4);
        }

        Ok(InstLog {
            pc: log_pc,
            inst,
            handling_trap: log_handling_trap,
            branching: did_branch,
            debug_mode: log_debug_mode,
            input_values: log_input_values,
            commit_values: log_commit_values,
        })
    }
}

// ==== Instruction Unit Tests =====================================================================

#[cfg(test)]
mod tests {
    use crate::*;

    const ROM_START: u32 = 0x1000000;
    const RAM_START: u32 = 0x2000000;

    // Create a new simulator with a given set of instructions and register values
    // pre-loaded.
    fn new_simulator(
        insts: Vec<u32>,
        reg_vals: Vec<(Register, u32)>,
        mem_vals: Vec<(u32, u32)>,
    ) -> DRVSim {
        // Create new simulator:
        let mut sim = DRVSim::new(DRVSimConfig {
            entry: ROM_START,
            mtvec: 0,
            dvec: 0,
            mem_regions: vec![
                MemoryRegionConfig {
                    adr_range: ROM_START..ROM_START + 0x8000,
                    init: ValueInit::Error,
                    region_type: MemoryRegionType::ROM,
                },
                MemoryRegionConfig {
                    adr_range: RAM_START..RAM_START + 0x8000,
                    init: ValueInit::FixedByte(0xAB),
                    region_type: MemoryRegionType::RAM,
                },
            ],
            reg_init: ValueInit::Error,
        });

        // Load instructions, memory and registers:
        for (idx, inst) in insts.iter().enumerate() {
            sim.program_w(ROM_START + (idx as u32) * 4, *inst).unwrap();
        }
        for (adr, val) in mem_vals.iter() {
            sim.program_w(*adr, *val).unwrap();
        }
        for (reg, val) in reg_vals.iter() {
            sim.write_register(*reg, *val);
        }

        sim
    }

    // ==== Register-Register & Register-Immediate Instructions ====

    // Assert that instruction $inst produces $rd in x3 given $rs1 in x1 and $rs2 in x2:
    macro_rules! test_register_inst {
        ($name:tt, $inst:expr, $rs1:expr, $rs2:expr, $rd:expr) => {
            #[test]
            fn $name() {
                let mut sim = new_simulator(
                    vec![$inst],
                    vec![(Register::X1, $rs1), (Register::X2, $rs2)],
                    vec![],
                );
                // Step:
                println!("{:?}", sim.step().unwrap());
                // Compare x3:
                let is = sim.read_register(Register::X3).unwrap().val;
                println!("x3 is: 0x{:x}, expect: 0x{:x}", is, $rd);
                assert_eq!(is, ($rd as u32));
            }
        };
    }

    // LUI x3, 0x1BEEF:
    test_register_inst!(inst_lui, 0x1BEEF1B7, 0x0, 0x0, 0x1BEEF000);

    // AUIPC x3, 0x0:
    test_register_inst!(inst_auipc_0, 0x00000197, 0x0, 0x0, ROM_START + 0);
    // AUIPC x3, 0x1:
    test_register_inst!(inst_auipc_1, 0x00001197, 0x0, 0x0, ROM_START + 0x1000);
    // AUIPC x3, 0xfffff:
    test_register_inst!(inst_auipc_2, 0xfffff197, 0x0, 0x0, ROM_START - 0x1000);

    // ADD x3, x1, x2:
    test_register_inst!(inst_add_0, 0x002081B3, 0x100, 0x200, 0x300);
    test_register_inst!(inst_add_1, 0x002081B3, 0xFFFFFFFF, 0x1, 0x0);

    // SUB x3, x1, x2:
    test_register_inst!(inst_sub_0, 0x402081B3, 0x300, 0x200, 0x100);
    test_register_inst!(inst_sub_1, 0x402081B3, 0x0, 0x1, -1_i32 as u32);

    // XOR x3, x1, x2:
    test_register_inst!(inst_xor_0, 0x0020c1b3, 0x0, 0x0, 0x0);
    test_register_inst!(inst_xor_1, 0x0020c1b3, 0x0, 0x1, 0x1);
    test_register_inst!(inst_xor_2, 0x0020c1b3, 0x1, 0x0, 0x1);
    test_register_inst!(inst_xor_3, 0x0020c1b3, 0x1, 0x1, 0x0);
    test_register_inst!(
        inst_xor_4,
        0x0020c1b3,
        0xFFFF0000_u32,
        0xF0F08421_u32,
        0x0F0F8421_u32
    );

    // OR x3, x1, x3:
    test_register_inst!(inst_or_0, 0x0020e1b3, 0x0, 0x0, 0x0);
    test_register_inst!(inst_or_1, 0x0020e1b3, 0x0, 0x1, 0x1);
    test_register_inst!(inst_or_2, 0x0020e1b3, 0x1, 0x0, 0x1);
    test_register_inst!(inst_or_3, 0x0020e1b3, 0x1, 0x1, 0x1);
    test_register_inst!(
        inst_or_4,
        0x0020e1b3,
        0xFFFF0000_u32,
        0x000FFFF_u32,
        0xFFFFFFFF_u32
    );

    // AND x3, x1, x2
    test_register_inst!(inst_and_0, 0x0020f1b3, 0x0, 0x0, 0x0);
    test_register_inst!(inst_and_1, 0x0020f1b3, 0x0, 0x1, 0x0);
    test_register_inst!(inst_and_2, 0x0020f1b3, 0x1, 0x0, 0x0);
    test_register_inst!(inst_and_3, 0x0020f1b3, 0x1, 0x1, 0x1);
    test_register_inst!(
        inst_and_4,
        0x0020f1b3,
        0xFFFFFFFF_u32,
        0xDEADBEEF_u32,
        0xDEADBEEF_u32
    );

    // ADDI x3, x1, 0x1
    test_register_inst!(inst_addi_0, 0x00108193, 0x100, 0x00, 0x101);
    test_register_inst!(inst_addi_1, 0x00108193, 0xFFFFFFFF, 0x00, 0x0);
    // ADDI x3, x1, -0x1
    test_register_inst!(inst_addi_2, 0xfff08193, 0x100, 0x00, 0xFF);
    test_register_inst!(inst_addi_3, 0xfff08193, 0x0, 0x00, 0xFFFFFFFF_u32);

    // SLTI x3, x1, 0x2
    test_register_inst!(inst_slti_0, 0x0020a193, 0x1, 0x00, 0x1);
    test_register_inst!(inst_slti_1, 0x0020a193, 0x2, 0x00, 0x0);
    test_register_inst!(inst_slti_2, 0x0020a193, 0x3, 0x00, 0x0);
    // SLTI x3, x1, -200
    test_register_inst!(inst_slti_3, 0xf380a193, -201_i32 as u32, 0x00, 0x1);
    test_register_inst!(inst_slti_4, 0xf380a193, -200_i32 as u32, 0x00, 0x0);
    test_register_inst!(inst_slti_5, 0xf380a193, -199_i32 as u32, 0x00, 0x0);

    // SLTIU x3, x1, 0x2
    test_register_inst!(inst_sltiu_0, 0x0020b193, 0x1, 0x00, 0x1);
    test_register_inst!(inst_sltiu_1, 0x0020b193, 0x2, 0x00, 0x0);
    test_register_inst!(inst_sltiu_2, 0x0020b193, 0x3, 0x00, 0x0);
    // SLTIU x3, x1, -200
    // Note comparison against -200 sign extended -> 0xffffffe
    test_register_inst!(inst_sltiu_3, 0xffe0b193, 0xfffffffd, 0x00, 0x1);
    test_register_inst!(inst_sltiu_4, 0xffe0b193, 0xfffffffe, 0x00, 0x0);
    test_register_inst!(inst_sltiu_5, 0xffe0b193, 0xffffffff, 0x00, 0x0);

    // XORI x3, x1, 0x0
    test_register_inst!(inst_xori_0, 0x0000c193, 0x0, 0x0, 0x0);
    test_register_inst!(inst_xori_1, 0x0000c193, 0x1, 0x0, 0x1);
    // XORI x3, x1, 0x1
    test_register_inst!(inst_xori_2, 0x0010c193, 0x0, 0x0, 0x1);
    test_register_inst!(inst_xori_3, 0x0010c193, 0x1, 0x0, 0x0);
    // XORI x3, x1, -1
    test_register_inst!(inst_xori_4, 0xfff0c193, 0x0, 0x0, 0xFFFFFFFF_u32);
    test_register_inst!(inst_xori_5, 0xfff0c193, 0xF0F0F0F0, 0x0, 0x0F0F0F0F_u32);

    // ORI x3, x1, 0x0
    test_register_inst!(inst_ori_0, 0x0000e193, 0x0, 0x0, 0x0);
    test_register_inst!(inst_ori_1, 0x0000e193, 0x1, 0x0, 0x1);
    // ORI x3, x1, 0x1
    test_register_inst!(inst_ori_2, 0x0010e193, 0x0, 0x0, 0x1);
    test_register_inst!(inst_ori_3, 0x0010e193, 0x1, 0x0, 0x1);
    // ORI x3, x1, -1
    test_register_inst!(inst_ori_4, 0xf010e193, 0x0, 0x0, 0xFFFFFF01_u32);
    test_register_inst!(inst_ori_5, 0xf010e193, 0xF0, 0x0, 0xFFFFFFF1_u32);

    // ANDI x3, x1, 0x0
    test_register_inst!(inst_andi_0, 0x0000f193, 0x0, 0x0, 0x0);
    test_register_inst!(inst_andi_1, 0x0000f193, 0x1, 0x0, 0x0);
    // ANDI x3, x1, 0x1
    test_register_inst!(inst_andi_2, 0x0010f193, 0x0, 0x0, 0x0);
    test_register_inst!(inst_andi_3, 0x0010f193, 0x1, 0x0, 0x1);
    // ANDI x3, x1, -0x800
    test_register_inst!(inst_andi_4, 0x8000f193, 0xFFFFFFFF_u32, 0x0, 0xFFFFF800_u32);
    test_register_inst!(inst_andi_5, 0x8000f193, 0xF4F30000_u32, 0x0, 0xF4F30000_u32);

    // SLLI x3, x1, 0
    test_register_inst!(inst_slli_0, 0x00009193, 0xDEADBEEF, 0x0, 0xDEADBEEF_u32);
    // SLLI x3, x1, 1
    test_register_inst!(
        inst_slli_1,
        0x00109193,
        0xDEADBEEF,
        0x0,
        (0xDEADBEEF_u32 << 1)
    );
    // SLLI x3, x1, 31
    test_register_inst!(
        inst_slli_2,
        0x01F09193,
        0xDEADBEEF,
        0x0,
        (0xDEADBEEF_u32 << 31)
    );

    // SRLI x3, x1, 0
    test_register_inst!(inst_srli_0, 0x0000d193, 0xDEADBEEF, 0x0, 0xDEADBEEF_u32);
    // SRLI x3, x1, 1
    test_register_inst!(inst_srli_1, 0x0010D193, 0x80000000, 0x0, 0x40000000_u32);
    test_register_inst!(inst_srli_2, 0x0010D193, 0x40000000, 0x0, 0x20000000_u32);
    // SRLI x3, x1, 31
    test_register_inst!(inst_srli_3, 0x01F0D193, 0x80000000, 0x0, 0x1);
    test_register_inst!(inst_srli_4, 0x01F0D193, 0x7FFFFFFF, 0x0, 0x0);

    // SRAI x3, x1, 0
    test_register_inst!(inst_srai_0, 0x4000d193, 0xDEADBEEF, 0x0, 0xDEADBEEF_u32);
    // SRAI x3, x1, 1
    test_register_inst!(inst_srai_1, 0x4010d193, 0x80000000, 0x0, 0xC0000000_u32);
    test_register_inst!(inst_srai_2, 0x4010d193, 0x40000000, 0x0, 0x20000000_u32);
    // SRAI x3, x1, 31
    test_register_inst!(inst_srai_3, 0x41F0D193, 0x80000000, 0x0, 0xFFFFFFFF_u32);
    test_register_inst!(inst_srai_4, 0x41F0D193, 0x7FFFFFFF, 0x0, 0x0);

    // SLL x3, x1, x2
    test_register_inst!(inst_sll_0, 0x002091b3, 0xDEADBEEF, 0x0, 0xDEADBEEF_u32);
    test_register_inst!(
        inst_sll_1,
        0x002091b3,
        0xDEADBEEF,
        0x1,
        (0xDEADBEEF_u32 << 1)
    );
    test_register_inst!(inst_sll_2, 0x002091b3, 0xDEADBEEF, 31, 0x80000000_u32);
    test_register_inst!(inst_sll_3, 0x002091b3, 0xDEADBEEF, 0xF000, 0xDEADBEEF_u32);
    test_register_inst!(
        inst_sll_4,
        0x002091b3,
        0xDEADBEEF,
        0xF001,
        (0xDEADBEEF_u32 << 1)
    );

    // SRL x3, x1, x2
    test_register_inst!(inst_srl_0, 0x0020d1b3, 0xDEADBEEF, 0x0, 0xDEADBEEF_u32);
    test_register_inst!(
        inst_srl_1,
        0x0020d1b3,
        0xDEADBEEF,
        0x1,
        (0xDEADBEEF_u32 >> 1)
    );
    test_register_inst!(inst_srl_3, 0x0020d1b3, 0x80000000, 0x1, 0x40000000_u32);
    test_register_inst!(inst_srl_4, 0x0020d1b3, 0x40000000, 0x1, 0x20000000_u32);
    test_register_inst!(inst_srl_5, 0x0020d1b3, 0x80000000, 31, 0x1);
    test_register_inst!(inst_srl_6, 0x0020d1b3, 0x7FFFFFFF, 31, 0x0);
    test_register_inst!(inst_srl_7, 0x0020d1b3, 0x40000000, 0xFE1, 0x20000000_u32);
    test_register_inst!(inst_srl_8, 0x0020d1b3, 0x80000000, 0xFFF, 0x1);
    test_register_inst!(inst_srl_9, 0x0020d1b3, 0x7FFFFFFF, 0xFFF, 0x0);

    // SRA x3, x1, x2
    test_register_inst!(inst_sra_0, 0x4020d1b3, 0xDEADBEEF, 0x0, 0xDEADBEEF_u32);
    test_register_inst!(
        inst_sra_1,
        0x4020d1b3,
        0xDEADBEEF,
        0x1,
        ((0xDEADBEEF_u32 as i32) >> 1_i32) as u32
    );
    test_register_inst!(inst_sra_3, 0x4020d1b3, 0x80000000, 0x1, 0xC0000000_u32);
    test_register_inst!(inst_sra_4, 0x4020d1b3, 0x40000000, 0x1, 0x20000000_u32);
    test_register_inst!(inst_sra_5, 0x4020d1b3, 0x80000000, 31, 0xFFFFFFFF_u32);
    test_register_inst!(inst_sra_6, 0x4020d1b3, 0x7FFFFFFF, 31, 0x0);
    test_register_inst!(inst_sra_7, 0x4020d1b3, 0x40000000, 0xFE1, 0x20000000_u32);
    test_register_inst!(inst_sra_8, 0x4020d1b3, 0x80000000, 0xFFF, 0xFFFFFFFF_u32);
    test_register_inst!(inst_sra_9, 0x4020d1b3, 0x7FFFFFFF, 0xFFF, 0x0);

    // SLT x3, x1, x2
    test_register_inst!(inst_slt_0, 0x0020a1b3, 0x0, 0x1, 0x1);
    test_register_inst!(inst_slt_1, 0x0020a1b3, 0x1, 0x1, 0x0);
    test_register_inst!(inst_slt_2, 0x0020a1b3, 0x2, 0x1, 0x0);
    test_register_inst!(inst_slt_3, 0x0020a1b3, -1_i32 as u32, 1, 0x1);
    test_register_inst!(
        inst_slt_4,
        0x0020a1b3,
        -101_i32 as u32,
        -100_i32 as u32,
        0x1
    );
    test_register_inst!(
        inst_slt_5,
        0x0020a1b3,
        -100_i32 as u32,
        -100_i32 as u32,
        0x0
    );
    test_register_inst!(inst_slt_6, 0x0020a1b3, -99_i32 as u32, -100_i32 as u32, 0x0);

    // SLTU x3, x1, x2
    test_register_inst!(inst_sltu_0, 0x0020b1b3, 0x0, 0x1, 0x1);
    test_register_inst!(inst_sltu_1, 0x0020b1b3, 0x1, 0x1, 0x0);
    test_register_inst!(inst_sltu_2, 0x0020b1b3, 0x2, 0x1, 0x0);
    test_register_inst!(inst_sltu_3, 0x0020b1b3, 0xFFFFFFFD_u32, 0xFFFFFFFE_u32, 0x1);
    test_register_inst!(inst_sltu_4, 0x0020b1b3, 0xFFFFFFFE_u32, 0xFFFFFFFE_u32, 0x0);
    test_register_inst!(inst_sltu_5, 0x0020b1b3, 0xFFFFFFFF_u32, 0xFFFFFFFE_u32, 0x0);

    // ==== Jump Instructions ====

    // Assert that instruction $inst, given $rs1 in x1, causes the PC to advance to $dest
    // and place $rd in x3:
    macro_rules! test_jump_inst {
        ($name:tt, $inst:expr, $rs1:expr, $rd:expr, $dest:expr) => {
            #[test]
            fn $name() {
                let mut sim = new_simulator(vec![$inst], vec![(Register::X1, $rs1)], vec![]);
                // Step:
                println!("{:?}", sim.step().unwrap());
                // Compare PC:
                let is = sim.pc;
                println!("PC is: 0x{:x}, expect: 0x{:x}", is, $dest);
                assert_eq!(is, $dest);
                // Compare x3:
                let is = sim.read_register(Register::X3).unwrap().val;
                println!("x3 is: 0x{:x}, expect: 0x{:x}", is, $rd);
                assert_eq!(is, $rd);
            }
        };
    }

    //  JAL x3, .+0
    test_jump_inst!(inst_jal_0, 0x000001ef, 0x0, ROM_START + 0x4, ROM_START + 0);
    //  JAL x3, .+4
    test_jump_inst!(inst_jal_1, 0x004001ef, 0x0, ROM_START + 0x4, ROM_START + 4);
    //  JAL x3, .-4
    test_jump_inst!(inst_jal_2, 0xffdff1ef, 0x0, ROM_START + 0x4, ROM_START - 4);

    // JALR x3, 0x4(x1)
    test_jump_inst!(inst_jalr_0, 0x004081e7, 0x100, ROM_START + 0x4, 4 + 0x100);
    test_jump_inst!(inst_jalr_1, 0x004081e7, 0x101, ROM_START + 0x4, 4 + 0x100);
    // JALR x3, 0x104(x1)
    test_jump_inst!(
        inst_jalr_3,
        0x104081e7,
        -8_i32 as u32,
        ROM_START + 0x4,
        0xFC
    );
    // JALR x3, 0x5(x1)
    test_jump_inst!(inst_jalr_2, 0x005081e7, 0x100, ROM_START + 0x4, 4 + 0x100);

    // ==== Branch Instructions ====

    // Assert that instruction $inst, given $rs1 in x1 and $rs2 in x2, causes
    // PC to advance to $dest
    macro_rules! test_branch_inst {
        ($name:tt, $inst:expr, $rs1:expr, $rs2:expr, $dest:expr) => {
            #[test]
            fn $name() {
                let mut sim = new_simulator(
                    vec![$inst],
                    vec![(Register::X1, $rs1), (Register::X2, $rs2)],
                    vec![],
                );
                // Step:
                println!("{:?}", sim.step().unwrap());
                // Compare PC:
                let is = sim.pc;
                println!("PC is: 0x{:x}, expect: 0x{:x}", is, $dest);
                assert_eq!(is, $dest);
            }
        };
    }

    // BEQ x1, x2, .+8
    test_branch_inst!(inst_beq_1, 0x00208463, 0x0, 0x0, ROM_START + 8);
    test_branch_inst!(inst_beq_2, 0x00208463, 0x0, 0x1, ROM_START + 4);
    // BEQ x1, x2, .-4
    test_branch_inst!(inst_beq_3, 0xfe208ee3, 0x0, 0x0, ROM_START - 4);
    test_branch_inst!(inst_beq_4, 0xfe208ee3, 0x0, 0x1, ROM_START + 4);

    // BNE x1, x2, .+8
    test_branch_inst!(inst_bne_1, 0x00209463, 0x0, 0x0, ROM_START + 4);
    test_branch_inst!(inst_bne_2, 0x00209463, 0x0, 0x1, ROM_START + 8);
    // BNE x1, x2, .-4
    test_branch_inst!(inst_bne_3, 0xfe209ee3, 0x0, 0x0, ROM_START + 4);
    test_branch_inst!(inst_bne_4, 0xfe209ee3, 0x0, 0x1, ROM_START - 4);

    // BLT x1, x2, .+16
    test_branch_inst!(
        inst_blt_0,
        0x0020c863,
        -1_i32 as u32,
        1_i32 as u32,
        ROM_START + 16
    );
    test_branch_inst!(
        inst_blt_1,
        0x0020c863,
        1_i32 as u32,
        -1_i32 as u32,
        ROM_START + 4
    );
    // BLT x1, x2, .-16
    test_branch_inst!(inst_blt_3, 0xfe20c8e3, 0x1, 0x0, ROM_START + 4);
    test_branch_inst!(inst_blt_4, 0xfe20c8e3, 0x1, 0x1, ROM_START + 4);
    test_branch_inst!(inst_blt_5, 0xfe20c8e3, 0x0, 0x1, ROM_START - 16);

    // BGE x1, x2, .+8
    test_branch_inst!(inst_bge_0, 0x0020d463, 0x0, 0x1, ROM_START + 4);
    test_branch_inst!(inst_bge_1, 0x0020d463, 0x1, 0x1, ROM_START + 8);
    test_branch_inst!(inst_bge_2, 0x0020d463, 0x2, 0x1, ROM_START + 8);
    test_branch_inst!(
        inst_bge_3,
        0x0020d463,
        0xFFFFFFFA_u32,
        0xFFFFFFFB_u32,
        ROM_START + 4
    );
    test_branch_inst!(
        inst_bge_4,
        0x0020d463,
        0xFFFFFFFB_u32,
        0xFFFFFFFB_u32,
        ROM_START + 8
    );
    test_branch_inst!(
        inst_bge_5,
        0x0020d463,
        0xFFFFFFFC_u32,
        0xFFFFFFFB_u32,
        ROM_START + 8
    );
    // BGE x1, x2, .-8
    test_branch_inst!(inst_bge_6, 0xfe20dce3, 0x0, 0x1, (ROM_START + 4));
    test_branch_inst!(inst_bge_7, 0xfe20dce3, 0x2, 0x1, (ROM_START - 8));

    // BLTU x1, x2, .+8
    test_branch_inst!(inst_bltu_0, 0x0020e463, 0x0, 0x1, (ROM_START + 8));
    test_branch_inst!(inst_bltu_1, 0x0020e463, 0x1, 0x1, (ROM_START + 4));
    test_branch_inst!(inst_bltu_2, 0x0020e463, 0x2, 0x1, (ROM_START + 4));
    test_branch_inst!(
        inst_bltu_3,
        0x0020e463,
        0xFFFFFFFD_u32,
        0xFFFFFFFE_u32,
        (ROM_START + 8)
    );
    test_branch_inst!(
        inst_bltu_4,
        0x0020e463,
        0xFFFFFFFE_u32,
        0xFFFFFFFE_u32,
        (ROM_START + 4)
    );
    test_branch_inst!(
        inst_bltu_5,
        0x0020e463,
        0xFFFFFFFF_u32,
        0xFFFFFFFE_u32,
        (ROM_START + 4)
    );
    // BLTU x1, x2, .-8
    test_branch_inst!(inst_bltu_6, 0xfe20ece3, 0x0, 0x1, (ROM_START - 8));
    test_branch_inst!(inst_bltu_7, 0xfe20ece3, 0x1, 0x1, (ROM_START + 4));
    test_branch_inst!(inst_bltu_8, 0xfe20ece3, 0x2, 0x1, (ROM_START + 4));
    test_branch_inst!(
        inst_bltu_9,
        0xfe20ece3,
        0xFFFFFFFD_u32,
        0xFFFFFFFE_u32,
        (ROM_START - 8)
    );
    test_branch_inst!(
        inst_bltu_10,
        0xfe20ece3,
        0xFFFFFFFE_u32,
        0xFFFFFFFE_u32,
        (ROM_START + 4)
    );
    test_branch_inst!(
        inst_bltu_11,
        0xfe20ece3,
        0xFFFFFFFF_u32,
        0xFFFFFFFE_u32,
        (ROM_START + 4)
    );

    // BGEU x1, x2, .+8
    test_branch_inst!(inst_bgeu_0, 0x0020f463, 0x0, 0x1, (ROM_START + 4));
    test_branch_inst!(inst_bgeu_1, 0x0020f463, 0x1, 0x1, (ROM_START + 8));
    test_branch_inst!(inst_bgeu_2, 0x0020f463, 0x2, 0x1, (ROM_START + 8));
    test_branch_inst!(
        inst_bgeu_3,
        0x0020f463,
        0xFFFFFFFD_u32,
        0xFFFFFFFE_u32,
        (ROM_START + 4)
    );
    test_branch_inst!(
        inst_bgeu_4,
        0x0020f463,
        0xFFFFFFFE_u32,
        0xFFFFFFFE_u32,
        (ROM_START + 8)
    );
    test_branch_inst!(
        inst_bgeu_5,
        0x0020f463,
        0xFFFFFFFF_u32,
        0xFFFFFFFE_u32,
        (ROM_START + 8)
    );
    // BGEU x1, x2, .-8
    test_branch_inst!(inst_bgeu_6, 0xfe20fce3, 0x0, 0x1, (ROM_START + 4));
    test_branch_inst!(inst_bgeu_7, 0xfe20fce3, 0x1, 0x1, (ROM_START - 8));
    test_branch_inst!(inst_bgeu_8, 0xfe20fce3, 0x2, 0x1, (ROM_START - 8));
    test_branch_inst!(
        inst_bgeu_9,
        0xfe20fce3,
        0xFFFFFFFD_u32,
        0xFFFFFFFE_u32,
        (ROM_START + 4)
    );
    test_branch_inst!(
        inst_bgeu_10,
        0xfe20fce3,
        0xFFFFFFFE_u32,
        0xFFFFFFFE_u32,
        (ROM_START - 8)
    );
    test_branch_inst!(
        inst_bgeu_11,
        0xfe20fce3,
        0xFFFFFFFF_u32,
        0xFFFFFFFE_u32,
        (ROM_START - 8)
    );

    // ==== Load Instructions ====

    // Assert that instruction $inst, given $rs1 in x1 and the word $w stored at $adr, produces
    // $rd in x3.
    macro_rules! test_load_inst {
        ($name:tt, $inst:expr, $rs1:expr, $w:expr, $adr:expr, $rd:expr) => {
            #[test]
            fn $name() {
                let mut sim =
                    new_simulator(vec![$inst], vec![(Register::X1, $rs1)], vec![($adr, $w)]);
                // Step:
                println!("{:?}", sim.step().unwrap());
                // Compare x3:
                let is = sim.read_register(Register::X3).unwrap().val;
                println!("x3 is: 0x{:x}, expect: 0x{:x}", is, $rd);
                assert_eq!(is, ($rd as u32));
            }
        };
    }

    // LW x3, 0x100(x1)
    test_load_inst!(
        inst_lw_0,
        0x1000a183_u32,
        RAM_START,
        0xDEADBEEF,
        RAM_START + 0x100,
        0xDEADBEEF_u32
    );
    // LW x3, 0x0(x1)
    test_load_inst!(
        inst_lw_1,
        0x0000a183_u32,
        RAM_START,
        0xDEADBEEF,
        RAM_START,
        0xDEADBEEF_u32
    );
    // LW x3, -0x100(x1)
    test_load_inst!(
        inst_lw_2,
        0xf000a183_u32,
        RAM_START + 0x100,
        0xDEADBEEF,
        RAM_START,
        0xDEADBEEF_u32
    );

    // LH x3, 0x100(x1)
    test_load_inst!(
        inst_lh_0,
        0x10009183_u32,
        RAM_START,
        0xDEADBEEF,
        RAM_START + 0x100,
        0xFFFFBEEF_u32
    );
    test_load_inst!(
        inst_lh_1,
        0x10009183_u32,
        RAM_START,
        0x00000EEF,
        RAM_START + 0x100,
        0x00000EEF_u32
    );
    // LH x3, 0x0(x1)
    test_load_inst!(
        inst_lh_2,
        0x00009183_u32,
        RAM_START,
        0xDEADBEEF,
        RAM_START,
        0xFFFFBEEF_u32
    );
    test_load_inst!(
        inst_lh_3,
        0x00009183_u32,
        RAM_START,
        0x00000EEF,
        RAM_START,
        0x00000EEF_u32
    );
    // LH x3, -0x0(x1)
    test_load_inst!(
        inst_lh_4,
        0xF0009183_u32,
        RAM_START + 0x100,
        0xDEADBEEF,
        RAM_START,
        0xFFFFBEEF_u32
    );
    test_load_inst!(
        inst_lh_5,
        0xF0009183_u32,
        RAM_START + 0x100,
        0x00000EEF,
        RAM_START,
        0x00000EEF_u32
    );

    // LHU x3, 0x100(x1)
    test_load_inst!(
        inst_lhu_0,
        0x1000d183_u32,
        RAM_START,
        0xDEADBEEF,
        RAM_START + 0x100,
        0x0000BEEF_u32
    );
    test_load_inst!(
        inst_lhu_1,
        0x1000d183_u32,
        RAM_START,
        0x00000EEF,
        RAM_START + 0x100,
        0x00000EEF_u32
    );
    // LHU x3, 0x0(x1)
    test_load_inst!(
        inst_lhu_2,
        0x0000d183_u32,
        RAM_START,
        0xDEADBEEF,
        RAM_START,
        0x0000BEEF_u32
    );
    test_load_inst!(
        inst_lhu_3,
        0x0000d183_u32,
        RAM_START,
        0x00000EEF,
        RAM_START,
        0x00000EEF_u32
    );
    // LHU x3, -0x0(x1)
    test_load_inst!(
        inst_lhu_4,
        0xF000d183_u32,
        RAM_START + 0x100,
        0xDEADBEEF,
        RAM_START,
        0x0000BEEF_u32
    );
    test_load_inst!(
        inst_lhu_5,
        0xF000d183_u32,
        RAM_START + 0x100,
        0x00000EEF,
        RAM_START,
        0x00000EEF_u32
    );

    // LB x3, 0x100(x1)
    test_load_inst!(
        inst_lb_0,
        0x10008183_u32,
        RAM_START,
        0xDEADBEEF,
        RAM_START + 0x100,
        0xFFFFFFEF_u32
    );
    test_load_inst!(
        inst_lb_1,
        0x10008183_u32,
        RAM_START,
        0x00000007F,
        RAM_START + 0x100,
        0x00000007F_u32
    );
    // LB x3, 0x0(x1)
    test_load_inst!(
        inst_lb_2,
        0x00008183_u32,
        RAM_START,
        0xDEADBEEF,
        RAM_START,
        0xFFFFFFEF_u32
    );
    test_load_inst!(
        inst_lb_3,
        0x00008183_u32,
        RAM_START,
        0x00000007F,
        RAM_START,
        0x00000007F_u32
    );
    // LB x3, -0x0(x1)
    test_load_inst!(
        inst_lb_4,
        0xF0008183_u32,
        RAM_START + 0x100,
        0xDEADBEEF,
        RAM_START,
        0xFFFFFFEF_u32
    );
    test_load_inst!(
        inst_lb_5,
        0xF0008183_u32,
        RAM_START + 0x100,
        0x00000007F,
        RAM_START,
        0x00000007F_u32
    );

    // LBU x3, 0x100(x1)
    test_load_inst!(
        inst_lbu_0,
        0x1000c183_u32,
        RAM_START,
        0xDEADBEEF,
        RAM_START + 0x100,
        0x000000EF_u32
    );
    test_load_inst!(
        inst_lbu_1,
        0x1000c183_u32,
        RAM_START,
        0x00000007F,
        RAM_START + 0x100,
        0x00000007F_u32
    );
    // LBU x3, 0x0(x1)
    test_load_inst!(
        inst_lbu_2,
        0x0000c183_u32,
        RAM_START,
        0xDEADBEEF,
        RAM_START,
        0x000000EF_u32
    );
    test_load_inst!(
        inst_lbu_3,
        0x0000c183_u32,
        RAM_START,
        0x00000007F,
        RAM_START,
        0x00000007F_u32
    );
    // LBU x3, -0x0(x1)
    test_load_inst!(
        inst_lbu_4,
        0xF000c183_u32,
        RAM_START + 0x100,
        0xDEADBEEF,
        RAM_START,
        0x000000EF_u32
    );
    test_load_inst!(
        inst_lbu_5,
        0xF000c183_u32,
        RAM_START + 0x100,
        0x00000007F,
        RAM_START,
        0x00000007F_u32
    );

    // ==== Store Instructions ====

    // Assert that instruction $inst, given $rs1 in x1 and $rs2 in x2, results in the word
    // $w being stored at $adr in memory.
    macro_rules! test_store_inst {
        ($name:tt, $inst:expr, $rs1:expr, $rs2:expr,  $adr:expr, $w:expr) => {
            #[test]
            fn $name() {
                let mut sim = new_simulator(
                    vec![$inst],
                    vec![(Register::X1, $rs1), (Register::X2, $rs2)],
                    vec![],
                );
                // Step:
                println!("{:?}", sim.step().unwrap());
                // Compare mem[$adr]:
                let is = sim.read_w($adr).unwrap().val;
                println!("x3 is: 0x{:x}, expect: 0x{:x}", is, $w);
                assert_eq!(is, ($w as u32));
            }
        };
    }

    // SB x2, 0x100(x1)
    test_store_inst!(
        inst_sb_0,
        0x10208023_u32,
        RAM_START - 0x100,
        0xDEADBEEF_u32,
        RAM_START,
        0xABAbABEF_u32
    );
    // SB x2, -0x100(x1)
    test_store_inst!(
        inst_sb_1,
        0xf0208023_u32,
        RAM_START + 0x100,
        0xDEADBEEF_u32,
        RAM_START,
        0xABAbABEF_u32
    );

    // SH x2, 0x100(x1)
    test_store_inst!(
        inst_sh_0,
        0x10209023_u32,
        RAM_START - 0x100,
        0xDEADBEEF_u32,
        RAM_START,
        0xABAbBEEF_u32
    );
    // SH x2, -0x100(x1)
    test_store_inst!(
        inst_sh_1,
        0xf0209023_u32,
        RAM_START + 0x100,
        0xDEADBEEF_u32,
        RAM_START,
        0xABAbBEEF_u32
    );

    // SW x2, 0x100(x1)
    test_store_inst!(
        inst_sw_0,
        0x1020a023_u32,
        RAM_START - 0x100,
        0xDEADBEEF_u32,
        RAM_START,
        0xDEADBEEF_u32
    );
    // SW x2, -0x100(x1)
    test_store_inst!(
        inst_sw_1,
        0xf020a023_u32,
        RAM_START + 0x100,
        0xDEADBEEF_u32,
        RAM_START,
        0xDEADBEEF_u32
    );
}
