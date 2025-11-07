cmake_minimum_required(VERSION 3.26)

if(NOT DEFINED CMAKE_TOOLCHAIN_FILE AND DEFINED ENV{VCPKG_ROOT})
  set(CMAKE_TOOLCHAIN_FILE "$ENV{VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake" CACHE STRING "")
endif()

if(NOT DEFINED VCPKG_TARGET_TRIPLET)
  if(DEFINED ENV{VCPKG_DEFAULT_TRIPLET})
    set(VCPKG_TARGET_TRIPLET "$ENV{VCPKG_DEFAULT_TRIPLET}" CACHE STRING "")
  else()
    set(VCPKG_TARGET_TRIPLET "x64-windows" CACHE STRING "")
  endif()
endif()

project(helloworldstringupper LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

message(STATUS "VCPKG_ROOT=$ENV{VCPKG_ROOT}")
message(STATUS "CMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}")
message(STATUS "VCPKG_DEFAULT_TRIPLET=$ENV{VCPKG_DEFAULT_TRIPLET}")
message(STATUS "VCPKG_TARGET_TRIPLET=${VCPKG_TARGET_TRIPLET}")
message(STATUS "CMAKE_PREFIX_PATH=${CMAKE_PREFIX_PATH}")

find_package(Boost CONFIG REQUIRED)

add_executable(helloworldstringupper main.cpp)

if(TARGET Boost::boost)
  set(_boost_headers_target Boost::boost)
elseif(TARGET Boost::headers)
  set(_boost_headers_target Boost::headers)
else()
  message(FATAL_ERROR "Boost headers target not found (neither Boost::boost nor Boost::headers)")
endif()

target_link_libraries(helloworldstringupper PRIVATE ${_boost_headers_target})