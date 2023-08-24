module xbar_controller_port #(
    parameter int unsigned WORD_ADDR_WIDTH
) (
    // Controller Port:
    input  logic                       c_req_i,
    input  logic [WORD_ADDR_WIDTH-1:0] c_addr_i,
    input  logic                       c_wen_i,
    input  logic [               31:0] c_wdata_i,
    input  logic [                3:0] c_be_i,
    output logic [               31:0] c_rdata_o,
    output logic                       c_ready_o,

    // One-hot encoded requested peripheral port:
    output logic [3:0] port_requested_o,

    // Peripheral ready & this controller port selected:
    input logic [3:0] p_ready_and_selected_i,

    // Peripheral read data
    input logic [31:0] p1_rdata_i,
    input logic [31:0] p2_rdata_i,
    input logic [31:0] p3_rdata_i,
    input logic [31:0] p4_rdata_i

);

  assign c_ready_o = |p_ready_and_selected_i;

  always_comb begin
    port_requested_o = 'b0;
    c_rdata_o = 'b0;

    if (c_addr_i[WORD_ADDR_WIDTH-1:WORD_ADDR_WIDTH-2] == 2'b00) begin
      port_requested_o = c_req_i == 1'b1 ? 4'b0001 : 'b0;
      c_rdata_o = p1_rdata_i;
    end else if (c_addr_i[WORD_ADDR_WIDTH-1:WORD_ADDR_WIDTH-2] == 2'b01) begin
      port_requested_o = c_req_i == 1'b1 ? 4'b0010 : 'b0;
      c_rdata_o = p2_rdata_i;
    end else if (c_addr_i[WORD_ADDR_WIDTH-1:WORD_ADDR_WIDTH-2] == 2'b10) begin
      port_requested_o = c_req_i == 1'b1 ? 4'b0100 : 'b0;
      c_rdata_o = p3_rdata_i;
    end else if (c_addr_i[WORD_ADDR_WIDTH-1:WORD_ADDR_WIDTH-2] == 2'b11) begin
      port_requested_o = c_req_i == 1'b1 ? 4'b1000 : 'b0;
      c_rdata_o = p4_rdata_i;
    end
  end

endmodule

