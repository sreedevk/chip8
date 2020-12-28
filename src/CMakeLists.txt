cmake_minimum_required(VERSION "3.19.1")
project(chip8 LANGUAGES CXX VERSION "0.0.1")

set(LIBRARIES_PATH /usr/include /usr/lib /usr/share/lib)
set(CMAKE_BUILD_TYPE Debug)
set(CMAKE_CXX_STANDARD 20)

find_library(SDL2_LIBRARY
  NAMES SDL2
  PATHS ${LIBRARY_PATHS}
)

find_library(SDL2_IMAGE_LIBRARY
  NAMES SDL2_image
  PATHS ${LIBRARY_PATHS}
)

add_subdirectory(lib)
add_executable(${PROJECT_NAME} main.cpp)
target_link_libraries(${PROJECT_NAME} ${SDL2_LIBRARY} ${SDL2_IMAGE_LIBRARY} vm keyboard display iseq charset)
