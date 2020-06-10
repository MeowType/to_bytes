use to_bytes::*;

#[to_bytes]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
struct A {
    a: u8,
    b: u16,
    c: u32,
}

#[test]
fn test_a() {
    let a = dbg!(A { a: 3, b: 5, c: 7 });
    let ua = dbg!(a.to_bytes());
    let ra: A = dbg!(unsafe { ua.transmute_back() });
    assert_eq!(ra, a);
}

#[to_bytes]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
struct A2(u8, u16, u32);

#[test]
fn test_a2() {
    let a = dbg!(A2(3, 5, 7));
    let ua = dbg!(a.to_bytes());
    let ra: A2 = dbg!(unsafe { ua.transmute_back() });
    assert_eq!(ra, a);
}

#[to_bytes]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum AE {
    A { a: u8, b: u16, c: u32 },
    A2(u8, u16, u32),
}

#[test]
fn test_ae_a() {
    let a = dbg!(AE::A { a: 3, b: 5, c: 7 });
    let ua = dbg!(a.to_bytes());
    let ra: AE = dbg!(unsafe { ua.transmute_back() });
    assert_eq!(ra, a);
}

#[test]
fn test_ae_a2() {
    let a = dbg!(AE::A2(3, 5, 7));
    let ua = dbg!(a.to_bytes());
    let ra: AE = dbg!(unsafe { ua.transmute_back() });
    assert_eq!(ra, a);
}

#[to_bytes(asis)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
struct B {
    a: u8,
    b: u16,
    c: u32,
}
impl Drop for B {
    fn drop(&mut self) {
        println!("drop B");
    }
}

#[test]
fn test_b() {
    let a = dbg!(B { a: 3, b: 5, c: 7 });
    let ua = dbg!(a.clone().to_bytes());
    let ra: &B = dbg!(unsafe { ua.read_back() });
    assert_eq!(ra, &a);
}

#[to_bytes(asis)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
struct B2(u8, u16, u32);
impl Drop for B2 {
    fn drop(&mut self) {
        println!("drop B2");
    }
}

#[test]
fn test_b2() {
    let a = dbg!(B2(3, 5, 7));
    let ua = dbg!(a.clone().to_bytes());
    let ra: &B2 = dbg!(unsafe { ua.read_back() });
    assert_eq!(ra, &a);
}

#[to_bytes(asis)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum BE {
    B { a: u8, b: u16, c: u32 },
    B2(u8, u16, u32),
}
impl Default for BE {
    fn default() -> Self {
        Self::B2(0, 0, 0)
    }
}
impl Drop for BE {
    fn drop(&mut self) {
        match self {
            Self::B { .. } => println!("drop BE::B"),
            Self::B2(..) => println!("drop BE::B2"),
        }
    }
}

#[test]
fn test_be_b() {
    let a = dbg!(BE::B { a: 3, b: 5, c: 7 });
    let ua = dbg!(a.clone().to_bytes());
    let ra: &BE = dbg!(unsafe { ua.read_back() });
    assert_eq!(ra, &a);
}

#[test]
fn test_be_b2() {
    let a = dbg!(BE::B2(3, 5, 7));
    let ua = dbg!(a.clone().to_bytes());
    let ra: &BE = dbg!(unsafe { ua.read_back() });
    assert_eq!(ra, &a);
}

#[to_bytes(asis copy)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Default)]
struct C {
    a: u8,
    b: u16,
    c: u32,
}

#[test]
fn test_c() {
    let a = dbg!(C { a: 3, b: 5, c: 7 });
    let ua = dbg!(a.clone().to_bytes());
    let ra: &C = dbg!(unsafe { ua.read_back() });
    assert_eq!(ra, &a);
}

#[test]
fn test_c_2() {
    let a = dbg!(C { a: 3, b: 5, c: 7 });
    let ua = dbg!(a.clone().to_bytes());
    let ra: C = dbg!(unsafe { ua.transmute_back() });
    assert_eq!(ra, a);
}

#[to_bytes(asis copy)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Default)]
struct C2(u8, u16, u32);

#[test]
fn test_c2() {
    let a = dbg!(C2(3, 5, 7));
    let ua = dbg!(a.clone().to_bytes());
    let ra: &C2 = dbg!(unsafe { ua.read_back() });
    assert_eq!(ra, &a);
}

#[test]
fn test_c2_2() {
    let a = dbg!(C2(3, 5, 7));
    let ua = dbg!(a.clone().to_bytes());
    let ra: C2 = dbg!(unsafe { ua.transmute_back() });
    assert_eq!(ra, a);
}

#[to_bytes(asis copy)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum CE {
    C { a: u8, b: u16, c: u32 },
    C2(u8, u16, u32),
}
impl Default for CE {
    fn default() -> Self {
        Self::C2(0, 0, 0)
    }
}

#[test]
fn test_ce_c() {
    let a = dbg!(CE::C { a: 3, b: 5, c: 7 });
    let ua = dbg!(a.to_bytes());
    let ra: &CE = dbg!(unsafe { ua.read_back() });
    assert_eq!(ra, &a);
}

#[test]
fn test_ce_c2() {
    let a = dbg!(CE::C2(3, 5, 7));
    let ua = dbg!(a.to_bytes());
    let ra: &CE = dbg!(unsafe { ua.read_back() });
    assert_eq!(ra, &a);
}

#[test]
fn test_ce_c_2() {
    let a = dbg!(CE::C { a: 3, b: 5, c: 7 });
    let ua = dbg!(a.to_bytes());
    let ra: CE = dbg!(unsafe { ua.transmute_back() });
    assert_eq!(ra, a);
}

#[test]
fn test_ce_c2_2() {
    let a = dbg!(CE::C2(3, 5, 7));
    let ua = dbg!(a.to_bytes());
    let ra: CE = dbg!(unsafe { ua.transmute_back() });
    assert_eq!(ra, a);
}
