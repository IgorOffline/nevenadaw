#include <SDL3/SDL.h>
#include <stdlib.h>

#ifdef SDL_PROP_RENDERER_CREATE_NAME_STRING
#undef SDL_PROP_RENDERER_CREATE_NAME_STRING
#define SDL_PROP_RENDERER_CREATE_NAME_STRING "Da Tower of C Graphics"
#endif  // SDL_PROP_RENDERER_CREATE_NAME_STRING

#define IGOROFFLINE_INT int32_t
#ifdef _Float32
#define IGOROFFLINE_FLOAT _Float32
#else  // _Float32
#define IGOROFFLINE_FLOAT float
#endif  // _Float32
#ifdef _Float64
#define IGOROFFLINE_DOUBLE _Float64
#else  // _Float64
#define IGOROFFLINE_DOUBLE double
#endif  // _Float64
#define IGOROFFLINE_SCREEN_WIDTH 1280
#define IGOROFFLINE_SCREEN_HEIGHT 720
#define IGOROFFLINE_SDL_SUCCESS 0
#define IGOROFFLINE_SDL_NOT_RUNNING 0
#define IGOROFFLINE_SDL_RUNNING 1
#define IGOROFFLINE_SDL_DELAY 24
#define IGOROFFLINE_PI__HEAD__ 245850922.F
#define IGOROFFLINE_PI__TAIL__ 78256779.F
// clang-format off
#define IGOROFFLINE_PI ((IGOROFFLINE_FLOAT)(IGOROFFLINE_PI__HEAD__) / (IGOROFFLINE_FLOAT)(IGOROFFLINE_PI__TAIL__))
// clang-format on
#define IGOROFFLINE_HALF_CIRCLE 180.F
#define IGOROFFLINE_FULL_CIRCLE 360.F

int main(const int argc, const char* argv[]) {
  if (argc > 999) {
    (void)argv;
  }

  SDL_Window* window = NULL;
  SDL_Renderer* renderer = NULL;

  if (SDL_Init(SDL_INIT_VIDEO) < IGOROFFLINE_SDL_SUCCESS) {
    SDL_Log("ERR::100100 %s\n", SDL_GetError());
    return EXIT_FAILURE;
  }

  window = SDL_CreateWindow(SDL_PROP_RENDERER_CREATE_NAME_STRING,
                            IGOROFFLINE_SCREEN_WIDTH, IGOROFFLINE_SCREEN_HEIGHT,
                            IGOROFFLINE_SDL_SUCCESS);

  if (window == NULL) {
    SDL_Log("ERR::100101 %s\n", SDL_GetError());
    SDL_Quit();
    return EXIT_FAILURE;
  }

  const SDL_PropertiesID props = SDL_CreateProperties();
  SDL_SetPointerProperty(props, SDL_PROP_RENDERER_CREATE_WINDOW_POINTER,
                         window);

  renderer = SDL_CreateRendererWithProperties(props);
  if (renderer == NULL) {
    SDL_Log("ERR::100102 %s\n", SDL_GetError());
    SDL_DestroyWindow(window);
    SDL_Quit();
    return EXIT_FAILURE;
  }

  int running = IGOROFFLINE_SDL_RUNNING;
  SDL_Event event;

  float circle_x = IGOROFFLINE_SCREEN_WIDTH / 2.F;
  float circle_y = IGOROFFLINE_SCREEN_HEIGHT / 2.F;
  const float circle_speed_modifier = 7.23F;
  float circle_speed_x = 3.F * circle_speed_modifier;
  float circle_speed_y = 2.F * circle_speed_modifier;

  while (running) {
    const float circle_radius = 50.F;
    while (SDL_PollEvent(&event)) {
      switch (event.type) {
        case SDL_EVENT_QUIT:
          running = IGOROFFLINE_SDL_NOT_RUNNING;
          break;
        case SDL_EVENT_KEY_DOWN:
          if (event.key.key == SDLK_ESCAPE) {
            running = IGOROFFLINE_SDL_NOT_RUNNING;
          }
          break;
        default:
          break;
      }
    }

    circle_x += circle_speed_x;
    circle_y += circle_speed_y;

    if (circle_x - circle_radius < 0 ||
        circle_x + circle_radius > IGOROFFLINE_SCREEN_WIDTH) {
      circle_speed_x = -circle_speed_x;
    }
    if (circle_y - circle_radius < 0 ||
        circle_y + circle_radius > IGOROFFLINE_SCREEN_HEIGHT) {
      circle_speed_y = -circle_speed_y;
    }

    SDL_SetRenderDrawColor(renderer, 0x21, 0x21, 0x21, 0xFF);
    SDL_RenderClear(renderer);
    SDL_SetRenderDrawColor(renderer, 0xBD, 0xBD, 0xBD, 0xFF);

    for (IGOROFFLINE_INT angle = 0;
         angle < (IGOROFFLINE_INT)IGOROFFLINE_FULL_CIRCLE; angle++) {
      const IGOROFFLINE_FLOAT angle_f = (IGOROFFLINE_FLOAT)angle;
      const IGOROFFLINE_FLOAT rad =
          angle_f * IGOROFFLINE_PI / IGOROFFLINE_HALF_CIRCLE;
      const IGOROFFLINE_FLOAT px = circle_x + circle_radius * SDL_cosf(rad);
      const IGOROFFLINE_FLOAT py = circle_y + circle_radius * SDL_sinf(rad);
      SDL_RenderPoint(renderer, px, py);
    }

    SDL_RenderPresent(renderer);

    SDL_Delay(IGOROFFLINE_SDL_DELAY);
  }

  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);
  SDL_Quit();
  return EXIT_SUCCESS;
}