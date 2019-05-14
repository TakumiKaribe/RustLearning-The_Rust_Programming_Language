#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.grow();
        }
        self.elements[self.len] = element;
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(elem)
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &'a T {
        match self.get(index) {
            Some(v) => v,
            None => default,
        }
    }

    fn grow(&mut self) {
        if self.capacity() == 0 {
            self.elements = Self::allocate_in_heap(1);
        } else {
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            let old_elements = std::mem::replace(&mut self.elements, new_elements);
            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }
}
