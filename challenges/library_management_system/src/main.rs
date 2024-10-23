use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    available: bool,
}

#[derive(Debug)]
struct Library {
    books: Vec<Book>,
    borrowed_books: Vec<(String, String)>
}

#[derive(Debug)]
enum ErrorBook {
    AlreadyBorrowed(String),
    NotFound(String),
    IsNotBorrowed(String),
}

impl Error for ErrorBook {

}

impl Display for ErrorBook {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorBook::AlreadyBorrowed(title) => writeln!(f, "Book {} already borrowed", title),
            ErrorBook::NotFound(title) => writeln!(f, "Book title {} not found", title),
            ErrorBook::IsNotBorrowed(title) => writeln!(f, "Book title {} is not borrowed", title),
        }
    }
}

impl Book {
    fn new(title: String, author: String) -> Self {
        Self {
            title,
            author,
            available: true,
        }
    }

    fn get_info(&self) -> String {
        format!("Title: {}, Author: {}, Available: {}", self.title, self.author, self.available)
    }
}

impl Library {
    fn new() -> Self {
        Self {
            books: Vec::new(),
            borrowed_books: Vec::new(),
        }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book)
    }

    fn find_book(&self, title: &str) -> Option<&Book> {
        for book in &self.books {
            if book.title == title {
                return Some(book);
            }
        }
        None
    }

    fn borrow_book(&mut self, title: &str, borrower: &str) -> Result<(), ErrorBook>{
        for book in &mut self.books {
            if book.title == title {
                return if book.available {
                    book.available = false;
                    self.borrowed_books.push((book.title.clone(), borrower.to_string()));
                    Ok(())
                } else {
                    Err(ErrorBook::AlreadyBorrowed(book.title.clone()))
                }
            }
        }
        Err(ErrorBook::NotFound(title.to_string()))
    }

    fn return_book(&mut self, title: &str) -> Result<(), ErrorBook> {
        for book in &mut self.books {
            if book.title == title {
                return if !book.available {
                    book.available = true;
                    self.borrowed_books.retain(|(t, _)| t != title);
                    Ok(())
                } else {
                    Err(ErrorBook::IsNotBorrowed(title.to_string()))
                }
            }
        }
        Err(ErrorBook::NotFound(title.to_string()))
    }

    fn get_books_by_author(&self, author: &str) -> Vec<&Book> {
        let mut books_by_author = Vec::new();
        for book in &self.books {
            if book.author == author {
                books_by_author.push(book);
            }
        }
        books_by_author
    }

    fn get_statistics(&self) -> (usize, usize) {
        (self.books.len(), self.borrowed_books.len())
    }

}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_find_book() {
        let mut library = Library::new();
        let book = Book::new(
            "The Rust Programming Language".to_string(),
            "Steve Klabnik".to_string(),
        );
        library.add_book(book);

        let found = library.find_book("The Rust Programming Language");
        assert!(found.is_some());
    }

    #[test]
    fn test_borrow_and_return_book() {
        let mut library = Library::new();
        let book = Book::new(
            "The Rust Book".to_string(),
            "Steve Klabnik".to_string(),
        );
        library.add_book(book);

        assert!(library.borrow_book("The Rust Book", "Alice").is_ok());
        assert!(library.borrow_book("The Rust Book", "Bob").is_err());
        assert!(library.return_book("The Rust Book").is_ok());
        assert!(library.borrow_book("The Rust Book", "Bob").is_ok());
    }

    #[test]
    fn test_get_books_by_author() {
        let mut library = Library::new();
        library.add_book(Book::new(
            "Book 1".to_string(),
            "Author A".to_string(),
        ));
        library.add_book(Book::new(
            "Book 2".to_string(),
            "Author A".to_string(),
        ));
        library.add_book(Book::new(
            "Book 3".to_string(),
            "Author B".to_string(),
        ));

        let books = library.get_books_by_author("Author A");
        assert_eq!(books.len(), 2);
    }

    #[test]
    fn test_statistics() {
        let mut library = Library::new();
        library.add_book(Book::new(
            "Book 1".to_string(),
            "Author A".to_string(),
        ));
        library.add_book(Book::new(
            "Book 2".to_string(),
            "Author B".to_string(),
        ));

        library.borrow_book("Book 1", "Alice").unwrap();

        let (total, borrowed) = library.get_statistics();
        assert_eq!(total, 2);
        assert_eq!(borrowed, 1);
    }
}