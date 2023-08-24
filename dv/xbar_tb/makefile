# Parameters:
PARAM_WORD_ADDR_WIDTH = 18

# Settings:
DBG_LOG_EN=0

# Sources:
SV_SRCS = src/mock_memory.sv 
SV_SRCS+=$(DRV_REPO_ROOT)/rtl/xbar/xbar_priority_selector.sv 
SV_SRCS+=$(DRV_REPO_ROOT)/rtl/xbar/xbar_controller_port.sv 
SV_SRCS+=$(DRV_REPO_ROOT)/rtl/xbar/xbar_peripheral_port.sv 
SV_SRCS+=$(DRV_REPO_ROOT)/rtl/xbar/xbar_top.sv 
SV_SRCS+=src/xbar_tb.sv

CXX_SRCS = src/xbar_tb.cpp 
CXX_SRCS+= src/apply_request_sequence.cpp
CXX_SRCS+= src/tests.cpp

.PHONY: all
all: build/Vxbar_tb
	@echo ""
	@echo ""
	-./build/Vxbar_tb
	@echo ""
	@echo ""

.PHONY: clean
clean:
	-rm -r build/

.PHONY: verilate
verilate: build/Vxbar_tb

.PHONY: view
view: 
	gtkwave build/trace.vcd waves/default.gtkw

.PHONY: debug
debug: build/Vxbar_tb
	gdbserver :12345 build/Vxbar_tb


compile_commands.json: clean
	bear -- make verilate

build/Vxbar_tb: $(SV_SRCS) $(CXX_SRCS) makefile | build
	verilator --cc --build --exe --Mdir build --trace -j 0 \
		-GWORD_ADDR_WIDTH=$(PARAM_WORD_ADDR_WIDTH) \
		-CFLAGS "-g --std=c++17 -O0" \
		-CFLAGS "-DPARAM_WORD_ADDR_WIDTH=$(PARAM_WORD_ADDR_WIDTH)" \
		-CFLAGS "-DDBG_LOG_EN=$(DBG_LOG_EN)" \
		--top-module xbar_tb \
		$(SV_SRCS) $(CXX_SRCS)

build:
	mkdir -p build

ifeq ($(DRV_REPO_ROOT),)
$(error Variable "DRV_REPO_ROOT" is not set!)
endif
