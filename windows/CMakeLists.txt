cmake_minimum_required(VERSION 3.10)
set(PROJECT_NAME "ldd_system_listen")
project(${PROJECT_NAME} LANGUAGES CXX)
include("../cargokit/cmake/cargokit.cmake")
apply_cargokit(${PROJECT_NAME} ../rust ldd_system_listen "")
set(ldd_system_listen_bundled_libraries
  "${${PROJECT_NAME}_cargokit_lib}"
  PARENT_SCOPE
)