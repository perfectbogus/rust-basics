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


}


fn main() {
    println!("Hello, world!");
}


































