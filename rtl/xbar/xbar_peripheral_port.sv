module xbar_peripheral_port #(
    parameter int unsigned WORD_ADDR_WIDTH
) (
    input logic clk_i,
    input logic rst_ni,

    // Controller 1:
    input  logic                       c1_matching_req_i,
    input  logic [WORD_ADDR_WIDTH-1:0] c1_addr_i,
    output logic                       c1_ready_and_selected_o,

    // Controller 2:
    input  logic                       c2_matching_req_i,
    input  logic [WORD_ADDR_WIDTH-1:0] c2_addr_i,
    input  logic                       c2_wen_i,
    input  logic [               31:0] c2_wdata_i,
    input  logic [                3:0] c2_be_i,
    output logic                       c2_ready_and_selected_o,

    // Controller 3:
    input  logic                       c3_matching_req_i,
    input  logic [WORD_ADDR_WIDTH-1:0] c3_addr_i,
    input  logic                       c3_wen_i,
    input  logic [               31:0] c3_wdata_i,
    input  logic [                3:0] c3_be_i,
    output logic                       c3_ready_and_selected_o,

    // Peripheral Port:
    output logic                       p_req_o,
    output logic [WORD_ADDR_WIDTH-3:0] p_addr_o,
    output logic                       p_wen_o,
    output logic [               31:0] p_wdata_o,
    output logic [                3:0] p_be_o,
    input  logic                       p_ready_i
);


  // Rotating priority request selector:
  logic [2:0] selected_request;

  xbar_priority_selector priority_selector_i (
      .clk_i(clk_i),
      .rst_ni(rst_ni),
      .available_requests_i({c3_matching_req_i, c2_matching_req_i, c1_matching_req_i}),
      .p_ready_i(p_ready_i),
      .selected_request_o(selected_request)
  );

  // Multiplexing:
  always_comb begin
    c1_ready_and_selected_o = 'b0;
    c2_ready_and_selected_o = 'b0;
    c3_ready_and_selected_o = 'b0;
    case (selected_request)
      3'b001: begin
        p_req_o = 1'b1;
        p_addr_o = c1_addr_i[WORD_ADDR_WIDTH-3:0];
        p_wen_o = 'b0;
        p_wdata_o = 'b0;
        p_be_o = 'b0;
        c1_ready_and_selected_o = p_ready_i;
      end
      3'b010: begin
        p_req_o = 1'b1;
        p_addr_o = c2_addr_i[WORD_ADDR_WIDTH-3:0];
        p_wen_o = c2_wen_i;
        p_wdata_o = c2_wdata_i;
        p_be_o = c2_be_i;
        c2_ready_and_selected_o = p_ready_i;
      end
      3'b100: begin
        p_req_o = 1'b1;
        p_addr_o = c3_addr_i[WORD_ADDR_WIDTH-3:0];
        p_wen_o = c3_wen_i;
        p_wdata_o = c3_wdata_i;
        p_be_o = c3_be_i;
        c3_ready_and_selected_o = p_ready_i;
      end
      default: begin
        p_req_o = 1'b0;
        p_addr_o = 'b0;
        p_wen_o = 'b0;
        p_wdata_o = 'b0;
        p_be_o = 'b0;
      end
    endcase
  end

endmodule
