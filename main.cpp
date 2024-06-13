#include <cassert>
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
  // read one byte from data
  Byte operator[](u32 Address) const {
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

  void Execute(u32 Cycles, Memory &memory) {
    while (Cycles > 0) {
      Byte Ins = FetchByte(Cycles, memory);
    }
  }
};

int main() {
  Memory mem;
  CPU cpu;
  cpu.Reset(mem);
  cpu.Execute(2, mem);
  return 0;
}
