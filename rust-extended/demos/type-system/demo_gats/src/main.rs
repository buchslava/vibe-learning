trait LendingIter {
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

struct Words<'s> {
    rest: &'s str,
}

impl<'s> LendingIter for Words<'s> {
    type Item<'a> = &'a str where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        let trimmed = self.rest.trim_start();
        if trimmed.is_empty() {
            return None;
        }
        let (word, tail) = match trimmed.find(char::is_whitespace) {
            Some(i) => (&trimmed[..i], &trimmed[i..]),
            None => (trimmed, ""),
        };
        self.rest = tail;
        Some(word)
    }
}

fn main() {
    let mut it = Words {
        rest: "hello async gats",
    };
    while let Some(word) = it.next() {
        println!("word: {}", word);
    }
}
