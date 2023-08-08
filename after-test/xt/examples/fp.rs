struct Stack<T> {
    data: Vec<T>,
    top: usize,
}

impl<T> Stack<T> {
    fn new(size: usize) -> Self {
        Stack { data: Vec::with_capacity(size), top: 0 }
    }

    fn push(&mut self, item: T) -> Result<(), String> {
        if self.top >= self.data.capacity() {
            return Err(String::from("There is no space in stack"));
        }

        self.data.push(item);
        self.top += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None
        }

        self.top -= 1;
        self.data.pop()
    }

    fn top(&self) -> usize {
        self.top
    }
}

fn main() {
}