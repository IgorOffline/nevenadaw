#include <GLFW/glfw3.h>

#include <iostream>
#include <toml++/toml.hpp>

#include "hopeternal_primitives.h"

hopeternal_int glfw_graphics();

static void key_callback([[maybe_unused]] GLFWwindow* window,
                         hopeternal_int key,
                         [[maybe_unused]] hopeternal_int scancode,
                         hopeternal_int action,
                         [[maybe_unused]] hopeternal_int mods);

hopeternal_int main() {
  hopeternal_cout << "<START>" << hopeternal_endl;

  try {
    const toml::table tbl = toml::parse_file(
        R"(C:\igoroffline\nevenadaw\__practice__\hopeternal\config\main.toml)");
    hopeternal_cout << tbl << hopeternal_endl;
  } catch (const toml::parse_error& err) {
    std::cerr << "Parsing failed:" << hopeternal_endl << err << hopeternal_endl;
    return EXIT_FAILURE;
  }

  const hopeternal_int graphics = glfw_graphics();

  hopeternal_cout << "graphics: " << graphics << " <END>" << hopeternal_endl;

  return EXIT_SUCCESS;
}

hopeternal_int glfw_graphics() {
  if (!glfwInit()) return EXIT_FAILURE;

  glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE);

  constexpr hopeternal_int WINDOW_WIDTH = 1280;
  constexpr hopeternal_int WINDOW_HEIGHT = 720;

  GLFWwindow* window = glfwCreateWindow(WINDOW_WIDTH, WINDOW_HEIGHT,
                                        "hopeternal 0.1.0", NULL, NULL);

  if (!window) {
    glfwTerminate();
    return EXIT_FAILURE;
  }

  glfwMakeContextCurrent(window);

  glfwSetKeyCallback(window, key_callback);

  glViewport(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);

  glMatrixMode(GL_PROJECTION);
  glLoadIdentity();

  glOrtho(0.0, WINDOW_WIDTH, WINDOW_HEIGHT, 0.0, -1.0, 1.0);
  glMatrixMode(GL_MODELVIEW);
  glLoadIdentity();

  glClearColor(0.1294f, 0.1294f, 0.1294f, 1.0f);

  while (!glfwWindowShouldClose(window)) {
    glClear(GL_COLOR_BUFFER_BIT);
    glColor3f(0.0f, 0.0f, 1.0f);
    constexpr hopeternal_int x = 200;
    constexpr hopeternal_int y = 150;
    constexpr hopeternal_int w = 50;
    constexpr hopeternal_int h = 50;
    glBegin(GL_QUADS);
    glVertex2i(x, y);
    glVertex2i(x + w, y);
    glVertex2i(x + w, y + h);
    glVertex2i(x, y + h);
    glEnd();
    glfwSwapBuffers(window);
    glfwPollEvents();
  }

  glfwTerminate();

  return EXIT_SUCCESS;
}

static void key_callback([[maybe_unused]] GLFWwindow* window,
                         const hopeternal_int key,
                         [[maybe_unused]] hopeternal_int scancode,
                         const hopeternal_int action,
                         [[maybe_unused]] hopeternal_int mods) {
  if (action == GLFW_PRESS) {
    if (key == GLFW_KEY_F) {
      hopeternal_cout << "[F]" << hopeternal_endl;
    }
  }
}