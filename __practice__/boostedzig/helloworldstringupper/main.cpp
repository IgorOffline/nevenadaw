#include <iostream>
#include <string>
#include <boost/algorithm/string.hpp>

int main() {
  std::string text = "Hello, World!";
  boost::to_upper(text);
  std::cout << text << std::endl;
  return 0;
}