#ifndef DAMANAMI_BGFX_BRIDGE_H
#define DAMANAMI_BGFX_BRIDGE_H

#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef void* Bridge_NativeWindowHandle;

typedef struct Bridge_InitResult {
  bool success;
  int renderer_type;
} Bridge_InitResult;

Bridge_InitResult bgfx_bridge_init_with_hwnd(Bridge_NativeWindowHandle hwnd,
                                             uint32_t width, uint32_t height,
                                             bool vsync);

void bgfx_bridge_shutdown(void);

void bgfx_bridge_touch(uint16_t viewId);

void bgfx_bridge_set_view_clear_color_depth(uint16_t viewId, uint32_t rgba,
                                            float depth);
void bgfx_bridge_set_view_rect(uint16_t viewId, uint16_t x, uint16_t y,
                               uint16_t width, uint16_t height);

uint32_t bgfx_bridge_frame(bool capture);

int bgfx_bridge_get_renderer_type(void);
const char* bgfx_bridge_get_renderer_name(int rendererType);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // DAMANAMI_BGFX_BRIDGE_H
