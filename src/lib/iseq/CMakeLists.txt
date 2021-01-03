add_library(iseq SHARED iseq.cpp iseq.hpp)
target_link_libraries(iseq PUBLIC keyboard)
target_link_libraries(iseq PUBLIC log)
