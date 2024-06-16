#include <cassert>
#include <iostream>
#include <ostream>
#include <stdio.h>
#include <stdlib.h>

// https://web.archive.org/web/20210909190432/http://www.obelisk.me.uk/6502/
using Byte = unsigned char;
using Word = unsigned short;

using u32 = unsigned int;

struct Memory {
  static constexpr u32 MAX_MEM = 1024 * 64;
  Byte Data[MAX_MEM];

  void Init() {
    for (u32 i = 0; i < MAX_MEM; i++) {
      Data[i] = 0;
    }
  }
  // read 1 byte from data
  Byte operator[](u32 Address) const {
    assert((Address < MAX_MEM));

    return Data[Address];
  }
  Byte &operator[](u32 Address) {
    assert((Address < MAX_MEM));

    return Data[Address];
  }
};

struct CPU {
  Word PC; // program counter
  Byte SP; // stack pointer

  Byte A, X, Y; // registers

  // status flags
  Byte C : 1; // carry flag
  Byte Z : 1; // zero flag
  Byte I : 1; // interrupt disable
  Byte D : 1; // decimal mode
  Byte B : 1; // break command
  Byte V : 1; // overglow flag
  Byte N : 1; // negative flag

  void Reset(Memory &memory) {
    PC = 0xFFFC;
    SP = 0xFF;
    C = Z = I = D = B = V = N = 0;
    A = X = Y = 0;
    memory.Init();
  }

  Byte FetchByte(u32 &Cycles, Memory &memory) {
    Byte Data = memory[PC];
    PC++;
    Cycles--;

    return Data;
  }

  // opcodes
  static constexpr Byte INS_LDA_IM = 0xA9;

  void Execute(u32 Cycles, Memory &memory) {
    while (Cycles > 0) {
      Byte Ins = FetchByte(Cycles, memory);
      switch (Ins) {
      case INS_LDA_IM: {
        Byte Value = FetchByte(Cycles, memory);
        A = Value;
        Z = (A == 0);
        N = (A & 0b10000000) > 0;
      } break;

      default:
        std::cout << "instruction is unknown" << std::endl;
      }
    }
  }
};

int main() {
  Memory mem;
  CPU cpu;
  cpu.Reset(mem);
  mem[0xFFFC] = CPU::INS_LDA_IM;
  mem[0xFFFD] = 0x42;
  printf("%.2X %.2X %.2X %.2X %.2X \n", mem[0xFFFA], mem[0xFFFB], mem[0xFFFC],
         mem[0xFFFD], mem[0xFFFE]);
  cpu.Execute(2, mem);

  return 0;
}
