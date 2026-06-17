struct DropTag(&'static str);

impl Drop for DropTag {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

struct Pair {
    first: DropTag,
    second: DropTag,
}

fn demo_field_order() {
    println!("--- field drop order ---");
    println!("enter scope");
    let _p = Pair {
        first: DropTag("first"),
        second: DropTag("second"),
    };
    println!("leave scope");
}

fn demo_manually_drop() {
    use std::mem::ManuallyDrop;

    struct Controlled {
        early: DropTag,
        late: ManuallyDrop<DropTag>,
    }

    impl Drop for Controlled {
        fn drop(&mut self) {
            println!("Controlled::drop start");
            unsafe {
                ManuallyDrop::drop(&mut self.late);
            }
            println!("Controlled::drop end");
        }
    }

    println!("--- ManuallyDrop ---");
    let _c = Controlled {
        early: DropTag("early"),
        late: ManuallyDrop::new(DropTag("late")),
    };
}

fn main() {
    demo_field_order();
    demo_manually_drop();
}
