cmake_minimum_required(VERSION 3.20)
project(practiCal C CXX)

set(CMAKE_C_COMPILER "clang")
set(CMAKE_C_STANDARD 17)
set(CMAKE_C_FLAGS "-pedantic-errors -Wall -Wextra -Werror -Wpedantic")

add_executable(practiCal main.c)