module xbar_priority_selector (
    input logic clk_i,
    input logic rst_ni,

    input logic [2:0] available_requests_i,

    input logic p_ready_i,

    output logic [2:0] selected_request_o
);


  logic request_active_d, request_active_q;

  // Counter selecting which input currently has the highest
  // priority:
  logic [1:0] priority_cnt_d, priority_cnt_q;

  always_comb begin
    priority_cnt_d = priority_cnt_q;
    if (request_active_q == 1'b0) begin
      priority_cnt_d = priority_cnt_q == 2'h2 ? 2'b0 : priority_cnt_q + 1;
    end
  end

  always_ff @(posedge clk_i, negedge rst_ni) begin
    if (!rst_ni) begin
      priority_cnt_q <= 'b0;
    end else begin
      priority_cnt_q <= priority_cnt_d;
    end
  end

  // Determine the currently highest priority request:
  logic [2:0] priority_request;
  always_comb begin
    priority_request = 'b0;

    if (priority_cnt_q == 2'b00) begin
      priority_request = available_requests_i[0] ? 3'b001 :
                         available_requests_i[1] ? 3'b010 :
                         available_requests_i[2] ? 3'b100 :
                         3'b000;
    end else if (priority_cnt_q == 2'b01) begin
      priority_request = available_requests_i[1] ? 3'b010 :
                         available_requests_i[2] ? 3'b100 :
                         available_requests_i[0] ? 3'b001 :
                         3'b000;
    end else begin
      priority_request = available_requests_i[2] ? 3'b100 :
                         available_requests_i[0] ? 3'b001 :
                         available_requests_i[1] ? 3'b010 :
                         3'b000;
    end
  end

  // Lock the selected request until it is completed:
  logic [2:0] selected_request_d, selected_request_q;

  always_comb begin
    selected_request_d = selected_request_q;
    selected_request_o = priority_request;
    request_active_d   = request_active_q;

    if (!request_active_q) begin
      // No Request active:
      // Reset locked request & present any new request:
      selected_request_d = priority_request;
      selected_request_o = priority_request;

      if (|priority_request && !p_ready_i) begin
        // New request present that was not immediatly completed.
        request_active_d = 1'b1;
      end
    end else begin
      // Request active:
      // Preserve selected request:
      selected_request_d = selected_request_q;
      selected_request_o = selected_request_q;

      if (p_ready_i) begin
        request_active_d = 1'b0;
      end
    end
  end

  always_ff @(posedge clk_i, negedge rst_ni) begin
    if (!rst_ni) begin
      selected_request_q <= 'b0;
      request_active_q   <= 'b0;
    end else begin
      selected_request_q <= selected_request_d;
      request_active_q   <= request_active_d;
    end
  end

endmodule
