add_executable(core_capa_module main.cpp)

set_target_properties(core_capa_module PROPERTIES
        CXX_STANDARD 17
        CXX_STANDARD_REQUIRED TRUE
)

target_link_libraries(core_capa_module)