use anyhow::anyhow;
use std::collections::HashMap;

use rand::Rng;

use crate::ValueInit;

// ==== Type/Constant Definitions ==================================================================

const BLOCK_SIZE: u32 = 0x100;

pub struct Memory {
    pub start_adr: u32,
    write_protected: bool,
    init: ValueInit,
    mem: HashMap<u32, [Option<u8>; BLOCK_SIZE as usize]>,
}

// ==== Memory Implementation ======================================================================

impl Memory {
    pub fn new(start_adr: u32, init: ValueInit, write_protected: bool) -> Memory {
        Memory {
            start_adr,
            write_protected,
            init,
            mem: HashMap::new(),
        }
    }

    pub fn read_b(&mut self, adr: u32) -> Result<u8, anyhow::Error> {
        assert!(adr >= self.start_adr);

        let adr_abs = adr;
        let adr = adr - self.start_adr;
        let block = adr / BLOCK_SIZE;

        self.mem.entry(block).or_insert([None; BLOCK_SIZE as usize]);

        if let Some(val) = self.mem[&block][(adr % BLOCK_SIZE) as usize] {
            Ok(val)
        } else {
            let init_val: u8 = match self.init {
                ValueInit::Random => rand::thread_rng().gen(),
                ValueInit::Zero => 0,
                ValueInit::Ones => 0xFF,
                ValueInit::FixedByte(b) => b,
                ValueInit::FixedWord(w) => ((w >> (8 * (adr_abs % 4))) & 0xff).try_into().unwrap(),
                ValueInit::Error => {
                    return Err(anyhow!(
                        "Attempted to read uninitialized memory at 0x{adr:x}."
                    ))
                }
            };

            self.mem.get_mut(&block).unwrap()[(adr % BLOCK_SIZE) as usize] = None;

            Ok(init_val)
        }
    }

    pub fn read_h(&mut self, adr: u32) -> Result<u16, anyhow::Error> {
        let b0 = self.read_b(adr)? as u16;
        let b1 = self.read_b(adr + 1)? as u16;
        Ok((b1 << 8) | (b0))
    }

    pub fn read_w(&mut self, adr: u32) -> Result<u32, anyhow::Error> {
        let b0 = self.read_b(adr)? as u32;
        let b1 = self.read_b(adr + 1)? as u32;
        let b2 = self.read_b(adr + 2)? as u32;
        let b3 = self.read_b(adr + 3)? as u32;
        Ok((b3 << 24) | (b2 << 16) | (b1 << 8) | (b0))
    }

    pub fn program_b(&mut self, adr: u32, val: u8) {
        assert!(adr >= self.start_adr);

        let adr = adr - self.start_adr;

        let block = adr / BLOCK_SIZE;

        self.mem.entry(block).or_insert([None; BLOCK_SIZE as usize]);

        self.mem.get_mut(&block).unwrap()[(adr % BLOCK_SIZE) as usize] = Some(val);
    }

    pub fn write_b(&mut self, adr: u32, val: u8) -> Result<(), anyhow::Error> {
        if self.write_protected {
            return Err(anyhow!("Attempted to write to read-only memory!"));
        }

        self.program_b(adr, val);
        Ok(())
    }

    pub fn write_h(&mut self, adr: u32, val: u16) -> Result<(), anyhow::Error> {
        self.write_b(adr, (val & 0xff).try_into().unwrap())?;
        self.write_b(adr + 1, ((val >> 8) & 0xff).try_into().unwrap())?;
        Ok(())
    }

    pub fn write_w(&mut self, adr: u32, val: u32) -> Result<(), anyhow::Error> {
        self.write_b(adr, (val & 0xff).try_into().unwrap())?;
        self.write_b(adr + 1, ((val >> 8) & 0xff).try_into().unwrap())?;
        self.write_b(adr + 2, ((val >> 16) & 0xff).try_into().unwrap())?;
        self.write_b(adr + 3, ((val >> 24) & 0xff).try_into().unwrap())?;
        Ok(())
    }
}

// ==== Memory Tests ===============================================================================

#[cfg(test)]
mod tests {
    use crate::memory::*;

    #[test]
    fn memory_write_read() {
        let mut mem = Memory::new(0xABC, ValueInit::Error, false);
        mem.write_w(0xABC, 0xA1B2C3D4).unwrap();
        assert_eq!(mem.read_b(0xABC).unwrap(), 0xD4);
        assert_eq!(mem.read_b(0xABC + 1).unwrap(), 0xC3);
        assert_eq!(mem.read_b(0xABC + 2).unwrap(), 0xB2);
        assert_eq!(mem.read_b(0xABC + 3).unwrap(), 0xA1);

        mem.write_w(5 * BLOCK_SIZE + 0xABC, 0xDEADBEEF).unwrap();
        assert_eq!(mem.read_h(5 * BLOCK_SIZE + 0xABC + 0).unwrap(), 0xBEEF);
        assert_eq!(mem.read_h(5 * BLOCK_SIZE + 0xABC + 2).unwrap(), 0xDEAD);

        mem.write_w(BLOCK_SIZE - 2 + 0xABC, 0xF1BE0102).unwrap();
        assert_eq!(mem.read_w(BLOCK_SIZE - 2 + 0xABC).unwrap(), 0xF1BE0102);
    }

    #[test]
    #[should_panic]
    fn memory_acc_before_start() {
        let mut mem = Memory::new(0xABC, ValueInit::Error, false);
        mem.write_w(0xABC - 1, 0).unwrap();
    }

    #[test]
    #[should_panic]
    fn memory_acc_uninit_error() {
        let mut mem = Memory::new(0xABC, ValueInit::Error, false);
        let _ = mem.read_b(0xABC).unwrap();
    }

    #[test]
    #[should_panic]
    fn memory_write_protection() {
        let mut mem = Memory::new(0xABC, ValueInit::Error, true);
        mem.write_b(0xABC, 0x0).unwrap();
    }

    #[test]
    fn memory_acc_fixed() {
        let mut mem = Memory::new(0, ValueInit::Zero, false);
        assert_eq!(mem.read_b(0).unwrap(), 0);

        let mut mem = Memory::new(0, ValueInit::Ones, false);
        assert_eq!(mem.read_b(0).unwrap(), 0xFF);

        let mut mem = Memory::new(0, ValueInit::FixedByte(0xAB), false);
        assert_eq!(mem.read_b(0).unwrap(), 0xAB);
        assert_eq!(mem.read_b(1).unwrap(), 0xAB);
        assert_eq!(mem.read_b(2).unwrap(), 0xAB);
        assert_eq!(mem.read_b(3).unwrap(), 0xAB);

        let mut mem = Memory::new(0x2, ValueInit::FixedWord(0xDEADBEEF), false);
        assert_eq!(mem.read_b(0x4).unwrap(), 0xEF);
        assert_eq!(mem.read_b(0x5).unwrap(), 0xBE);
        assert_eq!(mem.read_b(0x6).unwrap(), 0xAD);
        assert_eq!(mem.read_b(0x7).unwrap(), 0xDE);
    }
}
