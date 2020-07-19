
//Struct defining registers for 8-bit CPU
//CPU has 8 different registers a,b,c,d,e,f,h,l
//u8 are 8-bit unsigned integers
struct Registers {
   a: u8,
   b: u8,
   c: u8,
   d: u8,
   e: u8,
   f: FlagsRegister,
   h: u8,
   l: u8,
}




//Instructions to allow game to read and write 16 bits
// i.e. u16
//Two registers are combined and are refered as
//af, bc, de and hl

impl Registers {

    fn get_af(&self) -> u16{
        (self.a as u16) << 8
        | self.f as u16
    }

    fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = (value & 0xFF) as u8;
    }

    fn get_bc(&self) -> u16{
        (self.b as u16) << 8
        | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn get_de(&self) -> u16{
        (self.d as u16) << 8
        | self.e as u16
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16{
        (self.h as u16) << 8
        | self.l as u16
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }


}


//Struct defining Flags register
//lower 4 bits always 0
//upper 4 bits represent states

struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}


//Methods to translate from FlagsRegister to u8 and vice-versa

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

//from FlagsRegister to u8
impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}


//from u8 to FlagsRegister
impl std::convert::From<u8> for FlagsRegister{
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & Ob1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & Ob1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & Ob1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}


//define Instructions

enum Instruction {
    ADD(ArithmeticTarget),
}

//Specifies target register
enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}



fn main() {
    println!("Hello, world!");
}
