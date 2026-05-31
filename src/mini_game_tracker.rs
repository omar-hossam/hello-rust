/*
Task: Create a program that tracks game scores using Rust's ownership system.

Requirements:

Define a struct Game with fields:
    name: String
    score: u32
    completed: bool

Implement methods (not functions):

    new(name: String) -> Game — constructor

    complete_game(&mut self) — sets completed to true and adds 100 points

    display(&self) — prints game info

    compare_score(&self, other: &Game) -> &str — returns "higher", "lower", or "equal"

In main():

    Create 2 games using Game::new

    Complete one game using complete_game

    Display both games

    Compare their scores and print which is higher 
*/

struct Game {
    name: String,
    score: u32,
    completed: bool
}

impl Game {
    fn new(name: String) -> Self {
        Game {
            name, // name: name
            score: 0,
            completed: false
        }
    }
    
    fn complete_game(&mut self, score: u32) {
        self.completed = true;
        self.score += score;
    }
    
    fn display(&self) {
        println!("Game: {} | Score: {} | Completed {}", self.name, self.score, self.completed);
    }
    
    fn compare_score(&self, other: &Game) {
        if self.score > other.score { 
            println!("{} has HIGHER score than {}", self.name, other.name) 
        }
        else if self.score < other.score { 
            println!("{} has LOWER score than {}", self.name, other.name)
        } else { println!("{} has EQUAL score to {}", self.name, other.name) }
    }
}

fn main() {
    let mut game1 = Game::new(String::from("Cyberpunk 2077"));
    let mut game2 = Game::new(String::from("Stardew Valley"));
    
    game1.complete_game(75);
    game2.complete_game(98);
    
    game1.display();
    game2.display();
    
    game1.compare_score(&game2);
}
