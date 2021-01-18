#pragma once
#include <cstdint>
#include <array>
#include <SDL2/SDL.h>

#define DISPLAY_SCALE               10
#define DISPLAY_WIDTH               64
#define DISPLAY_HEIGHT              32
#define GRAPHICS_HARDWARE_SELECTOR  -1

class Display {
  private:
    SDL_Window                        *window;
    SDL_Renderer                      *renderer;
    void                              *sys;
    int                               flags;
    std::array<uint8_t, DISPLAY_WIDTH * DISPLAY_HEIGHT>        display_pixel_data;

  public:
    void clear();
    void draw_sprite(uint16_t opcode);
    void initializeDisplayInternals();
    void loadSplash();
    void render();
    Display(void *);
    ~Display();
};
