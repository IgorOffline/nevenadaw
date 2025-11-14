#include <iostream>
#include <toml++/toml.hpp>

int main() {
  std::cout << "<START>" << std::endl;

  try {
    const toml::table tbl = toml::parse_file(
        R"(C:\igoroffline\nevenadaw\__archive__\otokoislandia\config\main.toml)");
    std::cout << tbl << std::endl;
  } catch (const toml::parse_error& err) {
    std::cerr << "Parsing failed:" << std::endl << err << std::endl;

    return EXIT_FAILURE;
  }

  return EXIT_SUCCESS;
}