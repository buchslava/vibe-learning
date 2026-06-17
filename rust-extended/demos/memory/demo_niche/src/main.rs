use std::cell::UnsafeCell;
use std::mem::size_of;
use std::num::NonZeroU8;

fn main() {
    println!("&i32:              {}", size_of::<&i32>());
    println!("Option<&i32>:      {}", size_of::<Option<&i32>>());
    println!("Option<NonZeroU8>: {}", size_of::<Option<NonZeroU8>>());
    println!("Option<bool>:      {}", size_of::<Option<bool>>());

    enum Inner {
        A(u8),
        B,
    }
    enum Outer {
        Some(Inner),
        None,
    }
    println!("Inner:             {}", size_of::<Inner>());
    println!("Outer:             {}", size_of::<Outer>());

    struct Wrapped(UnsafeCell<u8>);
    println!("Option<Wrapped>:   {}", size_of::<Option<Wrapped>>());
    let _ = (Inner::A(0), Outer::None, Wrapped(UnsafeCell::new(0)));
}
