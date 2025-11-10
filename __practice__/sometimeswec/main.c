#include <stdbool.h>
#include <stdio.h>
#include <string.h>

#include "json.h"
#include "json_inttypes.h"

#define sometimeswec_memcmp_ok 0

int main(void) {
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
  json_object *maybe_json = json_tokener_parse("{ \"maybe\": -1 }");
  json_object *maybe_field = NULL;
  if (!json_object_object_get_ex(maybe_json, "maybe", &maybe_field)) {
    fprintf(stderr, "err-ab4fa52b\n");
    return true;
  }
  const int32_t maybe = json_object_get_int(maybe_field);
  printf("%s ; %d\n", json_object_to_json_string(maybe_json), maybe);
  json_object_put(maybe_json);
  const int32_t arr1[] = {1, 2, 3};
  const int32_t arr2[] = {1, 2, 3};
  int32_t arr3[3];
  memcpy(arr3, arr1, sizeof(arr1));
  const bool comparison12 = memcmp(arr1, arr2, sizeof(arr1)) == sometimeswec_memcmp_ok;
  const bool comparison23 = memcmp(arr2, arr3, sizeof(arr1)) == sometimeswec_memcmp_ok;
  if (comparison12 && comparison23) {
    printf("+memcmp\n");
  }

  return false;
}
