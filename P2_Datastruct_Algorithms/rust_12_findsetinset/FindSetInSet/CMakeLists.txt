cmake_minimum_required(VERSION 3.12)
project(MyProject)

find_package(fmt REQUIRED)

add_executable(findsetinset findsetinset.cpp)
target_link_libraries(findsetinset PRIVATE fmt::fmt)