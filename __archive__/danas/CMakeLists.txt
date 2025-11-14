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

project(danas C CXX)

if (WIN32)
    if (NOT CMAKE_BUILD_TYPE)
        set(CMAKE_BUILD_TYPE "Debug" CACHE STRING "Build type" FORCE)
    endif ()

    add_compile_options(/W4 /WX)

    set(CMAKE_MSVC_DEBUG_INFORMATION_FORMAT "$<$<CONFIG:Debug>:ProgramDatabase>")

    add_link_options(
            $<$<CONFIG:Debug>:/DEBUG>
    )

    if (DEFINED ENV{USE_LLD} OR DEFINED ENV{CI})
        find_program(LLD_LINK lld-link)
        if (LLD_LINK)
            message(STATUS "Using lld-link: ${LLD_LINK}")
            set(CMAKE_C_LINKER ${LLD_LINK})
            set(CMAKE_CXX_LINKER ${LLD_LINK})
        else ()
            message(WARNING "lld-link not found but was requested")
        endif ()
    endif ()
endif ()

set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED True)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

find_package(SDL3 CONFIG REQUIRED)
find_package(SDL3_ttf CONFIG REQUIRED)
find_package(bgfx CONFIG REQUIRED)

add_executable(
        danas
        main.cpp
        lib/common/font/font_manager.hpp
        lib/common/font/font_manager.cpp
        lib/common/common.h
        lib/common/cube_atlas.h
)

target_compile_definitions(danas PRIVATE
        WIN32_LEAN_AND_MEAN
        _WIN32_WINNT=0x0A00
)

target_include_directories(danas PRIVATE include)

target_include_directories(danas PRIVATE lib/tiny)

target_include_directories(danas PRIVATE lib/stb)

target_link_libraries(danas PRIVATE
        SDL3::SDL3
        SDL3_ttf::SDL3_ttf
        bgfx::bgfx
)

if (TARGET SDL3::SDL3main)
    target_link_libraries(danas PRIVATE SDL3::SDL3main)
endif ()

if (MSVC)
    target_compile_options(danas PRIVATE
            $<$<COMPILE_LANGUAGE:CXX>:/Zc:__cplusplus /Zc:preprocessor /permissive->
    )
endif ()

if (WIN32)
    target_link_libraries(danas PRIVATE Advapi32 Rpcrt4)
endif ()

if (CMAKE_TOOLCHAIN_FILE AND EXISTS "${CMAKE_TOOLCHAIN_FILE}")
    if (NOT EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/vcpkg.json")
        message(WARNING "Using vcpkg toolchain but no vcpkg.json manifest file found!")
    endif ()
endif ()