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
    pub branching: bool,
    pub debug_mode: bool,
    pub input_values: Vec<Value>,
    pub commit_values: Vec<Value>,
}

// ==== Quick Value Constructors ===================================================================

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
