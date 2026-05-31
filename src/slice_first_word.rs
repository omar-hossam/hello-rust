fn first_word(my_str: &str) -> &str { 
    for (i, item) in my_str.char_indices() {
        if item == ' ' {
            return &my_str[0..i]; // slicing the input, not new data!
        }
    }
    &my_str
}

fn main() {
    let my_str = String::from("HelloWorld!");
    
    println!("first word is: {}", first_word(&my_str));
    
    let hardcoded = "Learning Rust daily";
    println!("First word: {}", first_word(hardcoded));
}
