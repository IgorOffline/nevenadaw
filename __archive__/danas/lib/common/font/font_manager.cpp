#include <bgfx/bgfx.h>
#include <bx/bx.h>

#include "../common.h"
#include "stb_truetype.h"

BX_PRAGMA_DIAGNOSTIC_PUSH()
BX_PRAGMA_DIAGNOSTIC_IGNORED_MSVC(
    4244)  //  warning C4244: '=': conversion from 'double' to 'float', possible
           //  loss of data
BX_PRAGMA_DIAGNOSTIC_IGNORED_MSVC(
    4701)  //  warning C4701: potentially uninitialized local variable 'pt' used
#define SDF_IMPLEMENTATION
// LATER #include <sdf/sdf.h>
BX_PRAGMA_DIAGNOSTIC_POP()

#include <wchar.h>  // wcslen

#include "../cube_atlas.h"
#include "allocator.h"
#include "font_manager.hpp"
#include "unordered_map.h"
