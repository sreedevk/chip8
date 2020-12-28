#pragma once
#include <cstdint>

class Keyboard {
  public:
    Keyboard();
    uint8_t expectKeyDown();
};
