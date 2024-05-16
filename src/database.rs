use std::collections::HashMap;

use crate::movie::Movie;

pub struct Database(HashMap<String, Movie>);

impl Database {
    pub fn new() -> Self {
        Self(HashMap::default())
    }

    pub fn lookup_by_id(&self, id: &str) -> Option<&Movie> {
        self.0.get(id)
    }

    pub fn insert(&mut self, movie: Movie) -> Result<(), String> {
        if self.0.contains_key(&movie.id) {
            return Err("movie already present".to_string())
        }

        self.0.insert(movie.id.clone(), movie);

        Ok(())
    }
}