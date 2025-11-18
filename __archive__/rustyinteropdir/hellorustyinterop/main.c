#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

int32_t cinterop(int32_t flag) {
    printf("C::HelloRustyInterop::%" PRId32 "\n", flag);
    return flag;
}