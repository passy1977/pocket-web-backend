cmake_minimum_required(VERSION 3.25)
project(pocketbridge VERSION "0.1.0" LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED ON)

message(STATUS "PROJECT_SOURCE_DIR=${PROJECT_SOURCE_DIR}")
message(STATUS "CMAKE_BINARY_DIR=${CMAKE_BINARY_DIR}")

add_definitions(-DPOCKET_ENABLE_LOG)
add_definitions(-DPOCKET_ENABLE_AES)
add_definitions(-DPOCKET_MAX_BUFFER_RESPONSE_SIZE=10485760)
add_definitions(-DPOCKET_DISABLE_LOCK)

include_directories(pocket-lib/inc)
include_directories(pocket-lib/pocket-controllers/inc)
include_directories(pocket-lib/pocket-daos/inc)
include_directories(pocket-lib/pocket-iface/inc)
include_directories(pocket-lib/pocket-pods/inc)
include_directories(pocket-lib/pocket-services/inc)
include_directories(pocket-lib/pocket-views/inc)

add_subdirectory(pocket-lib)

set(LIB_DIR ${CMAKE_SOURCE_DIR}/libs)

file(GLOB_RECURSE POCKET_INCLUDES CONFIGURE_DEPENDS "inc/pocket/*.h")
file(GLOB_RECURSE POCKET_SOURCES CONFIGURE_DEPENDS "src/*.cpp" "src/*.c")

add_library(${PROJECT_NAME} STATIC
        ${POCKET_INCLUDES} ${POCKET_SOURCES}
        inc/pocket/group_field.h
        src/group_field.cpp
        inc/pocket/user.h
        src/user.cpp
)

target_link_directories(${PROJECT_NAME} PRIVATE ${LIB_DIR})

target_include_directories(${PROJECT_NAME} PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}/inc
)

target_sources(${PROJECT_NAME} PUBLIC
        ${POCKET_INCLUDES} ${POCKET_SOURCES}
)

target_link_libraries(${PROJECT_NAME} pocket)

install(TARGETS ${PROJECT_NAME} DESTINATION ${CMAKE_BINARY_DIR}/.)