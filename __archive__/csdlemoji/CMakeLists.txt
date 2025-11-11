cmake_minimum_required(VERSION 3.20)
project(csdlemoji C)

set(CMAKE_C_STANDARD 23)

add_executable(csdlemoji main.c)

set(THIRD_PARTY_DIR ${CMAKE_CURRENT_SOURCE_DIR}/third_party)
set(SDL3_INCLUDE_DIR ${THIRD_PARTY_DIR}/include)
set(SDL3_LIB_DIR ${THIRD_PARTY_DIR}/lib)

target_include_directories(csdlemoji PRIVATE ${SDL3_INCLUDE_DIR})

target_link_libraries(csdlemoji PRIVATE ${SDL3_LIB_DIR}/SDL3.lib)

file(COPY "C:/igoroffline/nevenadaw/__practice__/csdlemoji/third_party/dll/"
        DESTINATION ${CMAKE_BINARY_DIR}
)
