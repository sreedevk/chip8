#pragma once
#include <cstdint>
#include <array>
#include <string>
#include <cstddef>
#include <unordered_map>
#include "../keyboard/keyboard.hpp"
#include "../display/display.hpp"

#define PROGRAM_START_ADDR 0x200
#define MEMORY_END_ADDR    0xFFF
#define FLAG_REG_ADDR      0xF
#define MEMORY_SIZE        4096
#define STACK_SIZE         16
#define SYS_REG_ADDR       0xF
#define CHAR_SPRITE_SIZE   5
#define SPRITE_START_ADDR  0x050
#define FRAMERATE          60
#define HALT_F             1
#define DRAW_F             2

class VM {
  private:
    std::array<uint8_t, 16>  V;
    int                      program_size;
    void update_timers();
    void playSound();
    std::string inspect_stack();
    std::string inspect_memory();
    std::string inspect_registers();
    std::string inspect_timers();

  public:
    std::array<unsigned char, MEMORY_SIZE> memory;
    uint16_t                   PC;     /* Program Counter */
    uint8_t                    SP;     /* Stack Pointer */
    uint8_t                    flags;
    bool                       run;
    std::array<uint16_t, 16>   STACK;
    uint16_t                   I;
    uint8_t                    DT, ST; /*Delay Timer & Sound Timer*/
    Display                    *display;
    Keyboard                   *keyboard;

    VM();
    ~VM();
    void load_program(std::string);
    void load_charset();
    uint8_t fetch_register(uint8_t);
    void incr_pc();
    void set_register(uint8_t value, uint8_t register_addr);
    void emulate_cycle();
    void exec();
    void destroy_internals();
    void print_machine_state();
    std::unordered_map<std::string, std::string> inspect();
};
