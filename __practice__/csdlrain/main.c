#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER

#include <stdio.h>
#include <stdlib.h>

#define SDL_MAIN_USE_CALLBACKS 1
#include <SDL3/SDL.h>
#include <SDL3/SDL_main.h>
#include <SDL3_ttf/SDL_ttf.h>

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
  TTF_Font *fps_font;
  SDL_Texture *fps_texture;
  int fps_tex_w;
  int fps_tex_h;
  char fps_text[64];
  Uint64 last_fps_update_ms;
  float fps_smooth;
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
    // Clear background
    SDL_SetRenderDrawColor(as->renderer, grey_secondary_text.r, grey_secondary_text.g, grey_secondary_text.b,
                           grey_secondary_text.a);
    SDL_RenderClear(as->renderer);

    // Compute FPS using EMA smoothing and update texture at ~4Hz
    Uint64 now_ms = SDL_GetTicks();
    float dt_ms = (float) (now_ms - as->last_step_ms);
    if (dt_ms < 0.0f) dt_ms = 0.0f;
    as->last_step_ms = now_ms;
    float inst_fps = (dt_ms > 0.0f) ? (1000.0f / dt_ms) : 0.0f;
    if (as->fps_smooth <= 0.0f) {
      as->fps_smooth = inst_fps;
    } else {
      const float alpha = 0.1f;
      as->fps_smooth = alpha * inst_fps + (1.0f - alpha) * as->fps_smooth;
    }

    if (as->fps_font) {
      if ((now_ms - as->last_fps_update_ms) >= 250) {
        as->last_fps_update_ms = now_ms;
        char buf[64];
        SDL_snprintf(buf, sizeof(buf), "FPS: %.1f", as->fps_smooth);
        if (SDL_strcmp(buf, as->fps_text) != 0) {
          // Re-render text into texture
          SDL_strlcpy(as->fps_text, buf, sizeof(as->fps_text));
          if (as->fps_texture) {
            SDL_DestroyTexture(as->fps_texture);
            as->fps_texture = NULL;
          }
          const SDL_Color white = {255, 255, 255, 255};
          SDL_Surface *surf = TTF_RenderText_Blended(as->fps_font, as->fps_text, SDL_strlen(as->fps_text), white);
          if (surf) {
            SDL_Texture *tex = SDL_CreateTextureFromSurface(as->renderer, surf);
            if (tex) {
              as->fps_texture = tex;
              as->fps_tex_w = surf->w;
              as->fps_tex_h = surf->h;
            } else {
              SDL_LogWarn(SDL_LOG_CATEGORY_APPLICATION, "CreateTextureFromSurface failed: %s", SDL_GetError());
            }
            SDL_DestroySurface(surf);
          } else {
            SDL_LogWarn(SDL_LOG_CATEGORY_APPLICATION, "TTF_RenderUTF8_Blended failed: %s", SDL_GetError());
          }
        }
      }

      // Draw texture in top-right corner
      if (as->fps_texture) {
        int rw = 0, rh = 0;
        SDL_GetRenderOutputSize(as->renderer, &rw, &rh);
        const int pad = 8;
        SDL_FRect dst = {
          .x = (float) (rw - as->fps_tex_w - pad),
          .y = (float) pad,
          .w = (float) as->fps_tex_w,
          .h = (float) as->fps_tex_h
        };
        SDL_RenderTexture(as->renderer, as->fps_texture, NULL, &dst);
      }
    }

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

  if (TTF_Init() != 0) {
    SDL_LogWarn(SDL_LOG_CATEGORY_APPLICATION, "TTF_Init failed: %s", SDL_GetError());
  } else {
    const char *font_path = SDL_getenv("FPS_FONT_PATH");
    if (font_path && *font_path) {
      const float font_pt_size = 18;
      as->fps_font = TTF_OpenFont(font_path, font_pt_size);
      if (!as->fps_font) {
        SDL_LogWarn(SDL_LOG_CATEGORY_APPLICATION, "TTF_OpenFont('%s') failed: %s", font_path, SDL_GetError());
      }
    } else {
      SDL_Log("Set environment variable FPS_FONT_PATH to a .ttf file to enable FPS text");
    }
    as->fps_text[0] = '\0';
    as->last_fps_update_ms = 0;
    as->fps_smooth = 0.0f;
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
    if (as->fps_texture) {
      SDL_DestroyTexture(as->fps_texture);
      as->fps_texture = NULL;
    }
    if (as->fps_font) {
      TTF_CloseFont(as->fps_font);
      as->fps_font = NULL;
    }
    if (TTF_WasInit()) {
      TTF_Quit();
    }
    SDL_DestroyRenderer(as->renderer);
    SDL_DestroyWindow(as->window);
    SDL_free(as);
  }
}
