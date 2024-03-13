mod utils;

use rand::seq::SliceRandom;
// use rand::prelude::SliceRandom;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-todo-wasm!");
}

// Generate Random Integer Array 6 length Function
#[wasm_bindgen]
pub fn random_array() -> Vec<i32> {
    let mut array = Vec::new();
    for _ in 0..6 {
        array.push(rand::random::<i32>());
    }
    array
}

// A function that takes a "random_array()" function and sorts the resulting values in ascending order
#[wasm_bindgen]
pub fn sort_array(array: Vec<i32>) -> Vec<i32> {
    let mut sorted_array = array.clone();
    sorted_array.sort();
    sorted_array
}

// Creates a deck of poker cards
#[wasm_bindgen]
pub fn create_deck() -> Vec<String> {
    let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
    let ranks = ["Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King"];
    let mut deck = Vec::new();
    for suit in suits {
        for rank in ranks {
            deck.push(format!("{} of {}", rank, suit));
        }
    }
    deck
}

// Shuffles a deck of poker cards
#[wasm_bindgen]
pub fn shuffle_deck(deck: Vec<String>) -> Vec<String> {
    let mut shuffled_deck = deck.clone();
    let mut rng = rand::thread_rng();
    shuffled_deck.shuffle(&mut rng);
    shuffled_deck
}

// Draws a card from a shuffled deck of poker cards
#[wasm_bindgen]
pub fn draw_card(mut deck: Vec<String>) -> String {
    let mut drawn_card = String::new();
    if !deck.is_empty() {
        drawn_card = deck.pop().unwrap();
    }
    drawn_card
}

// #[wasm_bindgen]
// pub fn draw_card(mut shuffled_deck: Vec<String>) -> String {
//     let mut drawn_card = shuffled_deck.pop().unwrap();
//     drawn_card.push_str(" drawn from shuffled deck");
//     drawn_card
// }