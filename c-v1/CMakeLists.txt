cmake_minimum_required(VERSION 3.12)

project(shocked
    VERSION 0.0.1)
set(CMAKE_C_STANDARD 90)
set(CMAKE_C_STANDARD_REQUIRED True)
set(CMAKE_GENERATOR_PLATFORM x64)

# These need to be updated to fit per-person. The current FindCurses module
# in CMake and vcpkg.cmake script does not properly allow pdcurses to be found
# automatically.
set(CMAKE_PREFIX_PATH "C:\\Users\\aytas\\prog\\vcpkg\\packages\\pdcurses_x64-windows\\include")
set(PDCURSES_INCLUDE_DIR "C:\\Users\\aytas\\prog\\vcpkg\\packages\\pdcurses_x64-windows\\include")
set(PDCURSES_LIBRARIES "C:\\Users\\aytas\\prog\\vcpkg\\packages\\pdcurses_x64-windows\\lib\\pdcurses.lib")

add_executable(shocked
    ${PROJECT_SOURCE_DIR}/src/main.c)
target_include_directories(shocked
    PUBLIC "${PROJECT_SOURCE_DIR}/include")
configure_file(${PROJECT_SOURCE_DIR}/config/compile/shocked_cmake_config.h.in
    ${PROJECT_SOURCE_DIR}/include/shocked/generated/shocked_cmake_config.h)

target_include_directories(shocked PRIVATE ${PDCURSES_INCLUDE_DIR})
target_link_libraries(shocked PRIVATE ${PDCURSES_LIBRARIES})
