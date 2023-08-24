#ifndef XBAR_TB_H_
#define XBAR_TB_H_

#include "Vxbar_tb.h"
#include "verilated_vcd_c.h"

struct Xbar_Port {
  CData *req;
  CData *wen;
  CData *be;
  CData *ready;
  IData *addr;
  IData *rdata;
  IData *wdata;
};

class Xbar_TB {
public:
  // Verilator:
  std::unique_ptr<VerilatedContext> context;
  std::unique_ptr<Vxbar_tb> dut;
  VerilatedVcdC *tfp;

  // Errors:
  unsigned int err_cnt;

  Xbar_TB(int argc, char **argv);
  ~Xbar_TB();
  void finish_trace();

  void tb_dbg(char const *fmt, ...);
  void tb_info(char const *fmt, ...);
  void tb_err(char const *fmt, ...);
  void tb_assert(bool condition, char const *fmt, ...);

  void step_clock(unsigned int n = 1);
  void step_to_stim_application();
  void step_to_resp_acquisition();
  void reset();

  Xbar_Port controller_port(unsigned int n);
};

#endif /* XBAR_TB_H_ */
