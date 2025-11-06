#include <iostream>
#include "json.hpp"
#include "nihil.hpp"

using json = nlohmann::json;

int main() {
  std::cout << __IGOROFFLINE_AND_AMIDST_THE_DARKEST_OF_TIMES_HEARTS_AGLOW_WITH_NIHIL__ << std::endl;
  json j = {
    {"maybe", -1}
  };
  const int maybe = j["maybe"];
  std::cout << "maybe: " << maybe << std::endl;
  std::cout << "json" << std::endl << std::setw(4) << j << std::endl;
  std::cout << __IGOROFFLINE_AND_AMIDST_THE_DARKEST_OF_TIMES_HEARTS_AGLOW_WITH_NIHIL__ << std::endl;
  return 0;
}
