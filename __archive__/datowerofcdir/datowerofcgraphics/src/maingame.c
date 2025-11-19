#include <SDL3/SDL.h>
#include <stdlib.h>

#ifdef SDL_PROP_RENDERER_CREATE_NAME_STRING
#undef SDL_PROP_RENDERER_CREATE_NAME_STRING
#define SDL_PROP_RENDERER_CREATE_NAME_STRING "Da Tower of C Graphics"
#endif  // SDL_PROP_RENDERER_CREATE_NAME_STRING

#define IGOROFFLINE_SCREEN_WIDTH 1280
#define IGOROFFLINE_SCREEN_HEIGHT 720
#define IGOROFFLINE_SDL_SUCCESS 0

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

  int running = 1;
  SDL_Event event;

  float circle_x = IGOROFFLINE_SCREEN_WIDTH / 2.0f;
  float circle_y = IGOROFFLINE_SCREEN_HEIGHT / 2.0f;
  const float circle_speed_modifier = 6.45f;
  float circle_speed_x = 3.0f * circle_speed_modifier;
  float circle_speed_y = 2.0f * circle_speed_modifier;

  while (running) {
    const float circle_radius = 50.0f;
    while (SDL_PollEvent(&event)) {
      switch (event.type) {
        case SDL_EVENT_QUIT:
          running = 0;
          break;
        case SDL_EVENT_KEY_DOWN:
          if (event.key.key == SDLK_ESCAPE) {
            running = 0;
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

    SDL_SetRenderDrawColor(renderer, 0x00, 0x00, 0x00, 0xFF);
    SDL_RenderClear(renderer);

    SDL_SetRenderDrawColor(renderer, 0xFF, 0x00, 0x00, 0xFF);

    for (int angle = 0; angle < 360; angle++) {
      const float angle_f = (float)angle;
      const float rad = angle_f * 3.14159f / 180.0f;
      const float px = circle_x + circle_radius * SDL_cosf(rad);
      const float py = circle_y + circle_radius * SDL_sinf(rad);
      SDL_RenderPoint(renderer, px, py);
    }

    SDL_RenderPresent(renderer);

    SDL_Delay(24);
  }

  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);
  SDL_Quit();
  return EXIT_SUCCESS;
}