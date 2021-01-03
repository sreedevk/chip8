#pragma once
#include <string>
#include "../VM/vm.hpp"
#include <unordered_map>

/* LOG FORMATTERS */
#define LOG_BOLD            1
#define LOG_RESET           0
#define LOG_UNDERLINE       4
#define LOG_INVERT          7
#define LOG_BOLD_OFF        21
#define LOG_UNDERLINE_OFF   24
#define LOG_INVERT_OFF      27

/* LOG COLORS (+10 for foreground color) */
#define LOG_BCOLOR_RED      31
#define LOG_BCOLOR_GREEN    32
#define LOG_BCOLOR_YELLOW   33
#define LOG_BCOLOR_BLUE     34
#define LOG_BCOLOR_MAGENTA  35
#define LOG_BCOLOR_CYAN     36
#define LOG_BCOLOR_WHITE    37

#define LOG_FORMAT_ESC      "\033["
#define LOG_FORMAT_ESC_END  "m"

#define LOG_INFO  "[INFO]\t"
#define LOG_FATAL "[FATAL]\t"
#define LOG_ERROR "[ERROR]\t"

class Log {
  public:
    static void print_fatal_error(std::string);
    static void print_debug(std::string);
    static void print_std_error(std::string);
    static void print_info(std::string);
    static void print_color(std::string, int);
    static std::string format_color(std::string, int);

    /* spcific errors */
    static void unsupported_opcode(uint16_t);
    static void program_error();
    static void stack_overflow();
    static void running_routine_at(uint16_t);

    /* VM INFO */
    static void print_vm_info(std::unordered_map<std::string, std::string>);
};
