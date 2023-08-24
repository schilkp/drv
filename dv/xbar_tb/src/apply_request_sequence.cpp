#include "apply_request_sequence.hpp"

#include <numeric>

void apply_request_sequence(Xbar_TB *tb, unsigned int p_delay[4], unsigned int iteration_limit,
                            RequestSequence sequences[3]) {

  // Set desired delays and reset request inputs and xbar:
  tb->dut->p1_delay = p_delay[0];
  tb->dut->p2_delay = p_delay[1];
  tb->dut->p3_delay = p_delay[2];
  tb->dut->p4_delay = p_delay[3];
  for (unsigned int i = 1; i <= 3; i++) {
    *(tb->controller_port(i).req) = 0;
  }
  tb->reset();

  // Track expected memory values:
  // Init values & memory size defined in mock_memory.sv
  // Memory wraps within one port's address space!
  // mem[n+4] == mem[n]
  uint32_t mem_expected[4][4] = {0}; // [port][adr]
  for (int peripheral = 0; peripheral < 4; peripheral++) {
    mem_expected[peripheral][0] = 0xDEADBEEF;
    mem_expected[peripheral][1] = 0xF1BEF1BE;
    mem_expected[peripheral][2] = 0x1234ABCD;
    mem_expected[peripheral][3] = 0xFFFFFFFF;
  }

  // Perform iterations:
  unsigned int iterations = 0;
  bool request_active[3] = {false, false, false};
  bool series_finished[3] = {false, false, false};

  while (1) {
    iterations++;
    if (iterations >= iteration_limit) {
      tb->tb_err("Iteration limit reached without completing test sequence!");
      break;
    }

    // ==== Stimulus Application =====
    tb->step_to_stim_application();

    // Apply next request from test sequence to port without active requests:
    for (unsigned int port_idx = 1; port_idx <= 3; port_idx++) {
      Xbar_Port port = tb->controller_port(port_idx);

      if (request_active[port_idx - 1]) {
        continue;
      }

      if (sequences[port_idx - 1].empty()) {
        series_finished[port_idx - 1] = true;
        continue;
      }

      std::optional<Request> &next_request = sequences[port_idx - 1].front();

      if (next_request.has_value()) {
        request_active[port_idx - 1] = true;
        *(port.req) = 1;
        *(port.addr) = next_request->addr;
        if (port.wen != 0) {
          if (next_request->is_write) {
            *(port.wen) = 1;
            *(port.wdata) = next_request->wdata;
            *(port.be) = next_request->be;
          } else {
            *(port.wen) = 0;
          }
        }
      } else {
        request_active[port_idx - 1] = false;
        *(port.req) = 0;
      }

      sequences[port_idx - 1].pop_front();
    }

    // ==== Response Acquisition =====
    tb->step_to_resp_acquisition();

    // Basic response assertions:
    for (unsigned int port = 1; port <= 3; port++) {
      if (*(tb->controller_port(port).req) == 0) {
        tb->tb_assert(*(tb->controller_port(port).ready) == 0,
                      "Port %u: Ready asserted althought no request is present!", port);
      }
    }

    // Validate finished requests & mark as completed:
    for (unsigned int port_idx = 1; port_idx <= 3; port_idx++) {
      Xbar_Port port = tb->controller_port(port_idx);

      if (*(port.req) && *(port.ready)) {

        // Request finished. Clear:
        request_active[port_idx - 1] = false;

        if (port.wen != 0 && *(port.wen) == 1) {
          // Completed request was a write. Update expected memory content:
          uint32_t mem_was = mem_expected[port_idx - 1][*(port.addr) % 4];
          uint32_t mem_is = 0;
          mem_is |= (*(port.be) & 0x1 ? *(port.wdata) : mem_was) & 0x000000FF;
          mem_is |= (*(port.be) & 0x2 ? *(port.wdata) : mem_was) & 0x0000FF00;
          mem_is |= (*(port.be) & 0x4 ? *(port.wdata) : mem_was) & 0x00FF0000;
          mem_is |= (*(port.be) & 0x8 ? *(port.wdata) : mem_was) & 0xFF000000;

          uint32_t peripheral = (*(port.addr) & (0x3 << (PARAM_WORD_ADDR_WIDTH - 2))) >> (PARAM_WORD_ADDR_WIDTH - 2);

          mem_expected[peripheral][*(port.addr) % 4] = mem_is;
        } else {
          // Completed request was a read. Validate read value:
          uint32_t peripheral = (*(port.addr) & (0x3 << (PARAM_WORD_ADDR_WIDTH - 2))) >> (PARAM_WORD_ADDR_WIDTH - 2);
          uint32_t read_expected = mem_expected[peripheral][*(port.addr) % 4];
          uint32_t read_is = *(port.rdata);
          tb->tb_assert(read_is == read_expected,
                        "Port %i: Incorrect read. Expected 0x%x, read 0x%x!", port_idx,
                        read_expected, read_is);
        }
      }
    }
    int sum = std::accumulate(series_finished, series_finished + 3, 0);
    if (sum == 3) {
      // All requests in series finished.
      break;
    }
  }
}
