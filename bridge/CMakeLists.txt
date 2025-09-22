
cmake_minimum_required(VERSION 3.25)
project(pocketbridge VERSION "0.1.0" LANGUAGES C CXX)

# Set policy for install path normalization (CMake 3.31+)
if(POLICY CMP0177)
    cmake_policy(SET CMP0177 NEW)
endif()

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED ON)

# Pocket Bridge Configuration Options
option(POCKET_DISABLE_LOCK "Disable locking mechanisms in Pocket library" ON)
option(POCKET_DISABLE_DB_LOCK "Disable database locking in Pocket library" OFF)
option(POCKET_ENABLE_LOG "Enable logging in Pocket library" ON)
option(POCKET_ENABLE_AES "Enable AES encryption in Pocket library" OFF)
option(POCKET_ENABLE_TEST "Enable test compilation in Pocket library" OFF)

# Advanced configuration options
set(POCKET_MAX_BUFFER_RESPONSE_SIZE 10485760 CACHE STRING "Maximum buffer response size for Pocket library")
set(POCKET_FORCE_TIMESTAMP_LAST_UPDATE 0 CACHE STRING "Force timestamp last update to specific value (0 = disabled)")

# Display configuration summary
message(STATUS "Pocket Bridge Configuration:")
message(STATUS "  POCKET_DISABLE_LOCK: ${POCKET_DISABLE_LOCK}")
message(STATUS "  POCKET_DISABLE_DB_LOCK: ${POCKET_DISABLE_DB_LOCK}")
message(STATUS "  POCKET_ENABLE_LOG: ${POCKET_ENABLE_LOG}")
message(STATUS "  POCKET_ENABLE_AES: ${POCKET_ENABLE_AES}")
message(STATUS "  POCKET_ENABLE_TEST: ${POCKET_ENABLE_TEST}")
message(STATUS "  POCKET_MAX_BUFFER_RESPONSE_SIZE: ${POCKET_MAX_BUFFER_RESPONSE_SIZE}")
message(STATUS "  POCKET_FORCE_TIMESTAMP_LAST_UPDATE: ${POCKET_FORCE_TIMESTAMP_LAST_UPDATE}")

message(STATUS "PROJECT_SOURCE_DIR=${PROJECT_SOURCE_DIR}")
message(STATUS "CMAKE_BINARY_DIR=${CMAKE_BINARY_DIR}")

# Include FetchContent for dependencies
include(FetchContent)

# Fetch nlohmann_json - same as in pocket-lib
message(STATUS "start nlohmann_json download")
FetchContent_Declare(
        nlohmann_json
        GIT_REPOSITORY https://github.com/nlohmann/json.git
        GIT_TAG v3.11.3
)
set(nlohmann_json_INSTALL OFF CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(nlohmann_json)

# Fetch tinyxml2 - same as in pocket-lib
message(STATUS "start tinyxml2 download")
FetchContent_Declare(
        tinyxml2
        GIT_REPOSITORY https://github.com/leethomason/tinyxml2.git
        GIT_TAG 11.0.0
)
set(tinyxml2_INSTALL OFF CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(tinyxml2)

add_subdirectory(pocket-lib)

set(LIB_DIR ${CMAKE_SOURCE_DIR}/libs)

file(GLOB_RECURSE POCKET_INCLUDES CONFIGURE_DEPENDS "inc/pocket/*.h")
file(GLOB_RECURSE POCKET_SOURCES CONFIGURE_DEPENDS "src/*.cpp" "src/*.c")

add_library(${PROJECT_NAME} STATIC
        ${POCKET_INCLUDES} ${POCKET_SOURCES}
)

# Create alias target for better usability
add_library(PocketBridge::pocketbridge ALIAS ${PROJECT_NAME})

# Configure compile definitions based on options
target_compile_definitions(${PROJECT_NAME} PUBLIC
    POCKET_MAX_BUFFER_RESPONSE_SIZE=${POCKET_MAX_BUFFER_RESPONSE_SIZE}
)

if(POCKET_DISABLE_LOCK)
    target_compile_definitions(${PROJECT_NAME} PUBLIC POCKET_DISABLE_LOCK)
endif()

if(POCKET_DISABLE_DB_LOCK)
    target_compile_definitions(${PROJECT_NAME} PUBLIC POCKET_DISABLE_DB_LOCK)
endif()

if(POCKET_ENABLE_LOG)
    target_compile_definitions(${PROJECT_NAME} PUBLIC POCKET_ENABLE_LOG)
endif()

if(POCKET_ENABLE_AES)
    target_compile_definitions(${PROJECT_NAME} PUBLIC POCKET_ENABLE_AES)
endif()

if(POCKET_FORCE_TIMESTAMP_LAST_UPDATE AND NOT POCKET_FORCE_TIMESTAMP_LAST_UPDATE EQUAL 0)
    target_compile_definitions(${PROJECT_NAME} PUBLIC POCKET_FORCE_TIMESTAMP_LAST_UPDATE=${POCKET_FORCE_TIMESTAMP_LAST_UPDATE})
endif()

if(POCKET_ENABLE_TEST)
    target_compile_definitions(${PROJECT_NAME} PUBLIC POCKET_ENABLE_TEST)
endif()

target_link_directories(${PROJECT_NAME} PRIVATE ${LIB_DIR})

target_include_directories(${PROJECT_NAME} 
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/inc>
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/pocket-lib/inc>
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/pocket-lib/pocket-controllers/inc>
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/pocket-lib/pocket-daos/inc>
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/pocket-lib/pocket-iface/inc>
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/pocket-lib/pocket-pods/inc>
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/pocket-lib/pocket-services/inc>
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/pocket-lib/pocket-views/inc>
        $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
)

target_link_libraries(${PROJECT_NAME} 
    PUBLIC
        pocket
        nlohmann_json::nlohmann_json
        tinyxml2
)

set_target_properties(${PROJECT_NAME} PROPERTIES LINK_FLAGS "-Wl,-Map=${PROJECT_NAME}.map")

# Install the library to a standard location
install(TARGETS ${PROJECT_NAME} 
    ARCHIVE DESTINATION lib
    LIBRARY DESTINATION lib
    RUNTIME DESTINATION bin
)