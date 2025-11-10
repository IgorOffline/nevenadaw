#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER

#include <stdio.h>
#include <stdlib.h>

#define SDL_MAIN_USE_CALLBACKS 1
#include <SDL3/SDL.h>
#include <SDL3/SDL_main.h>

typedef struct {
  int32_t rain_context_number;
} RainContext;

typedef struct {
  SDL_Window *window;
  SDL_Renderer *renderer;
  RainContext rain_context;
  Uint64 last_step_ms;
  bool button_message_pressed;
  bool should_quit;
} AppState;

SDL_AppResult SDL_AppIterate(void *appstate) {
  AppState *as = appstate;
  if (!as) {
    return SDL_APP_FAILURE;
  }

  if (as->button_message_pressed) {
    printf("[F]\n");
    fflush(stdout);
    as->button_message_pressed = false;
  }

  if (as->should_quit) {
    return SDL_APP_SUCCESS;
  }

  const SDL_Color grey_secondary_text = {.r = 117, .g = 117, .b = 117, .a = 255};
  if (as->renderer) {
    SDL_SetRenderDrawColor(as->renderer, grey_secondary_text.r, grey_secondary_text.g, grey_secondary_text.b,
                           grey_secondary_text.a);
    SDL_RenderClear(as->renderer);
    SDL_RenderPresent(as->renderer);
  }

  return SDL_APP_CONTINUE;
}

SDL_AppResult SDL_AppInit(void **appstate, int argc, char *argv[]) {
  printf("SDL_AppInit\n");
  (void) argc;
  (void) argv;

#ifdef __clang__
  printf("Successfully compiled with Clang %s (%d.%d.%d)\n", __clang_version__, __clang_major__, __clang_minor__,
         __clang_patchlevel__);
#elif defined(_MSC_VER)
  printf("Successfully compiled with MSVC version %d (%d.%d.%d.%d)\n",
         _MSC_VER,
         _MSC_VER / 100,
         _MSC_VER % 100,
         _MSC_FULL_VER % 100000,
         _MSC_BUILD);
#else
  printf("No Clang or MSVC detected - using another compiler\n");
#endif
#ifdef _MSC_VER
  _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
  _CrtSetReportMode(_CRT_WARN, _CRTDBG_MODE_FILE);
  _CrtSetReportFile(_CRT_WARN, _CRTDBG_FILE_STDERR);
  _CrtSetReportMode(_CRT_ERROR, _CRTDBG_MODE_FILE);
  _CrtSetReportFile(_CRT_ERROR, _CRTDBG_FILE_STDERR);
  _CrtSetReportMode(_CRT_ASSERT, _CRTDBG_MODE_FILE);
  _CrtSetReportFile(_CRT_ASSERT, _CRTDBG_FILE_STDERR);
#endif

  AppState *as = SDL_calloc(1, sizeof(AppState));
  if (!as) {
    SDL_LogError(SDL_LOG_CATEGORY_ERROR, "Failed to allocate AppState\n");
    return SDL_APP_FAILURE;
  }
  as->last_step_ms = SDL_GetTicks();

  as->window = SDL_CreateWindow("SDL Rain", 1280, 720, 0);
  if (!as->window) {
    SDL_LogError(SDL_LOG_CATEGORY_ERROR, "Failed to create window: %s\n", SDL_GetError());
    SDL_free(as);
    return SDL_APP_FAILURE;
  }

  as->renderer = SDL_CreateRenderer(as->window, NULL);
  if (!as->renderer) {
    SDL_LogError(SDL_LOG_CATEGORY_ERROR, "Failed to create renderer: %s\n", SDL_GetError());
    SDL_DestroyWindow(as->window);
    SDL_free(as);
    return SDL_APP_FAILURE;
  }

  *appstate = as;
  return SDL_APP_CONTINUE;
}

SDL_AppResult SDL_AppEvent(void *appstate, SDL_Event *event) {
  AppState *as = appstate;

  if (event) {
    switch (event->type) {
      case SDL_EVENT_QUIT:
      case SDL_EVENT_WINDOW_CLOSE_REQUESTED:
        if (as) as->should_quit = true;
        return SDL_APP_CONTINUE;
      case SDL_EVENT_KEY_DOWN:
        if (event->key.scancode == SDL_SCANCODE_F) {
          as->button_message_pressed = true;
          return SDL_APP_CONTINUE;
        }
        if (event->key.scancode == SDL_SCANCODE_ESCAPE) {
          as->should_quit = true;
          return SDL_APP_CONTINUE;
        }
        return SDL_APP_CONTINUE;
      default:
        return SDL_APP_CONTINUE;
    }
  }

  return SDL_APP_CONTINUE;
}

void SDL_AppQuit(void *appstate, SDL_AppResult result) {
  printf("SDL_AppQuit\n");

  (void) result;
  if (appstate != NULL) {
    AppState *as = appstate;
    SDL_DestroyRenderer(as->renderer);
    SDL_DestroyWindow(as->window);
    SDL_free(as);
  }
}
