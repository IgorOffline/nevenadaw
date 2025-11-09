#include <iostream>
#include "BosonogaExecute.hpp"
#include "BosonogaPrimitives.hpp"

int main() {
  std::cout << BOSONOGA_START_STRING << std::endl;
  BosonogaExecute::execute();
  std::cout << BOSONOGA_END_STRING << std::endl;
  return 0;
}
