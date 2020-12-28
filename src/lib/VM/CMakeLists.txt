add_library(vm SHARED vm.cpp vm.hpp)
target_link_libraries(vm PUBLIC keyboard)
