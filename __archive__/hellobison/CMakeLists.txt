cmake_minimum_required(VERSION 3.20)
project(hellobison LANGUAGES C)

set(CMAKE_C_STANDARD 17)

find_package(FLEX)
find_package(BISON)

if(FLEX_FOUND AND BISON_FOUND)
    bison_target(parser parser.y parser.tab.c)
    flex_target(lexer lexer.l lexer.c)
    
    add_executable(HelloBison ${BISON_parser_OUTPUTS} ${FLEX_lexer_OUTPUTS})
    target_link_libraries(HelloBison ${FLEX_LIBRARIES})
    target_include_directories(HelloBison PRIVATE ${CMAKE_CURRENT_BINARY_DIR})
endif()