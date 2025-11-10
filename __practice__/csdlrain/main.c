#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif

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
  Uint64 last_step;
} AppState;

SDL_AppResult SDL_AppIterate(void *appstate) {
  printf("Null? [%d]\n", appstate == NULL);

  return SDL_APP_CONTINUE;
}

SDL_AppResult SDL_AppInit(void **appstate, int argc, char *argv[]) {
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


#ifdef _MSC_VER
  _CrtDumpMemoryLeaks();
#endif

  printf("Null? [%d] [%d] [%d]\n", appstate == NULL, argc == 0, argv == NULL);

  return SDL_APP_CONTINUE;
}

SDL_AppResult SDL_AppEvent(void *appstate, SDL_Event *event) {
  printf("Null? [%d] [%d]\n", appstate == NULL, event == NULL);

  return SDL_APP_CONTINUE;
}

void SDL_AppQuit(void *appstate, SDL_AppResult result) {
  printf("Null? [%d]\n", result);

  if (appstate != NULL) {
    AppState *as = appstate;
    SDL_DestroyRenderer(as->renderer);
    SDL_DestroyWindow(as->window);
    SDL_free(as);
  }
}
