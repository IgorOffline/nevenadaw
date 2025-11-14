cmake_minimum_required(VERSION 3.20)

project(
        otokoislandia
        VERSION 0.1.0
        LANGUAGES CXX
)

set(EXTERNAL_ROOT "C:\\igoroffline\\nevenadaw\\external")
message(STATUS "Consider: git submodule add https://github.com/marzer/tomlplusplus.git external/tomlplusplus , git submodule add https://github.com/glfw/glfw.git external/glfw , git submodule add https://github.com/g-truc/glm.git external/glm , git submodule add https://github.com/Perlmint/glew-cmake.git external/glew-cmake , when considering is considerably within consideration?")

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_EXTENSIONS False)

set(SOURCE_FILES
        src/main.cpp
)

add_compile_options(/W4 /WX)

add_executable(otokoislandia_app ${SOURCE_FILES})

target_include_directories(
        otokoislandia_app
        PRIVATE ${EXTERNAL_ROOT}/tomlplusplus/include
)

target_link_libraries(otokoislandia_app)