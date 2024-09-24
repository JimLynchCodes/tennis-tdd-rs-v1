fn main() {
    println!("Hello, world!");
}

mod tennis_scoring;
// mod tennis_scoring_tests;

// pub use crate::tennis_scoring::TennisGame;

pub trait TennisGame {
    fn clear(&mut self);
    fn won_point(&mut self, player_name: &str);
    fn get_score(&self) -> String;
}
