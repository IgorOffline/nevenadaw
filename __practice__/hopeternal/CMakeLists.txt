cmake_minimum_required(VERSION 3.20)

project(
        hopeternal
        VERSION 0.1.0
        LANGUAGES CXX
)

set(EXTERNAL_ROOT "C:/igoroffline/nevenadaw/external")
message(STATUS "Consider: git submodule update --init --recursive , when considering is considerably within consideration?")

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_EXTENSIONS False)

set(SOURCE_FILES
        src/main.cpp
)

add_executable(hopeternal_app ${SOURCE_FILES})

if (MSVC)
    target_compile_options(hopeternal_app PRIVATE /W4)
else ()
    target_compile_options(hopeternal_app PRIVATE -Wall -Wextra)
endif ()

set(GLFW_BUILD_DOCS OFF CACHE BOOL "" FORCE)
set(GLFW_BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)
set(GLFW_BUILD_TESTS OFF CACHE BOOL "" FORCE)
set(GLFW_INSTALL OFF CACHE BOOL "" FORCE)
add_subdirectory(${EXTERNAL_ROOT}/glfw ${CMAKE_BINARY_DIR}/external/glfw)

target_include_directories(
        hopeternal_app
        PRIVATE ${EXTERNAL_ROOT}/tomlplusplus/include
        PRIVATE ${EXTERNAL_ROOT}/glfw/include
)

target_link_libraries(hopeternal_app
        glfw
        opengl32
)