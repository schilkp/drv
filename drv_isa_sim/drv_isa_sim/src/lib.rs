mod inst;
mod inst_decoding;
pub mod inst_log;
mod inst_sim;
mod memory;

use crate::{inst::Register, inst_log::Value, memory::Memory};
use anyhow::anyhow;
use rand::Rng;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::Range;
use std::path::PathBuf;

use elf::endian::LittleEndian;
use elf::ElfBytes;

// ===== Type Definitions ==========================================================================

#[derive(Copy, Clone)]
pub enum ValueInit {
    Random,
    Zero,
    Ones,
    Error,
    FixedByte(u8),
    FixedWord(u32),
}

#[derive(Copy, Clone)]
pub enum MemoryRegionType {
    RAM,
    ROM,
}

#[derive(Clone)]
pub struct MemoryRegionConfig {
    pub adr_range: Range<u32>,
    pub init: ValueInit,
    pub region_type: MemoryRegionType,
}

pub struct DRVSimConfig {
    pub entry: u32, // Address of first instruction to be executed.
    pub mtvec: u32, // Address of interrupt handler.
    pub dvec: u32,  // Address of debug program buffer.
    pub mem_regions: Vec<MemoryRegionConfig>, // Available memory.
    pub reg_init: ValueInit, // Initial value of registers after reset.
}

struct MemoryRegion {
    adr_range: Range<u32>,
    mem: Memory,
}

pub struct DRVSim {
    core_reg: HashMap<Register, u32>, // Core registers.
    pc: u32,                          // Program Counter.
    mems: Vec<MemoryRegion>,          // Memories.
    config: DRVSimConfig,             // Simulation Settings
}

// ===== DRVSim Implementation =====================================================================

impl DRVSim {
    pub fn new(config: DRVSimConfig) -> DRVSim {
        let mut mem = vec![];

        for config in config.mem_regions.iter() {
            let start_adr = config.adr_range.start;
            let init = config.init;

            let memory = match config.region_type {
                MemoryRegionType::RAM => Memory::new(start_adr, init, false),
                MemoryRegionType::ROM => Memory::new(start_adr, init, true),
            };

            mem.push(MemoryRegion {
                adr_range: config.adr_range.clone(),
                mem: memory,
            });
        }

        DRVSim {
            core_reg: HashMap::new(),
            pc: config.entry,
            mems: mem,
            config,
        }
    }

    fn find_mem_region(&mut self, adr: u32, access_len: u32) -> Result<usize, anyhow::Error> {
        assert!(access_len > 0 && access_len <= 4);
        for (idx, region) in self.mems.iter().enumerate() {
            if region.adr_range.contains(&adr) {
                if region.adr_range.contains(&(adr + access_len - 1)) {
                    return Ok(idx);
                } else {
                    return Err(anyhow!("Attempted to access {access_len} bytes of memory at 0x{adr:0x}, which crosses a memory region boundary."));
                }
            }
        }

        Err(anyhow!("Access to unknown memory address 0x{adr:x}"))
    }

    pub fn program_b(&mut self, adr: u32, val: u8) -> Result<(), anyhow::Error> {
        let region_idx = self.find_mem_region(adr, 1)?;
        self.mems[region_idx].mem.program_b(adr, val);
        Ok(())
    }

    pub fn program_w(&mut self, adr: u32, val: u32) -> Result<(), anyhow::Error> {
        let region_idx = self.find_mem_region(adr, 4)?;
        self.mems[region_idx].mem.program_b(adr, (val & 0xFF) as u8);
        self.mems[region_idx]
            .mem
            .program_b(adr + 1, ((val >> 8) & 0xFF) as u8);
        self.mems[region_idx]
            .mem
            .program_b(adr + 2, ((val >> 16) & 0xFF) as u8);
        self.mems[region_idx]
            .mem
            .program_b(adr + 3, ((val >> 24) & 0xFF) as u8);
        Ok(())
    }

    pub fn write_b(&mut self, adr: u32, val: u8) -> Result<Value, anyhow::Error> {
        let region_idx = self.find_mem_region(adr, 1)?;
        self.mems[region_idx].mem.write_b(adr, val)?;
        Ok(Value::memory_value(adr, 1, val as u32))
    }

    pub fn write_h(&mut self, adr: u32, val: u16) -> Result<Value, anyhow::Error> {
        let region_idx = self.find_mem_region(adr, 2)?;
        self.mems[region_idx].mem.write_h(adr, val)?;
        Ok(Value::memory_value(adr, 2, val as u32))
    }

    pub fn write_w(&mut self, adr: u32, val: u32) -> Result<Value, anyhow::Error> {
        let region_idx = self.find_mem_region(adr, 4)?;
        self.mems[region_idx].mem.write_w(adr, val)?;
        Ok(Value::memory_value(adr, 4, val))
    }

    pub fn read_b(&mut self, adr: u32) -> Result<Value, anyhow::Error> {
        let region_idx = self.find_mem_region(adr, 1)?;
        let val = self.mems[region_idx].mem.read_b(adr)?;
        Ok(Value::memory_value(adr, 1, val as u32))
    }

    pub fn read_h(&mut self, adr: u32) -> Result<Value, anyhow::Error> {
        let region_idx = self.find_mem_region(adr, 2)?;
        let val = self.mems[region_idx].mem.read_h(adr)?;
        Ok(Value::memory_value(adr, 2, val as u32))
    }

    pub fn read_w(&mut self, adr: u32) -> Result<Value, anyhow::Error> {
        let region_idx = self.find_mem_region(adr, 4)?;
        let val = self.mems[region_idx].mem.read_w(adr)?;
        Ok(Value::memory_value(adr, 4, val))
    }

    pub fn read_register(&mut self, reg: Register) -> Result<Value, anyhow::Error> {
        if reg == Register::X0 {
            Ok(Value::register_value(reg, 0))
        } else {
            if let Entry::Vacant { .. } = self.core_reg.entry(reg) {
                let val = match self.config.reg_init {
                    ValueInit::Random => rand::thread_rng().gen(),
                    ValueInit::Zero => 0,
                    ValueInit::Ones => 0xFFFFFFFF,
                    ValueInit::FixedByte(b) => b as u32,
                    ValueInit::FixedWord(w) => w,
                    ValueInit::Error => {
                        return Err(anyhow!("Attempted to read uninitialized register {reg:?}."))
                    }
                };
                self.core_reg.insert(reg, val);
            }
            Ok(Value::register_value(reg, self.core_reg[&reg]))
        }
    }

    pub fn write_register(&mut self, reg: Register, val: u32) -> Value {
        if reg != Register::X0 {
            self.core_reg.insert(reg, val);
        }
        Value::register_value(reg, val)
    }

    pub fn load_elf(&mut self, file: PathBuf) -> Result<(), anyhow::Error> {
        let file_data = std::fs::read(file)?;
        let file = ElfBytes::<LittleEndian>::minimal_parse(file_data.as_slice())?;

        if let Some(header_table) = file.section_headers() {
            for header in header_table.into_iter() {
                if header.sh_type == 0x1 {
                    // sh_type == 0x1 indicates 'SHT_PROGBITS'/Program data
                    let (section_data, _) = file.section_data(&header)?;
                    for (offset, byte) in section_data.iter().enumerate() {
                        self.program_b((header.sh_addr + (offset as u64)).try_into()?, *byte)?;
                    }
                }
            }
        }
        Ok(())
    }
}
