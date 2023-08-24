#ifndef APPLY_REQUEST_SEQUENCE_H_
#define APPLY_REQUEST_SEQUENCE_H_

#include <deque>
#include <optional>

#include "xbar_tb.hpp"

struct Request {
  uint32_t addr;
  bool is_write;
  uint32_t be;
  uint32_t wdata;
};

typedef std::deque<std::optional<Request>> RequestSequence;

void apply_request_sequence(Xbar_TB *tb, unsigned int p_delay[4], unsigned int iteration_limit,
                              RequestSequence sequences[3]);

#endif /* APPLY_REQUEST_SEQUENCE_H_ */
