cmake_minimum_required(VERSION 3.20)

if (NOT DEFINED CMAKE_TOOLCHAIN_FILE)
    if (DEFINED ENV{VCPKG_ROOT})
        set(CMAKE_TOOLCHAIN_FILE "$ENV{VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake"
                CACHE FILEPATH "VCPKG toolchain file")
    elseif (EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/vcpkg/scripts/buildsystems/vcpkg.cmake")
        set(CMAKE_TOOLCHAIN_FILE "${CMAKE_CURRENT_SOURCE_DIR}/vcpkg/scripts/buildsystems/vcpkg.cmake"
                CACHE FILEPATH "VCPKG toolchain file")
    elseif (EXISTS "$ENV{HOME}/vcpkg/scripts/buildsystems/vcpkg.cmake")
        set(CMAKE_TOOLCHAIN_FILE "$ENV{HOME}/vcpkg/scripts/buildsystems/vcpkg.cmake"
                CACHE FILEPATH "VCPKG toolchain file")
    endif ()
endif ()

project(datowerofcgraphics C)

add_compile_options(/W4 /WX) # WIN32

set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED True)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(datowerofcgraphics src/maingame.c)

target_compile_definitions(datowerofcgraphics PRIVATE
        WIN32_LEAN_AND_MEAN
        _WIN32_WINNT=0x0A00
)

target_include_directories(datowerofcgraphics PRIVATE include)

find_package(SDL3 CONFIG REQUIRED)
find_package(SDL3_ttf CONFIG REQUIRED)

target_link_libraries(datowerofcgraphics PRIVATE
        SDL3::SDL3
        SDL3_ttf::SDL3_ttf
)

if (TARGET SDL3::SDL3main)
    target_link_libraries(datowerofcgraphics PRIVATE SDL3::SDL3main)
endif ()

target_link_libraries(datowerofcgraphics PRIVATE Advapi32 Rpcrt4) # WIN32

if (CMAKE_TOOLCHAIN_FILE AND EXISTS "${CMAKE_TOOLCHAIN_FILE}")
    if (EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/vcpkg.json")
        message(STATUS "vcpkg manifest mode detected")
    else()
        message(WARNING "vcpkg.json not found - consider adding it for dependency management")
    endif()
endif ()