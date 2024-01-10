pub struct WordCounter {
    text: String,
}

impl WordCounter {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn count(&self) -> usize {
        self.text.split_whitespace().count()
    }
}
