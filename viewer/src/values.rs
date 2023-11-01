use egui::ahash::HashMap;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct Values {
    values: HashMap<String, VecDeque<f32>>,
    max_len: usize,
}

impl Default for Values {
    fn default() -> Self {
        Self::with_capacity(600)
    }
}

impl Values {
    pub fn with_capacity(max_len: usize) -> Self {
        Self {
            values: Default::default(),
            max_len,
        }
    }

    pub fn push(&mut self, key: String, value: f32) {
        let v = self
            .values
            .entry(key)
            .or_insert_with(|| VecDeque::with_capacity(self.max_len + 1));
        v.push_back(value);
        while v.len() > self.max_len {
            v.pop_front();
        }
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.values.keys()
    }

    pub fn get_last_value_for_key(&self, key: &str) -> Option<f32> {
        self.values
            .get(key)
            .as_ref()
            .and_then(|v| v.back())
            .cloned()
    }
}