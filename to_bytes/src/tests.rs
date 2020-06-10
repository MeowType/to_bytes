use crate::*;

#[test]
fn test_bytes() {
    let byts = bytes![1u8 2u16 3u32];
    println!("{:?}", byts);
    let a: u8 = unsafe { byts.transmute_back_at(0) };
    let b: u16 = unsafe { byts.transmute_back_at(1) };
    let c: u32 = unsafe { byts.transmute_back_at(3) };
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    assert_eq!(a, 1);
    assert_eq!(b, 2);
    assert_eq!(c, 3);
}

#[test]
fn test_bool() {
    let a = true;
    let u = a.to_bytes();
    println!("{:?}", u);
    let b: bool = unsafe { u.transmute_back() };
    println!("{:?}", b);
    assert_eq!(b, a);
}

#[test]
fn test_bool2() {
    let a = false;
    let u = a.to_bytes();
    println!("{:?}", u);
    let b: bool = unsafe { u.transmute_back() };
    println!("{:?}", b);
    assert_eq!(b, a);
}

#[test]
fn test_u8() {
    let a = 123u8;
    let u = a.to_bytes();
    println!("{:?}", u);
    let b: u8 = unsafe { u.transmute_back() };
    println!("{:?}", b);
    assert_eq!(b, a);
}

#[test]
fn test_i8() {
    let a = -128i8;
    let u = a.to_bytes();
    println!("{:?}", u);
    println!("{:016b}", unsafe { *(u.as_ptr() as *const i16) });
    println!("{:016b}", unsafe { *(u.as_ptr()) });
    println!("{:016b}", a);
    let b: i8 = unsafe { u.transmute_back() };
    println!("{:?}", b);
    assert_eq!(b, a);
}

#[test]
fn test_u32() {
    let a = 12345u32;
    let u = a.to_bytes();
    println!("{:?}", u);
    let b: u32 = unsafe { u.transmute_back() };
    println!("{:?}", b);
    assert_eq!(b, a);
}

#[test]
fn test_usize() {
    let a = 12345usize;
    let u = a.to_bytes();
    println!("{:?}", u);
    const SIZE: usize = std::mem::size_of::<usize>();
    assert_eq!(u.len(), SIZE);
    let b: usize = unsafe { u.transmute_back() };
    println!("{:?}", b);
    assert_eq!(b, a);
}

#[test]
fn test_unit() {
    let a = ();
    let u = a.to_bytes();
    println!("{:?}", u);
    const SIZE: usize = std::mem::size_of::<()>();
    assert_eq!(u.len(), SIZE);
    let b: () = unsafe { u.transmute_back() };
    println!("{:?}", b);
    assert_eq!(b, a);
}

#[test]
fn test_option_bool() {
    let a = Some(true);
    let b = Some(false);
    let c = None::<bool>;

    let ua = a.to_bytes();
    let ub = b.to_bytes();
    let uc = c.to_bytes();

    println!("{:?}", ua);
    println!("{:?}", ub);
    println!("{:?}", uc);

    let ra = unsafe { ua.transmute_back() };
    let rb = unsafe { ub.transmute_back() };
    let rc = unsafe { uc.transmute_back() };

    println!("{:?}", ra);
    println!("{:?}", rb);
    println!("{:?}", rc);

    assert_eq!(a, ra);
    assert_eq!(b, rb);
    assert_eq!(c, rc);
}

#[test]
fn test_option_u8() {
    let a = Some(123u8);
    let b = None::<u8>;

    let ua = a.to_bytes();
    let ub = b.to_bytes();

    println!("{:?}", ua);
    println!("{:?}", ub);

    let ra = unsafe { ua.transmute_back() };
    let rb = unsafe { ub.transmute_back() };

    println!("{:?}", ra);
    println!("{:?}", rb);

    assert_eq!(a, ra);
    assert_eq!(b, rb);
}

