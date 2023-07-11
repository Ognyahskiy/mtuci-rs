struct Vector<T> {
    data: Box<[T]>,
    len: usize,
    capacity: usize,
}

impl<T> Vector<T> {
    fn new() -> Self {
        Vector {
            data: Self::allocate(0),
            len: 0,
            capacity: 0,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Vector {
            data: Self::allocate(capacity),
            len: 0,
            capacity,
        }
    }

    fn allocate(size: usize) -> Box<[T]> {
        if size == 0 {
            Vec::new().into_boxed_slice()
        } else {
            Vec::with_capacity(size).into_boxed_slice()
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.resize(self.capacity * 2);
        }

        self.data[self.len] = value;
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            Some(std::mem::replace(&mut self.data[self.len], Default::default()))
        } else {
            None
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.len {
            let value = std::mem::replace(&mut self.data[index], Default::default());
            for i in index..self.len - 1 {
                self.data[i] = std::mem::replace(&mut self.data[i + 1], Default::default());
            }
            self.len -= 1;
            Some(value)
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn resize(&mut self, new_capacity: usize) {
        let mut new_data = Self::allocate(new_capacity);
        let elements_to_copy = std::cmp::min(self.len, new_capacity);
        new_data[..elements_to_copy].copy_from_slice(&self.data[..elements_to_copy]);
        self.data = new_data;
        self.capacity = new_capacity;
        self.len = elements_to_copy;
    }
}

fn main() {
    let mut vector: Vector<i32> = Vector::new();

    vector.push(1);
    vector.push(2);
    vector.push(3);

    println!("{:?}", vector.pop()); // Output: Some(3)
    println!("{:?}", vector.remove(1)); // Output: Some(2)

    println!("{:?}", vector.get(0)); // Output: Some(1)
    println!("{:?}", vector.get(1)); // Output: None

    vector.resize(2);

    println!("{:?}", vector.get(0)); // Output: Some(1)
    println!("{:?}", vector.get(1)); // Output: None
}
