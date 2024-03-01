#[cfg(test)]
mod traits_in_action_test {
    use super::{book::Book, impl_me::Loggable};
    #[test]
    fn it_constructs_book_properly() {
        let book = Book::new("Family Problems: The Randi Rona", "Raman Raghav");
        assert_eq!(book.title, "Family Problems: The Randi Rona");
        assert_eq!(book.author, "Raman Raghav");
    }
    #[test]
    fn it_gets_description_properly() {
        let book = Book::new("Family Problems: The Randi Rona", "Raman Raghav");
        let description = book.get_description();
        assert_eq!(
            description,
            "Family Problems: The Randi Rona by Raman Raghav"
        );
    }
}

pub mod book {
    pub struct Book {
        pub title: String,
        pub author: String,
    }
}

#[allow(dead_code)]
pub mod book_impl {
    use super::book::Book;
    impl Book {
        pub fn new(title: &str, author: &str) -> Self {
            Self {
                title: title.to_string(),
                author: author.to_string(),
            }
        }
    }
}

pub mod book_loggable_impl {
    use super::book::Book;
    use super::impl_me::Loggable;

    impl Loggable for Book {
        fn get_description(&self) -> String {
            format!("{} by {}", self.title, self.author)
        }
    }
}

pub mod impl_me {
    pub trait Loggable {
        fn get_description(&self) -> String;
    }
}
