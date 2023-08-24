#include "tests.hpp"
#include "apply_request_sequence.hpp"
#include <cstdlib>
#include <numeric>

void test_00_simple_read(Xbar_TB *tb) {
  tb->tb_info("Test 00 - Simple Read...");

  // Perform a single read from every controller every port:
  unsigned int delays[4] = {1, 2, 3, 4};
  RequestSequence seq[3] = {RequestSequence(), RequestSequence(), RequestSequence()};
  for (unsigned int i = 0; i < 50; i++) {
    seq[0].push_back(std::nullopt);
    seq[1].push_back(std::nullopt);
    seq[2].push_back(std::nullopt);
  }
  seq[0][10] = {
      .addr = (0x0) << (PARAM_WORD_ADDR_WIDTH - 2),
      .is_write = false,
  };
  seq[1][20] = {
      .addr = (0x1) << (PARAM_WORD_ADDR_WIDTH - 2),
      .is_write = false,
  };
  seq[2][30] = {
      .addr = (0x2) << (PARAM_WORD_ADDR_WIDTH - 2),
      .is_write = false,
  };
  apply_request_sequence(tb, delays, 100, seq);
  tb->tb_info("Done.");
}

void test_01_simple_write(Xbar_TB *tb) {
  tb->tb_info("Test 01 - Simple Write...");

  // Perform a single read from every controller every port:
  unsigned int delays[4] = {0, 0, 0, 0};
  RequestSequence seq[3] = {RequestSequence(), RequestSequence(), RequestSequence()};
  for (unsigned int i = 0; i < 30; i++) {
    seq[1].push_back(std::nullopt);
    seq[2].push_back(std::nullopt);
  }
  seq[1][10] = {
      .addr = (0x1) << (PARAM_WORD_ADDR_WIDTH - 2),
      .is_write = true,
      .be = 0xF,
      .wdata = 0xABABABAB,
  };
  seq[1][11] = {
      .addr = (0x1) << (PARAM_WORD_ADDR_WIDTH - 2),
      .is_write = false,
  };

  seq[2][20] = {
      .addr = (0x3) << (PARAM_WORD_ADDR_WIDTH - 2),
      .is_write = true,
      .be = 0x3,
      .wdata = 0xABABABAB,
  };
  seq[2][21] = {
      .addr = (0x3) << (PARAM_WORD_ADDR_WIDTH - 2),
      .is_write = false,
  };
  apply_request_sequence(tb, delays, 50, seq);
  tb->tb_info("Done.");
}

void test_02_concurrent_read(Xbar_TB *tb) {
  tb->tb_info("Test 02 - Concurrent Read...");

  for (unsigned int delay = 0; delay < 3; delay++) {

    unsigned int delays[4] = {delay, delay, delay, delay};
    RequestSequence seq[3] = {RequestSequence(), RequestSequence(), RequestSequence()};
    for (unsigned int i = 0; i < 10; i++) {
      seq[0].push_back(std::nullopt);
      seq[1].push_back(std::nullopt);
      seq[2].push_back(std::nullopt);
    }
    for (unsigned int c = 0; c < 3; c++) {
      seq[c][0] = {
          .addr = (0x1) << (PARAM_WORD_ADDR_WIDTH - 2),
          .is_write = false,
      };
    }
    apply_request_sequence(tb, delays, 50, seq);
  }

  tb->tb_info("Done.");
}

void test_03_variable_delay_fuzz(Xbar_TB *tb) {
  tb->tb_info("Test 03 - Variable Delay Fuzz...");

  const unsigned int REQUEST_COUNT = 150;

  for (unsigned int p1_delay = 0; p1_delay < 3; p1_delay++) {
    for (unsigned int p2_delay = 0; p2_delay < 3; p2_delay++) {
      for (unsigned int p3_delay = 0; p3_delay < 3; p3_delay++) {
        for (unsigned int p4_delay = 0; p4_delay < 3; p4_delay++) {
          unsigned int delays[4] = {p1_delay, p2_delay, p3_delay, p4_delay};

          tb->tb_dbg("%d/%d/%d/%d", delays[0], delays[1], delays[2], delays[3]);

          // Generate random request sequence:
          RequestSequence seq[3] = {RequestSequence(), RequestSequence(), RequestSequence()};
          std::srand(42);
          for (unsigned int i = 0; i < REQUEST_COUNT; i++) {
            for (unsigned int port = 0; port < 3; port++) {
              if (std::rand() % 2) {
                Request req = {};
                req.addr = std::rand();

                if (port != 0 && ((req.addr & (0x2 << (PARAM_WORD_ADDR_WIDTH - 2))) != 0)) {
                  req.is_write = std::rand() % 2;
                  req.be = std::rand() % 0x10;
                  req.be = 0;
                  req.wdata = std::rand();
                }

                seq[port].push_back(req);
              } else {
                seq[port].push_back(std::nullopt);
              }
            }
          }

          unsigned int max_delay = std::accumulate(delays, delays + 4, 0) + 2;
          unsigned int max_iters = (max_delay * REQUEST_COUNT * 120) / 100;

          // Apply random request sequence:
          apply_request_sequence(tb, delays, max_iters, seq);
        }
      }
    }
  }

  tb->tb_info("Done.");
}
