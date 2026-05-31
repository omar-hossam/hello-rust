use chrono::{Local, DateTime};

#[derive(Debug)]
enum Category { // the only allowed categories
    None,
    Shooting,
    Adventure,
    Strategy,
}

#[derive(PartialEq)]
enum SortType {
    Default,
    Category, 
    NewestDate,
    OldestDate,
    HighestScore,
    LowestScore,
}

struct Game {
    name: String,
    score: u32,
    completed: bool,
    category: Category,
    added_date: DateTime<Local>, // date with time format
}

struct GameLibrary {
    games: Vec<Game>, // a vector contains items following the struct Game
}


fn print_game(game: &Game) {
    println!("\nName: {} | Score: {} | Category: {:?}", game.name, game.score, game.category);
    println!("Added Date: {} | Completed? {}", game.added_date.format("%Y-%m-%d %H:%M:%S"), game.completed);
}

impl GameLibrary {
    fn new() -> Self {
        GameLibrary { games: Vec::new() }
    }
    
    fn add_game(&mut self, game: Game) {
        self.games.push(game)
    }
    
    fn complete_game_by_name(&mut self, name: String, new_score: u32) -> Result<(), &str> { // return Ok(()) if found, Err message if not found
        let game_index = self.games.iter().position(|g| g.name == name);
        
        match game_index {
            Some(i) => {
                self.games[i].completed = true;
                self.games[i].score += new_score;
                Ok(())
            },
            None => Err("Not found")
        }
    }
    
    fn show_all(&self, sort: SortType, category: Category) { // sort is optional, if passed use it, if not make it show all by Default as they are
        if sort == SortType::Default {
            for game in &self.games {
                print_game(&game)
            }
        /*} else if sort == SortType::Category {
            let category_games: Vec<&Game> = self.games.iter().filter(|game| matches!(game.category, category)).collect();
            
            for game in &category_games {
                print_game(&game)
            }*/
        } else if sort == SortType::NewestDate {
            
        } else if sort == SortType::OldestDate {
            
        } else if sort == SortType::HighestScore {
            
        } else if sort == SortType::LowestScore {
            
        }
    }
}

fn main() {
    let mut library = GameLibrary::new();
    
    library.add_game( Game {
        name: String::from("Stronghold Crusder"),
        score: 0,
        completed: false,
        category: Category::Strategy,
        added_date: Local::now()
    });
    
    library.show_all(SortType::Default, Category::None);
}
