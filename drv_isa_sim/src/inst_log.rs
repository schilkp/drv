use crate::inst::{Instruction, Register};

// ==== Type Definitions ===========================================================================

#[derive(Debug, Clone, Copy)]
pub enum ValueOrigin {
    Register(Register),
    Memory { adr: u32, bytes: u32 },
}

#[derive(Debug, Clone, Copy)]
pub struct Value {
    pub origin: ValueOrigin,
    pub val: u32,
}

#[derive(Debug)]
pub struct InstLog {
    pub pc: u32,
    pub inst: Instruction,
    pub handling_trap: bool,
    pub branching: Option<u32>,
    pub debug_mode: bool,
    pub input_values: Vec<Value>,
    pub commit_values: Vec<Value>,
}

// ==== Value Implementation =======================================================================

impl Value {
    pub fn register_value(reg: Register, val: u32) -> Value {
        Value {
            origin: ValueOrigin::Register(reg),
            val,
        }
    }

    pub fn memory_value(adr: u32, bytes: u32, val: u32) -> Value {
        Value {
            origin: ValueOrigin::Memory { adr, bytes },
            val,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.origin {
            ValueOrigin::Register(reg) => write!(f, "{:?} = 0x{:08x}", reg, self.val),
            ValueOrigin::Memory { adr, bytes: 1 } => {
                write!(f, "mem[0x{:02x}] = 0x{:08x}", adr, self.val)
            }
            ValueOrigin::Memory { adr, bytes: 2 } => {
                write!(f, "mem[0x{:04x}] = 0x{:08x}", adr, self.val)
            }
            ValueOrigin::Memory { adr, bytes: 4 } => {
                write!(f, "mem[0x{:08x}] = 0x{:08x}", adr, self.val)
            }
            ValueOrigin::Memory { .. } => panic!(),
        }
    }
}

// ==== InstLog Implementation =====================================================================

impl InstLog {
    pub fn to_log_string(&self) -> String {
        let mut result = String::new();
        result.push_str(format!("0x{:08x}: ", self.pc).as_str());
        result.push_str(
            format!(
                "[{}{}] ",
                if self.handling_trap { "T" } else { " " },
                if self.debug_mode { "D" } else { " " }
            )
            .as_str(),
        );
        result.push_str(format!("{:>25} |", self.inst.to_string()).as_str());
        if let Some(destination) = self.branching {
            result.push_str(format!(" Branching: 0x{:08x}", destination).as_str());
        }
        if !self.input_values.is_empty() {
            result.push_str(" Input: [".to_string().as_str());
            for (idx, input) in self.input_values.iter().enumerate() {
                if idx != 0 {
                    result.push_str(", ".to_string().as_str());
                }
                result.push_str(format!("{:}", input).as_str());
            }
            result.push_str("]".to_string().as_str());
        }
        if !self.commit_values.is_empty() {
            result.push_str(" Commited: [".to_string().as_str());
            for (idx, input) in self.commit_values.iter().enumerate() {
                if idx != 0 {
                    result.push_str(", ".to_string().as_str());
                }
                result.push_str(format!("{:}", input).as_str());
            }
            result.push_str("]".to_string().as_str());
        }

        result
    }
}
