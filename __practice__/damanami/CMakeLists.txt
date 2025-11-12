cmake_minimum_required(VERSION 3.20)
project(damanami C CXX)
set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED True)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(SDL3_BASE_PATH "C:/sdl/SDL3-3.2.26")
set(SDL3_TTF_BASE_PATH "C:/sdl/SDL3_ttf-devel-3.2.2-VC/SDL3_ttf-3.2.2")
add_executable(damanami main.c bgfx_bridge.cpp)
if (WIN32)
    set(WINDOWS_SYSTEM_LIBS
            user32.lib
            gdi32.lib
            winmm.lib
            imm32.lib
            version.lib
            shell32.lib
            setupapi.lib
            Usp10.lib
            Rpcrt4.lib
    )
    target_include_directories(damanami PRIVATE
            "${SDL3_BASE_PATH}/include"
            "${SDL3_TTF_BASE_PATH}/include"
            $<$<COMPILE_LANGUAGE:CXX>:C:/Users/igor/scoop/apps/vcpkg/current/installed/x64-windows/include>
            $<$<COMPILE_LANGUAGE:CXX>:C:/Users/igor/scoop/apps/vcpkg/current/installed/x64-windows/include/bx>
            $<$<COMPILE_LANGUAGE:CXX>:C:/Users/igor/scoop/apps/vcpkg/current/installed/x64-windows/include/bimg>
    )
    target_link_directories(damanami PRIVATE
            "${SDL3_BASE_PATH}/lib/x64"
            "${SDL3_TTF_BASE_PATH}/lib/x64"
            "${BGFX_BIN_PATH}"
    )
    target_link_libraries(damanami PRIVATE
            SDL3.lib
            SDL3_ttf.lib
            C:/Users/igor/scoop/apps/vcpkg/current/buildtrees/bgfx/x64-windows-dbg/cmake/bgfx/bgfx.lib
            C:/Users/igor/scoop/apps/vcpkg/current/buildtrees/bgfx/x64-windows-dbg/cmake/bimg/bimg.lib
            C:/Users/igor/scoop/apps/vcpkg/current/buildtrees/bgfx/x64-windows-dbg/cmake/bx/bx.lib
            ${WINDOWS_SYSTEM_LIBS}
    )
    set(DEST_DIR "$<TARGET_FILE_DIR:damanami>")
    add_custom_command(TARGET damanami POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy
            "${SDL3_BASE_PATH}/lib/x64/SDL3.dll"
            ${DEST_DIR}
            COMMAND ${CMAKE_COMMAND} -E copy
            "${SDL3_TTF_BASE_PATH}/lib/x64/SDL3_ttf.dll"
            ${DEST_DIR}
    )

    target_compile_definitions(damanami PRIVATE
            $<$<CONFIG:Debug>:BX_CONFIG_DEBUG=1>
            $<$<NOT:$<CONFIG:Debug>>:BX_CONFIG_DEBUG=0>
    )

    if (MSVC)
        target_compile_options(damanami PRIVATE $<$<COMPILE_LANGUAGE:CXX>:/Zc:__cplusplus /Zc:preprocessor>)
    endif ()
endif ()