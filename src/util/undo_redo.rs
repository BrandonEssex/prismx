#[derive(Debug, Clone)]
pub struct History<T> {
    pub past: Vec<T>,
    pub present: T,
    pub future: Vec<T>,
}

impl<T: Clone> History<T> {
    pub fn new(initial: T) -> Self {
        Self {
            past: Vec::new(),
            present: initial,
            future: Vec::new(),
        }
    }

    pub fn undo(&mut self) {
        if let Some(previous) = self.past.pop() {
            self.future.push(self.present.clone());
            self.present = previous;
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.future.pop() {
            self.past.push(self.present.clone());
            self.present = next;
        }
    }

    pub fn push(&mut self, new_state: T) {
        self.past.push(self.present.clone());
        self.present = new_state;
        self.future.clear();
    }
}