cmake_minimum_required(VERSION 3.20)

project(
        otokoislandia
        VERSION 0.1.0
        LANGUAGES CXX
)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_EXTENSIONS False)

set(SOURCE_FILES
        src/main.cpp
)

add_compile_options(/W4 /WX)

add_executable(otokoislandia_app ${SOURCE_FILES})