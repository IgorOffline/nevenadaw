#include <GLFW/glfw3.h>

#include <iostream>
#include <toml++/toml.hpp>

int graphics();

int main() {
  std::cout << "<START>" << std::endl;

  try {
    const toml::table tbl = toml::parse_file(
        R"(C:\igoroffline\nevenadaw\__practice__\otokoislandia\config\main.toml)");
    std::cout << tbl << std::endl;
  } catch (const toml::parse_error& err) {
    std::cerr << "Parsing failed:" << std::endl << err << std::endl;

    return EXIT_FAILURE;
  }

  const auto graphics_status = graphics();

  std::cout << "<END>" << graphics_status << std::endl;

  return EXIT_SUCCESS;
}

int graphics() {
  if (!glfwInit()) return EXIT_FAILURE;

  GLFWwindow* window = glfwCreateWindow(1280, 720, "otokoislandia", NULL, NULL);

  if (!window) {
    glfwTerminate();
    return EXIT_FAILURE;
  }

  glfwMakeContextCurrent(window);

  while (!glfwWindowShouldClose(window)) {
    glClear(GL_COLOR_BUFFER_BIT);
    glfwSwapBuffers(window);
    glfwPollEvents();
  }

  glfwTerminate();

  return EXIT_SUCCESS;
}