#define SDL_MAIN_HANDLED

#include <SDL3/SDL.h>
#include <bgfx/bgfx.h>

#include <iostream>

bool initialize_bgfx(SDL_Window* window, const int width, const int height) {
  bgfx::PlatformData pd{};

#ifdef SDL_PLATFORM_WIN32
  void* hwnd =
      SDL_GetPointerProperty(SDL_GetWindowProperties(window),
                             SDL_PROP_WINDOW_WIN32_HWND_POINTER, nullptr);
  pd.nwh = hwnd;
  pd.ndt = nullptr;
#else
  pd.ndt = nullptr;
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

  if (!SDL_Init(SDL_INIT_VIDEO)) {
    std::cerr << "SDL init failed: " << SDL_GetError() << std::endl;
    return EXIT_FAILURE;
  }

  SDL_Window* window = SDL_CreateWindow("SDL/BGFX Danas", kWindowWidth,
                                        kWindowHeight, SDL_WINDOW_RESIZABLE);

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

  bool quit = false;
  SDL_Event event;
  int current_width = kWindowWidth;
  int current_height = kWindowHeight;

  while (!quit) {
    while (SDL_PollEvent(&event)) {
      if (event.type == SDL_EVENT_QUIT) {
        quit = true;
      } else if (event.type == SDL_EVENT_KEY_DOWN) {
        if (event.key.scancode == SDL_SCANCODE_F) {
          std::cout << "[F]" << std::endl;
        }
      } else if (event.type == SDL_EVENT_WINDOW_RESIZED) {
        current_width = event.window.data1;
        current_height = event.window.data2;

        bgfx::reset(static_cast<uint32_t>(current_width),
                    static_cast<uint32_t>(current_height), BGFX_RESET_VSYNC);

        bgfx::setViewRect(0, 0, 0, static_cast<uint16_t>(current_width),
                          static_cast<uint16_t>(current_height));
      }
    }
    bgfx::touch(0);
    bgfx::setViewClear(0, BGFX_CLEAR_COLOR | BGFX_CLEAR_DEPTH, 0x757575ff, 1.0f,
                       0);
    bgfx::frame(false);
  }

  bgfx::shutdown();
  SDL_DestroyWindow(window);
  SDL_Quit();

  return EXIT_SUCCESS;
}