#[test]
fn test_option_unit() {
    let a = Some(());
    let b = None::<()>;

    let ua = a.to_bytes();
    let ub = b.to_bytes();

    println!("{:?}", ua);
    println!("{:?}", ub);

    let ra = unsafe { ua.transmute_back() };
    let rb = unsafe { ub.transmute_back() };

    println!("{:?}", ra);
    println!("{:?}", rb);

    assert_eq!(a, ra);
    assert_eq!(b, rb);
}

#[test]
fn test_option_usize() {
    let a = Some(12345usize);
    let b = None::<usize>;

    let ua = a.to_bytes();
    let ub = b.to_bytes();

    println!("{:?}", ua);
    println!("{:?}", ub);

    let ra = unsafe { ua.transmute_back() };
    let rb = unsafe { ub.transmute_back() };

    println!("{:?}", ra);
    println!("{:?}", rb);

    assert_eq!(a, ra);
    assert_eq!(b, rb);
}

#[test]
fn test_option_u16() {
    let a = Some(12345u16);
    let b = None::<u16>;

    let ua = a.to_bytes();
    let ub = b.to_bytes();

    println!("{:?}", ua);
    println!("{:?}", ub);

    let ra = unsafe { ua.transmute_back() };
    let rb = unsafe { ub.transmute_back() };

    println!("{:?}", ra);
    println!("{:?}", rb);

    assert_eq!(a, ra);
    assert_eq!(b, rb);
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
struct A {
    a: u8,
    b: u8,
}
impl ToBytes for A {
    fn to_bytes(self) -> Bytes {
        bytes![self.a self.b]
    }
}
#[allow(deprecated)]
unsafe impl ToSimpleDataBytes for A {}
impl TransmuteBack for A {
    unsafe fn transmute_back(ptr: *const u8) -> Self {
        Self { a: *ptr, b: *(ptr.add(1)) }
    }
}

#[test]
fn test_struct() {
    let a = A { a: 1, b: 2 };
    let ua = a.to_bytes();

    println!("{:?}", ua);

    let ra: A = unsafe { ua.transmute_back() };

    println!("{:?}", ra);
    assert_eq!(ra, a);
}

#[test]
fn test_box() {
    let a = AsIs::new(Box::new(1));
    let ua = a.to_bytes();

    println!("{:?}", ua);

    let ra: &AsIs<Box<i32>> = unsafe { ua.read_back() };

    println!("{:?}", ra);
    assert_eq!(ra, &AsIs::new(Box::new(1)));
}

#[test]
fn test_more_box() {
    let x = (1..10)
        .into_iter()
        .map(|_| {
            std::thread::spawn(|| {
                for _ in 1..100 {
                    let a = AsIs::new(Box::new(1));
                    let ua = a.to_bytes();
                    let ra: &AsIs<Box<i32>> = unsafe { ua.read_back() };
                    assert_eq!(ra, &AsIs::new(Box::new(1)));
                }
            })
        })
        .collect::<Vec<_>>();
    for c in x.into_iter().map(|c| c.join()) {
        c.unwrap();
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
struct B {
    show: bool,
    a: u8,
    b: u8,
}
impl ToBytes for B {
    fn to_bytes(self) -> Bytes {
        bytes![self.show self.a self.b]
    }
}
#[allow(deprecated)]
unsafe impl ToComplexDataBytes for B {}
impl Drop for B {
    fn drop(&mut self) {
        if self.show {
            println!("drop B")
        }
    }
}
impl<'a> ReadBack<'a> for B {
    unsafe fn read_back(ptr: *const u8) -> &'a Self {
        &*(ptr as *const Self)
    }
}

#[test]
fn test_drop() {
    let a = AsIs::new(B { show: true, a: 1, b: 2 });
    let ua = a.to_bytes();

    println!("{:?}", ua);

    let ra: &B = unsafe { ua.read_back() };

    println!("{:?}", ra);
    assert_eq!(ra, &B { show: true, a: 1, b: 2 });
}

#[test]
fn test_more_drop() {
    let x = (1..10)
        .into_iter()
        .map(|_| {
            std::thread::spawn(|| {
                for _ in 1..100 {
                    let a = AsIs::new(B { show: false, a: 1, b: 2 });
                    let ua = a.to_bytes();
                    let ra: &B = unsafe { ua.read_back() };
                    assert_eq!(ra, &B { show: false, a: 1, b: 2 });
                }
            })
        })
        .collect::<Vec<_>>();
    for c in x.into_iter().map(|c| c.join()) {
        c.unwrap();
    }
}

#[test]
fn test_option_struct() {
    let a = Some(A { a: 3, b: 5 });
    let b = None::<A>;
    let ua = a.to_bytes();
    let ub = b.to_bytes();

    println!("{:?}", ua);
    println!("{:?}", ub);

    let ra: Option<A> = unsafe { ua.transmute_back() };
    let rb: Option<A> = unsafe { ub.transmute_back() };

    println!("{:?}", ra);
    println!("{:?}", rb);
    assert_eq!(ra, a);
    assert_eq!(rb, b);
}

#[test]
fn test_option_box() {
    let a = Some(AsIs::new(Box::new(5)));
    let b = None::<BoxIs<i32>>;
    let ua = a.to_bytes();
    let ub = b.to_bytes();

    println!("{:?}", ua);
    println!("{:?}", ub);

    let ra: &Option<BoxIs<i32>> = unsafe { ua.read_back() };
    let rb: &Option<BoxIs<i32>> = unsafe { ub.read_back() };

    println!("{:?}", ra);
    println!("{:?}", rb);
    assert_eq!(ra, &Some(AsIs::new(Box::new(5))));
    assert_eq!(rb, &None::<BoxIs<i32>>);
}

#[test]
fn test_more_option_box() {
    let x = (1..10)
        .into_iter()
        .map(|_| {
            std::thread::spawn(|| {
                for _ in 1..100 {
                    let a = Some(AsIs::new(Box::new(5)));
                    let b = None::<BoxIs<i32>>;
                    let ua = a.to_bytes();
                    let ub = b.to_bytes();
                    let ra: &Option<BoxIs<i32>> = unsafe { ua.read_back() };
                    let rb: &Option<BoxIs<i32>> = unsafe { ub.read_back() };
                    assert_eq!(ra, &Some(AsIs::new(Box::new(5))));
                    assert_eq!(rb, &None::<BoxIs<i32>>);
                }
            })
        })
        .collect::<Vec<_>>();
    for c in x.into_iter().map(|c| c.join()) {
        c.unwrap();
    }
}

#[test]
fn test_option_drop() {
    let a = Some(AsIs::new(B { show: true, a: 3, b: 5 }));
    let b = None::<AsIs<B>>;
    let ua = a.to_bytes();
    let ub = b.to_bytes();

    println!("{:?}", ua);
    println!("{:?}", ub);

    let ra: &Option<AsIs<B>> = unsafe { ua.read_back() };
    let rb: &Option<AsIs<B>> = unsafe { ub.read_back() };

    println!("{:?}", ra);
    println!("{:?}", rb);
    assert_eq!(ra, &Some(AsIs::new(B { show: true, a: 3, b: 5 })));
    assert_eq!(rb, &None);
}

#[test]
fn test_more_option_drop() {
    let x = (1..10)
        .into_iter()
        .map(|_| {
            std::thread::spawn(|| {
                for _ in 1..100 {
                    let a = Some(AsIs::new(B { show: false, a: 3, b: 5 }));
                    let b = None::<AsIs<B>>;
                    let ua = a.to_bytes();
                    let ub = b.to_bytes();
                    let ra: &Option<AsIs<B>> = unsafe { ua.read_back() };
                    let rb: &Option<AsIs<B>> = unsafe { ub.read_back() };
                    assert_eq!(ra, &Some(AsIs::new(B { show: false, a: 3, b: 5 })));
                    assert_eq!(rb, &None);
                }
            })
        })
        .collect::<Vec<_>>();
    for c in x.into_iter().map(|c| c.join()) {
        c.unwrap();
    }
}
