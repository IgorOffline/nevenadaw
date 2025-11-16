cmake_minimum_required(VERSION 3.20)
project(nativaneinitcore CXX)

set(CMAKE_C_STANDARD 17)

add_library(nativaneinitcore SHARED nativaneinitcore.cpp)
target_compile_definitions(nativaneinitcore PRIVATE NATIVANEINITCORE_EXPORTS)