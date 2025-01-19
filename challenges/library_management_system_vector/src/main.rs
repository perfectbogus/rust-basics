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

    fn clear_category(&mut self, category: &str) {
        self.books.retain(|book| book.category != category);
    }

    fn get_book(&self, index: usize) -> Option<&Book> {
        if index >= self.books.len() {
            None
        } else {
            self.books.get(index)
        }
    }

    fn get_mut_book(&mut self, index: usize) -> Option<&mut Book> {
        if index >= self.books.len() {
            None
        } else {
            self.books.get_mut(index)
        }
    }

    fn first_book(&self) -> Option<&Book> {
        self.books.first()
    }

    fn last_book(&self) -> Option<&Book> {
        self.books.last()
    }

    // Capacity Management
    fn reserve_additional(&mut self, additional: usize) {
        self.books.reserve(additional);
    }

    fn shrink_capacity(&mut self) {

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

    #[test]
    fn test_insert_at(index: usize, book: Book) {
        let mut library = Library::new();
        library.add_book(create_test_book(1));

        // Insert at valid index
        assert!(library.insert_book_at(0, create_test_book(2)).is_ok());

        // Insert at invalid index
        assert!(library.insert_book_at(5, create_test_book(3)).is_err());
    }

    #[test]
    fn test_capacity_management() {
        let mut library = Library::with_capacity(10);
        assert!(library.books.capacity() >= 10);

        library.add_book(create_test_book(1));
        library.shink_capacity();
        assert!(library.books.capacity() >= 1);
    }

    #[test]
    fn test_sorting() {
        let mut library = Library::new();
        library.add_book(create_test_book(3));
        library.add_book(create_test_book(1));
        library.add_book(create_test_book(2));

        library.sort_by_id();
        assert_eq!(library.books[0].id, 1);

        library.reverse_order();
        assert_eq!(library.books[0].id, 3);
    }

    #[test]
    fn test_searching() {
        let mut library = Library::new();
        library.add_book(Book {
            id: 1,
            title: String::from("Test"),
            category: String::from("Fiction"),
        });

        assert!(library.find_by_id(1).is_some());
        assert_eq!(library.books_in_category("Fiction").len(), 1);
    }


}


fn main() {
    println!("Hello, world!");
}


































