struct Closure<F> {
    data: (u8, u16),
    func: F,
}

impl<F> Closure<F>
where
    for<'a> F: Fn(&'a (u8, u16)) -> &'a u8,
{
    fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }
}

fn pick_first(data: &(u8, u16)) -> &u8 {
    &data.0
}

fn main() {
    let clo = Closure {
        data: (42, 99),
        func: pick_first,
    };
    println!("first byte = {}", clo.call());
}
