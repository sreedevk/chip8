#include "../VM/vm.hpp"
#include "iseq.hpp"
#include "../keyboard/keyboard.hpp"
#include "../log/log.hpp"
#include <iostream>

Iseq::Iseq(VM *machine, uint16_t opcode) {
  this->sys = machine;
  process(opcode);
}

void Iseq::process(uint16_t opcode){
  //uint16_t opcode = this->sys->;
  switch(opcode & 0xF000){
    case 0x0000: // contains more data in the last nibble
      handle_class0_opcode(opcode);
      break;
    case 0x1000:
      handle_class1_opcode(opcode);
      break;
    case 0x2000:
      handle_class2_opcode(opcode);
      break;
    case 0x3000:
      handle_class3_opcode(opcode);
      break;
    case 0x4000:
      handle_class4_opcode(opcode);
      break;
    case 0x5000:
      handle_class5_opcode(opcode);
      break;
    case 0x6000:
      handle_class6_opcode(opcode);
      break;
    case 0x7000:
      handle_class7_opcode(opcode);
      break;
    case 0x8000:
      handle_class8_opcode(opcode);
      break;
    case 0x9000:
      handle_class9_opcode(opcode);
      break;
    case 0xA000:
      handle_classA_opcode(opcode);
      break;
    case 0xB000:
      handle_classB_opcode(opcode);
      break;
    case 0xC000:
      handle_classC_opcode(opcode);
      break;
    case 0xD000:
      handle_classD_opcode(opcode);
      break;
    case 0xE000:
      handle_classE_opcode(opcode);
      break;
    case 0xF000:
      handle_classF_opcode(opcode);
      break;
    default:
      Log::unsupported_opcode(opcode);
      this->sys->print_machine_state();
      this->sys->run = false;
      break;
  };
}


void Iseq::handle_class0_opcode(uint16_t opcode) {
  switch(opcode & 0x0FFF) {
    case 0x00E0:
      /*00E0 - CLS*/
      this->sys->display->clear();
      this->sys->incr_pc();
      break;
    case 0x00EE:
      /*00EE - RET*/
      if(this->sys->SP > 0) {
        this->sys->PC = (this->sys->STACK[--this->sys->SP]);
        this->sys->incr_pc();
      } else {
        Log::program_error();
        this->sys->print_machine_state();
        this->sys->run = false;
      }
      break;
    default:
      this->sys->incr_pc();
      break;
  }
}

void Iseq::handle_class1_opcode(uint16_t opcode){
  /*1nnn - JP*/
  this->sys->PC = (opcode & 0x0FFF);
}

void Iseq::handle_class2_opcode(uint16_t opcode) {
  /*2nnn - CALL*/
  this->sys->STACK[this->sys->SP] = this->sys->PC;
  this->sys->SP++;
  this->sys->PC = opcode & 0x0FFF;
}

void Iseq::handle_class3_opcode(uint16_t opcode) {
  /*3xkk - SE Vx, byte*/
  uint8_t comp_val   = opcode & 0x00FF;
  uint8_t reg_value  = this->sys->fetch_register((opcode & 0x0F00) >> 8);
  if(reg_value == comp_val) this->sys->incr_pc();
  this->sys->incr_pc();
}

void Iseq::handle_class4_opcode(uint16_t opcode) {
  /*4xkk - SNE Vx, byte*/
  uint8_t comp_val  = opcode & 0x00FF;
  uint8_t reg_value = this->sys->fetch_register((opcode & 0x0F00) >> 8);
  if(reg_value != comp_val) this->sys->incr_pc();
  this->sys->incr_pc();
}

void Iseq::handle_class5_opcode(uint16_t opcode) {
  /*5xy0 - SE Vx, Vy*/
  uint8_t x_value = this->sys->fetch_register(((opcode & 0x0F00) >> 8));
  uint8_t y_value = this->sys->fetch_register(((opcode & 0x00F0) >> 4));
  if(x_value == y_value) this->sys->incr_pc();
  this->sys->incr_pc();
}

void Iseq::handle_class6_opcode(uint16_t opcode) {
  /*6xkk - LD Vx, byte*/
  this->sys->set_register(((opcode & 0x0F00) >> 8), (opcode & 0x00FF));
  this->sys->incr_pc();
}

void Iseq::handle_class7_opcode(uint16_t opcode) {
  /*7xkk - ADD Vx, byte*/
  uint8_t add_byte = (opcode & 0x00FF);
  uint8_t reg_addr = ((opcode & 0x0F00) >> 8);
  uint8_t reg_val = this->sys->fetch_register(reg_addr);
  if(add_byte > 0xFF - reg_val) {
    this->sys->set_register(SYS_REG_ADDR, 1);
  } else {
    this->sys->set_register(SYS_REG_ADDR, 0);
  }
  reg_val += add_byte;
  this->sys->set_register(reg_addr, reg_val);
  this->sys->incr_pc();
}

