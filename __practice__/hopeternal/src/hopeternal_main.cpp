#include <GLFW/glfw3.h>

#include <toml++/toml.hpp>

#include "hopeternal_primitives.h"

static hopeternal_int glfw_graphics();

static void key_callback([[maybe_unused]] GLFWwindow* window,
                         hopeternal_int key,
                         [[maybe_unused]] hopeternal_int scancode,
                         hopeternal_int action,
                         [[maybe_unused]] hopeternal_int mods);

static hopeternal_int process_config();

static hopeternal_int parse_toml_str_to_int(const std::string& input);

hopeternal_float lastFrameTime = 0.F;
hopeternal_float deltaTime = 0.F;

hopeternal_rectangle rect{0, 0, 1, 1};
hopeternal_float rect_x_pos_f = 0.0F;
hopeternal_float rect_y_pos_f = 0.0F;

hopeternal_int main() {
  hopeternal_cout << hopeternal_start_message << hopeternal_endl;

  const hopeternal_int graphics = glfw_graphics();

  hopeternal_cout << hopeternal_graphics_end_message << graphics
                  << hopeternal_post_graphics_end_message << hopeternal_endl;

  return HOPETERNAL_EXIT_SUCCESS;
}

static hopeternal_int glfw_graphics() {
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

  rect_x_pos_f = static_cast<hopeternal_float>(rect.x);
  rect_y_pos_f = static_cast<hopeternal_float>(rect.y);

  lastFrameTime = static_cast<float>(glfwGetTime());
  while (!glfwWindowShouldClose(window)) {
    const auto currentFrameTime = static_cast<float>(glfwGetTime());
    deltaTime = currentFrameTime - lastFrameTime;
    lastFrameTime = currentFrameTime;

    //
    // INPUT, UPDATE
    //
    constexpr hopeternal_float RECT_SPEED = 420.F;
    rect_x_pos_f += deltaTime * RECT_SPEED;
    rect.x = static_cast<hopeternal_int>(rect_x_pos_f);
    rect.y = static_cast<hopeternal_int>(rect_y_pos_f);

    //
    // RENDER
    //
    glClear(GL_COLOR_BUFFER_BIT);
    glColor3f(HOPETERNAL_GRAPHICS_ZERO_F, HOPETERNAL_GRAPHICS_ZERO_F,
              HOPETERNAL_GRAPHICS_COLOR_ALPHA);
    glBegin(GL_QUADS);
    glVertex2i(rect.x, rect.y);
    glVertex2i(rect.x + rect.width, rect.y);
    glVertex2i(rect.x + rect.width, rect.y + rect.height);
    glVertex2i(rect.x, rect.y + rect.height);
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
      process_config();

      hopeternal_cout << hopeternal_key_f_message << hopeternal_endl;
    }
  }
}

static hopeternal_int process_config() {
  try {
    const toml::table tbl = toml::parse_file(hopeternal_main_toml_location);
    hopeternal_cout << tbl << hopeternal_endl;
    const auto new_x_raw = tbl["root"]["rectangle"]["x"].as_string()->get();
    const auto new_x = parse_toml_str_to_int(new_x_raw);
    const auto new_y_raw = tbl["root"]["rectangle"]["y"].as_string()->get();
    const auto new_y = parse_toml_str_to_int(new_y_raw);
    const auto new_width_raw = tbl["root"]["rectangle"]["w"].as_string()->get();
    const auto new_width = parse_toml_str_to_int(new_width_raw);
    const auto new_height_raw =
        tbl["root"]["rectangle"]["h"].as_string()->get();
    const auto new_height = parse_toml_str_to_int(new_height_raw);
    rect = hopeternal_rectangle{new_x, new_y, new_width, new_height};

    rect_x_pos_f = static_cast<hopeternal_float>(rect.x);
    rect_y_pos_f = static_cast<hopeternal_float>(rect.y);

  } catch (const toml::parse_error& err) {
    hopeternal_cerr << hopeternal_parsing_error_message << hopeternal_endl
                    << err << hopeternal_endl;
    return HOPETERNAL_EXIT_FAILURE;
  }
  return HOPETERNAL_EXIT_SUCCESS;
}

static hopeternal_int parse_toml_str_to_int(const std::string& input) {
  try {
    return std::stoi(input);
  } catch ([[maybe_unused]] const std::invalid_argument& e) {
    hopeternal_cerr << "Error: Invalid argument (not a number)."
                    << hopeternal_endl;
  } catch ([[maybe_unused]] const std::out_of_range& e) {
    hopeternal_cerr << "Error: Value out of range for int32."
                    << hopeternal_endl;
  }

  return -1;
}