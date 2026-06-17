use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Self-referential: `ptr` points at `text` inside the same struct.
struct SelfRefFuture {
    text: String,
    ptr: *const String,
    _pin: PhantomPinned,
}

impl SelfRefFuture {
    fn new(text: String) -> Pin<Box<Self>> {
        let mut boxed = Box::pin(SelfRefFuture {
            text,
            ptr: std::ptr::null(),
            _pin: PhantomPinned,
        });
        let raw: *const String = &boxed.text;
        unsafe {
            boxed.as_mut().get_unchecked_mut().ptr = raw;
        }
        boxed
    }
}

impl Future for SelfRefFuture {
    type Output = usize;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        Poll::Ready(unsafe { &*this.ptr }.len())
    }
}

fn main() {
    let fut = SelfRefFuture::new(String::from("pinned"));
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");
    let n = rt.block_on(fut);
    println!("len = {}", n);
}
