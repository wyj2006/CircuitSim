cmake_minimum_required(VERSION 3.22)

project(CircuitSim)

find_package(SUNDIALS REQUIRED)

file(GLOB_RECURSE sources CONFIGURE_DEPENDS "src/*.cpp")

add_executable(CircuitSim)
target_include_directories(CircuitSim PUBLIC include)
target_sources(CircuitSim PUBLIC ${sources})
target_link_libraries(CircuitSim PUBLIC SUNDIALS::core_static SUNDIALS::ida_static)