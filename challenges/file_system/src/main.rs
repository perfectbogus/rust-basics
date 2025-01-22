#[derive(Debug)]
enum FSItem {
    File {
        name: String,
        size: usize,
    },
    Directory {
        name: String,
        contents: Box<Vec<FSItem>>,
    },
}

struct FileSystem {
    root: FSItem,
}

impl FSItem {
    fn new_file(name: String, size: usize) -> Self {
        // TODO: Create a new file
        FSItem::File { name, size }
    }

    fn new_directory(name: String) -> Self {
        // TODO: Create a new empty directory
        FSItem::Directory {
            name,
            contents: Box::new(Vec::new()),
        }
    }

    fn name(&self) -> &str {
        // TODO: Return name of file or directory
        match self {
            FSItem::File { name, .. } => name,
            FSItem::Directory { name, .. } => name,
        }
    }

    fn size(&self) -> usize {
        // TODO: Calculate total size
        // For file, return size
        // For directories, sum sizes of all contents recursively
        match self {
            FSItem::File { size, .. } => *size,
            FSItem::Directory { contents, .. } => {
                contents.as_ref().iter().map(|item| item.size()).sum()
            }
        }
    }

    fn add_item(&mut self, item: FSItem) -> Result<(), String> {
        // TODO: Add item to directory
        // Error if self is a file
        unimplemented!()
    }

    fn find(&self, name: &str) -> Option<&FSItem> {
        // TODO: Find item by name (non-recursively, only in current directory)
        unimplemented!()
    }

    fn find_recursive(&self, path: &str) -> Option<&FSItem> {
        // TODO: Find item by path (e.g., "documents/work/file.txt")
        unimplemented!()
    }

    fn delete(&mut self, name: &str) -> Result<FSItem, String> {
        // TODO: Remove and return item from directory
        // Error if self is a file or item not found
        unimplemented!()
    }
}

impl FileSystem {
    fn new() -> Self {
        // TODO: Create new filesystem with root directory
        unimplemented!()
    }

    fn add_path(&mut self, path: &str, item: FSItem) -> Result<(), String> {
        // TODO: Add item at specified path , creating parent directories if needed
        unimplemented!()
    }

    fn find(&self, path: &str) -> Option<&FSItem> {
        // TODO: Find item at specified path
        unimplemented!()
    }

    fn delete(&mut self, path: &str) -> Result<FSItem, String> {
        // TODO: Delete item at specified path
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_file() {
        let file = FSItem::new_file("test.txt".to_string(), 100);
        assert_eq!(file.name(), "test.txt");
        assert_eq!(file.size(), 100);
    }

    #[test]
    fn test_new_directory() {
        let dir = FSItem::new_directory("docs".to_string());
        assert_eq!(dir.name(), "docs");
        assert_eq!(dir.size(), 0);
    }

    #[test]
    fn test_add_item() {
        let mut dir = FSItem::new_directory("docs".to_string());
        let file = FSItem::new_file("test.txt".to_string(), 100);

        assert!(dir.add_item(file).is_ok());
        // Try to add to a file (should fail)
        let mut file2 = FSItem::new_file("file2.txt".to_string(), 100);
        assert!(file2
            .add_item(FSItem::new_file("test.txt".to_string(), 100))
            .is_err());
    }

    #[test]
    fn test_find() {
        let mut dir = FSItem::new_directory("docs".to_string());
        dir.add_item(FSItem::new_file("test.txt".to_string(), 100))
            .unwrap();

        assert!(dir.find("test.txt").is_some());
        assert!(dir.find("nonexistent.txt").is_none());
    }

    #[test]
    fn test_find_recursive() {
        let mut fs = FileSystem::new();
        fs.add_path(
            "/docs/work/file.txt",
            FSItem::new_file("file.txt".to_string(), 100),
        )
        .unwrap();

        assert!(fs.root.find_recursive("docs/work/file.txt").is_some());
        assert!(fs.root.find_recursive("nonexistent/path").is_none());
    }

    #[test]
    fn test_delete() {
        let mut dir = FSItem::new_directory("docs".to_string());
        dir.add_item(FSItem::new_file("test.txt".to_string(), 100))
            .unwrap();

        assert!(dir.delete("test.txt").is_ok());
        assert!(dir.find("test.txt").is_none());
    }

    #[test]
    fn test_filesystem_new() {
        let fs = FileSystem::new();
        match fs.root {
            FSItem::Directory { name, .. } => assert_eq!(name, "/"),
            _ => panic!("Root should be a directory"),
        }
    }

    #[test]
    fn test_filesystem_add_path() {
        let mut fs = FileSystem::new();
        let result = fs.add_path(
            "/docs/work/file.txt",
            FSItem::new_file("file.txt".to_string(), 100),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_filesystem_find() {
        let mut fs = FileSystem::new();
        fs.add_path(
            "/docs/file.txt",
            FSItem::new_file("file.txt".to_string(), 100),
        )
        .unwrap();

        assert!(fs.find("/docs/file.txt").is_some());
        assert!(fs.find("/nonexistent").is_none());
    }

    #[test]
    fn test_filesystem_delete() {
        let mut fs = FileSystem::new();
        fs.add_path("/test.txt", FSItem::new_file("test.txt".to_string(), 100))
            .unwrap();

        assert!(fs.delete("/test.txt").is_ok());
        assert!(fs.find("/test.txt").is_none());
    }
}

fn main() {
    println!("Hello, world!");
}
