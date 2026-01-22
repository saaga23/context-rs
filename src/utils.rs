// src/utils.rs

pub fn estimate_tokens(text: &str) -> usize {
    // A simple simulation of token counting logic
    text.len() / 4
}