#include <stdio.h>
#include "json.h"
#include "json_inttypes.h"

#ifndef __clang__
#error "Please compile with Clang"
#endif

int main(void) {
  printf("Successfully compiled with Clang %s (%d.%d.%d)\n", __clang_version__, __clang_major__, __clang_minor__,
         __clang_patchlevel__);
  json_object *maybe_json = json_tokener_parse("{ \"maybe\": -1 }");
  json_object *maybe_field = NULL;
  if (!json_object_object_get_ex(maybe_json, "maybe", &maybe_field)) {
    fprintf(stderr, "err-ab4fa52b\n");
    return 1;
  }
  const int32_t maybe = json_object_get_int(maybe_field);
  printf("%s ; %d\n", json_object_to_json_string(maybe_json), maybe);
  json_object_put(maybe_json);
  return 0;
}
