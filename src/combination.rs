use std::fmt::Debug;

pub struct Combinations<T>
where
    T: Clone
{
    set: Vec<T>,
    k: usize,
    stack: Vec<usize>,
    selected: Vec<T>
}

impl<T> Combinations<T>
where
    T: Clone
{
    pub fn new(set: Vec<T>, k: usize) -> Self {
        Combinations {
            set,
            k,
            stack: Vec::with_capacity(k),
            selected: Vec::with_capacity(k)
        }
    }
}

impl<T> Iterator for Combinations<T>
where
    T: Debug + Clone
{
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        // println!("n at start: {}", self.selected.len());
        let mut idx;

        loop {
            idx = match self.stack.pop() {
                Some(v) => v + 1,
                None => 0
            };
            self.selected.pop();

            let remaining = self.k - self.selected.len();
            let available = self.set.len() - idx;
            let stack_len = self.stack.len();

            if remaining > available && stack_len == 0 {
                return None
            }
            if idx < self.set.len() && available >= remaining {
                break
            }
        }

        while self.selected.len() < self.k {
            // TODO: try to remove clone
            self.selected.push(self.set[idx].clone());
            self.stack.push(idx);
            idx += 1;
        }
        return Some(self.selected.clone())
    }
}
