#include <cstdint>
#include <cstdlib>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <bitset>
#include <iomanip>
#include "vm.hpp"
#include <sys/stat.h>
#include <algorithm>
#include <unordered_map>
#include "../iseq/iseq.hpp"
#include "../display/display.hpp"
#include "../keyboard/keyboard.hpp"
#include "../log/log.hpp"
#include "../charset/charset.hpp"
#include <SDL2/SDL.h>
#include <SDL2/SDL_keyboard.h>

VM::VM(){ 
  this->V        = { 0 };
  this->STACK    = { 0 };
  this->SP       = 0;
  this->I        = 0;
  this->PC       = PROGRAM_START_ADDR;
  this->DT       = 0;
  this->ST       = 0;
  this->display  = new Display((void *) this);
  this->keyboard = new Keyboard();
  this->load_charset();
}

void VM::load_charset(){
  for(auto i=0; i<80; i++) {
    this->memory[i] = Charset::chip8_charset[i];
  }
}

void VM::load_program(std::string pgmfile){
  std::ifstream program_file(pgmfile, std::ios::binary | std::ios::ate);
  this->program_size = program_file.tellg();
  program_file.seekg(0, std::ios::beg);

  char *rom_buffer = (char *) malloc(this->program_size);
  program_file.read(rom_buffer, this->program_size);
  for(int i=0; i<this->program_size; ++i) {
    this->memory[PROGRAM_START_ADDR+i] = (unsigned char) *(rom_buffer+i);
  }
  free(rom_buffer);
}

uint8_t VM::fetch_register(uint8_t register_addr){
  return this->V[register_addr];
}

void VM::set_register(uint8_t value, uint8_t register_addr){
  this->V[register_addr] = value;
}

void VM::playSound() {
  /* std::cout << "SOUND PLAYED" << std::endl; */
}

void VM::update_timers(){
  if(this->DT > 0) this->DT--;
  if(this->ST > 0) this->ST--;
  if(this->ST == 0) this->playSound();
}

void VM::incr_pc(){
  this->PC+=2;
}

void VM::emulate_cycle(){
  uint16_t next_opcode;
  next_opcode = this->memory[this->PC] << 8 | this->memory[this->PC+1];
  this->incr_pc();
  Iseq *instruction = new Iseq(this, next_opcode);
  delete(instruction);
  this->update_timers();
}

void VM::exec(){
  this->run = true;
  while(run) {
    this->emulate_cycle();
    if(this->keyboard->keystates[SDL_SCANCODE_ESCAPE]) this->run = false;
  }
  exit(0);
}

void VM::destroy_internals(){
  delete(this->display);
}

VM::~VM(){
  this->destroy_internals();
}


/* LOGGING INFO */

std::unordered_map<std::string, std::string> VM::inspect() {
  std::unordered_map<std::string, std::string> info_table;
  std::ostringstream info_buffer;
  info_table["STACK"]           = this->inspect_stack();
  info_table["MEMORY_LAYOUT"]   = this->inspect_memory();
  info_table["GP_REGISTERS"]    = this->inspect_registers();
  info_table["TIMERS"]          = this->inspect_timers();

  info_buffer << "0x" << std::hex << std::setw(4) << std::setfill('0') << this->PC << std::endl;
  info_table["PROGRAM_COUNTER"] = info_buffer.str();
  info_buffer.str(""); info_buffer.clear();

  info_buffer << std::hex << std::setw(4) << std::setfill('0') << this->SP << std::endl;
  info_table["STACK_POINTER"]   = info_buffer.str();;
  info_buffer.str(""); info_buffer.clear();

  uint16_t current_opcode = this->memory[this->PC] << 8 | this->memory[this->PC+1];
  info_buffer << "0x" << std::hex << std::setw(4) << std::setfill('0') << current_opcode << std::endl;
  info_table["CURRENT_OPCODE"]  = info_buffer.str();;
  info_buffer.str(""); info_buffer.clear();

  info_buffer << "0x" << std::hex << std::setw(4) << std::setfill('0') << this->I << std::endl << std::endl;
  info_table["I_REGISTER"]      = info_buffer.str();
  info_buffer.str(""); info_buffer.clear();

  info_buffer << std::bitset<8>(this->flags).to_string() << std::endl;
  info_table["FLAGS"]           = info_buffer.str();
  info_buffer.str(""); info_buffer.clear();

  return info_table;
}

std::string VM::inspect_stack(){
  std::ostringstream stack_data;
  for (auto i = std::begin(this->STACK); i != std::end(this->STACK); ++i)  {
    stack_data << *i << ' ';
  }
  stack_data << "\n";
  return stack_data.str();
}

std::string VM::inspect_memory() {
  std::ostringstream mem_map;
  mem_map.str("");
  mem_map.clear();
  mem_map << std::endl;
  mem_map << std::endl << Log::format_color(" ---- PROGRAM MEMORY  ----", LOG_BCOLOR_YELLOW) << std::endl; 

  int tmp_pc = PROGRAM_START_ADDR;
  while(tmp_pc < (PROGRAM_START_ADDR+this->program_size)) {
    uint16_t ci_opcode = this->memory[tmp_pc] << 8 | this->memory[++tmp_pc];
    mem_map << std::hex << std::setw(4) << std::setfill('0');
    if(tmp_pc == this->PC || tmp_pc == this->PC+1) {
      mem_map << Log::format_color(std::to_string(ci_opcode), LOG_BCOLOR_GREEN) << ' ';
    } else {
      mem_map << ci_opcode << ' ';
    }
    if((tmp_pc+1) % 16 == 0) mem_map << std::endl;
    tmp_pc++;
  }

  mem_map << std::endl;
  return mem_map.str();
}

std::string VM::inspect_registers(){
  std::ostringstream reg_map;
  reg_map << "\n";
  for(int i=0; i<16; i++) {
    reg_map << "V[" << i << "]: " << std::hex << std::setw(4) << std::setfill('0') << unsigned(fetch_register(i)) << "\t";
    if((i+1) % 4 == 0) reg_map << std::endl;
  }
  reg_map << std::endl;
  return reg_map.str();
}

std::string VM::inspect_timers(){
  std::ostringstream timers;
  timers << "DELAY TIMER(" << unsigned(this->DT) << ") " << "SOUND TIMER(" << unsigned(this->ST) << ")\n";
  return timers.str();
}

void VM::print_machine_state(){
  Log::print_vm_info(this->inspect());
}
