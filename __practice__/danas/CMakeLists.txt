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

configure_file("${CMAKE_CURRENT_SOURCE_DIR}/danas.toml" "${CMAKE_CURRENT_BINARY_DIR}/danas.toml" COPYONLY)

if (NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE "Debug" CACHE STRING "Build type" FORCE)
endif ()

if (WIN32)
    if (NOT CMAKE_C_COMPILER)
        set(CMAKE_C_COMPILER clang-cl)
    endif ()
    if (NOT CMAKE_CXX_COMPILER)
        set(CMAKE_CXX_COMPILER clang-cl)
    endif ()

    set(CMAKE_C_STANDARD 17)
    set(CMAKE_CXX_STANDARD 23)
    set(CMAKE_C_STANDARD_REQUIRED ON)
    set(CMAKE_CXX_STANDARD_REQUIRED ON)

    add_compile_options(/W4 /WX)

    set(CMAKE_MSVC_DEBUG_INFORMATION_FORMAT "$<$<CONFIG:Debug>:ProgramDatabase>")

    add_link_options(
            $<$<CONFIG:Debug>:/DEBUG>
    )

    if (DEFINED ENV{USE_LLD} OR DEFINED ENV{CI})
        find_program(LLD_LINK lld-link)
        if (LLD_LINK)
            set(CMAKE_LINKER ${LLD_LINK})
        endif ()
    endif ()
else ()
    if (NOT CMAKE_C_COMPILER)
        set(CMAKE_C_COMPILER clang)
    endif ()
    if (NOT CMAKE_CXX_COMPILER)
        set(CMAKE_CXX_COMPILER clang++)
    endif ()

    set(CMAKE_C_STANDARD 17)
    set(CMAKE_CXX_STANDARD 23)
    set(CMAKE_C_STANDARD_REQUIRED ON)
    set(CMAKE_CXX_STANDARD_REQUIRED ON)

    add_compile_options(-pedantic-errors -Wall -Wextra -Werror -Wpedantic)
    add_compile_options($<$<CONFIG:Debug>:-g>)
endif ()

if (NOT WIN32)
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fsanitize=address -fno-omit-frame-pointer")
endif ()

find_package(tomlplusplus CONFIG REQUIRED)

add_executable(danas main.cpp)

target_include_directories(danas PRIVATE include)

target_link_libraries(danas PRIVATE tomlplusplus::tomlplusplus)
