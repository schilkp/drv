module xbar_tb #(
    parameter int unsigned WORD_ADDR_WIDTH
) (
    input logic clk_i,
    input logic rst_ni,

    input int unsigned p1_delay,
    input int unsigned p2_delay,
    input int unsigned p3_delay,
    input int unsigned p4_delay,

    // Controller 1 - Instruction Fetch:
    input  logic                       c1_req_i,
    input  logic [WORD_ADDR_WIDTH-1:0] c1_addr_i,
    output logic [               31:0] c1_rdata_o,
    output logic                       c1_ready_o,

    // Controller 2 - LSU:
    input  logic                       c2_req_i,
    input  logic [WORD_ADDR_WIDTH-1:0] c2_addr_i,
    input  logic                       c2_wen_i,
    input  logic [               31:0] c2_wdata_i,
    input  logic [                3:0] c2_be_i,
    output logic [               31:0] c2_rdata_o,
    output logic                       c2_ready_o,

    // Controller 3 - Debugger:
    input  logic                       c3_req_i,
    input  logic [WORD_ADDR_WIDTH-1:0] c3_addr_i,
    input  logic                       c3_wen_i,
    input  logic [               31:0] c3_wdata_i,
    input  logic [                3:0] c3_be_i,
    output logic [               31:0] c3_rdata_o,
    output logic                       c3_ready_o
);

  logic p1_req, p2_req, p3_req, p4_req;
  logic [WORD_ADDR_WIDTH-3:0] p1_addr, p2_addr, p3_addr, p4_addr;
  logic p2_wen, p3_wen, p4_wen;
  logic [31:0] p2_wdata, p3_wdata, p4_wdata;
  logic [3:0] p2_be, p3_be, p4_be;
  logic [31:0] p1_rdata, p2_rdata, p3_rdata, p4_rdata;
  logic p1_ready, p2_ready, p3_ready, p4_ready;

  xbar_top #(
      .WORD_ADDR_WIDTH(WORD_ADDR_WIDTH)
  ) xbar_top_i (
      .clk_i (clk_i),
      .rst_ni(rst_ni),

      // Controller 1 - Instruction Fetch:
      .c1_req_i  (c1_req_i),
      .c1_addr_i (c1_addr_i),
      .c1_rdata_o(c1_rdata_o),
      .c1_ready_o(c1_ready_o),

      .c2_req_i(c2_req_i),
      .c2_addr_i(c2_addr_i),
      .c2_wen_i(c2_wen_i),
      .c2_wdata_i(c2_wdata_i),
      .c2_be_i(c2_be_i),
      .c2_rdata_o(c2_rdata_o),
      .c2_ready_o(c2_ready_o),

      .c3_req_i(c3_req_i),
      .c3_addr_i(c3_addr_i),
      .c3_wen_i(c3_wen_i),
      .c3_wdata_i(c3_wdata_i),
      .c3_be_i(c3_be_i),
      .c3_rdata_o(c3_rdata_o),
      .c3_ready_o(c3_ready_o),

      .p1_req_o  (p1_req),
      .p1_addr_o (p1_addr),
      .p1_rdata_i(p1_rdata),
      .p1_ready_i(p1_ready),

      .p2_req_o(p2_req),
      .p2_addr_o(p2_addr),
      .p2_wen_o(p2_wen),
      .p2_wdata_o(p2_wdata),
      .p2_be_o(p2_be),
      .p2_rdata_i(p2_rdata),
      .p2_ready_i(p2_ready),

      .p3_req_o(p3_req),
      .p3_addr_o(p3_addr),
      .p3_wen_o(p3_wen),
      .p3_wdata_o(p3_wdata),
      .p3_be_o(p3_be),
      .p3_rdata_i(p3_rdata),
      .p3_ready_i(p3_ready),

      .p4_req_o(p4_req),
      .p4_addr_o(p4_addr),
      .p4_wen_o(p4_wen),
      .p4_wdata_o(p4_wdata),
      .p4_be_o(p4_be),
      .p4_rdata_i(p4_rdata),
      .p4_ready_i(p4_ready)

  );

  mock_memory #(
      .WORD_ADDR_WIDTH(WORD_ADDR_WIDTH)
  ) mock_memory_p1 (
      .clk_i (clk_i),
      .rst_ni(rst_ni),

      .delay(p1_delay),

      .req_i(p1_req),
      .addr_i(p1_addr),
      .wen_i('b0),
      .wdata_i('b0),
      .be_i('b0),
      .rdata_o(p1_rdata),
      .ready_o(p1_ready)
  );

  mock_memory #(
      .WORD_ADDR_WIDTH(WORD_ADDR_WIDTH)
  ) mock_memory_p2 (
      .clk_i (clk_i),
      .rst_ni(rst_ni),

      .delay(p2_delay),

      .req_i(p2_req),
      .addr_i(p2_addr),
      .wen_i(p2_wen),
      .wdata_i(p2_wdata),
      .be_i(p2_be),
      .rdata_o(p2_rdata),
      .ready_o(p2_ready)
  );

  mock_memory #(
      .WORD_ADDR_WIDTH(WORD_ADDR_WIDTH)
  ) mock_memory_p3 (
      .clk_i (clk_i),
      .rst_ni(rst_ni),

      .delay(p3_delay),

      .req_i(p3_req),
      .addr_i(p3_addr),
      .wen_i(p3_wen),
      .wdata_i(p3_wdata),
      .be_i(p3_be),
      .rdata_o(p3_rdata),
      .ready_o(p3_ready)
  );

  mock_memory #(
      .WORD_ADDR_WIDTH(WORD_ADDR_WIDTH)
  ) mock_memory_p4 (
      .clk_i (clk_i),
      .rst_ni(rst_ni),

      .delay(p4_delay),

      .req_i(p4_req),
      .addr_i(p4_addr),
      .wen_i(p4_wen),
      .wdata_i(p4_wdata),
      .be_i(p4_be),
      .rdata_o(p4_rdata),
      .ready_o(p4_ready)
  );

endmodule
