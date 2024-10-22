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

impl Book {
    fn new(title: String, author: String) -> Self {
        unimplemented!()
    }

    fn get_info(&self) -> String {
        unimplemented!()
    }
}

impl Library {
    fn new() -> Self {
        unimplemented!()
    }

    fn add_book(&mut self, book: Book) {
        unimplemented!()
    }

    fn find_book(&self, title: &str) -> Option<&Book> {
        unimplemented!()
    }

    fn borrow_book(&mut self, title: &str, borrower: &str) -> Result<(), String>{
        unimplemented!()
    }

    fn return_book(&mut self, title: &str) -> Result<(), String> {
        unimplemented!()
    }

    fn get_books_by_author(&self, author: &str) -> Vec<&Book> {
        unimplemented!()
    }

    fn get_statistics(&self) -> (usize, usize) {
        unimplemented!()
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