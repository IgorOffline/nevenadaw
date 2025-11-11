cmake_minimum_required(VERSION 3.20)
project(damanami C CXX)

set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED True)

set(SDL3_BASE_PATH "C:/sdl/SDL3-3.2.26")
set(SDL3_TTF_BASE_PATH "C:/sdl/SDL3_ttf-devel-3.2.2-VC/SDL3_ttf-3.2.2")

add_executable(damanami main.c)

if (WIN32)
set(WINDOWS_SYSTEM_LIBS
user32.lib
gdi32.lib
winmm.lib
imm32.lib
version.lib
shell32.lib
setupapi.lib
Usp10.lib
Rpcrt4.lib
)

target_include_directories(damanami PRIVATE
"${SDL3_BASE_PATH}/include/SDL3"
"${SDL3_TTF_BASE_PATH}/include/SDL3_ttf"
)

target_link_directories(damanami PRIVATE
"${SDL3_BASE_PATH}/lib/x64"
"${SDL3_TTF_BASE_PATH}/lib/x64"
)

target_link_libraries(damanami PRIVATE
SDL3.lib
SDL3_ttf.lib

${WINDOWS_SYSTEM_LIBS}
)
endif ()