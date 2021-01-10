#include <SDL2/SDL.h>
#include "display.hpp"
#include <array>
#include "../VM/vm.hpp"

Display::Display(void *machine){
  SDL_Init(SDL_INIT_EVERYTHING);
  this->flags  = 0;
  this->sys    = machine;
  this->window = SDL_CreateWindow(
    "CHIP 8 EMULATOR",
    SDL_WINDOWPOS_CENTERED,
    SDL_WINDOWPOS_CENTERED,
    DISPLAY_WIDTH * DISPLAY_SCALE,
    DISPLAY_HEIGHT * DISPLAY_SCALE,
    this->flags
  );
  this->renderer = SDL_CreateRenderer(this->window, GRAPHICS_HARDWARE_SELECTOR, SDL_RENDERER_ACCELERATED);
  this->initializeDisplayInternals();
  clear();
}

void Display::initializeDisplayInternals(){
  for(int rows = 0; rows < DISPLAY_HEIGHT; rows++) {
    for(int cols = 0; cols < DISPLAY_WIDTH; cols++) {
      this->display_pixel_data[rows*cols] = 0;
    }
  }
}

void Display::clear(){
  memset(&this->display_pixel_data[0], 0, this->display_pixel_data.size() * sizeof this->display_pixel_data[0]);
  SDL_RenderClear(this->renderer);
  SDL_RenderPresent(this->renderer);
}

void Display::loadSplash() {
  SDL_Rect srect;
  srect.x = 0;
  srect.y = 0;
  srect.w = 10;
  srect.h = 10;

  SDL_SetRenderDrawColor(this->renderer, 128, 0, 128, 255);
  SDL_RenderFillRect(this->renderer, &srect);
  SDL_RenderPresent(this->renderer);
}

void Display::render(){
  for(int row_index=0; row_index < DISPLAY_HEIGHT; row_index++) {
    for(int col_index=0; col_index < DISPLAY_WIDTH; col_index++) {
      SDL_Rect pixrect;
      pixrect.h = DISPLAY_SCALE;
      pixrect.w = DISPLAY_SCALE;
      pixrect.x = col_index * DISPLAY_SCALE;
      pixrect.y = row_index * DISPLAY_SCALE;
      if(this->display_pixel_data[row_index * col_index] == 1) {
        SDL_SetRenderDrawColor(this->renderer, 255, 255, 255, 255);
      } else {
        SDL_SetRenderDrawColor(this->renderer, 0, 0, 0, 255);
      }
      SDL_RenderFillRect(this->renderer, &pixrect);
    }
  }
  SDL_RenderPresent(this->renderer);
}


void Display::draw_sprite(uint16_t opcode) {
  VM *sys     = (VM *) this->sys;
  sys->set_register(0xF, 0); /* collision detection flag set to 0 */
  int sprite_size = (opcode & 0x000Fu);
  uint8_t display_x = sys->fetch_register(opcode & 0x0F00);
  uint8_t display_y = sys->fetch_register(opcode & 0x00F0);

  for(int i=0; i < sprite_size; i++) {
    uint8_t sprite_row = sys->memory[sys->I+i];
    int display_yi = 0;
    for(int column=0x01; column < 0xFF; column <<= 1) {
      int sprite_column_px = (sprite_row & column) == 0 ? 0 : 1;
      if(this->display_pixel_data[(display_x + i) * (display_y + display_yi)] == 1) sys->set_register(0xF, 1);
      this->display_pixel_data[(display_x + i) * (display_y + display_yi)] = sprite_column_px;
      display_yi++;
    }
  }
  render();
}

Display::~Display(){
  SDL_DestroyRenderer(this->renderer);
  SDL_DestroyWindow(this->window);
  SDL_Quit();
}
