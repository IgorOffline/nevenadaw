#include <SDL3/SDL.h>
#include <stdio.h>
#include <stdlib.h>

int main(void) {
  printf("Compiled against SDL version %d.%d.%d\n",
         SDL_MAJOR_VERSION, SDL_MINOR_VERSION, SDL_MICRO_VERSION);

  const int linked_version = SDL_GetVersion();
  const int linked_major = SDL_VERSIONNUM_MAJOR(linked_version);
  const int linked_minor = SDL_VERSIONNUM_MINOR(linked_version);
  const int linked_patch = SDL_VERSIONNUM_MICRO(linked_version);
  printf("Linked SDL version %d.%d.%d\n", linked_major, linked_minor, linked_patch);

  printf("SDL version query successful!\n");

  if (SDL_Init(SDL_INIT_VIDEO) != 0) {
    fprintf(stderr, "SDL_Init failed: %s\n", SDL_GetError());

    if (SDL_Init(0) != 0) {
      fprintf(stderr, "Even basic SDL_Init failed: %s\n", SDL_GetError());
    } else {
      printf("Basic SDL init worked, issue is with video subsystem\n");
      SDL_Quit();
    }
    return EXIT_FAILURE;
  }

  printf("SDL initialized successfully!\n");

  SDL_Quit();
  return EXIT_SUCCESS;
}
