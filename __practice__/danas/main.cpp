#define SDL_MAIN_HANDLED

#include <iostream>

int main(const int argc, char* argv[]) {
  (void)argv;
  std::cout << "argc: " << argc << std::endl;

  return EXIT_SUCCESS;
}