#[derive(Debug, Clone, PartialEq)]
struct Book {
    id: u32,
    title: String,
    category: String,
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    // Basic Operations
    fn new() -> Self {
        Self { books: Vec::new() }
    }

    fn with_capacity(size: usize) -> Self {
        Self { books: Vec::with_capacity(size) }
    }

    // Adding Elements
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn add_multiple_books(&mut self, mut new_books: Vec<Book>) {
        self.books.append(&mut new_books);
    }

    fn insert_book_at(&mut self, index: usize, book: Book) -> Result<(), String>{
        if index >= self.books.len() {
            Err(String::from("index out of bounds"))
        } else {
            self.books.insert(index, book);
            Ok(())
        }
    }

    fn remove_book_by_id(&mut self, id: u32) -> Option<Book> {
        for (i, book) in self.books.iter().enumerate() {
            if book.id == id {
                Some(self.books.remove(i));
            }
        }
        None
    }

    fn remove_last(&mut self) -> Option<Book> {
        self.books.pop()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_book(id: u32) -> Book {
        Book {
            id,
            title: format!("Books {}", id),
            category: String::from("Test")
        }
    }

    #[test]
    fn test_add_and_remove() {
        let mut library = Library::new();

        // Test adding
        library.add_book(create_test_book(1));
        assert_eq!(library.books.len(), 1);

        // Test removing
        let removed = library.remove_book_by_id(1);
        assert!(removed.is_some());
        assert_eq!(library.books.len(), 0);
    }
}


fn main() {
    println!("Hello, world!");
}


































