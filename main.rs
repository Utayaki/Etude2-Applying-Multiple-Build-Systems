// https://web.archive.org/web/20210909190432/http://www.obelisk.me.uk/6502/

type Byte = u8;
type Word = u16;

struct Memory {
    data: [Byte; 1024 * 64],
}

impl Memory {
    fn new() -> Self {
        let mut data = [0; 1024 * 64];
        Memory {data}
    }
    
    fn init (&mut self) {
        for i in 0..1024 * 64 {
            self.data[i] = 0;
        }
    }

    // read 1 byte from data
    fn get(&self, address: u32) -> Byte {
        assert!(address < 1024 * 64);
        self.data[address]
    }
}

struct CPU {
    PC: Word, // program counter
    SP: Byte, // stack pointer

    // registers
    A: Byte,
    X: Byte,
    Y: Byte,

    // status flags
    c: bool, // carry flag
    z: bool, // zero flag
    i: bool, // interrupt disable
    d: bool, // decimal mode
    b: bool, // break command
    v: bool, // overglow flag
    n: bool, // negative flag
}

impl CPU {
    fn new() -> Self {
        CPU {
            PC: 0xFFFC,
            SP: 0xFF,
            A : 0,
            X : 0,
            Y : 0,
            c : false,
            z : false,
            i : false,
            d : false,
            b : false,
            v : false,
            n : false,
        }
    }

    fn reset(&mut self, memory: &mut Memory) {
        self.PC = 0xFFFC;
        self.SP = 0xFF;
        self.A = 0;
        self.X = 0;
        self.Y = 0;
        self.c = false;
        self.z = false;
        self.i = false;
        self.d = false;
        self.b = false;
        self.v = false;
        self.n = false;

        memory.init();
    }

    fn fetch_byte(&mut self, cycles: &mut u32, memory: &mut Memory) -> Byte {
        let data = memory.get(self.PC as u32);
        self.PC += 1;
        *cycles -= 1;
        data
    }

    fn execute(&mut self, cycles: &mut u32, memory: &mut Memory) {
        while *cycles > 0 {
            let ins = self.fetch_byte(cycles, memory);
            match ins {
                0xA9 => {
                    let value = self.fetch_byte(cycles, memory);
                    self.A = value;
                    self.Z = self.A == 0;
                    self.n = (self.A & 0b10000000) > 0;
                }
                _ => println!("instruction is unknown"),
            }
        }
    }
}

fn main() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();

    cpu.reset(&mut mem);
    cpu.execute(&mut 2, &mut mem);
}
