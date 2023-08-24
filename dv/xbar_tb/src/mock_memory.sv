module mock_memory #(
    parameter int unsigned WORD_ADDR_WIDTH
) (
    // System:
    input logic clk_i,
    input logic rst_ni,

    // Verification parameters:
    input logic [31:0] delay,

    // Memory Interface
    input  logic                       req_i,
    input  logic [WORD_ADDR_WIDTH-3:0] addr_i,
    input  logic                       wen_i,
    input  logic [               31:0] wdata_i,
    input  logic [                3:0] be_i,
    output logic [               31:0] rdata_o,
    output logic                       ready_o
);

  logic [31:0] mem_d[4];
  logic [31:0] mem_q[4];

  logic [31:0] delay_cnt_d, delay_cnt_q;


  always_comb begin
    mem_d = mem_q;
    delay_cnt_d = delay_cnt_q;
    rdata_o = 'b0;
    ready_o = 'b0;

    if (req_i == 1'b1 && delay_cnt_q == delay) begin
      // Perform applied request when the req_i line has been asserted for the specified delay:

      ready_o = 1'b1;

      if (wen_i == 1'b1) begin
        // Write request:
        if (be_i[0] == 1'b1) begin
          mem_d[addr_i[1:0]][7:0] = wdata_i[7:0];
        end
        if (be_i[1] == 1'b1) begin
          mem_d[addr_i[1:0]][15:8] = wdata_i[15:8];
        end
        if (be_i[2] == 1'b1) begin
          mem_d[addr_i[1:0]][23:16] = wdata_i[23:16];
        end
        if (be_i[3] == 1'b1) begin
          mem_d[addr_i[1:0]][31:24] = wdata_i[31:24];
        end
      end else begin
        // Read request:
        rdata_o = mem_q[addr_i[1:0]];
      end
    end

    // Count delay:
    if (req_i == 1'b1 && delay_cnt_q == delay) begin
      // Request completed. Reset.
      delay_cnt_d = 'b0;
    end else if (req_i== 1'b1) begin
      // Request present & uncomplete. Count.
      delay_cnt_d = delay_cnt_q + 1;
    end else begin
      // No Request. Reset.
      delay_cnt_d = 'b0;
    end
  end

  always_ff @(posedge clk_i, negedge rst_ni) begin
    if (!rst_ni) begin
      mem_q[0] <= 'hDEADBEEF;
      mem_q[1] <= 'hF1BEF1BE;
      mem_q[2] <= 'h1234ABCD;
      mem_q[3] <= 'hFFFFFFFF;
      delay_cnt_q <= 'b0;
    end else begin
      mem_q[0] <= mem_d[0];
      mem_q[1] <= mem_d[1];
      mem_q[2] <= mem_d[2];
      mem_q[3] <= mem_d[3];
      delay_cnt_q <= delay_cnt_d;
    end
  end

endmodule

