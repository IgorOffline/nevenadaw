cmake_minimum_required(VERSION 3.20)

if (NOT DEFINED CMAKE_TOOLCHAIN_FILE AND DEFINED ENV{VCPKG_ROOT})
    set(CMAKE_TOOLCHAIN_FILE "$ENV{VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake" CACHE FILEPATH "VCPKG toolchain file")
endif ()

project(practiCal C CXX)

set(CMAKE_C_COMPILER "clang")
set(CMAKE_C_STANDARD 17)
set(CMAKE_C_FLAGS "-pedantic-errors -Wall -Wextra -Werror -Wpedantic")

add_executable(practiCal main.c)

find_package(json-c REQUIRED)
target_link_libraries(practiCal PRIVATE json-c::json-c)