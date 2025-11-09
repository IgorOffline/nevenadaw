cmake_minimum_required(VERSION 3.20)

if(NOT DEFINED CMAKE_TOOLCHAIN_FILE AND DEFINED ENV{VCPKG_ROOT})
    set(CMAKE_TOOLCHAIN_FILE "$ENV{VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake" CACHE FILEPATH "VCPKG toolchain file")
endif()

project(bosonogaimplementation LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

find_package(antlr4-runtime CONFIG REQUIRED)

add_executable(bosonogaimplementation
        main.cpp
        generated/BosonogaBaseListener.cpp generated/BosonogaBaseListener.h
        generated/BosonogaLexer.cpp        generated/BosonogaLexer.h
        generated/BosonogaListener.cpp     generated/BosonogaListener.h
        generated/BosonogaParser.cpp       generated/BosonogaParser.h
        BosonogaBaseListenerExtended.hpp
        BosonogaExecute.hpp
        BosonogaPrimitives.hpp
)

target_include_directories(bosonogaimplementation PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}
        ${CMAKE_CURRENT_SOURCE_DIR}/generated
)

if (TARGET antlr4_shared)
    target_link_libraries(bosonogaimplementation PRIVATE antlr4_shared)
elseif (TARGET antlr4_static)
    target_link_libraries(bosonogaimplementation PRIVATE antlr4_static)
    target_compile_definitions(bosonogaimplementation PRIVATE ANTLR4CPP_STATIC)
else()
    message(FATAL_ERROR "ERR-c03cfa57")
endif()
