use std::marker::PhantomData;

struct Owns<T> {
    _marker: PhantomData<T>,
}

impl<T> Drop for Owns<T> {
    fn drop(&mut self) {
        println!("dropping Owns container");
    }
}

fn main() {
    let _o = Owns::<String> {
        _marker: PhantomData,
    };
    println!("main done");
}
