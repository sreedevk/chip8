#pragma once
#include <cstdint>
#include <string>
#include "../VM/vm.hpp"

/*
nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
n or nibble - A 4-bit value, the lowest 4 bits of the instruction
x - A 4-bit value, the lower 4 bits of the high byte of the instruction
y - A 4-bit value, the upper 4 bits of the low byte of the instruction
kk or byte - An 8-bit value, the lowest 8 bits of the instruction
*/

class Iseq {
  public:
    VM *sys;
    void parse(std::string);

    Iseq(VM *, uint16_t opcode);
    void process(uint16_t opcode);
    void handle_class0_opcode(uint16_t opcode);
    void handle_class1_opcode(uint16_t opcode);
    void handle_class2_opcode(uint16_t opcode);
    void handle_class3_opcode(uint16_t opcode);
    void handle_class4_opcode(uint16_t opcode);
    void handle_class5_opcode(uint16_t opcode);
    void handle_class6_opcode(uint16_t opcode);
    void handle_class7_opcode(uint16_t opcode);
    void handle_class8_opcode(uint16_t opcode);
    void handle_class9_opcode(uint16_t opcode);
    void handle_classA_opcode(uint16_t opcode);
    void handle_classB_opcode(uint16_t opcode);
    void handle_classC_opcode(uint16_t opcode);
    void handle_classD_opcode(uint16_t opcode);
    void handle_classE_opcode(uint16_t opcode);
    void handle_classF_opcode(uint16_t opcode);

};
