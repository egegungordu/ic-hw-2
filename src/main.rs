mod word_counter;

use std::io;

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

fn main() {
    println!("Enter some text:");
    let text = get_user_input();
    let counter = word_counter::WordCounter::new(text);
    println!("Word count: {}", counter.count());
}
