trait HasCode {
    fn code(&self) -> u8;
    fn label(&self) -> String {
        format!("code {}", self.code())
    }
}

enum Fault { OverTemp(u8), CommLost }

impl HasCode for Fault {
    fn code(&self) -> u8 {
        match self {
            Fault::OverTemp(c) => *c,
            Fault::CommLost => 0xFF,
        }
    }
    fn label(&self) -> String {
        match self {
            Fault::CommLost => "communication lost".into(),
            other => <Self as HasCode>::label(other),
        }
    }
}

fn main() {
    println!("{}", Fault::OverTemp(0x0A).label());
}