void Iseq::handle_class8_opcode(uint16_t opcode) {
  switch(opcode & 0x000F){
    case 0x0000:
      /*8xy0 - LD Vx, Vy*/
      this->sys->set_register(((opcode & 0x0F00) >> 8), this->sys->fetch_register(((opcode & 0x00F0) >> 4)));
      break;
    case 0x0001: {
      /*8xy1 - OR Vx, Vy*/
      uint8_t xaddr = ((opcode & 0x0F00) >> 8);
      uint8_t yaddr = ((opcode & 0x00F0) >> 4);
      uint8_t xsetval = (this->sys->fetch_register(xaddr) | this->sys->fetch_register(yaddr));
      this->sys->set_register(xaddr, xsetval);
      break;
    }
    case 0x0002: {
      uint8_t xaddr = ((opcode & 0x0F00) >> 8);
      uint8_t yaddr = ((opcode & 0x00F0) >> 4);
      uint8_t xsetval = (this->sys->fetch_register(xaddr) & this->sys->fetch_register(yaddr));
      this->sys->set_register(xaddr, xsetval);
      break;
    }
    case 0x0003: {
      uint8_t xaddr = ((opcode & 0x0F00) >> 8);
      uint8_t yaddr = ((opcode & 0x00F0) >> 4);
      uint8_t xsetval = (this->sys->fetch_register(xaddr) ^ this->sys->fetch_register(yaddr));
      this->sys->set_register(xaddr, xsetval);
      break;
    }
    case 0x0004: {
      /*8xy4 - ADD Vx, Vy*/
      uint8_t xaddr = ((opcode & 0x0F00) >> 8);
      uint8_t yaddr = ((opcode & 0x00F0) >> 4);
      uint8_t yval  = this->sys->fetch_register(yaddr);
      uint8_t xval  = this->sys->fetch_register(xaddr);
      this->sys->set_register(SYS_REG_ADDR, ((yval > 0xFF - xval) ? 1:0));
      this->sys->set_register(xaddr, (yval + xval));
      break;
    }
    case 0x0005:{
      /*8xy5 - SUB Vx, Vy*/
      uint8_t xaddr = ((opcode & 0x0F00) >> 8);
      uint8_t yaddr = ((opcode & 0x00F0) >> 4);
      uint8_t xval  = this->sys->fetch_register(xaddr);
      uint8_t yval  = this->sys->fetch_register(yaddr);
      this->sys->set_register(SYS_REG_ADDR, ((xval > yval) ? 1:0));
      this->sys->set_register(xaddr, (xval - yval));
      break;
    }
    case 0x0006: {
      /*8xy6 - SHR Vx {, Vy}*/
      uint8_t xaddr = ((opcode & 0x0F00) >> 8);
      uint8_t xval  = this->sys->fetch_register(xaddr);
      this->sys->set_register(SYS_REG_ADDR, ((xval & 0x01) ? 1:0));
      this->sys->set_register(xaddr, xval >> 1);
      break;
    }
    case 0x0007: {
      /*8xy7 - SUBN Vx, Vy*/
      uint8_t xaddr = ((opcode & 0x0F00) >> 8);
      uint8_t yaddr = ((opcode & 0x00F0) >> 4);
      uint8_t xval  = this->sys->fetch_register(xaddr);
      uint8_t yval  = this->sys->fetch_register(yaddr);
      this->sys->set_register(SYS_REG_ADDR, ((yval > xval) ? 1:0));
      this->sys->set_register(xaddr, (yval - xval));
      break;
    }
    case 0x000E: {
      /*8xyE - SHL Vx {, Vy}*/
      uint8_t xaddr = ((opcode & 0x0F00) >> 8);
      uint8_t xval  = this->sys->fetch_register(xaddr);
      this->sys->set_register(SYS_REG_ADDR, ((xval & 0x01) ? 1:0));
      this->sys->set_register(xaddr,xval >> 1);
      break;
    }
    default:
      Log::unsupported_opcode(opcode);
      this->sys->print_machine_state();
      this->sys->run = false;
      break;
  }
  this->sys->incr_pc();
}

void Iseq::handle_class9_opcode(uint16_t opcode) {
  /*9xy0 - SNE Vx, Vy */
  uint8_t xaddr = ((opcode & 0x0F00) >> 8);
  uint8_t yaddr = ((opcode & 0x00F0) >> 4);
  if(this->sys->fetch_register(xaddr) != this->sys->fetch_register(yaddr)) this->sys->incr_pc();
  this->sys->incr_pc();
}

