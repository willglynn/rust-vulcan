use rand::Rng;
use crate::address::Address;
use crate::address::MEM_SIZE;

pub struct Memory([u8; MEM_SIZE as usize]);

impl Default for Memory {
    fn default() -> Self { Self([0u8; MEM_SIZE as usize]) }
}

impl<R: Rng> From<R> for Memory {
    fn from(mut rng: R) -> Self {
        let mut mem = Memory::default();
        for i in 0..(MEM_SIZE - 1) {
            mem.0[i as usize] = rng.gen()
        }
        mem
    }
}

impl std::ops::Index<Address> for Memory {
    type Output = u8;
    fn index(&self, index: Address) -> &Self::Output { &self.0[usize::from(index)] }
}

impl std::ops::IndexMut<Address> for Memory {
    fn index_mut(&mut self, index: Address) -> &mut Self::Output { &mut self.0[usize::from(index)] }
}

pub trait PeekPoke {
    fn peek<A: Into<Address>>(&self, addr: A) -> u8;
    fn poke<A: Into<Address>>(&mut self, addr: A, val: u8);

    fn peek24<A: Into<Address>>(&self, addr: A) -> u32 {
        let addr = addr.into();
        (self.peek(addr) as u32) | ((self.peek(addr + 1) as u32) << 8) | ((self.peek(addr + 2) as u32) << 16)
    }

    fn poke24<A: Into<Address>>(&mut self, addr: A, val: u32) {
        let addr = addr.into();
        self.poke(addr, val as u8);
        self.poke(addr + 1, (val >> 8) as u8);
        self.poke(addr + 2, (val >> 16) as u8);
    }
}

impl PeekPoke for Memory {
    fn peek<A: Into<Address>>(&self, addr: A) -> u8 { self[addr.into()] }
    fn poke<A: Into<Address>>(&mut self, addr: A, val: u8) { self[addr.into()] = val; }
}

#[test]
fn test_mem_peek_poke() {
    let mut mem = Memory::default();
    assert_eq!(mem.peek(35), 0);
    mem.poke(35, 45);
    assert_eq!(mem.peek(35), 45);
    assert_eq!(mem.peek(36), 0);
}

#[test]
fn test_mem_word_fns() {
    let mut mem = Memory::default();
    mem.poke24(10, 0x123456);
    assert_eq!(mem.peek(10), 0x56);
    assert_eq!(mem.peek(11), 0x34);
    assert_eq!(mem.peek(12), 0x12);
    assert_eq!(mem.peek24(10), 0x123456);
    assert_eq!(mem.peek24(11), 0x001234);
}
