pub struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement Iterator for Fibonacci
impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

pub fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}