void Iseq::handle_classA_opcode(uint16_t opcode) {
  uint8_t target_addr = (opcode & 0x0FFF);
  this->sys->I = target_addr % MEMORY_SIZE;
  this->sys->incr_pc();
}

void Iseq::handle_classB_opcode(uint16_t opcode){
  uint16_t jp_addr = ((opcode & 0x0FFF)) + this->sys->fetch_register(0);
  this->sys->PC = jp_addr % MEMORY_SIZE;
}

void Iseq::handle_classC_opcode(uint16_t opcode) {
  /*Cxkk - RND Vx, byte*/
  uint8_t regaddr = ((opcode & 0x0F00) >> 8);
  uint8_t ibytes  = (opcode & 0x00FF);
  this->sys->set_register(regaddr, (rand() % 0xFF) & ibytes);
  this->sys->incr_pc();
}

void Iseq::handle_classD_opcode(uint16_t opcode) {
  /*Dxyn - DRW Vx, Vy, nibble*/
  this->sys->display->draw_sprite(opcode);
  this->sys->incr_pc();
}

void Iseq::handle_classE_opcode(uint16_t opcode) {
  switch(opcode & 0x00FF) {
    case 0x009E: {
      /*Ex9E - SKP Vx*/
      uint8_t keyaddr = this->sys->fetch_register(((opcode & 0x0F00) >> 8));
      uint8_t key     = this->sys->fetch_register(keyaddr);
      if(key!=0) this->sys->incr_pc();
      this->sys->incr_pc();
      break;
    }
    case 0x00A1: {
      /*ExA1 - SKNP Vx*/
      uint8_t keyaddr = this->sys->fetch_register(((opcode & 0x0F00) >> 8));
      uint8_t key     = this->sys->fetch_register(keyaddr);
      if(key==0) this->sys->incr_pc();
      this->sys->incr_pc();
      break;
    }
    default:
      Log::unsupported_opcode(opcode);
      this->sys->print_machine_state();
      this->sys->run = false;
      break;
  }
}

void Iseq::handle_classF_opcode(uint16_t opcode) {
  switch(opcode & 0x00FF) {
    case 0x0007: {
      /*Fx07 - LD Vx, DT*/
      this->sys->set_register(((opcode & 0x0F00) >> 8), this->sys->DT);
      break;
    }
    case 0x000A: {
      /*Fx0A - LD Vx, K*/
      uint8_t key = this->sys->keyboard->expectKeyDown();
      this->sys->set_register(((opcode & 0x0F00) >> 8), key);
      break;
    }
    case 0x0015: {
      /*Fx15 - LD DT, Vx*/
      this->sys->DT = this->sys->fetch_register(((opcode & 0x0F00) >> 8));
      break;
    }
    case 0x0018: {
      /*Fx18 - LD ST, Vx*/
      break;
    }
    case 0x001E:
      /*Fx1E - ADD I, Vx*/
      this->sys->I += this->sys->fetch_register(((opcode & 0x0F00) >> 8));
      break;
    case 0x0029: {
      /*Fx29 - LD F, Vx; Set I = location of sprite for digit Vx.*/
      uint8_t charAddr = this->sys->fetch_register(((opcode & 0x0F00) >> 8));
      this->sys->I = SPRITE_START_ADDR + (charAddr * CHAR_SPRITE_SIZE);
      break;
    }
    case 0x0033: {
      /*Fx33 - LD B, Vx*/
      uint8_t Vx = this->sys->fetch_register(((opcode & 0x0F00) >> 8));
      uint16_t index = this->sys->I;
      this->sys->memory[index]   = (Vx/100);
      this->sys->memory[index+1] = (Vx%100)/10;
      this->sys->memory[index+1] = (Vx%10);
      break;
    }
    case 0x0055: {
      /*Fx55 - LD [I], Vx*/
      uint8_t VxAddr = ((opcode & 0x0F00) >> 8);
      uint16_t index = this->sys->I;
      for(uint8_t i=0; i<=VxAddr; i++) this->sys->memory[index+i] = this->sys->fetch_register(i);
      break;
    }
    case 0x0065: {
      /*Fx65 - LD Vx, [I]*/
      uint8_t VxAddr = ((opcode & 0x0F00) >> 8);
      for(uint8_t i=0; i<=VxAddr; i++) this->sys->set_register(i, this->sys->memory[this->sys->I+i]);
      break;
    }
    default:
      Log::unsupported_opcode(opcode);
      this->sys->print_machine_state();
      this->sys->run = false;
      break;
  }
  this->sys->incr_pc();
}

