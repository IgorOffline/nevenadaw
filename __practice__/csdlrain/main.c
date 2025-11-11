#define SDL_MAIN_HANDLED

#include <SDL3/SDL.h>
#include <bgfx/c99/bgfx.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

const int kWindowWidth = 1280;
const int kWindowHeight = 720;

bool InitWindow(SDL_Window *window) {
  bgfx_init_t init;
  bgfx_init_ctor(&init);

  bgfx_platform_data_t pd = {0};

#ifdef SDL_PLATFORM_WIN32
  void *hwnd = SDL_GetPointerProperty(SDL_GetWindowProperties(window), SDL_PROP_WINDOW_WIN32_HWND_POINTER, NULL);
  pd.nwh = hwnd;
#endif

  pd.ndt = NULL;
  pd.context = NULL;
  pd.backBuffer = NULL;
  pd.backBufferDS = NULL;

  bgfx_set_platform_data(&pd);

  init.resolution.width = (uint32_t) kWindowWidth;
  init.resolution.height = (uint32_t) kWindowHeight;
  init.resolution.reset = BGFX_RESET_VSYNC;
  init.platformData = pd;

  const bgfx_renderer_type_t renderers[] = {
    BGFX_RENDERER_TYPE_DIRECT3D12,
    BGFX_RENDERER_TYPE_DIRECT3D11,
    BGFX_RENDERER_TYPE_VULKAN,
    BGFX_RENDERER_TYPE_OPENGL,
  };

  for (int i = 0; i < sizeof(renderers) / sizeof(renderers[0]); i++) {
    init.type = renderers[i];
    if (bgfx_init(&init)) {
      bgfx_set_view_rect(0, 0, 0, (uint16_t) kWindowWidth, (uint16_t) kWindowHeight);

      return true;
    }
  }

  fprintf(stderr, "ERROR: Failed to initialize BGFX with any renderer!\n");

  return false;
}

int main(int argc, char *argv[]) {
  (void) argc;
  (void) argv;

  if (SDL_Init(SDL_INIT_VIDEO) < 0) {
    fprintf(stderr, "SDL initialization failed: %s\n", SDL_GetError());
    return EXIT_FAILURE;
  }

  SDL_Window *window = SDL_CreateWindow(
    "SDL/BGFX Rain",
    kWindowWidth,
    kWindowHeight,
    SDL_WINDOW_HIDDEN | SDL_WINDOW_RESIZABLE
  );

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

  printf("Renderer backend: %s\n", bgfx_get_renderer_name(bgfx_get_renderer_type()));

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

    bgfx_touch(0);
    const SDL_Color primary_text_dark_raw = {33, 33, 33, 255};
    const uint32_t primary_text_dark = primary_text_dark_raw.r << 24 | primary_text_dark_raw.g << 16 |
                                       primary_text_dark_raw.b << 8 | primary_text_dark_raw.a;
    bgfx_set_view_clear(0, BGFX_CLEAR_COLOR | BGFX_CLEAR_DEPTH, primary_text_dark, 1.0f, 0);
    bgfx_frame(false);
  }

  bgfx_shutdown();
  SDL_DestroyWindow(window);
  SDL_Quit();

  return EXIT_SUCCESS;
}
