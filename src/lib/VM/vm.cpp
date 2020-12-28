#include <cstdint>
#include <cstdlib>
#include <stdio.h>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include "vm.hpp"
#include <sys/stat.h>
#include <algorithm>
#include "../iseq/iseq.hpp"
#include "../display/display.hpp"
#include "../keyboard/keyboard.hpp"

VM::VM(){ 
  initialize_memory();
  initialize_gp_registers();
  initialize_stack();
}

void VM::initialize_memory(){
  this->memory = (uint16_t *) calloc(MEMORY_SIZE, sizeof(uint16_t));
}

void VM::initialize_gp_registers(){
  this->V = { 0 };
}

void VM::initialize_stack(){
  this->STACK = { 0 };
}

void VM::initialize_program_counter(){
  this->PC = PROGRAM_START_ADDR;
}

void VM::initialize_peripherals(){
  this->display  = new Display((void *) this);
  this->keyboard = new Keyboard();
}

void VM::load_program(std::string pgmfile){
  std::fstream program_file;
  program_file.open(pgmfile, std::ios::in | std::ios::binary);

  this->program_size = program_file.seekg(0, std::ios::end).tellg();
  program_file.seekg(0);

  uint16_t temp_pc = this->PC;
  while(program_file.tellg() < this->program_size) {
    char *buffer = (char *) calloc(2, sizeof(char));
    program_file.read(buffer, 2);
    std::swap(buffer[0], buffer[1]);
    *(this->memory+(++temp_pc)) = (uint16_t) *buffer;
  }
}

uint8_t VM::fetch_register(uint8_t register_addr){
  return this->V[register_addr];
}

void VM::set_register(uint8_t value, uint8_t register_addr){
  this->V[register_addr] = value;
}

void VM::playSound() {
  std::cout << "SOUND PLAYED" << std::endl;
}

void VM::update_timers(){
  if(this->DT > 0) this->DT--;
  if(this->ST > 0) this->ST--;
  if(this->ST == 0) this->playSound();
}

void VM::exec(){
  this->run = true;
  while(run) {
    Iseq *instruction = new Iseq(this, this->memory[this->PC]);
    delete(instruction);
    this->update_timers();
  }
}
