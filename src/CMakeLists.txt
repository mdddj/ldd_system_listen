# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

project(ldd_system_listen_library VERSION 0.0.1 LANGUAGES C)

add_library(ldd_system_listen SHARED
  "ldd_system_listen.c"
)

set_target_properties(ldd_system_listen PROPERTIES
  PUBLIC_HEADER ldd_system_listen.h
  OUTPUT_NAME "ldd_system_listen"
)

target_compile_definitions(ldd_system_listen PUBLIC DART_SHARED_LIB)
