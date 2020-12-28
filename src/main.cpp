#include <iostream>
#include <string>
#include "lib/VM/vm.hpp"

int main(int argc, char *argv[]){
  VM *chip = new VM();
  chip->load_program(argv[1]);
  chip->exec();
}
