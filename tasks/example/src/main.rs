struct Coll<T> {
    inner: Vec<T>
}

impl<T> Coll<T> {
    fn new() -> Self {
        Coll { inner: vec![] }
    }

    fn size(&self) -> usize {
        self.inner.len()
    }
}

fn main() {}