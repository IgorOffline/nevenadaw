#include <GLFW/glfw3.h>

#include <iostream>
#include <toml++/toml.hpp>

int glfw_graphics();

static void key_callback(GLFWwindow* window, int key, int scancode, int action,
                         int mods);

int main() {
  std::cout << "<START>" << std::endl;

  try {
    const toml::table tbl = toml::parse_file(
        R"(C:\igoroffline\nevenadaw\__practice__\hopeternal\config\main.toml)");
    std::cout << tbl << std::endl;
  } catch (const toml::parse_error& err) {
    std::cerr << "Parsing failed:" << std::endl << err << std::endl;
    return EXIT_FAILURE;
  }

  const int graphics = glfw_graphics();

  std::cout << "graphics: " << graphics << " <END>" << std::endl;

  return EXIT_SUCCESS;
}

int glfw_graphics() {
  if (!glfwInit()) return EXIT_FAILURE;

  glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE);

  GLFWwindow* window =
      glfwCreateWindow(1280, 720, "hopeternal 0.1.0", NULL, NULL);

  if (!window) {
    glfwTerminate();
    return EXIT_FAILURE;
  }

  glfwMakeContextCurrent(window);

  glfwSetKeyCallback(window, key_callback);

  glClearColor(0.1294f, 0.1294f, 0.1294f, 1.0f);

  while (!glfwWindowShouldClose(window)) {
    glClear(GL_COLOR_BUFFER_BIT);
    glfwSwapBuffers(window);
    glfwPollEvents();
  }

  glfwTerminate();

  return EXIT_SUCCESS;
}

static void key_callback([[maybe_unused]] GLFWwindow* window, int key,
                         [[maybe_unused]] int scancode, int action,
                         [[maybe_unused]] int mods) {
  if (action == GLFW_PRESS) {
    if (key == GLFW_KEY_F) {
      std::cout << "[F]" << std::endl;
    }
  }
}