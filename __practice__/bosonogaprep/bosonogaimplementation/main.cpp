#include <iostream>
#include "Execute.hpp"

int main() {
  std::cout << "<START>" << std::endl;
  Execute::execute();
  std::cout << "<END>" << std::endl;
  return 0;
}
