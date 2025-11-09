cmake_minimum_required(VERSION 3.20)

if (NOT DEFINED CMAKE_TOOLCHAIN_FILE AND DEFINED ENV{VCPKG_ROOT})
    set(_vcpkg_toolchain "$ENV{VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake")
    if (EXISTS "${_vcpkg_toolchain}")
        set(CMAKE_TOOLCHAIN_FILE "${_vcpkg_toolchain}" CACHE FILEPATH "VCPKG toolchain file")
    endif ()
endif ()

project(bosonogaimplementation LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

include(CTest)

find_package(antlr4-runtime CONFIG QUIET)

if (antlr4-runtime_FOUND)
    add_executable(bosonogaimplementation
            main.cpp
            generated/BosonogaBaseListener.cpp generated/BosonogaBaseListener.h
            generated/BosonogaLexer.cpp generated/BosonogaLexer.h
            generated/BosonogaListener.cpp generated/BosonogaListener.h
            generated/BosonogaParser.cpp generated/BosonogaParser.h
            BosonogaBaseListenerExtended.hpp
            BosonogaExecute.hpp
            BosonogaPrimitives.hpp
    )

    if (TARGET antlr4_shared)
        target_link_libraries(bosonogaimplementation PRIVATE antlr4_shared)
    elseif (TARGET antlr4_static)
        target_link_libraries(bosonogaimplementation PRIVATE antlr4_static)
    else ()
        message(FATAL_ERROR "ERR-c98f88f5")
    endif ()
else ()
    message(WARNING "ERR-c03cfa57")
endif ()
