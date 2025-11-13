#define SDL_MAIN_HANDLED

#include <SDL3/SDL.h>
#include <SDL3_ttf/SDL_ttf.h>
#include <bgfx/bgfx.h>
#include <bx/math.h>

#include <iostream>
#include <string>

const std::string kLatinFontPath =
    "C:/igoroffline/fonts/IosevkaTerm-Regular.ttf";
const std::string kHangulFontPath = "C:/igoroffline/fonts/NotoSansKR.ttf";
constexpr int kFontSize = 24;

bgfx::TextureHandle g_textTexture = BGFX_INVALID_HANDLE;
bgfx::ProgramHandle g_textureProgram = BGFX_INVALID_HANDLE;
bgfx::UniformHandle g_s_texColor = BGFX_INVALID_HANDLE;
bgfx::VertexLayout g_layout;
uint16_t g_textWidth = 0;
uint16_t g_textHeight = 0;

struct PosTexCoord0Vertex {
  float x;
  float y;
  float z;
  float u;
  float v;
};

static constexpr PosTexCoord0Vertex s_screenQuadVertices[] = {
    {-1.0f, -1.0f, 0.0f, 0.0f, 1.0f},
    {1.0f, -1.0f, 0.0f, 1.0f, 1.0f},
    {-1.0f, 1.0f, 0.0f, 0.0f, 0.0f},
    {1.0f, 1.0f, 0.0f, 1.0f, 0.0f},
};

static const uint16_t s_screenQuadIndices[] = {
    0, 1, 2, 2, 1, 3,
};

void setupBGFXTextData(TTF_Font* font) {
  if (g_textTexture.idx != UINT16_MAX) {
    bgfx::destroy(g_textTexture);
  }

  constexpr SDL_Color white = {255, 255, 255, 255};
  SDL_Surface* surface = TTF_RenderText_Blended(font, "LoremIpsum", 0, white);

  if (!surface) {
    std::cerr << "TTF_RenderText_Blended failed: " << SDL_GetError()
              << std::endl;
    return;
  }

  g_textWidth = static_cast<uint16_t>(surface->w);
  g_textHeight = static_cast<uint16_t>(surface->h);

  g_textTexture = bgfx::createTexture2D(
      g_textWidth, g_textHeight, false, 1, bgfx::TextureFormat::BGRA8,
      BGFX_TEXTURE_NONE | BGFX_SAMPLER_POINT,
      bgfx::copy(surface->pixels, g_textWidth * g_textHeight * 4));

  SDL_DestroySurface(surface);
}

void renderTextQuad(const float x, const float y, const float w,
                    const float h) {
  if (g_textTexture.idx == UINT16_MAX || g_textureProgram.idx == UINT16_MAX) {
    return;
  }

  float proj[16];
  bx::mtxOrtho(proj, 0.0f, 1280.0f, 720.0f, 0.0f, 0.0f, 100.0f, 0.0f, false);
  bgfx::setViewTransform(0, nullptr, proj);

  float textMtx[16];
  bx::mtxIdentity(textMtx);
  bx::mtxScale(textMtx, w, h, 1.0f);
  bx::mtxTranslate(textMtx, x, y, 0.0f);
  bgfx::setTransform(textMtx);
  bgfx::setTexture(0, g_s_texColor, g_textTexture);

  bgfx::TransientVertexBuffer tvb{};
  bgfx::TransientIndexBuffer tib{};

  bgfx::allocTransientBuffers(&tvb, g_layout, 4, &tib, 6);

  std::memcpy(tvb.data, s_screenQuadVertices, 4 * sizeof(PosTexCoord0Vertex));
  std::memcpy(tib.data, s_screenQuadIndices, 6 * sizeof(uint16_t));

  bgfx::setVertexBuffer(0, &tvb);
  bgfx::setIndexBuffer(&tib);
  bgfx::setState(BGFX_STATE_DEFAULT | BGFX_STATE_PT_TRISTRIP |
                 BGFX_STATE_BLEND_ALPHA);

  bgfx::submit(0, g_textureProgram);
}

