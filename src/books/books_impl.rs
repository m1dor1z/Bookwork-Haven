use super::models::{Books, BooksEntries};
use std::collections::HashMap;

impl BooksEntries {
    pub fn new() -> Self {
        BooksEntries { lists: HashMap::new() }
    }
    pub fn insert(
        &mut self, 
        title: String, 
        isbn: String, 
        author: String, 
        date: String
    ) -> Option<Books> {
        let res = self.lists.insert(isbn.clone(), Books::new(title, isbn, author, date));
        res
    }
}

impl Books {
    pub fn new( title: String, isbn: String, author: String, date: String) -> Self {
        Books { 
            title,
            isbn,
            author,
            date
        }
    }
}
