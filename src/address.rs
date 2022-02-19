// 128k, the amount of memory in a standard Vulcan machine
pub const MEM_SIZE: u32 = 128 * 1024;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Address(u32);

impl From<Address> for usize {
    fn from(a: Address) -> Self { (a.0 & (MEM_SIZE-1)) as usize }
}

impl From<u32> for Address {
    fn from(a: u32) -> Self { Self(a & 0xffffff) }
}

impl Into<u32> for Address {
    fn into(self) -> u32 { self.0 }
}

impl std::ops::Add<i32> for Address {
    type Output = Address;
    fn add(self, rhs: i32) -> Self::Output {
        Address::from((self.0 as i32).overflowing_add(rhs).0 as u32)
    }
}

impl std::ops::Sub<i32> for Address {
    type Output = Address;
    fn sub(self, rhs: i32) -> Self::Output { self + -rhs }
}

impl std::ops::SubAssign<i32> for Address {
    fn sub_assign(&mut self, rhs: i32) { *self = *self - rhs; }
}

impl std::ops::AddAssign<i32> for Address {
    fn add_assign(&mut self, rhs: i32) { *self = *self + rhs; }
}

#[test]
fn test_address_truncation() {
    let a: Address = 0x11223344.into();
    assert_eq!(a, 0x00223344.into());
}

#[test]
fn test_addressing_arrays() {
    let a: usize = Address::from(0xffffff).into();
    assert_eq!(a, 0x01ffff as usize);
}

#[test]
fn test_address_overflows() {
    let a = Address::from(0xfffffa);
    assert_eq!(a + 10, Address(4));

    let b = Address::from(3);
    assert_eq!(b - 10, Address(0xfffff9));

    let mut c = Address::from(5);
    c += 3;
    assert_eq!(c, Address(8));

    let mut d = Address::from(5);
    d -= 3;
    assert_eq!(d, Address(2));
}