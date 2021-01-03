#include <SDL2/SDL.h>
#include <vector>
#include "display.hpp"
#include "../VM/vm.hpp"

Display::Display(void *machine){
  SDL_Init(SDL_INIT_EVERYTHING);
  this->flags  = 0;
  this->sys    = machine;
  this->window = SDL_CreateWindow(
    "CHIP 8 EMULATOR",
    SDL_WINDOWPOS_CENTERED,
    SDL_WINDOWPOS_CENTERED,
    DISPLAY_WIDTH,
    DISPLAY_HEIGHT,
    this->flags
  );
  this->renderer = SDL_CreateRenderer(this->window, GRAPHICS_HARDWARE_SELECTOR, SDL_RENDERER_ACCELERATED);
  this->initializeDisplayInternals();
}

void Display::initializeDisplayInternals(){
  for(int rows = 0; rows <= 31; rows++) {
    for(int cols = 0; cols<=63; cols++) {
      this->display_pixel_data.insert(std::begin(this->display_pixel_data) + rows * cols, 0);
    }
  }
}

void Display::clear(){
  memset(&this->display_pixel_data[0], 0, this->display_pixel_data.size() * sizeof this->display_pixel_data[0]);
  SDL_RenderClear(this->renderer);
}

void Display::render(){
  SDL_LockSurface(this->surface);
  for(int row_index=0; row_index < DISPLAY_HEIGHT; row_index++) {
    for(int col_index=0; col_index < DISPLAY_WIDTH; col_index++) {
      SDL_Rect pixrect;
      pixrect.h = DISPLAY_SCALE;
      pixrect.w = DISPLAY_SCALE;
      pixrect.x = col_index * DISPLAY_SCALE;
      pixrect.y = row_index * DISPLAY_SCALE;
      uint8_t pixelColor = (this->display_pixel_data[(row_index * col_index)] == 1 ? 255 : 0);
      SDL_FillRect(this->surface, &pixrect, SDL_MapRGB(this->surface->format, pixelColor, pixelColor, pixelColor));
    }
  }
  SDL_UnlockSurface(this->surface);  
  this->texture = SDL_CreateTextureFromSurface(this->renderer, this->surface);
  SDL_FreeSurface(this->surface);
  SDL_RenderCopy(this->renderer, this->texture, NULL, NULL);
  SDL_DestroyTexture(this->texture);
  SDL_RenderPresent(this->renderer);
}

void Display::draw_sprite(uint16_t opcode) {
  VM *sys    = (VM *) this->sys;
  uint8_t Vx = sys->fetch_register((opcode & 0x0F00) >> 8);
  uint8_t Vy = sys->fetch_register((opcode & 0x00F0) >> 4);
  uint8_t spriteSize = (opcode & 0x000F);
  uint8_t currentSpriteStartAddr = sys->I;
  uint8_t pixelData;

  sys->set_register(SYS_REG_ADDR, 0); /* used for collision detection */
  for(uint8_t ccolumn=0; ccolumn<spriteSize; ccolumn++) {
    pixelData = sys->memory[currentSpriteStartAddr+ccolumn];
    for(uint8_t crow=0; crow<8; crow++) {
      if(((pixelData) & ((0x80) >> crow)) != 0) {
        if(this->display_pixel_data[(Vx + crow + ((Vy+ccolumn) * 64))] == 1) {
          this->display_pixel_data[(Vx+crow + ((Vy+ccolumn) * 64))] ^= 1;
        }
      }
    }
  }
  sys->flags |= DRAW_F;
}
