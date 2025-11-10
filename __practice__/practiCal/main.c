#include <stdio.h>

#ifndef __clang__
#error "Please compile with Clang"
#endif

int main(void) {
  printf("Successfully compiled with Clang %s (%d.%d.%d)\n", __clang_version__, __clang_major__, __clang_minor__,
         __clang_patchlevel__);
  return 0;
}
