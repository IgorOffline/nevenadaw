#include "mathlib.h"

extern "C" {
    MATHLIB_API int32_t sum(int32_t a, int32_t b) {
        return a + b;
    }
}