#include <SDL3/SDL.h>
#include <SDL3_ttf/SDL_ttf.h>
#include <stdbool.h>

int main() {
  SDL_Init(SDL_INIT_VIDEO);
  TTF_Init();

  SDL_Window *window = SDL_CreateWindow("SDL Rain", 1280, 720, 0);
  SDL_Renderer *renderer = SDL_CreateRenderer(window, NULL);
  const float font_size = 52;
  TTF_Font *font_latin = TTF_OpenFont("C:\\igoroffline\\fonts\\IosevkaTerm-Regular.ttf", font_size);
  TTF_Font *font_hangul = TTF_OpenFont("C:\\igoroffline\\fonts\\NotoSansKR.ttf", font_size);
  TTF_AddFallbackFont(font_latin, font_hangul);

  SDL_Event e;
  bool quit = false;
  while (!quit) {
    if (SDL_PollEvent(&e)) {
      switch (e.type) {
        case SDL_EVENT_QUIT:
          quit = true;
          break;
        default: break ;
      }
    }
    const SDL_Color light_green = {139, 195, 74, 255};
    const SDL_Color primary_text_dark = {33, 33, 33, 255};
    SDL_Surface *text_surf = TTF_RenderText_Solid(font_latin, "Hello반말Ipsum나LoremČćŠđŽRain", 0, light_green);
    SDL_Texture *text = SDL_CreateTextureFromSurface(renderer, text_surf);

    SDL_FRect dest = (SDL_FRect){
      .x = 200,
      .y = 200,
      .w = (float) text_surf->w,
      .h = (float) text_surf->h
    };
    SDL_RenderTexture(renderer, text, NULL, &dest);

    SDL_DestroyTexture(text);
    SDL_DestroySurface(text_surf);
    SDL_RenderPresent(renderer);

    SDL_SetRenderDrawColor(renderer, primary_text_dark.r, primary_text_dark.g, primary_text_dark.b,
                           primary_text_dark.a);
    SDL_RenderClear(renderer);
  }

  TTF_CloseFont(font_latin);
  TTF_CloseFont(font_hangul);
  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);
  TTF_Quit();
  SDL_Quit();

  return 0;
}
