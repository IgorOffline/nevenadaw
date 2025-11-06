cmake_minimum_required(VERSION 3.26)
project(app VERSION 1.0.0 LANGUAGES CXX)

# Set C++23 standard
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Create executable
add_executable(app main.cpp)

# Link with the mathlib DLL
target_link_libraries(app PRIVATE mathlib)

# Include mathlib headers
target_include_directories(app PRIVATE ../mathlib)

# Windows-specific settings
if(WIN32)
    # Optional: Set output directory
    set_target_properties(app PROPERTIES 
        RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
    )
endif()