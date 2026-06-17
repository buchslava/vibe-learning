fn print_str(s: &str) {
    println!("{}", s);
}

fn main() {
    let long_lived: &'static str = "static";
    let short: &str = long_lived; // covariant: 'static <: 'a
    print_str(short);

    let owned = String::from("owned");
    let borrow: &str = &owned;
    print_str(borrow);
}
