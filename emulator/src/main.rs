
//Struct defining registers for 8-bit CPU
//CPU has 8 different registers a,b,c,d,e,f,h,l
//u8 are 8-bit unsigned integers
struct Registers {
   a: u8,
   b: u8,
   c: u8,
   d: u8,
   e: u8,
   f: u8,
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




fn main() {
    println!("Hello, world!");
}
