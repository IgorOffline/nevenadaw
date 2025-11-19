cmake_minimum_required(VERSION 3.20)

project(datowerofcgraphics C)

set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED True)
set(CMAKE_C_EXTENSIONS OFF)

add_compile_options(/W4 /WX) # MSVC

include_directories("C:/igoroffline/nevenadaw/external/libsdl-org-SDL/include")

add_executable(datowerofcgraphics maingame.c)

target_link_libraries(datowerofcgraphics PRIVATE
        "C:/igoroffline/nevenadaw/external/libsdl-org-SDL/build/SDL3.dll"
)