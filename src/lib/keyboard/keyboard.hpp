#pragma once
#include <cstdint>
#include <cstdlib>
#include <SDL2/SDL.h>
#include <unordered_map>
#include <array>

class Keyboard {
  public:
    std::unordered_map<int, int> keymap;
    const Uint8 *keystates;
    std::array<int, 16> key_ids;

    Keyboard();
    uint8_t checkKeyState(uint8_t);
    void initializeKeyMap();
    uint8_t expectKeyDown();
};
