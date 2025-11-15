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
  hopeternal_cout << hopeternal_start_message << hopeternal_endl;

  try {
    const toml::table tbl = toml::parse_file(hopeternal_main_toml_location);
    hopeternal_cout << tbl << hopeternal_endl;
  } catch (const toml::parse_error& err) {
    std::cerr << hopeternal_parsing_error_message << hopeternal_endl << err
              << hopeternal_endl;
    return HOPETERNAL_EXIT_FAILURE;
  }

  const hopeternal_int graphics = glfw_graphics();

  hopeternal_cout << hopeternal_graphics_end_message << graphics
                  << hopeternal_post_graphics_end_message << hopeternal_endl;

  return HOPETERNAL_EXIT_SUCCESS;
}

hopeternal_int glfw_graphics() {
  if (!glfwInit()) return HOPETERNAL_EXIT_FAILURE;

  glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE);

  constexpr hopeternal_int WINDOW_WIDTH = HOPETERNAL_GRAPHICS_WINDOW_WIDTH;
  constexpr hopeternal_int WINDOW_HEIGHT = HOPETERNAL_GRAPHICS_WINDOW_HEIGHT;

  GLFWwindow* window = glfwCreateWindow(
      WINDOW_WIDTH, WINDOW_HEIGHT, hopeternal_window_title, nullptr, nullptr);

  if (!window) {
    glfwTerminate();
    return HOPETERNAL_EXIT_FAILURE;
  }

  glfwMakeContextCurrent(window);

  glfwSetKeyCallback(window, key_callback);

  glViewport(HOPETERNAL_GRAPHICS_ZERO, HOPETERNAL_GRAPHICS_ZERO, WINDOW_WIDTH,
             WINDOW_HEIGHT);

  glMatrixMode(GL_PROJECTION);
  glLoadIdentity();

  glOrtho(HOPETERNAL_GRAPHICS_ZERO_D, WINDOW_WIDTH, WINDOW_HEIGHT,
          HOPETERNAL_GRAPHICS_ZERO_D, HOPETERNAL_GRAPHICS_Z_NEAR,
          HOPETERNAL_GRAPHICS_Z_FAR);
  glMatrixMode(GL_MODELVIEW);
  glLoadIdentity();

  glClearColor(HOPETERNAL_GRAPHICS_CLEAR_COLOR, HOPETERNAL_GRAPHICS_CLEAR_COLOR,
               HOPETERNAL_GRAPHICS_CLEAR_COLOR,
               HOPETERNAL_GRAPHICS_COLOR_ALPHA);

  while (!glfwWindowShouldClose(window)) {
    glClear(GL_COLOR_BUFFER_BIT);
    glColor3f(HOPETERNAL_GRAPHICS_ZERO_F, HOPETERNAL_GRAPHICS_ZERO_F,
              HOPETERNAL_GRAPHICS_COLOR_ALPHA);
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

  return HOPETERNAL_EXIT_SUCCESS;
}

static void key_callback([[maybe_unused]] GLFWwindow* window,
                         const hopeternal_int key,
                         [[maybe_unused]] hopeternal_int scancode,
                         const hopeternal_int action,
                         [[maybe_unused]] hopeternal_int mods) {
  if (action == GLFW_PRESS) {
    if (key == GLFW_KEY_F) {
      hopeternal_cout << hopeternal_key_f_message << hopeternal_endl;
    }
  }
}