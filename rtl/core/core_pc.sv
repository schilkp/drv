module core_pc #(
    parameter logic [31:0] BOOT_ADDR,
    parameter logic [31:0] WORD_ADDR_WIDTH
) (
    input logic clk_i,
    input logic rst_ni,

    input logic ctrl_incr_i,
    input logic ctrl_latch_i,

    input logic [WORD_ADDR_WIDTH-1:0] input_i,

    output logic [WORD_ADDR_WIDTH-1:0] pc_waddr_o,
    output logic [WORD_ADDR_WIDTH-1:0] next_pc_waddr_o
);

  // Register:
  logic [WORD_ADDR_WIDTH-1:0] pc_d, pc_q;

  assign pc_o = pc_q;

  // Calculate next adr if no branch/jump takes place:
  logic [WORD_ADDR_WIDTH-1:0] pc_next_nobranch;
  assign pc_next_nobranch = pc_q + 1;
  assign next_pc_waddr_o  = pc_next_nobranch;

  // Perform operation indicated by ctrl intputs:
  always_comb begin
    pc_d = pc_q;

    if (ctrl_incr_i == 1'b1) begin
      pc_d = pc_next_nobranch;
    end

    if (ctrl_latch_i == 1'b1) begin
      pc_d = input_i[WORD_ADDR_WIDTH-1:0];
    end
  end

  always_ff @(posedge clk_i, negedge rst_ni) begin
    if (!rst_ni) begin
      pc_q <= BOOT_ADDR[WORD_ADDR_WIDTH-1:0];
    end else begin
      pc_q <= pc_d;
    end
  end

endmodule
