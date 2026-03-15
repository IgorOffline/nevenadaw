#include <stdio.h>
#include "raylib.h"

//
// VCPKG="$HOME/Documents/vcpkgdir/vcpkg/installed/x64-linux"
// clang march.c -o march -I "$VCPKG/include" -L "$VCPKG/lib" -lraylib -lglfw3 -lGL -lm -lpthread -ldl -lrt -lX11 -lXrandr -lXi -lXcursor -lXinerama
//
int main(void) {
  printf("<START>\n");
  InitWindow(600, 400, "March");

  while (!WindowShouldClose()) {
    BeginDrawing();
    ClearBackground(RAYWHITE);
    DrawText("March", 40, 25, 20, BLACK);
    EndDrawing();
  }

  CloseWindow();
  printf("<END>\n");
  return 0;
}
