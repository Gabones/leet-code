pub struct MinStack {
    stack: Vec<i64>,
    min: i64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    pub fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min: 0,
        }
    }

    pub fn push(&mut self, val: i32) {
        let val = val as i64;
        if self.stack.is_empty() {
            self.stack.push(0);
            self.min = val.into();
        } else {
            self.stack.push(val - self.min);
            if val < self.min {
                self.min = val;
            }
        }
    }

    pub fn pop(&mut self) {
        if !self.stack.is_empty() {
            let pop = self.stack.last().copied().unwrap();
            self.stack.pop();
            if pop < 0 {
                self.min -= pop;
            }
        }
    }

    pub fn top(&self) -> i32 {
        let top = *self.stack.last().unwrap();
        if top > 0 {
            return (top + self.min) as i32;
        } else {
            return self.min as i32;
        }
    }

    pub fn get_min(&self) -> i32 {
        self.min as i32
    }
}
