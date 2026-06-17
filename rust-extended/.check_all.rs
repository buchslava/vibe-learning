// Playground aggregator for article 01 — Five Deep Rust Facts.
// Cargo copy: demos/mindset/five_deep_facts/  (cargo run -p five_deep_facts)
use std::any::type_name;
use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::mem::{self, size_of};
use std::rc::Rc;

fn show_type<T>(_: T) { println!("x is {}", type_name::<T>()); }

fn main() {
    let n = Cell::new(0);
    let r1 = &n; let r2 = &n;
    r1.set(1); r2.set(2);
    let mut x = 5; *(&mut x) += 1;
    mem::forget(String::from("leak"));
    let s1 = String::from("hello"); let _s2 = s1;
    let a = 10_i32; let b = a; println!("{} {}", a, b);
    let opt = Some(42);
    match &opt { Some(x) => show_type(x), None => {} }
    struct Holds<T>(PhantomData<T>);
    println!("{}", size_of::<Holds<i32>>());
    let val = 42;
    struct RawRef<'a, T>(*const T, PhantomData<&'a T>);
    let r = RawRef(&val as *const _, PhantomData);
    println!("{}", unsafe { *r.0 });
}
