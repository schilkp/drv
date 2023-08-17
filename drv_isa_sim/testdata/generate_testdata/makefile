mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
dir := $(patsubst %/,%,$(dir $(mkfile_path)))
mkfile_location_abs := $(abspath $(dir))

RISCV_UNKNOWN_ELF_PREFIX=riscv64-unknown-elf

ASM_PROGRAMS =00_all_instructions.s
ASM_PROGRAMS+=01_jumps_and_adds.s
ASM_PROGRAMS+=02_mem_access.s
ASM_PROGRAMS+=03_branching.s
ASM_PROGRAMS+=04_call_return.s

ELFS = $(addprefix $(mkfile_location_abs)/../, $(ASM_PROGRAMS:.s=.elf))

.PHONY: all
all: $(ELFS)

.PHONY: clean
clean:
	rm $(ELFS)

$(mkfile_location_abs)/../%.elf: $(mkfile_location_abs)/%.s
	$(RISCV_UNKNOWN_ELF_PREFIX)-gcc -T$(mkfile_location_abs)/link.ld -nostdlib -march=rv32e -mabi=ilp32e -Wl,--print-gc-sections -lgcc -flto $< -o $@
