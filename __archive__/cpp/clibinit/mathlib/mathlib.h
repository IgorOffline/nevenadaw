#pragma once

#ifdef MATHLIB_EXPORTS
#define MATHLIB_API __declspec(dllexport)
#else
#define MATHLIB_API __declspec(dllimport)
#endif

#include <cstdint>

extern "C" {
    MATHLIB_API int32_t sum(int32_t a, int32_t b);
}