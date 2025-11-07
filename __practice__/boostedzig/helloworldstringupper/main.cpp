#include <iostream>
#include <string>
#include <boost/algorithm/string.hpp>
#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>

int main() {
  boost::uuids::random_generator generator;
  const auto uuid = generator();
  const auto uuid_str = boost::uuids::to_string(uuid);
  std::cout << uuid_str << std::endl;
  std::string text_to_upper = "Hello, To Upper";
  boost::to_upper(text_to_upper);
  std::cout << text_to_upper << std::endl;
  std::string text_to_lower = "Hello, To Lower";
  boost::to_lower(text_to_lower);
  std::cout << text_to_lower << std::endl;
  std::string text_trim = "   [ TRIM ]       ";
  boost::trim(text_trim);
  std::cout << text_trim << std::endl;
  std::string text_trimdot = "...[ TRIMDOT ]...";
  boost::trim_if(text_trimdot, [](char c) { return c == '.'; });
  std::cout << text_trimdot << std::endl;
  std::string text_replace = "Sometimes we lorem and sometimes we ipsum";
  boost::replace_all(text_replace, "lorem", "temp");
  boost::replace_all(text_replace, "ipsum", "lorem");
  boost::replace_all(text_replace, "temp", "ipsum");
  std::cout << text_replace << std::endl;
  return 0;
}
