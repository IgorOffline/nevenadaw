cmake_minimum_required(VERSION 3.26)
project(mathlib VERSION 1.0.0 LANGUAGES CXX)

# Set C++23 standard
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Create shared library (DLL)
add_library(mathlib SHARED mathlib.cpp)

# Define export macro for Windows
target_compile_definitions(mathlib 
    PRIVATE MATHLIB_EXPORTS
)

# Include current directory for headers
target_include_directories(mathlib
    PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}
)

# Windows-specific settings
if(WIN32)
    # Set the output name to avoid "lib" prefix
    set_target_properties(mathlib PROPERTIES PREFIX "")
    
    # Optional: Set output directory
    set_target_properties(mathlib PROPERTIES 
        RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
        LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
        ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
    )
endif()