bool initialize_bgfx(SDL_Window* window, const int width, const int height) {
  bgfx::PlatformData pd{};

#ifdef SDL_PLATFORM_WIN32
  void* hwnd =
      SDL_GetPointerProperty(SDL_GetWindowProperties(window),
                             SDL_PROP_WINDOW_WIN32_HWND_POINTER, nullptr);
  pd.nwh = hwnd;
  pd.ndt = nullptr;
#elif defined(SDL_PLATFORM_LINUX)
  pd.ndt = nullptr;
  pd.nwh = SDL_GetPointerProperty(SDL_GetWindowProperties(window),
                                  SDL_PROP_WINDOW_X11_WINDOW_POINTER, nullptr);
#else
  pd.ndt = nullptr;
  pd.nwh = nullptr;
#endif

  bgfx::Init init;

  init.type = bgfx::RendererType::Count;
  init.platformData = pd;

  init.resolution.width = static_cast<uint32_t>(width);
  init.resolution.height = static_cast<uint32_t>(height);
  init.resolution.reset = BGFX_RESET_VSYNC;

  if (!bgfx::init(init)) {
    std::cerr << "ERROR: bgfx::init() failed after setting platform data."
              << std::endl;

    return false;
  }

  bgfx::setViewRect(0, 0, 0, static_cast<uint16_t>(width),
                    static_cast<uint16_t>(height));

  return true;
}

int main(const int argc, char* argv[]) {
  (void)argv;
  std::cout << "argc: " << argc << std::endl;

  constexpr int kWindowWidth = 1280;
  constexpr int kWindowHeight = 720;

  TTF_Font* latin_font = nullptr;
  TTF_Font* hangul_font = nullptr;

  if (!SDL_Init(SDL_INIT_VIDEO)) {
    std::cerr << "SDL init failed: " << SDL_GetError() << std::endl;

    return EXIT_FAILURE;
  }

  SDL_Window* window =
      SDL_CreateWindow("SDL/BGFX Example", kWindowWidth, kWindowHeight, 0);

  if (window == nullptr) {
    std::cerr << "Window creation failed: " << SDL_GetError() << std::endl;
    SDL_Quit();

    return EXIT_FAILURE;
  }

  if (!initialize_bgfx(window, kWindowWidth, kWindowHeight)) {
    SDL_DestroyWindow(window);
    SDL_Quit();

    return EXIT_FAILURE;
  }

  if (!TTF_Init()) {
    bgfx::shutdown();
    SDL_DestroyWindow(window);
    SDL_Quit();

    return EXIT_FAILURE;
  }

  latin_font = TTF_OpenFont(kLatinFontPath.c_str(), kFontSize);
  if (!latin_font) {
    if (TTF_WasInit()) {
      TTF_Quit();
    }
    bgfx::shutdown();
    SDL_DestroyWindow(window);
    SDL_Quit();

    return EXIT_FAILURE;
  }

  hangul_font = TTF_OpenFont(kHangulFontPath.c_str(), kFontSize);
  if (!hangul_font) {
    TTF_CloseFont(latin_font);
    if (TTF_WasInit()) {
      TTF_Quit();
    }
    bgfx::shutdown();
    SDL_DestroyWindow(window);
    SDL_Quit();

    return EXIT_FAILURE;
  }

  if (!TTF_AddFallbackFont(latin_font, hangul_font)) {
    TTF_CloseFont(latin_font);
    TTF_CloseFont(hangul_font);
    if (TTF_WasInit()) {
      TTF_Quit();
    }
    bgfx::shutdown();
    SDL_DestroyWindow(window);
    SDL_Quit();
    return EXIT_FAILURE;
  }

  bool quit = false;
  SDL_Event event;

  while (!quit) {
    while (SDL_PollEvent(&event)) {
      if (event.type == SDL_EVENT_QUIT) {
        quit = true;
      } else if (event.type == SDL_EVENT_KEY_DOWN) {
        if (event.key.scancode == SDL_SCANCODE_F) {
          std::cout << "[F]" << std::endl;
        }
      }
    }

    bgfx::touch(0);
    bgfx::setViewClear(0, BGFX_CLEAR_COLOR | BGFX_CLEAR_DEPTH, 0x212121ff, 1.0f,
                       0);

    if (g_textTexture.idx != UINT16_MAX) {
      renderTextQuad(200.0f, 150.0f, g_textWidth, g_textHeight);
    }

    bgfx::frame(false);
  }

  if (g_textTexture.idx != UINT16_MAX) {
    bgfx::destroy(g_textTexture);
  }
  if (g_textureProgram.idx != UINT16_MAX) {
    bgfx::destroy(g_textureProgram);
  }
  if (g_s_texColor.idx != UINT16_MAX) {
    bgfx::destroy(g_s_texColor);
  }
  TTF_CloseFont(latin_font);
  TTF_CloseFont(hangul_font);
  if (TTF_WasInit()) {
    TTF_Quit();
  }
  bgfx::shutdown();
  SDL_DestroyWindow(window);
  SDL_Quit();

  return EXIT_SUCCESS;
}