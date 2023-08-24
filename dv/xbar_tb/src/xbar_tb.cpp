#include <iostream>
#include <stdarg.h>

#include "tests.hpp"
#include "xbar_tb.hpp"

int main(int argc, char **argv) {
  Xbar_TB *tb = new Xbar_TB(argc, argv);

  test_00_simple_read(tb);
  test_01_simple_write(tb);
  test_02_concurrent_read(tb);
  test_03_variable_delay_fuzz(tb);

  std::cout << "Testbench finished." << std::endl;
  tb->finish_trace();

  unsigned int err_cnt = tb->err_cnt;
  delete tb;

  if (tb->err_cnt == 0) {
    std::cout << "\u001b[1m\u001b[48;5;28mOK.\u001b[0m" << std::endl;
    return 0;
  } else {
    std::cout << "\u001b[1m\u001b[41;1m" << tb->err_cnt << " errors!\u001b[0m" << std::endl;
    return err_cnt;
  }
}

Xbar_TB::Xbar_TB(int argc, char **argv) {
  Verilated::traceEverOn(true);

  std::cout << "Starting Xbar TB.." << std::endl;

  err_cnt = 0;

  dut = std::unique_ptr<Vxbar_tb>(new Vxbar_tb);

  context = std::unique_ptr<VerilatedContext>(new VerilatedContext);
  context->commandArgs(argc, argv);

  tfp = new VerilatedVcdC();
  dut->trace(tfp, 99);

  std::cout << "Tracing to build/trace.vcd.." << std::endl;
  tfp->open("build/trace.vcd");

  dut->eval_step();
  tfp->dump(context->time());

  context->timeInc(1);
}

Xbar_TB::~Xbar_TB() { delete tfp; }

void Xbar_TB::finish_trace() {
  tfp->close();
  std::cout << "Trace saved." << std::endl;
}

Xbar_Port Xbar_TB::controller_port(unsigned int n) {
  switch (n) {
  case 1:
    return Xbar_Port{
        .req = &(dut->c1_req_i),
        .ready = &(dut->c1_ready_o),
        .addr = &(dut->c1_addr_i),
        .rdata = &(dut->c1_rdata_o),
    };
  case 2:
    return Xbar_Port{
        .req = &(dut->c2_req_i),
        .wen = &(dut->c2_wen_i),
        .be = &(dut->c2_be_i),
        .ready = &(dut->c2_ready_o),
        .addr = &(dut->c2_addr_i),
        .rdata = &(dut->c2_rdata_o),
        .wdata = &(dut->c2_wdata_i),
    };
  case 3:
    return Xbar_Port{
        .req = &(dut->c3_req_i),
        .wen = &(dut->c3_wen_i),
        .be = &(dut->c3_be_i),
        .ready = &(dut->c3_ready_o),
        .addr = &(dut->c3_addr_i),
        .rdata = &(dut->c3_rdata_o),
        .wdata = &(dut->c3_wdata_i),
    };
  default:
    throw std::string("Illegal controller port!");
  }
}

void Xbar_TB::step_clock(unsigned int n) {
  for (unsigned int i = 0; i < n; i++) {
    if (dut->clk_i == 1) {
      step_to_resp_acquisition();
      step_to_stim_application();
    } else {
      step_to_stim_application();
      step_to_resp_acquisition();
    }
  }
}

void Xbar_TB::step_to_stim_application() {

  if (dut->clk_i == 1) {
    // Already at stimulus application, advance to
    // response acquisition, then to next stimulus
    // application.
    step_to_resp_acquisition();
  }

  dut->eval_step();
  tfp->dump(context->time());

  context->timeInc(1);

  dut->clk_i = 1;

  dut->eval_step();
  tfp->dump(context->time());

  context->timeInc(1);
}

void Xbar_TB::step_to_resp_acquisition() {

  if (dut->clk_i == 0) {
    // Already at response acquisition, advance to
    // stimulus application, then to next response
    // acquisition.
    step_to_stim_application();
  }

  dut->eval_step();
  tfp->dump(context->time());

  context->timeInc(4);

  dut->clk_i = 0;

  dut->eval_step();
  tfp->dump(context->time());

  context->timeInc(4);
}

void Xbar_TB::reset() {
  step_to_stim_application();
  dut->rst_ni = 0;
  step_clock(10);
  dut->rst_ni = 1;
  step_clock(10);
}

void Xbar_TB::tb_dbg(char const *fmt, ...) {
#if DBG_LOG_EN == 1
  va_list a;
  va_start(a, fmt);
  printf("[%08lu] \u001b[1m\u001b[48;5;11mDBG\u001b[0m: ", context->time());
  vprintf(fmt, a);
  printf("\n");
  va_end(a);
#endif /* DBG_LOG_EN */
}

void Xbar_TB::tb_err(char const *fmt, ...) {
  va_list a;
  va_start(a, fmt);
  err_cnt++;
  printf("[%08lu] \u001b[1m\u001b[41;1mERR\u001b[0m: ", context->time());
  vprintf(fmt, a);
  printf("\n");
  va_end(a);
}

void Xbar_TB::tb_assert(bool condition, char const *fmt, ...) {
  va_list a;
  va_start(a, fmt);

  if (!condition) {
    err_cnt++;
    printf("[%08lu] \u001b[1m\u001b[41;1mERR\u001b[0m: ", context->time());
    vprintf(fmt, a);
    printf("\n");
  }

  va_end(a);
}

void Xbar_TB::tb_info(char const *fmt, ...) {
  va_list a;
  va_start(a, fmt);
  printf("[%08lu] \u001b[1m\u001b[48;5;28mINF\u001b[0m: ", context->time());
  vprintf(fmt, a);
  printf("\n");
  va_end(a);
}
