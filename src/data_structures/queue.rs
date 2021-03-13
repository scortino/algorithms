pub struct Queue {
    head: usize,
    tail: usize,
    items: Vec<i32>,
    capacity: usize,
}

impl Default for Queue {
    fn default() -> Self {
        Self::new(128)
    }
}

#[allow(dead_code)]
impl Queue {
    pub fn new(capacity: usize) -> Self {
        Self {
            head: 0,
            tail: 0,
            items: vec![0; capacity + 1], // one more than capacity to be able to tell empty from full
            capacity,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        if self.tail >= self.head {
            self.tail - self.head
        } else {
            self.capacity() + self.tail - self.head
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    pub fn is_full(&self) -> bool {
        self.head == (self.tail + 1) % self.items.len()
    }

    pub fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("queue overflow");
        }
        self.items[self.tail] = x;
        self.tail = (self.tail + 1) % self.items.len()
    }

    pub fn dequeue(&mut self) -> i32 {
        if self.is_empty() {
            panic!("queue underflow");
        }
        let x = self.items[self.head];
        self.head = (self.head + 1) % self.items.len();
        x
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default() {
        let q = Queue::default();
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn enqueue_dequeue() {
        let mut q = Queue::new(4);
        q.enqueue(1);
        q.enqueue(3);
        q.enqueue(5);
        q.enqueue(4);
        assert!(q.is_full());
        assert_eq!(q.len(), 4);
        assert_eq!(q.dequeue(), 1);
        assert_eq!(q.dequeue(), 3);
        assert_eq!(q.len(), 2);
        assert_eq!(q.dequeue(), 5);
        assert_eq!(q.dequeue(), 4);
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }
}
