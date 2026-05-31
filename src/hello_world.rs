fn main() {
    let first_name : &str = "Omar"; // references to a value BUT doesn't owning it!
    let last_name = String::from("Hossam"); // OWNS this value ("Hossam")!
    
    let ref1 = &last_name; // reference to the value of last_name
    
    let age: i32 = 16; // i32 shorter to int32!
    let loves_rust = true;
    
    let full_name = format!("{} {}, {} years old.. does he love rust? {}", first_name, ref1, age, loves_rust);
    
    println!("Let's introduce our friend!"); // must end with a semicolon!
    println!("{}", full_name); // must end with a semicolon!
}
