use std::mem::{self, MaybeUninit};

struct GrowBuf {
    buf: Vec<MaybeUninit<u32>>,
    len: usize,
}

impl GrowBuf {
    fn with_capacity(cap: usize) -> Self {
        let mut buf = Vec::with_capacity(cap);
        buf.resize_with(cap, MaybeUninit::uninit);
        Self { buf, len: 0 }
    }

    fn push(&mut self, value: u32) {
        if self.len == self.buf.len() {
            self.buf
                .resize_with(self.buf.len() * 2 + 1, MaybeUninit::uninit);
        }
        self.buf[self.len].write(value);
        self.len += 1;
    }

    fn as_slice(&self) -> &[u32] {
        unsafe { std::slice::from_raw_parts(self.buf.as_ptr() as *const u32, self.len) }
    }
}

impl Drop for GrowBuf {
    fn drop(&mut self) {
        for slot in &mut self.buf[..self.len] {
            unsafe {
                slot.assume_init_drop();
            }
        }
    }
}

fn main() {
    let mut g = GrowBuf::with_capacity(2);
    for i in 0..5 {
        g.push(i);
    }
    println!("{:?}", g.as_slice());
    mem::drop(g);
    println!("done");
}
