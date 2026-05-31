// This code explains the difference between moving the ownership of a value via passing to a new function AND borrowing the variable's value so we can use it while keeping the variable in it's scope without taking it's ownership or moving it.  


fn move_ownership(vec: Vec<String>) {
    for item in &vec {
        println!("{}", item);
    }
}

fn borrow_vec_value(vec: &Vec<String>) {
    for item in vec {
        println!("{}", item);
    }
}

fn main() {
    let games = vec![
        String::from("Elden Ring"),
        String::from("Avatar"),
        String::from("Need for speed")
    ];
    
    
    println!("Borrow\n------");
    borrow_vec_value(&games);
    
    
    println!("is `games` still here?");
    println!("{} -> yes it is :)", games[0]); // this work!
    
    println!("----\nMove\n----");
    move_ownership(games);
    // try to println!(games) -> you will get an error because it's not here anymore, it's moved to the function and cannot be used here anymore
    
    println!("is `games` still here?");
    println!("no it isn't here anymore :(");
    // println!("{}", games[0]); // WON't work, games is gone from here!
}
