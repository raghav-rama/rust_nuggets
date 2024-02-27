struct Book {
    title: String,
    author: String,
}

struct Game {
    name: String,
    platform: String,
}
trait Loggable {
    fn get_description(&self) -> String;
}

// Implement 'Loggable' for 'Book'
impl Loggable for Book {
    fn get_description(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

// Implement 'Loggable' for 'Game'
impl Loggable for Game {
    fn get_description(&self) -> String {
        format!("{} (Platform: {})", self.name, self.platform)
    }
}

fn log_item<T: Loggable>(item: &T) {
    println!("Logging item: {}", item.get_description());
}
fn main() {
    let book = Book {
        title: "The Hitchhiker's Guide to the Galaxy".to_string(),
        author: "Douglas Adams".to_string(),
    };
    let game = Game {
        name: "Super Metroid".to_string(),
        platform: "SNES".to_string(),
    };

    log_item(&book); // Output: "The Hitchhiker's Guide to the Galaxy by Douglas Adams"
    log_item(&game); // Output: "Super Metroid (Platform: SNES)"
}
