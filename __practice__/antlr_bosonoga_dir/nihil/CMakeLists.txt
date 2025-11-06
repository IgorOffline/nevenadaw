cmake_minimum_required(VERSION 3.20)
project(nihil)

set(CMAKE_CXX_STANDARD 23)

add_executable(nihil main.cpp
        json.hpp
        nihil.hpp)