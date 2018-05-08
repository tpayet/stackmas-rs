pub struct Stackmas<T> {
    inner: Vec<T>,
}

impl<T> Stackmas<T> {
    pub fn new() -> Stackmas<T> {
        Stackmas { inner: Vec::new() }
    }

    pub fn with_capacity(cap: usize) -> Stackmas<T> {
        Stackmas { inner: Vec::with_capacity(cap) }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn push(&mut self, elem: T) {
        self.inner.push(elem)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    pub fn pop_map<U, F: FnOnce(T) -> U>(&mut self, f: F) -> Option<U> {
        self.inner.pop().map(f)
    }

    pub fn peek(&self) -> Option<&T> {
        self.inner.last()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.inner.last_mut()
    }
}

impl<T> IntoIterator for Stackmas<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { inner: self }
    }
}

pub struct IntoIter<T> {
    inner: Stackmas<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
