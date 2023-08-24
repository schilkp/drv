#ifndef TESTS_H_
#define TESTS_H_

#include "xbar_tb.hpp"

void test_00_simple_read(Xbar_TB *tb);
void test_01_simple_write(Xbar_TB *tb);
void test_02_concurrent_read(Xbar_TB *tb);
void test_03_variable_delay_fuzz(Xbar_TB *tb);

#endif /* TESTS_H_ */
