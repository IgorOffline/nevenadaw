#include "bgfx_bridge.h"

#include <bgfx/c99/bgfx.h>
#include <stdint.h>

extern "C" {

Bridge_InitResult bgfx_bridge_init_with_hwnd(Bridge_NativeWindowHandle hwnd,
                                             uint32_t width, uint32_t height,
                                             bool vsync) {
  Bridge_InitResult result = {false, (int)BGFX_RENDERER_TYPE_NOOP};

  bgfx_init_t init;
  bgfx_init_ctor(&init);

  bgfx_platform_data_t pd = {};
  pd.nwh = hwnd;
  pd.ndt = nullptr;
  pd.context = nullptr;
  pd.backBuffer = nullptr;
  pd.backBufferDS = nullptr;

  bgfx_set_platform_data(&pd);

  init.resolution.width = width;
  init.resolution.height = height;
  init.resolution.reset = vsync ? BGFX_RESET_VSYNC : 0;
  init.platformData = pd;

  const bgfx_renderer_type_t renderers[] = {
      BGFX_RENDERER_TYPE_DIRECT3D12,
      BGFX_RENDERER_TYPE_DIRECT3D11,
      BGFX_RENDERER_TYPE_VULKAN,
      BGFX_RENDERER_TYPE_OPENGL,
  };

  for (int i = 0; i < (int)(sizeof(renderers) / sizeof(renderers[0])); ++i) {
    init.type = renderers[i];
    if (bgfx_init(&init)) {
      bgfx_set_view_rect(0, 0, 0, (uint16_t)width, (uint16_t)height);
      result.success = true;
      result.renderer_type = (int)init.type;
      return result;
    }
  }

  return result;
}

void bgfx_bridge_shutdown(void) { bgfx_shutdown(); }

void bgfx_bridge_touch(uint16_t viewId) { bgfx_touch(viewId); }

void bgfx_bridge_set_view_clear_color_depth(uint16_t viewId, uint32_t rgba,
                                            float depth) {
  bgfx_set_view_clear(viewId, (uint16_t)(BGFX_CLEAR_COLOR | BGFX_CLEAR_DEPTH),
                      rgba, depth, 0);
}

void bgfx_bridge_set_view_rect(uint16_t viewId, uint16_t x, uint16_t y,
                               uint16_t width, uint16_t height) {
  bgfx_set_view_rect(viewId, x, y, width, height);
}

uint32_t bgfx_bridge_frame(bool capture) { return bgfx_frame(capture); }

int bgfx_bridge_get_renderer_type(void) {
  return (int)bgfx_get_renderer_type();
}

const char* bgfx_bridge_get_renderer_name(int rendererType) {
  return bgfx_get_renderer_name((bgfx_renderer_type_t)rendererType);
}

}  // extern "C"
