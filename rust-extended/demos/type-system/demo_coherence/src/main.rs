use std::fmt;

struct Port(u16);

impl Port {
    fn get(self) -> u16 {
        self.0
    }
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "port:{}", self.0)
    }
}

trait Trimmed {
    fn trimmed(self) -> Self;
}

impl Trimmed for String {
    fn trimmed(self) -> Self {
        self.trim().to_string()
    }
}

fn main() {
    let p = Port(502);
    println!("{}", p);
    println!("id = {}", p.get());

    let s = String::from("  hello  ");
    println!("'{}'", s.trimmed());
}
