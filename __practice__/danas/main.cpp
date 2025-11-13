#define SDL_MAIN_HANDLED

#include <iostream>
#include <toml++/toml.hpp>

int main(int argc, char* argv[]) {
  (void)argv;
  if (argc > 99) {
    try {
      toml::table config = toml::parse_file("danas.toml");
      const std::string_view lorem1 =
          config["root"]["lorem1"].value_or("LOREM1_ERR");
      const std::string_view lorem2 =
          config["root"]["lorem2"].value_or("LOREM2_ERR");
      std::cout << lorem1 << std::endl << lorem2 << std::endl;

    } catch (const toml::parse_error& err) {
      std::cerr << "danas.toml parsing error: " << err.what() << std::endl;

      return EXIT_FAILURE;
    }
  }

  return EXIT_SUCCESS;
}
