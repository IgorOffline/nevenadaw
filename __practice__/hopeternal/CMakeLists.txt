cmake_minimum_required(VERSION 3.20)

project(
        hopeternal
        VERSION 0.1.0
        LANGUAGES CXX
)

set(EXTERNAL_ROOT "C:\\igoroffline\\nevenadaw\\external")
message(STATUS "Consider: git submodule update --init --recursive , when considering is considerably within consideration?")

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_EXTENSIONS False)

set(SOURCE_FILES
        src/main.cpp
)

add_compile_options(/W4 /WX)

add_subdirectory(
        ${EXTERNAL_ROOT}/glfw
        ${CMAKE_CURRENT_BINARY_DIR}/external/glfw
)

add_executable(hopeternal_app ${SOURCE_FILES})

target_include_directories(
        hopeternal_app
        PRIVATE ${EXTERNAL_ROOT}/glew-cmake/include
        PRIVATE ${EXTERNAL_ROOT}/glfw/include
        PRIVATE ${EXTERNAL_ROOT}/glm/include
        PRIVATE ${EXTERNAL_ROOT}/tomlplusplus/include
)

target_link_libraries(hopeternal_app
        PRIVATE glfw
        PRIVATE opengl32
)