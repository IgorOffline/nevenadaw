cmake_minimum_required(VERSION 3.20)
project(datowerofcgraphics C)

set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED True)
set(CMAKE_C_EXTENSIONS ON)

add_compile_options(/W4 /WX)

add_executable(datowerofcgraphics maingame.c)