#pragma once
#include <cstdint>
#include <vector>
#include <SDL2/SDL.h>

#define DISPLAY_SCALE               5
#define DISPLAY_WIDTH               64 * DISPLAY_SCALE
#define DISPLAY_HEIGHT              32 * DISPLAY_SCALE
#define GRAPHICS_HARDWARE_SELECTOR -1

class Display {
  private:
    SDL_Window                        *window;
    SDL_Renderer                      *renderer;
    SDL_Texture                       *texture;
    SDL_Surface                       *surface;
    void                              *sys;
    int                               flags;
    std::vector<uint8_t>              display_pixel_data;

  public:
    void clear();
    void draw_sprite(uint16_t opcode);
    void initializeDisplayInternals();
    void render();
    Display(void *);
};
