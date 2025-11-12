#define SDL_MAIN_HANDLED

#include <SDL3/SDL.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "bgfx_bridge.h"

const int kWindowWidth = 1280;
const int kWindowHeight = 720;

bool InitGraphics();

bool InitWindow(SDL_Window *window);

int main(int argc, char *argv[]) {
  (void)argv;

  if (argc < 99) {
    if (InitGraphics() == EXIT_FAILURE) {
      return EXIT_FAILURE;
    }
  }

  return EXIT_SUCCESS;
}

bool InitGraphics() {
  if (SDL_Init(SDL_INIT_VIDEO) < 0) {
    fprintf(stderr, "SDL initialization failed: %s\n", SDL_GetError());
    return EXIT_FAILURE;
  }

  SDL_Window *window =
      SDL_CreateWindow("SDL/BGFX Rain", kWindowWidth, kWindowHeight,
                       SDL_WINDOW_HIDDEN | SDL_WINDOW_RESIZABLE);

  if (window == NULL) {
    fprintf(stderr, "Window creation failed: %s\n", SDL_GetError());
    SDL_Quit();
    return EXIT_FAILURE;
  }

  if (!InitWindow(window)) {
    SDL_DestroyWindow(window);
    SDL_Quit();
    return EXIT_FAILURE;
  }

  SDL_ShowWindow(window);
  bool quit = false;
  SDL_Event event;

  int renderer = bgfx_bridge_get_renderer_type();
  printf("Renderer backend: %s\n", bgfx_bridge_get_renderer_name(renderer));

  while (!quit) {
    while (SDL_PollEvent(&event)) {
      if (event.type == SDL_EVENT_QUIT) {
        quit = true;
      } else if (event.type == SDL_EVENT_KEY_DOWN) {
        if (event.key.scancode == SDL_SCANCODE_F) {
          printf("[F]\n");
        }
      }
    }

    bgfx_bridge_touch(0);
    const SDL_Color primary_text_dark_raw = {33, 33, 33, 255};
    const uint32_t primary_text_dark =
        primary_text_dark_raw.r << 24 | primary_text_dark_raw.g << 16 |
        primary_text_dark_raw.b << 8 | primary_text_dark_raw.a;
    bgfx_bridge_set_view_clear_color_depth(0, primary_text_dark, 1.0f);
    bgfx_bridge_frame(false);
  }

  bgfx_bridge_shutdown();
  SDL_DestroyWindow(window);
  SDL_Quit();

  return EXIT_SUCCESS;
}

bool InitWindow(SDL_Window *window) {
#ifdef SDL_PLATFORM_WIN32
  void *hwnd = SDL_GetPointerProperty(SDL_GetWindowProperties(window),
                                      SDL_PROP_WINDOW_WIN32_HWND_POINTER, NULL);
#else
  void *hwnd = NULL;
#endif

  Bridge_InitResult res = bgfx_bridge_init_with_hwnd(
      hwnd, (uint32_t)kWindowWidth, (uint32_t)kWindowHeight, true);

  if (res.success) {
    bgfx_bridge_set_view_rect(0, 0, 0, (uint16_t)kWindowWidth,
                              (uint16_t)kWindowHeight);
    return true;
  }

  fprintf(stderr, "ERROR: Failed to initialize BGFX with any renderer!\n");
  return false;
}
