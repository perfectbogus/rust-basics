use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Movie {
    id: u32,
    title: String,
    genres: Vec<String>,
    rating: f64,
    year: u32
}

#[derive(Debug)]
struct MovieSystem {
    movies: HashMap<u32, Movie>,
    user_ratings: HashMap<String, HashMap<u32, u8>>,  // user -> (movie_id -> rating)
    genre_index: HashMap<String, Vec<u32>>,  // genre -> movie_ids
}

impl Movie {
    fn new(id: u32, title: String, genres: Vec<String>, year: u32) -> Self {
        Movie {
            id,
            title,
            genres,
            rating: 0.0,
            year,
        }
    }
}

impl MovieSystem {
    fn new() -> Self {
        // TODO: Initialize empty system
        Self { movies: HashMap::new(), user_ratings: HashMap::new(), genre_index: HashMap::new() }
    }

    fn add_movie(&mut self, movie: Movie) -> Result<(), String> {
        // TODO: Add movie to system and update genre index
        // Return error if movie ID already exists
        if self.movies.contains_key(&movie.id) {
           return Err("Movie ID already exists".to_string());
        }

        movie.genres.iter()
            .for_each(|genre| {
                self.genre_index
                    .entry(genre.clone())
                    .or_default()
                    .push(movie.id)
            });

        self.movies.insert(movie.id, movie);
        Ok(())
    }

    fn add_user_rating(&mut self, user: &str, movie_id: u32, rating: u8) -> Result<(), String> {
        // TODO: Add or update user's rating for a movie
        // Rating should be between 1 and 5
        // Update movie's average rating
        if rating < 1 || rating > 5 {
            return Err("Rating must be between 1 and 5".to_string());
        }

        if !self.movies.contains_key(&movie_id) {
            return Err("Movie does not exit".to_string());
        }

        self.user_ratings
            .entry(user.to_string())
            .or_default()
            .insert(movie_id, rating);

        if let Some(movie) = self.movies.get_mut(&movie_id) {
            let ratings : Vec<u8> = self.user_ratings
                .values()
                .filter_map(|ratings| ratings.get(&movie_id))
                .copied()
                .collect();

            movie.rating = ratings.iter()
                .map(|&r| r as f64)
                .sum::<f64>() / ratings.len() as f64;
        }

        Ok(())
    }

    fn get_movies_by_genre(&self, genre: &str) -> Vec<&Movie> {
        // TODO: Return all movies of a specific genre, sorted by rating
        unimplemented!()
    }

    fn get_user_recommendations(&self, user: &str) -> Vec<&Movie> {
        // TODO: Return movie recommendations for user based on:
        // 1. Movies they haven't rated
        // 2. From genres they've rated highly (4 or 5)
        // 3. Sorted by rating
        unimplemented!()
    }

    fn get_similar_movies(&self, movie_id: u32) -> Result<Vec<&Movie>, String> {
        // TODO: Return movies that share genres with the given movie
        // Sort by number of shared genres and then by rating
        unimplemented!()
    }

    fn get_top_movies_by_year(&self, year: u32, limit: usize) -> Vec<&Movie> {
        // TODO: Return top rated movies for a specific year
        // Limited to 'limit' number of results
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_movie(id: u32, title: &str, genres: Vec<&str>, year: u32) -> Movie {
        Movie::new(
            id,
            title.to_string(),
            genres.into_iter().map(String::from).collect(),
            year,
        )
    }

    #[test]
    fn test_add_movie() {
        let mut system = MovieSystem::new();
        let movie = create_test_movie(1, "Test Movie", vec!["Action"], 2020);

        // Test successful addition
        assert!(system.add_movie(movie.clone()).is_ok());

        // Test duplicate ID
        assert!(system.add_movie(movie).is_err());

        // Verify genre index
        assert_eq!(system.genre_index.get("Action").unwrap().len(), 1);
    }

    #[test]
    fn test_add_user_rating() {
        let mut system = MovieSystem::new();
        let movie = create_test_movie(1, "Test Movie", vec!["Action"], 2020);
        system.add_movie(movie).unwrap();

        // Test valid rating
        assert!(system.add_user_rating("user1", 1, 5).is_ok());

        // Test invalid rating
        assert!(system.add_user_rating("user1", 1, 6).is_err());

        // Test rating non-existent movie
        assert!(system.add_user_rating("user1", 999, 5).is_err());

        // Verify movie rating was updated
        assert!(system.movies.get(&1).unwrap().rating > 0.0);
    }

    #[test]
    fn test_get_movies_by_genre() {
        let mut system = MovieSystem::new();

        // Add test movies
        system.add_movie(create_test_movie(1, "Movie 1", vec!["Action"], 2020)).unwrap();
        system.add_movie(create_test_movie(2, "Movie 2", vec!["Action"], 2020)).unwrap();
        system.add_movie(create_test_movie(3, "Movie 3", vec!["Comedy"], 2020)).unwrap();

        // Rate movies
        system.add_user_rating("user1", 1, 5).unwrap();
        system.add_user_rating("user1", 2, 3).unwrap();

        // Get action movies
        let action_movies = system.get_movies_by_genre("Action");
        assert_eq!(action_movies.len(), 2);
        // Verify sorting by rating
        assert!(action_movies[0].rating > action_movies[1].rating);
    }

    #[test]
    fn test_get_recommendations() {
        let mut system = MovieSystem::new();

        // Add test movies
        system.add_movie(create_test_movie(1, "Action 1", vec!["Action"], 2020)).unwrap();
        system.add_movie(create_test_movie(2, "Action 2", vec!["Action"], 2020)).unwrap();
        system.add_movie(create_test_movie(3, "Comedy 1", vec!["Comedy"], 2020)).unwrap();

        // Add ratings
        system.add_user_rating("user1", 1, 5).unwrap();  // Likes action

        // Get recommendations
        let recommendations = system.get_user_recommendations("user1");
        assert!(!recommendations.is_empty());
        // Should recommend Action 2 before Comedy 1
        assert_eq!(recommendations[0].id, 2);
    }

    // Add more test cases for other functions...
}

fn main() {
    println!("Hello, world!");
}
