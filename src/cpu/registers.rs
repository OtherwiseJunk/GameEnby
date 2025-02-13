use super::flags_register::FlagsRegister;

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
  }  
impl Registers {
    pub fn new() -> Self {
      Registers {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: FlagsRegister::new(),
        h: 0,
        l: 0,
      }
    }
    fn combine(high: u8, low: u8) -> u16 {
      (high as u16) << 8 | low as u16
    }
    fn split(value: u16) -> (u8, u8) {
      (((value & 0xFF00) >> 8) as u8, (value & 0xFF) as u8)
    }
    // 16-bit registers accessors
    pub fn get_af(&self) -> u16 {
      Self::combine(self.a, self.f.into())
    }
    pub fn set_af(&mut self, value: u16) {
      let (high, low) = Self::split(value);
      self.a = high;
      self.f = low.into();
    }
    pub fn get_bc(&self) -> u16 {
        Self::combine(self.b, self.c)      
    }
    pub fn set_bc(&mut self, value: u16) {
        let (high, low) = Self::split(value);
        self.b = high;
        self.c = low;
    }
    pub fn get_de(&self) -> u16 {
        Self::combine(self.d, self.e)      
    }
    pub fn set_de(&mut self, value: u16) {
        let (high, low) = Self::split(value);
        self.d = high;
        self.e = low;
    }
    pub fn get_hl(&self) -> u16 {
        Self::combine(self.h, self.l)      
    }
    pub fn set_hl(&mut self, value: u16) {
        let (high, low) = Self::split(value);
        self.h = high;
        self.l = low;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_set_af() {
        let mut registers = Registers::new();
        registers.set_af(0b1010_1010_0101_0101);
        assert_eq!(registers.a, 0b1010_1010u8);
        assert_eq!(registers.f, FlagsRegister::from(0b0101_0101u8));
    }
    #[test]
    fn can_set_bc() {
        let mut registers = Registers::new();
        registers.set_bc(0b1010_1010_0101_0101);
        assert_eq!(registers.b, 0b1010_1010u8);
        assert_eq!(registers.c, 0b0101_0101u8);
    }
    #[test]
    fn can_set_de() {
        let mut registers = Registers::new();
        registers.set_de(0b1010_1010_0101_0101);
        assert_eq!(registers.d, 0b1010_1010u8);
        assert_eq!(registers.e, 0b0101_0101u8);
    }
    #[test]
    fn can_set_hl() {
        let mut registers = Registers::new();
        registers.set_hl(0b1010_1010_0101_0101);
        assert_eq!(registers.h, 0b1010_1010u8);
        assert_eq!(registers.l, 0b0101_0101u8);
    }
}