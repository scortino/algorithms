pub struct Stack {
    top: usize,
    items: Vec<i32>,
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl Stack {
    pub fn new() -> Self {
        Stack {
            top: 0,
            items: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.top
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn push(&mut self, x: i32) {
        self.top += 1;
        self.items.push(x);
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            panic!("stack underflow")
        }
        self.top -= 1;
        self.items.pop()
    }

    pub fn from_vector(v: Vec<i32>) -> Self {
        Self {
            top: v.len(),
            items: v,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let s = Stack::new();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn from_vector() {
        let v = vec![1, 3, 5, 4];
        let s = Stack::from_vector(v);
        assert_eq!(s.len(), 4);
    }

    #[test]
    fn push() {
        let mut s = Stack::new();
        assert!(s.is_empty());
        s.push(1);
        s.push(3);
        s.push(5);
        s.push(4);
        assert_eq!(s.len(), 4);
    }

    #[test]
    fn pop() {
        let v = vec![1, 3, 5, 4];
        let mut s = Stack::from_vector(v);
        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.pop(), Some(5));
        assert_eq!(s.len(), 2);
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(1));
        assert!(s.is_empty());
    }
}
