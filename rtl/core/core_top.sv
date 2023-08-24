module core_top #(
    parameter logic [31:0] BOOT_ADDR,
    parameter int unsigned WORD_ADDR_WIDTH
) (
    input logic clk_i,
    input logic rst_ni,

    // Control:
    input logic dbg_req_i,

    // Instruction fetch interface:
    output logic                  ifetch_req_o,
    output logic [ADDR_WIDTH-1:0] ifetch_addr_o,
    input  logic [          31:0] ifetch_rdata_i,
    input  logic                  ifetch_ready_i,

    // LSU interface:
    output logic                  lsu_req_o,
    output logic [ADDR_WIDTH-1:0] lsu_addr_o,
    output logic                  lsu_wen_o,
    output logic [          31:0] lsu_wdata_o,
    output logic [           3:0] lsu_be_o,
    input  logic [          31:0] lsu_rdata_i,
    input  logic                  lsu_ready_i

);


  // TODO:
  assign ifetch_req_o = 'b0;
  assign ifetch_addr_o = 'b0;
  assign lsu_req_o = 'b0;
  assign lsu_addr_o = 'b0;
  assign lsu_wen_o = 'b0;
  assign lsu_wdata_o = 'b0;
  assign lsu_be_o = 'b0;

endmodule
