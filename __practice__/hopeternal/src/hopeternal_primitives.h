#ifndef HOPETERNAL_HOPETERNAL_PRIMITIVES_H
#define HOPETERNAL_HOPETERNAL_PRIMITIVES_H

using hopeternal_int = std::int32_t;
using hopeternal_float = float;
using hopeternal_double = double;
inline std::ostream& hopeternal_cout = std::cout;
using hopeternal_endl_manipulator = std::ostream& (*)(std::ostream&);
constexpr hopeternal_endl_manipulator hopeternal_endl = std::endl;

inline constexpr hopeternal_int HOPETERNAL_EXIT_SUCCESS = 0;
inline constexpr hopeternal_int HOPETERNAL_EXIT_FAILURE = 1;
inline constexpr hopeternal_int HOPETERNAL_GRAPHICS_ZERO = 0;
inline constexpr hopeternal_float HOPETERNAL_GRAPHICS_ZERO_F = 0.F;
inline constexpr hopeternal_double HOPETERNAL_GRAPHICS_ZERO_D = 0.0;
inline constexpr hopeternal_double HOPETERNAL_GRAPHICS_Z_NEAR = -1.0;
inline constexpr hopeternal_double HOPETERNAL_GRAPHICS_Z_FAR = 1.0;
inline constexpr hopeternal_int HOPETERNAL_GRAPHICS_WINDOW_WIDTH = 1280;
inline constexpr hopeternal_int HOPETERNAL_GRAPHICS_WINDOW_HEIGHT = 720;
inline constexpr hopeternal_float HOPETERNAL_GRAPHICS_COLOR_ALPHA = 1.0F;
inline constexpr hopeternal_float HOPETERNAL_GRAPHICS_CLEAR_COLOR = 0.1294F;

inline constexpr const char* hopeternal_window_title = "hopeternal 0.1.0";
inline constexpr const char* hopeternal_start_message = "<START>";
inline constexpr const char* hopeternal_main_toml_location =
    R"(C:\igoroffline\nevenadaw\__practice__\hopeternal\config\main.toml)";
inline constexpr const char* hopeternal_parsing_error_message =
    "Parsing failed:";
inline constexpr const char* hopeternal_graphics_end_message = "graphics: ";
inline constexpr const char* hopeternal_post_graphics_end_message = " <END>";
inline constexpr const char* hopeternal_key_f_message = "[F]";

#endif  // HOPETERNAL_HOPETERNAL_PRIMITIVES_H
