cmake_minimum_required(VERSION 3.20)

project(
        otokoislandia
        VERSION 0.1.0
        LANGUAGES CXX
)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_EXTENSIONS False)

set(SOURCE_FILES
        src/main.cpp
)

add_compile_options(/W4 /WX)

add_executable(otokoislandia_app ${SOURCE_FILES})

set(GLFW_BUILD_DOCS OFF CACHE BOOL "" FORCE)
set(GLFW_BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)
set(GLFW_BUILD_TESTS OFF CACHE BOOL "" FORCE)
set(GLFW_INSTALL OFF CACHE BOOL "" FORCE)
add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/external/glfw)

target_include_directories(
        otokoislandia_app
        PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/external/tomlplusplus/include
        PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/external/glfw/include
)

target_link_libraries(otokoislandia_app
        glfw
        opengl32
)