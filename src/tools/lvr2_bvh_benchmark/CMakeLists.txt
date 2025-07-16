#####################################################################################
# Add executable
#####################################################################################

find_package(OpenMP REQUIRED)

add_executable(lvr2_bvh_benchmark Main.cpp)
target_link_libraries(lvr2_bvh_benchmark lvr2_static OpenMP::OpenMP_CXX)

install(TARGETS lvr2_bvh_benchmark
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR})
