use std::path::PathBuf;

use drv_isa_sim::*;
use insta::assert_debug_snapshot;

// Note: Must match input elf file linker script!
const ROM_START: u32 = 0x1000000;
const RAM_START: u32 = 0x2000000;

fn new_simulator(elf_file: PathBuf) -> DRVSim {
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
                init: ValueInit::Error,
                region_type: MemoryRegionType::RAM,
            },
        ],
        reg_init: ValueInit::Error,
    });
    sim.load_elf(elf_file).unwrap();
    sim
}

#[test]
fn run_01_jumps_and_adds() {
    let mut sim = new_simulator("testdata/01_jumps_and_adds.elf".into());
    let mut log = vec![];
    for _ in 0..9 {
        log.push(sim.step().unwrap().to_log_string());
    }
    assert_debug_snapshot!(log);
}

#[test]
fn run_02_mem_access() {
    let mut sim = new_simulator("testdata/02_mem_access.elf".into());
    let mut log = vec![];
    for _ in 0..20 {
        log.push(sim.step().unwrap().to_log_string());
    }
    assert_debug_snapshot!(log);
}

#[test]
fn run_03_branching() {
    let mut sim = new_simulator("testdata/03_branching.elf".into());
    let mut log = vec![];
    for _ in 0..24 {
        log.push(sim.step().unwrap().to_log_string());
    }
    assert_debug_snapshot!(log);
}

#[test]
fn run_04_call_return() {
    let mut sim = new_simulator("testdata/04_call_return.elf".into());
    let mut log = vec![];
    for _ in 0..15 {
        log.push(sim.step().unwrap().to_log_string());
    }
    assert_debug_snapshot!(log);
}
