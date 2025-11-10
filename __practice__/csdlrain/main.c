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
} AppState;

SDL_AppResult SDL_AppIterate(void *appstate) {
  printf("1\n");

  AppState *as = appstate;
  if (!as) {
    return SDL_APP_FAILURE;
  }

  Uint64 now_ms = SDL_GetTicks();
  Uint64 delta_ms = now_ms - as->last_step_ms;

  const Uint64 target_step_ms = 250;
  if (delta_ms < target_step_ms) {
    SDL_Delay((Uint32) (target_step_ms - delta_ms));
    now_ms = SDL_GetTicks();
    delta_ms = now_ms - as->last_step_ms;
  }

  as->last_step_ms = now_ms;

  return SDL_APP_CONTINUE;
}

SDL_AppResult SDL_AppInit(void **appstate, int argc, char *argv[]) {
  printf("2\n");
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

  *appstate = as;
  return SDL_APP_CONTINUE;
}

SDL_AppResult SDL_AppEvent(void *appstate, SDL_Event *event) {
  printf("3\n");

  (void) appstate;
  (void) event;
  return SDL_APP_CONTINUE;
}

void SDL_AppQuit(void *appstate, SDL_AppResult result) {
  printf("4\n");

  (void) result;
  if (appstate != NULL) {
    AppState *as = appstate;
    SDL_DestroyRenderer(as->renderer);
    SDL_DestroyWindow(as->window);
    SDL_free(as);
  }
}
