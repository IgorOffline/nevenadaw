#include <SDL3/SDL.h>
#include <stdlib.h>

#ifdef SDL_PROP_RENDERER_CREATE_NAME_STRING
#undef SDL_PROP_RENDERER_CREATE_NAME_STRING
#define SDL_PROP_RENDERER_CREATE_NAME_STRING "Da Tower of C Graphics"
#endif  // SDL_PROP_RENDERER_CREATE_NAME_STRING

#define IGOROFFLINE_SCREEN_WIDTH 1280
#define IGOROFFLINE_SCREEN_HEIGHT 720
#define IGOROFFLINE_SDL_SUCCESS 0

int main(int argc, char* argv[]) {
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

  SDL_SetRenderDrawColor(renderer, 0x00, 0x00, 0x00, 0xFF);
  SDL_RenderClear(renderer);
  SDL_RenderPresent(renderer);
  SDL_Delay(3200);

  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);
  SDL_Quit();
  return EXIT_SUCCESS;
}