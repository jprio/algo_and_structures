// ref :
// * https://applied-math-coding.medium.com/implementing-a-queue-in-rust-583cf068b2b5
// * https://developpaper.com/implementation-of-queue-with-rust/

struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { items: Vec::new() }
    }
    pub fn is_empty(&self) -> bool {
        match self.items.len() {
            0 => true,
            _ => false,
        }
    }

    fn add(&mut self, value: T) {
        self.items.push(value);
    }

    fn remove(&mut self) -> Option<T> {
        let l = self.items.len();

        if l > 0 {
            let v = self.items.remove(0);
            Some(v)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;
    #[test]
    fn test_new() {
        let queue = Queue::<u16>::new();
        assert_eq!(queue.is_empty(), true);
    }
    #[test]
    fn test_add() {
        let mut queue = Queue::<u16>::new();
        queue.add(1);
        assert_eq!(queue.is_empty(), false);
    }
}
