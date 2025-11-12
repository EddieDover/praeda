cmake_minimum_required(VERSION 3.10)
project(PraedaTest)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find the praeda library
find_library(PRAEDA_LIB praeda PATHS ../../target/release)

if(NOT PRAEDA_LIB)
    message(FATAL_ERROR "Praeda library not found. Please build it first with: cargo build --release")
endif()

# Create the test executable
add_executable(test_praeda test_praeda.cpp)

# Include the praeda header from the project root
target_include_directories(test_praeda PRIVATE ../../include)

# Link the praeda library
target_link_libraries(test_praeda ${PRAEDA_LIB})

# Set runtime library path
if(UNIX AND NOT APPLE)
    set_target_properties(test_praeda PROPERTIES
        BUILD_RPATH "../../target/release"
        INSTALL_RPATH "../../target/release"
    )
endif()

message(STATUS "Praeda library: ${PRAEDA_LIB}")
message(STATUS "Build type: ${CMAKE_BUILD_TYPE}")
