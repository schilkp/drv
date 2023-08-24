module core_imm_gen #(
) (
    input logic [31:0] inst_i,

    input logic ctrl_sel_imm_i_i,
    input logic ctrl_sel_imm_s_i,
    input logic ctrl_sel_imm_b_i,
    input logic ctrl_sel_imm_u_i,
    input logic ctrl_sel_imm_j_i,

    output logic [31:0] imm_o
);

  // Decode Immedate:
  logic [31:0] imm_i;
  logic [31:0] imm_s;
  logic [31:0] imm_b;
  logic [31:0] imm_u;
  logic [31:0] imm_j;

  assign imm_i = {{20{inst_i[31]}}, inst_i[31:20]};
  assign imm_s = {{20{inst_i[31]}}, inst_i[31:25], inst_i[11:7]};
  assign imm_b = {{19{inst_i[31]}}, inst_i[31], inst_i[7], inst_i[30:25], inst_i[11:8], 1'b0};
  assign imm_u = {inst_i[31:12], 12'b0};
  assign imm_j = {{12{inst_i[31]}}, inst_i[19:12], inst_i[20], inst_i[30:21], 1'b0};

  // Select requested immediate value:
  always_comb begin
    if (ctrl_sel_imm_i_i == 1'b1) begin
      imm_o = imm_i;
    end else if (ctrl_sel_imm_s_i == 1'b1) begin
      imm_o = imm_s;
    end else if (ctrl_sel_imm_b_i == 1'b1) begin
      imm_o = imm_b;
    end else if (ctrl_sel_imm_u_i == 1'b1) begin
      imm_o = imm_u;
    end else if (ctrl_sel_imm_j_i == 1'b1) begin
      imm_o = imm_j;
    end else begin
      imm_o = 'b0;
    end
  end

  // TODO: Assert onehot or off
  // TODO-PART: MUX

endmodule
