#include <SDL2/SDL.h>
#include <SDL2/SDL_keyboard.h>
#include "keyboard.hpp"
#include <array>

Keyboard::Keyboard() {
  this->keystates = SDL_GetKeyboardState(NULL);
  initializeKeyMap();
}

void Keyboard::initializeKeyMap(){
  this->key_ids = {
    0x1, 0x2, 0x3, 0xC,
    0x4, 0x5, 0x6, 0xD,
    0x7, 0x8, 0x9, 0xE,
    0xA, 0x0, 0xB, 0xF
  };

  this->keymap[0x1] = SDL_SCANCODE_1;
  this->keymap[0x2] = SDL_SCANCODE_2;
  this->keymap[0x3] = SDL_SCANCODE_3;
  this->keymap[0xC] = SDL_SCANCODE_4;

  this->keymap[0x4] = SDL_SCANCODE_Q;
  this->keymap[0x5] = SDL_SCANCODE_W;
  this->keymap[0x6] = SDL_SCANCODE_E;
  this->keymap[0xD] = SDL_SCANCODE_R;

  this->keymap[0x7] = SDL_SCANCODE_A;
  this->keymap[0x8] = SDL_SCANCODE_S;
  this->keymap[0x9] = SDL_SCANCODE_D;
  this->keymap[0xE] = SDL_SCANCODE_F;

  this->keymap[0xA] = SDL_SCANCODE_Z;
  this->keymap[0x0] = SDL_SCANCODE_X;
  this->keymap[0xB] = SDL_SCANCODE_C;
  this->keymap[0xF] = SDL_SCANCODE_V;
}

uint8_t Keyboard::checkKeyState(uint8_t keycode) {
  SDL_PumpEvents();
  
  if(this->keystates[this->keymap[keycode]]) return 1u;
  return 0u;
}

uint8_t Keyboard::expectKeyDown(){
  bool anyKeyPressed = false;
  uint8_t pressedKey;
  while(!anyKeyPressed) {
    SDL_PumpEvents();
    for(int keyi=0; keyi < this->key_ids.size(); keyi++) {
      if(this->keystates[this->keymap[this->key_ids[keyi]]]) {
        anyKeyPressed = true;
        pressedKey = this->key_ids[keyi];
        break;
      }
    }
  }
  return pressedKey;
} 
