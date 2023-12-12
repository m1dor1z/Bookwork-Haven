use serde::{Deserialize, Serialize};
use std::{
    sync::Mutex,
    collections::HashMap
};


#[derive(Deserialize, Serialize)]
pub struct AppState {
    pub books_entries: Mutex<BooksEntries>
}


#[derive(Deserialize, Serialize)]
pub struct Books {
    pub title: String,
    pub isbn: String,
    pub author: String,
    pub date: String,
}


#[derive(Deserialize, Serialize)]
pub struct BooksEntries {
    pub lists: HashMap<String, Books>
}

