DRV_REPO_ROOT := justfile_directory()

_default: help

# Run all unit and integration tests, and all DV testbenches.
test_all: drv_isa_sim_test xbar_tb

# Run the DRV-ISA-SIM tests.
drv_isa_sim_test:
    cd drv_isa_sim && cargo test

# Run the XBAR testbench.
xbar_tb target="all":
    cd dv/xbar_tb && make {{ target }} DRV_REPO_ROOT="{{ DRV_REPO_ROOT }}" 

# Print help message.
help:
    @echo "DRV Project Runner."
    @just --list
