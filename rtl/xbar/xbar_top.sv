module xbar_top #(
    parameter int unsigned WORD_ADDR_WIDTH
) (
    input logic clk_i,
    input logic rst_ni,

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
    output logic                       c3_ready_o,

    // Peripheral 1 - ROM:
    output logic                       p1_req_o,
    output logic [WORD_ADDR_WIDTH-3:0] p1_addr_o,
    input  logic [               31:0] p1_rdata_i,
    input  logic                       p1_ready_i,

    // Peripheral 2 - Any:
    output logic                       p2_req_o,
    output logic [WORD_ADDR_WIDTH-3:0] p2_addr_o,
    output logic                       p2_wen_o,
    output logic [               31:0] p2_wdata_o,
    output logic [                3:0] p2_be_o,
    input  logic [               31:0] p2_rdata_i,
    input  logic                       p2_ready_i,

    // Peripheral 3 - Any:
    output logic                       p3_req_o,
    output logic [WORD_ADDR_WIDTH-3:0] p3_addr_o,
    output logic                       p3_wen_o,
    output logic [               31:0] p3_wdata_o,
    output logic [                3:0] p3_be_o,
    input  logic [               31:0] p3_rdata_i,
    input  logic                       p3_ready_i,

    // Peripheral 4 - Any:
    output logic                       p4_req_o,
    output logic [WORD_ADDR_WIDTH-3:0] p4_addr_o,
    output logic                       p4_wen_o,
    output logic [               31:0] p4_wdata_o,
    output logic [                3:0] p4_be_o,
    input  logic [               31:0] p4_rdata_i,
    input  logic                       p4_ready_i

);

  logic [ 3:0] c1_requested_port;
  logic [ 3:0] c1_ready_and_selected;
  logic [ 3:0] c2_requested_port;
  logic [ 3:0] c2_ready_and_selected;
  logic [ 3:0] c3_requested_port;
  logic [ 3:0] c3_ready_and_selected;

  logic [31:0] p1_rdata;
  logic [31:0] p2_rdata;
  logic [31:0] p3_rdata;
  logic [31:0] p4_rdata;

  // ==== Controller Ports =====================================================

  xbar_controller_port #(
      .WORD_ADDR_WIDTH(WORD_ADDR_WIDTH)
  ) controller_c1_i (
      // Controller Port:
      .c_req_i(c1_req_i),
      .c_addr_i(c1_addr_i),
      .c_wen_i('b0),
      .c_wdata_i('b0),
      .c_be_i('b0),
      .c_rdata_o(c1_rdata_o),
      .c_ready_o(c1_ready_o),

      // One-hot encoded requested peripheral port:
      .port_requested_o(c1_requested_port),

      // Peripheral ready & this controller port selected:
      .p_ready_and_selected_i(c1_ready_and_selected),

      // Peripheral read data
      .p1_rdata_i(p1_rdata_i),
      .p2_rdata_i(p2_rdata_i),
      .p3_rdata_i(p3_rdata_i),
      .p4_rdata_i(p4_rdata_i)

  );

  xbar_controller_port #(
      .WORD_ADDR_WIDTH(WORD_ADDR_WIDTH)
  ) controller_c2_i (
      // Controller Port:
      .c_req_i(c2_req_i),
      .c_addr_i(c2_addr_i),
      .c_wen_i(c2_wen_i),
      .c_wdata_i(c2_wdata_i),
      .c_be_i(c2_be_i),
      .c_rdata_o(c2_rdata_o),
      .c_ready_o(c2_ready_o),

      // One-hot encoded requested peripheral port:
      .port_requested_o(c2_requested_port),

      // Peripheral ready & this controller port selected:
      .p_ready_and_selected_i(c2_ready_and_selected),

      // Peripheral read data
      .p1_rdata_i(p1_rdata_i),
      .p2_rdata_i(p2_rdata_i),
      .p3_rdata_i(p3_rdata_i),
      .p4_rdata_i(p4_rdata_i)

  );

  xbar_controller_port #(
      .WORD_ADDR_WIDTH(WORD_ADDR_WIDTH)
  ) controller_c3_i (
      // Controller Port:
      .c_req_i(c3_req_i),
      .c_addr_i(c3_addr_i),
      .c_wen_i(c3_wen_i),
      .c_wdata_i(c3_wdata_i),
      .c_be_i(c3_be_i),
      .c_rdata_o(c3_rdata_o),
      .c_ready_o(c3_ready_o),

      // One-hot encoded requested peripheral port:
      .port_requested_o(c3_requested_port),

      // Peripheral ready & this controller port selected:
      .p_ready_and_selected_i(c3_ready_and_selected),

      // Peripheral read data
      .p1_rdata_i(p1_rdata_i),
      .p2_rdata_i(p2_rdata_i),
      .p3_rdata_i(p3_rdata_i),
      .p4_rdata_i(p4_rdata_i)

  );

  // ==== Peripheral Ports =====================================================

  xbar_peripheral_port #(
      .WORD_ADDR_WIDTH(WORD_ADDR_WIDTH)
  ) peripheral_port_p1_i (
      .clk_i (clk_i),
      .rst_ni(rst_ni),

      // Controller 1:
      .c1_matching_req_i(c1_requested_port[0]),
      .c1_addr_i(c1_addr_i),
      .c1_ready_and_selected_o(c1_ready_and_selected[0]),

      // Controller 2:
      .c2_matching_req_i(c2_requested_port[0]),
      .c2_addr_i(c2_addr_i),
      .c2_wen_i(c2_wen_i),
      .c2_wdata_i(c2_wdata_i),
      .c2_be_i(c2_be_i),
      .c2_ready_and_selected_o(c2_ready_and_selected[0]),

      // Controller 3:
      .c3_matching_req_i(c3_requested_port[0]),
      .c3_addr_i(c3_addr_i),
      .c3_wen_i(c3_wen_i),
      .c3_wdata_i(c3_wdata_i),
      .c3_be_i(c3_be_i),
      .c3_ready_and_selected_o(c3_ready_and_selected[0]),

      // Peripheral Port:
      .p_req_o(p1_req_o),
      .p_addr_o(p1_addr_o),
      .p_wen_o(),
      .p_wdata_o(),
      .p_be_o(),
      .p_ready_i(p1_ready_i)
  );

  xbar_peripheral_port #(
      .WORD_ADDR_WIDTH(WORD_ADDR_WIDTH)
  ) peripheral_port_p2_i (
      .clk_i (clk_i),
      .rst_ni(rst_ni),

      // Controller 1:
      .c1_matching_req_i(c1_requested_port[1]),
      .c1_addr_i(c1_addr_i),
      .c1_ready_and_selected_o(c1_ready_and_selected[1]),

      // Controller 2:
      .c2_matching_req_i(c2_requested_port[1]),
      .c2_addr_i(c2_addr_i),
      .c2_wen_i(c2_wen_i),
      .c2_wdata_i(c2_wdata_i),
      .c2_be_i(c2_be_i),
      .c2_ready_and_selected_o(c2_ready_and_selected[1]),

      // Controller 3:
      .c3_matching_req_i(c3_requested_port[1]),
      .c3_addr_i(c3_addr_i),
      .c3_wen_i(c3_wen_i),
      .c3_wdata_i(c3_wdata_i),
      .c3_be_i(c3_be_i),
      .c3_ready_and_selected_o(c3_ready_and_selected[1]),

      // Peripheral Port:
      .p_req_o(p2_req_o),
      .p_addr_o(p2_addr_o),
      .p_wen_o(p2_wen_o),
      .p_wdata_o(p2_wdata_o),
      .p_be_o(p2_be_o),
      .p_ready_i(p2_ready_i)
  );

  xbar_peripheral_port #(
      .WORD_ADDR_WIDTH(WORD_ADDR_WIDTH)
  ) peripheral_port_p3_i (
      .clk_i (clk_i),
      .rst_ni(rst_ni),

      // Controller 1:
      .c1_matching_req_i(c1_requested_port[2]),
      .c1_addr_i(c1_addr_i),
      .c1_ready_and_selected_o(c1_ready_and_selected[2]),

      // Controller 2:
      .c2_matching_req_i(c2_requested_port[2]),
      .c2_addr_i(c2_addr_i),
      .c2_wen_i(c2_wen_i),
      .c2_wdata_i(c2_wdata_i),
      .c2_be_i(c2_be_i),
      .c2_ready_and_selected_o(c2_ready_and_selected[2]),

      // Controller 3:
      .c3_matching_req_i(c3_requested_port[2]),
      .c3_addr_i(c3_addr_i),
      .c3_wen_i(c3_wen_i),
      .c3_wdata_i(c3_wdata_i),
      .c3_be_i(c3_be_i),
      .c3_ready_and_selected_o(c3_ready_and_selected[2]),

      // Peripheral Port:
      .p_req_o(p3_req_o),
      .p_addr_o(p3_addr_o),
      .p_wen_o(p3_wen_o),
      .p_wdata_o(p3_wdata_o),
      .p_be_o(p3_be_o),
      .p_ready_i(p3_ready_i)
  );

  xbar_peripheral_port #(
      .WORD_ADDR_WIDTH(WORD_ADDR_WIDTH)
  ) peripheral_port_p4_i (
      .clk_i (clk_i),
      .rst_ni(rst_ni),

      // Controller 1:
      .c1_matching_req_i(c1_requested_port[3]),
      .c1_addr_i(c1_addr_i),
      .c1_ready_and_selected_o(c1_ready_and_selected[3]),

      // Controller 2:
      .c2_matching_req_i(c2_requested_port[3]),
      .c2_addr_i(c2_addr_i),
      .c2_wen_i(c2_wen_i),
      .c2_wdata_i(c2_wdata_i),
      .c2_be_i(c2_be_i),
      .c2_ready_and_selected_o(c2_ready_and_selected[3]),

      // Controller 3:
      .c3_matching_req_i(c3_requested_port[3]),
      .c3_addr_i(c3_addr_i),
      .c3_wen_i(c3_wen_i),
      .c3_wdata_i(c3_wdata_i),
      .c3_be_i(c3_be_i),
      .c3_ready_and_selected_o(c3_ready_and_selected[3]),

      // Peripheral Port:
      .p_req_o(p4_req_o),
      .p_addr_o(p4_addr_o),
      .p_wen_o(p4_wen_o),
      .p_wdata_o(p4_wdata_o),
      .p_be_o(p4_be_o),
      .p_ready_i(p4_ready_i)
  );

endmodule
