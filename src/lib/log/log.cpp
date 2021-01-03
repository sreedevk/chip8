#include <iostream>
#include <string>
#include <sstream>
#include <array>
#include <unordered_map>
#include <iomanip>
#include "log.hpp"

void Log::print_color(std::string message, int color) {
  std::cout << Log::format_color(message, color);
}

std::string Log::format_color(std::string message, int color) {
  std::ostringstream msg_cache;
  msg_cache << LOG_FORMAT_ESC << LOG_BOLD << ';' << color << LOG_FORMAT_ESC_END << message <<
    LOG_FORMAT_ESC << LOG_RESET << LOG_FORMAT_ESC_END;
  return msg_cache.str();
}

void Log::print_std_error(std::string message){
  std::ostringstream msg_cache;
  msg_cache << LOG_ERROR << message << std::endl;
  print_color(msg_cache.str(), LOG_BCOLOR_RED);
}

void Log::print_fatal_error(std::string message){
  std::ostringstream msg_cache;
  msg_cache << LOG_FATAL << message << std::endl;
  print_color(msg_cache.str(), LOG_BCOLOR_RED);
}

void Log::print_info(std::string message){
  std::ostringstream msg_cache;
  msg_cache << LOG_INFO << message << std::endl;
  print_color(msg_cache.str(), LOG_BCOLOR_BLUE);
}

/* specific errors */

void Log::unsupported_opcode(uint16_t opcode) {
  std::ostringstream log_message;
  log_message << "UNSUPPORTED OPCODE: " << std::hex << std::setw(4) << std::setfill('0') << opcode << ".";
  print_fatal_error(log_message.str());
}

void Log::program_error(){
  print_fatal_error("PROGRAM ERROR. RESTART REQUIRED.");
}

void Log::stack_overflow(){
  print_fatal_error("STACK OVERFLOW.");
}

void Log::running_routine_at(uint16_t opcode) {
  std::ostringstream log_message;
  log_message << "SYS ADDR - RUNNING ROUTINE AT: " << opcode << ".";
  print_info(log_message.str());
}

void Log::print_vm_info(std::unordered_map<std::string, std::string> sysinfo) {
  std::array<std::string, 9> info_keys = {
    "PROGRAM_COUNTER", "STACK_POINTER", "STACK", "CURRENT_OPCODE", "FLAGS", "TIMERS",
    "I_REGISTER", "GP_REGISTERS", "MEMORY_LAYOUT"
  };

  std::ostringstream info_cache;
  info_cache << "SYSTEM INFO:";
  print_info(info_cache.str());
  info_cache.str(""); info_cache.clear();

  for(auto i = std::begin(info_keys); i != std::end(info_keys); ++i) {
    info_cache << *i << ':' << std::string((16 - (*i).size()), ' ') << "\t";
    print_color(info_cache.str(), LOG_BCOLOR_GREEN);
    info_cache.str(""); info_cache.clear();

    info_cache << sysinfo[*i];
    std::cout << info_cache.str();

    info_cache.str(""); info_cache.clear();

  }
}
