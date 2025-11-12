cmake_minimum_required(VERSION 3.20)

project(capabeata
        VERSION 0.1.0
        LANGUAGES C CXX
)

# add_subdirectory(core_c_module)
add_subdirectory(core_capa_module)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)