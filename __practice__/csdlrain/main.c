#include <SDL3/SDL.h>
#include <SDL3_ttf/SDL_ttf.h>
#include <bgfx/c99/bgfx.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "platform.h"

#if BX_PLATFORM_WINDOWS == 0
#error "Windows expected"
#endif

int main() {
  SDL_Init(SDL_INIT_VIDEO);
  TTF_Init();

  const int window_width = 1280;
  const int window_height = 720;
  SDL_Window *window = SDL_CreateWindow("SDL/BGFX Rain", window_width, window_height, 0);

  const float font_size = 52.F;
  TTF_Font *font_latin = TTF_OpenFont("C:\\igoroffline\\fonts\\IosevkaTerm-Regular.ttf", font_size);
  TTF_Font *font_hangul = TTF_OpenFont("C:\\igoroffline\\fonts\\NotoSansKR.ttf", font_size);
  TTF_AddFallbackFont(font_latin, font_hangul);

  bgfx_init_t init;
  bgfx_init_ctor(&init);
  init.type = BGFX_RENDERER_TYPE_COUNT;

  bgfx_platform_data_t pd = {0};
#ifdef SDL_PLATFORM_WIN32
  void *hwnd = SDL_GetPointerProperty(SDL_GetWindowProperties(window), SDL_PROP_WINDOW_WIN32_HWND_POINTER, NULL);
  pd.nwh = hwnd;
#else
  pd.nwh = NULL;
#endif
  bgfx_set_platform_data(&pd);

  init.resolution.width = (uint32_t) window_width;
  init.resolution.height = (uint32_t) window_height;
  init.resolution.reset = BGFX_RESET_VSYNC;

  const uint32_t view_width_32 = window_width;
  const uint32_t view_height_32 = window_height;

  bgfx_set_view_clear(0, BGFX_CLEAR_COLOR | BGFX_CLEAR_DEPTH, 0xFF212121, 1.0f, 0);
  bgfx_set_view_rect(0, 0, 0, (uint16_t) view_width_32, (uint16_t) view_height_32);

  SDL_Event e;
  bool quit = false;
  while (!quit) {
    while (SDL_PollEvent(&e)) {
      switch (e.type) {
        case SDL_EVENT_QUIT:
          quit = true;
          break;
        default:
          break;
      }
    }

    bgfx_touch(0);
    bgfx_frame(false);
  }

  bgfx_shutdown();
  if (font_latin) TTF_CloseFont(font_latin);
  if (font_hangul) TTF_CloseFont(font_hangul);
  SDL_DestroyWindow(window);
  TTF_Quit();
  SDL_Quit();

  return EXIT_SUCCESS;
}
