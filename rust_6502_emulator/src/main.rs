// https://web.archive.org/web/20210909190432/http://www.obelisk.me.uk/6502/

type Byte = u8;
type Word = u16;

struct Memory {
    data: [Byte; 1024 * 64],
}

impl Memory {
    fn new() -> Self {
        let data = [0; 1024 * 64];
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
        self.data[address as usize]
    }
}

struct CPU {
    pc: Word, // program counter
    sp: Byte, // stack pointer

    // registers
    a: Byte,
    x: Byte,
    y: Byte,

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
            pc: 0xFFFC,
            sp: 0xFF,
            a : 0,
            x : 0,
            y : 0,
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
        self.pc = 0xFFFC;
        self.sp = 0xFF;
        self.a = 0;
        self.x = 0;
        self.y = 0;
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
        let data = memory.get(self.pc as u32);
        self.pc += 1;
        *cycles -= 1;
        data
    }

    fn execute(&mut self, cycles: &mut u32, memory: &mut Memory) {
        while *cycles > 0 {
            let ins = self.fetch_byte(cycles, memory);
            match ins {
                0xA9 => {
                    let value = self.fetch_byte(cycles, memory);
                    self.a = value;
                    self.z = self.a == 0;
                    self.n = (self.a & 0b10000000) > 0;
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
