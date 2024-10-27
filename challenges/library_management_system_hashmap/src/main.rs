use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Book {
    isbn: String,
    title: String,
    author: String,
    total_copies: u32,
    available_copies: u32,
}

#[derive(Debug)]
struct Library {
    // Primary storage by ISBN
    books_by_isbn: HashMap<String, Book>,
    // Index by title for quick title search
    books_by_title: HashMap<String, String>,  // title -> isbn
    // Index by author for quick author search
    books_by_author: HashMap<String, Vec<String>>,  // author -> Vec<isbn>
    // Track borrowed books
    borrowed_books: HashMap<String, Vec<String>>,  // user -> Vec<isbn>
}

impl Book {
    fn new(isbn: String, title: String, author: String, copies: u32) -> Self {
        Book {
            isbn,
            title,
            author,
            total_copies: copies,
            available_copies: copies,
        }
    }
}

impl Library {
    fn new() -> Self {
        // TODO: Initialize empty library
        Self {
            books_by_isbn: HashMap::new(),
            books_by_title: HashMap::new(),
            books_by_author: HashMap::new(),
            borrowed_books: HashMap::new(),
        }
    }

    fn add_book(&mut self, book: Book) -> Result<(), String> {
        // TODO: Add book to all indices
        // If ISBN exists, update copies instead of adding new book
        match self.books_by_isbn.entry(book.isbn.clone()) {
            Entry::Occupied(mut entry) => {
                let existing_book = entry.get_mut();
                existing_book.total_copies += book.total_copies;
                existing_book.available_copies += book.available_copies;
            }
            Entry::Vacant(entry) => {
                self.books_by_title.insert(book.title.clone(), book.isbn.clone());

                self.books_by_author
                    .entry(book.author.clone())
                    .or_default()
                    .push(book.isbn.clone());

                entry.insert(book);
            }
        }
        Ok(())
    }

    fn remove_book(&mut self, isbn: &str) -> Result<Book, String> {
        // TODO: Remove book from all indices
        // Error if book is currently borrowed
        if self.borrowed_books.values().any(|books| books.contains(&isbn.to_string())) {
            return Err("Cannot remove book: currently borrowed".to_string());
        }

        let book = self.books_by_isbn
            .remove(isbn)
            .ok_or_else(|| "Book does not exist".to_string())?;

        self.books_by_title.remove(&book.title);

        if let Some(author_books) = self.books_by_author.get_mut(&book.author) {
            author_books.retain(|book_isbn| book_isbn != isbn);
            if author_books.is_empty() {
                self.books_by_author.remove(&book.author);
            }
        }

        Ok(book)
    }

    fn get_by_isbn(&self, isbn: &str) -> Option<&Book> {
        // TODO: Retrieve book by ISBN
        unimplemented!()
    }

    fn find_by_title(&self, title: &str) -> Option<&Book> {
        // TODO: Find book by title
        unimplemented!()
    }

    fn find_by_author(&self, author: &str) -> Vec<&Book> {
        // TODO: Find all books by author
        unimplemented!()
    }

    fn borrow_book(&mut self, user: &str, isbn: &str) -> Result<(), String> {
        // TODO: Handle book borrowing
        // Check if book exists and is available
        // Update available copies
        // Update borrowed_books
        unimplemented!()
    }

    fn return_book(&mut self, user: &str, isbn: &str) -> Result<(), String> {
        // TODO: Handle book return
        // Check if user actually borrowed the book
        // Update available copies
        // Update borrowed_books
        unimplemented!()
    }

    fn get_available_books(&self) -> Vec<&Book> {
        // TODO: Get all books with available copies
        unimplemented!()
    }

    fn get_user_borrowed_books(&self, user: &str) -> Vec<&Book> {
        // TODO: Get all books borrowed by user
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_book() -> Book {
        Book::new(
            "123".to_string(),
            "Test Book".to_string(),
            "Test Author".to_string(),
            5,
        )
    }

    #[test]
    fn test_add_book() {
        let mut library = Library::new();
        let book = create_test_book();

        assert!(library.add_book(book.clone()).is_ok());
        assert_eq!(library.get_by_isbn("123").unwrap().title, "Test Book");

        // Adding same ISBN should update copies
        let mut book2 = book.clone();
        book2.total_copies = 3;
        assert!(library.add_book(book2).is_ok());
        assert_eq!(library.get_by_isbn("123").unwrap().total_copies, 8);
    }

    #[test]
    fn test_find_methods() {
        let mut library = Library::new();
        let book = create_test_book();
        library.add_book(book).unwrap();

        assert!(library.find_by_title("Test Book").is_some());
        assert_eq!(library.find_by_author("Test Author").len(), 1);
        assert!(library.find_by_title("Not Found").is_none());
    }

    #[test]
    fn test_borrow_and_return() {
        let mut library = Library::new();
        let book = create_test_book();
        library.add_book(book).unwrap();

        assert!(library.borrow_book("user1", "123").is_ok());
        assert_eq!(library.get_by_isbn("123").unwrap().available_copies, 4);

        assert!(library.return_book("user1", "123").is_ok());
        assert_eq!(library.get_by_isbn("123").unwrap().available_copies, 5);

        // Can't borrow more than available
        for _ in 0..5 {
            library.borrow_book("user2", "123").unwrap();
        }
        assert!(library.borrow_book("user3", "123").is_err());
    }

    #[test]
    fn test_user_borrowed_books() {
        let mut library = Library::new();
        let book = create_test_book();
        library.add_book(book).unwrap();

        library.borrow_book("user1", "123").unwrap();
        assert_eq!(library.get_user_borrowed_books("user1").len(), 1);
        assert_eq!(library.get_user_borrowed_books("user2").len(), 0);
    }
}

fn main() {
    println!("Hello, world!");
}
