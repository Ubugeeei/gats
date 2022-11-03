trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

struct Iterable {
    cursor: usize,
    value: Vec<i32>,
}

impl LendingIterator for Iterable {
    type Item<'a> = &'a i32;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.cursor < self.value.len() {
            let result = &self.value[self.cursor];
            self.cursor += 1;
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    let mut iterable = Iterable {
        cursor: 0,
        value: vec![1, 2, 3],
    };

    while let Some(value) = iterable.next() {
        println!("{}", value);
    }
}
